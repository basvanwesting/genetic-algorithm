//! A solution strategy for finding the best chromosome using evolution
mod builder;
pub mod prelude;
mod reporter;

pub use self::builder::{
    Builder as EvolveBuilder, TryFromBuilderError as TryFromEvolveBuilderError,
};

use super::{
    Strategy, StrategyAction, StrategyConfig, StrategyReporter, StrategyReporterNoop,
    StrategyState, StrategyVariant,
};
use crate::chromosome::{Chromosome, Genes};
use crate::crossover::Crossover;
use crate::extension::{Extension, ExtensionNoop};
use crate::fitness::{Fitness, FitnessCache, FitnessOrdering, FitnessValue};
use crate::genotype::{EvolveGenotype, MutationType};
use crate::mutate::Mutate;
use crate::population::Population;
use crate::select::Select;
use rand::rngs::SmallRng;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::time::{Duration, Instant};
use thread_local::ThreadLocal;

pub use self::reporter::Simple as EvolveReporterSimple;
pub use crate::strategy::reporter::Duration as EvolveReporterDuration;
pub use crate::strategy::reporter::Noop as EvolveReporterNoop;

#[derive(Copy, Clone, Debug, Default)]
pub enum EvolveVariant {
    #[default]
    Standard,
}

/// The Evolve strategy initializes with a random population of chromosomes (unless the genotype
/// seeds specific genes to sample from), calculates [fitness](crate::fitness) for all chromosomes
/// and sets a first best chromosome (if any).
///
/// Then the Evolve strategy runs through generations of chromosomes in a loop:
/// * [select](crate::select) and pair up chromosomes for crossover
/// * [extension](crate::extension) an optional step triggering on population cardinality after selection (e.g. [MassExtinction](crate::extension::ExtensionMassExtinction))
/// * [crossover](crate::crossover) to produce new offspring with a mix of parents chromosome.
/// * [mutate](crate::mutate) the offspring chromosomes to add some additional diversity
/// * calculate [fitness](crate::fitness) for all chromosomes
/// * store best chromosome and check ending conditions
///
/// The ending conditions are one or more of the following:
/// * target_fitness_score: when the ultimate goal in terms of fitness score is known and reached
/// * max_stale_generations: when the ultimate goal in terms of fitness score is unknown and one depends on some convergion
///   threshold, or one wants a duration limitation next to the target_fitness_score
/// * max_generations: when the ultimate goal in terms of fitness score is unknown and there is a effort constraint
///
/// General Hyper-parameters:
/// * `replacement_rate` (selection): the target fraction of the population which exists of
///   children. Generational Replacement and Steady-State Replacement can both be
///   modelled with this parameter by setting it respectively to 1.0 and 0.2-0.8.
///   High values converge faster, but risk losing good solutions. Low values
///   convergence slower. If there is a shortage of population after the ideal
///   fraction, firstly remaining non-selected children and secondly remaining
///   non-selected parents will be used to fill the shortage to avoid population
///   collapse.
/// * `elitism_rate` (selection): a non-generational elite gate, which ensures passing of the
///   best chromosomes before selection and replacement takes place. Value should
///   typically be very low, between 0.01 and 0.05. Relevant for
///   `SelectTournament` where the best chromosome is not guaranteed to be
///   selected for a tournament if the `population_size` is larger than the
///   `target_population_size`
/// * `selection_rate` (crossover): the fraction of parents which are selected for
///   reproduction. This selection adds offspring to the population, the other
///   parents do not. The population now grows by the added offspring, as the
///   parents are not replaced yet. Value should typically be between 0.4 and
///   0.8. High values risk of premature convergence. Low values reduce diversity
///   if overused.
/// * `crossover_rate (or recombination-rate)` (crossover): the fraction of selected parents
///   to crossover, the remaining parents just clone as offspring. Value should
///   typically be between 0.5 and 0.8. High values converge faster, but risk
///   losing good solutions. Low values have poor exploration and risk of
///   premature convergence
/// * `mutation_probability` (mutation): the fraction of offspring which gets mutated.
///   Typically low, between 0.01 and 0.10. High values reduces convergence
///   ability. Low have a risk of stagnation.
///
///
/// There are optional mutation distance limitations for
/// [RangeGenotype](crate::genotype::RangeGenotype) and
/// [MultiRangeGenotype](crate::genotype::MultiRangeGenotype) chromosomes. Listed in descending
/// priority:
/// * With allele_mutation_scaled_range(s) set on genotype:
///     * Mutation distance only on edges of current scale (e.g. -1 and +1 for -1..-1 scale), pick random edge
///     * Scale down after max_stale_generations is reached and reset stale_generations to zero
///     * Only trigger max_stale_generations ending condition when already reached the smallest scale
/// * With allele_mutation_range(s) set on genotype:
///     * Mutation distance taken uniformly from mutation range
///     * Standard max_stale_generations ending condition
/// * With only allele_range(s) set on genotype:
///     * Mutate uniformly over the complete allele range
///     * Standard max_stale_generations ending condition
///
/// There are reporting hooks in the loop receiving the [EvolveState], which can by handled by an
/// [StrategyReporter] (e.g. [EvolveReporterDuration], [EvolveReporterSimple]). But you are encouraged to
/// roll your own, see [StrategyReporter].
///
/// For [Evolve] the reporting `on_new_generation` hook is called just after selection, because
/// that is a more interesting point in the loop.
///
/// From the [EvolveBuilder] level, there are several calling mechanisms:
/// * [call](EvolveBuilder::call): this runs a single evolve strategy
/// * [call_repeatedly](EvolveBuilder::call_repeatedly): this runs multiple independent evolve
///   strategies and returns the best one (or short circuits when the target_fitness_score is
///   reached)
/// * [call_par_repeatedly](EvolveBuilder::call_par_repeatedly): this runs multiple independent
///   evolve strategies in parallel and returns the best one (or short circuits when the
///   target_fitness_score is reached). This is separate and independent from the
///   `with_par_fitness()` flag on the builder, which determines multithreading of the fitness
///   calculation inside the evolve strategy. Both can be combined.
/// * [call_speciated](EvolveBuilder::call_speciated): this runs multiple independent
///   evolve strategies and then selects their best results against each other in one final evolve
///   strategy (or short circuits when the target_fitness_score is reached)
/// * [call_par_speciated](EvolveBuilder::call_par_speciated): this runs multiple independent
///   evolve strategies in parallel and then selects their best results against each other in one
///   final evolve strategy (or short circuits when the target_fitness_score is reached). This is
///   separate and independent from the `with_par_fitness()` flag on the builder, which determines
///   multithreading of the fitness calculation inside the evolve strategy. Both can be combined.
///
/// All multithreading mechanisms are implemented using [rayon::iter] and [std::sync::mpsc].
///
/// See [EvolveBuilder] for initialization options.
///
/// Example:
/// ```
/// use genetic_algorithm::strategy::evolve::prelude::*;
/// use genetic_algorithm::fitness::placeholders::CountTrue;
///
/// // the search space
/// let genotype = BinaryGenotype::builder() // boolean alleles
///     .with_genes_size(100)                // 100 genes per chromosome
///     .with_genes_hashing(true)            // store genes_hash on chromosome (required for fitness_cache and deduplication extension, optional for better population cardinality estimation)
///     .build()
///     .unwrap();
///
/// // the search strategy
/// let evolve = Evolve::builder()
///     .with_genotype(genotype)
///
///     .with_select(SelectElite::new(0.5, 0.02))               // sort the chromosomes by fitness to determine crossover order. Strive to replace 50% of the population with offspring. Allow 2% through the non-generational best chromosomes gate before selection and replacement
///     .with_extension(ExtensionMassExtinction::new(10, 0.1, 0.02)) // optional builder step, simulate cambrian explosion by mass extinction, when population cardinality drops to 10 after the selection, trim to 10% of population
///     .with_crossover(CrossoverUniform::new(0.7, 0.8))        // crossover all individual genes between 2 chromosomes for offspring with 70% parent selection (30% do not produce offspring) and 80% chance of crossover (20% of parents just clone)
///     .with_mutate(MutateSingleGene::new(0.2))                // mutate offspring for a single gene with a 20% probability per chromosome
///     .with_fitness(CountTrue)                                // count the number of true values in the chromosomes
///     .with_fitness_ordering(FitnessOrdering::Minimize)       // aim for the least true values
///     .with_fitness_cache(1000)                               // enable caching of fitness values (LRU size 1000), only works when genes_hash is stored in chromosome. Only useful for long stale runs, but better to increase population diversity
///     .with_par_fitness(true)                                 // optional, defaults to false, use parallel fitness calculation
///     .with_target_population_size(100)                       // evolve with 100 chromosomes
///     .with_target_fitness_score(0)                           // ending condition if 0 times true in the best chromosome
///     .with_valid_fitness_score(10)                           // block ending conditions until at most a 10 times true in the best chromosome
///     .with_max_stale_generations(1000)                       // stop searching if there is no improvement in fitness score for 1000 generations (per scaled_range)
///     .with_max_generations(1_000_000)                        // optional, stop searching after 1M generations
///     .with_max_chromosome_age(10)                            // kill chromosomes after 10 generations
///     .with_reporter(EvolveReporterSimple::new(100))          // optional builder step, report every 100 generations
///     .with_replace_on_equal_fitness(true)                    // optional, defaults to false, maybe useful to avoid repeatedly seeding with the same best chromosomes after mass extinction events
///     .with_rng_seed_from_u64(0)                              // for testing with deterministic results
///     .call()
///     .unwrap();
///
/// // it's all about the best genes after all
/// let (best_genes, best_fitness_score) = evolve.best_genes_and_fitness_score().unwrap();
/// assert_eq!(best_genes, vec![false; 100]);
/// assert_eq!(best_fitness_score, 0);
/// ```
pub struct Evolve<
    G: EvolveGenotype,
    M: Mutate<Genotype = G>,
    F: Fitness<Genotype = G>,
    S: Crossover<Genotype = G>,
    C: Select,
    E: Extension,
    SR: StrategyReporter<Genotype = G>,
> {
    pub genotype: G,
    pub fitness: F,
    pub plugins: EvolvePlugins<M, S, C, E>,
    pub config: EvolveConfig,
    pub state: EvolveState<G>,
    pub reporter: SR,
    pub rng: SmallRng,
}

pub struct EvolvePlugins<M: Mutate, S: Crossover, C: Select, E: Extension> {
    pub mutate: M,
    pub crossover: S,
    pub select: C,
    pub extension: E,
}

pub struct EvolveConfig {
    pub variant: EvolveVariant,
    pub fitness_ordering: FitnessOrdering,
    pub par_fitness: bool,
    pub replace_on_equal_fitness: bool,

    pub target_fitness_score: Option<FitnessValue>,
    pub max_stale_generations: Option<usize>,
    pub max_generations: Option<usize>,
    pub valid_fitness_score: Option<FitnessValue>,
    pub fitness_cache: Option<FitnessCache>,

    pub target_population_size: usize,
    pub max_chromosome_age: Option<usize>,
}

/// Stores the state of the Evolve strategy.
#[derive(Clone)]
pub struct EvolveState<G: EvolveGenotype> {
    pub current_iteration: usize,
    pub current_generation: usize,
    pub stale_generations: usize,
    pub scale_generation: usize,
    pub best_generation: usize,
    pub best_fitness_score: Option<FitnessValue>,
    pub durations: HashMap<StrategyAction, Duration>,
    pub chromosome: Option<Chromosome<G::Allele>>,
    pub population: Population<G::Allele>,
    pub current_scale_index: Option<usize>,
    pub population_cardinality: Option<usize>,
    pub best_chromosome: Option<Chromosome<G::Allele>>,
}

impl<
        G: EvolveGenotype,
        M: Mutate<Genotype = G>,
        F: Fitness<Genotype = G>,
        S: Crossover<Genotype = G>,
        C: Select,
        E: Extension,
        SR: StrategyReporter<Genotype = G>,
    > Strategy<G> for Evolve<G, M, F, S, C, E, SR>
{
    fn call(&mut self) {
        let now = Instant::now();
        self.reporter
            .on_enter(&self.genotype, &self.state, &self.config);
        let mut fitness_thread_local: Option<ThreadLocal<RefCell<F>>> = None;
        if self.config.par_fitness {
            fitness_thread_local = Some(ThreadLocal::new());
        }
        self.setup(fitness_thread_local.as_ref());

        self.reporter
            .on_start(&self.genotype, &self.state, &self.config);
        while !self.is_finished() {
            self.state.increment_generation();
            self.state
                .population_filter_age(&self.genotype, &self.config);

            self.plugins.select.call(
                &self.genotype,
                &mut self.state,
                &self.config,
                &mut self.reporter,
                &mut self.rng,
            );
            self.state
                .update_population_cardinality(&self.genotype, &self.config);
            self.reporter
                .on_new_generation(&self.genotype, &self.state, &self.config);

            self.plugins.extension.call(
                &self.genotype,
                &mut self.state,
                &self.config,
                &mut self.reporter,
                &mut self.rng,
            );

            self.state.population.increment_age();
            self.plugins.crossover.call(
                &self.genotype,
                &mut self.state,
                &self.config,
                &mut self.reporter,
                &mut self.rng,
            );
            self.plugins.mutate.call(
                &self.genotype,
                &mut self.state,
                &self.config,
                &mut self.reporter,
                &mut self.rng,
            );
            self.fitness.call_for_state_population(
                &self.genotype,
                &mut self.state,
                &self.config,
                fitness_thread_local.as_ref(),
            );
            self.state.update_best_chromosome_and_report(
                &self.genotype,
                &self.config,
                &mut self.reporter,
            );

            self.state.scale(&self.genotype, &self.config);
        }
        self.reporter
            .on_finish(&self.genotype, &self.state, &self.config);
        self.cleanup(fitness_thread_local.as_mut());
        self.state.close_duration(now.elapsed());
        self.reporter
            .on_exit(&self.genotype, &self.state, &self.config);
    }
    fn best_generation(&self) -> usize {
        self.state.best_generation
    }
    fn best_fitness_score(&self) -> Option<FitnessValue> {
        self.state.best_fitness_score()
    }
    fn best_genes(&self) -> Option<Genes<G::Allele>> {
        self.state
            .best_chromosome
            .as_ref()
            .map(|c| c.genes().clone())
    }
    fn flush_reporter(&mut self, output: &mut Vec<u8>) {
        self.reporter.flush(output);
    }
}
impl<
        G: EvolveGenotype,
        M: Mutate<Genotype = G>,
        F: Fitness<Genotype = G>,
        S: Crossover<Genotype = G>,
        C: Select,
        E: Extension,
        SR: StrategyReporter<Genotype = G>,
    > Evolve<G, M, F, S, C, E, SR>
{
    pub fn best_chromosome(&self) -> Option<Chromosome<G::Allele>> {
        if let Some(best_genes) = self.best_genes() {
            let mut chromosome = Chromosome::new(best_genes);
            chromosome.set_fitness_score(self.best_fitness_score());
            Some(chromosome)
        } else {
            None
        }
    }
}

impl<G: EvolveGenotype, M: Mutate<Genotype = G>, F: Fitness<Genotype = G>, S: Crossover<Genotype = G>, C: Select>
    Evolve<G, M, F, S, C, ExtensionNoop, StrategyReporterNoop<G>>
{
    pub fn builder() -> EvolveBuilder<G, M, F, S, C, ExtensionNoop, StrategyReporterNoop<G>> {
        EvolveBuilder::new()
    }
}

impl<
        G: EvolveGenotype,
        M: Mutate<Genotype = G>,
        F: Fitness<Genotype = G>,
        S: Crossover<Genotype = G>,
        C: Select,
        E: Extension,
        SR: StrategyReporter<Genotype = G>,
    > Evolve<G, M, F, S, C, E, SR>
{
    pub fn setup(&mut self, fitness_thread_local: Option<&ThreadLocal<RefCell<F>>>) {
        let now = Instant::now();
        self.state.population = self
            .genotype
            .population_constructor(self.config.target_population_size, &mut self.rng);
        self.state
            .add_duration(StrategyAction::SetupAndCleanup, now.elapsed());

        self.fitness.call_for_state_population(
            &self.genotype,
            &mut self.state,
            &self.config,
            fitness_thread_local,
        );
        self.state.update_best_chromosome_and_report(
            &self.genotype,
            &self.config,
            &mut self.reporter,
        );

        if self.state.best_fitness_score().is_none() {
            let chromosome = &self.state.population.chromosomes[0];
            self.state.best_generation = self.state.current_generation;
            self.state.best_chromosome = Some(chromosome.clone());
            self.reporter
                .on_new_best_chromosome(&self.genotype, &self.state, &self.config);
            self.state.reset_stale_generations();
        }
    }

    pub fn cleanup(&mut self, fitness_thread_local: Option<&mut ThreadLocal<RefCell<F>>>) {
        let now = Instant::now();
        self.state.chromosome.take();
        self.state.population.chromosomes.clear();
        if let Some(thread_local) = fitness_thread_local {
            thread_local.clear();
        }
        self.state
            .add_duration(StrategyAction::SetupAndCleanup, now.elapsed());
    }

    fn is_finished(&self) -> bool {
        self.allow_finished_by_valid_fitness_score()
            && (self.is_finished_by_max_stale_generations()
                || self.is_finished_by_max_generations()
                || self.is_finished_by_target_fitness_score())
    }

    fn is_finished_by_max_stale_generations(&self) -> bool {
        if let Some(max_stale_generations) = self.config.max_stale_generations {
            self.state.stale_generations >= max_stale_generations
        } else {
            false
        }
    }

    fn is_finished_by_max_generations(&self) -> bool {
        if let Some(max_generations) = self.config.max_generations {
            self.state.current_generation >= max_generations
        } else {
            false
        }
    }

    fn is_finished_by_target_fitness_score(&self) -> bool {
        if let Some(target_fitness_score) = self.config.target_fitness_score {
            if let Some(fitness_score) = self.best_fitness_score() {
                match self.config.fitness_ordering {
                    FitnessOrdering::Maximize => fitness_score >= target_fitness_score,
                    FitnessOrdering::Minimize => fitness_score <= target_fitness_score,
                }
            } else {
                false
            }
        } else {
            false
        }
    }

    fn allow_finished_by_valid_fitness_score(&self) -> bool {
        if let Some(valid_fitness_score) = self.config.valid_fitness_score {
            if let Some(fitness_score) = self.best_fitness_score() {
                match self.config.fitness_ordering {
                    FitnessOrdering::Maximize => fitness_score >= valid_fitness_score,
                    FitnessOrdering::Minimize => fitness_score <= valid_fitness_score,
                }
            } else {
                true
            }
        } else {
            true
        }
    }
}

impl StrategyConfig for EvolveConfig {
    fn fitness_ordering(&self) -> FitnessOrdering {
        self.fitness_ordering
    }
    fn fitness_cache(&self) -> Option<&FitnessCache> {
        self.fitness_cache.as_ref()
    }
    fn par_fitness(&self) -> bool {
        self.par_fitness
    }
    fn replace_on_equal_fitness(&self) -> bool {
        self.replace_on_equal_fitness
    }
    fn variant(&self) -> StrategyVariant {
        StrategyVariant::Evolve(self.variant)
    }
}

impl<G: EvolveGenotype> StrategyState<G> for EvolveState<G> {
    fn chromosome_as_ref(&self) -> &Option<Chromosome<G::Allele>> {
        &self.chromosome
    }
    fn population_as_ref(&self) -> &Population<G::Allele> {
        &self.population
    }
    fn chromosome_as_mut(&mut self) -> &mut Option<Chromosome<G::Allele>> {
        &mut self.chromosome
    }
    fn population_as_mut(&mut self) -> &mut Population<G::Allele> {
        &mut self.population
    }
    fn best_generation(&self) -> usize {
        self.best_generation
    }
    fn best_fitness_score(&self) -> Option<FitnessValue> {
        self.best_fitness_score
    }
    fn current_generation(&self) -> usize {
        self.current_generation
    }
    fn current_iteration(&self) -> usize {
        self.current_iteration
    }
    fn increment_generation(&mut self) {
        self.current_generation += 1;
        self.scale_generation += 1;
    }
    fn stale_generations(&self) -> usize {
        self.stale_generations
    }
    fn increment_stale_generations(&mut self) {
        self.stale_generations += 1;
    }
    fn reset_stale_generations(&mut self) {
        self.stale_generations = 0;
    }
    fn scale_generation(&self) -> usize {
        self.scale_generation
    }
    fn reset_scale_generation(&mut self) {
        self.scale_generation = 0;
    }
    fn current_scale_index(&self) -> Option<usize> {
        self.current_scale_index
    }
    fn population_cardinality(&self) -> Option<usize> {
        self.population_cardinality
    }
    fn durations(&self) -> &HashMap<StrategyAction, Duration> {
        &self.durations
    }
    fn add_duration(&mut self, action: StrategyAction, duration: Duration) {
        *self.durations.entry(action).or_default() += duration;
    }
    fn total_duration(&self) -> Duration {
        self.durations.values().sum()
    }
    fn best_genes(&self) -> Option<Genes<G::Allele>> {
        self.best_chromosome.as_ref().map(|c| c.genes().clone())
    }
}

impl<G: EvolveGenotype> EvolveState<G> {
    fn update_best_chromosome_and_report<SR: StrategyReporter<Genotype = G>>(
        &mut self,
        genotype: &G,
        config: &EvolveConfig,
        reporter: &mut SR,
    ) {
        let now = Instant::now();
        if let Some(contending_chromosome) =
            self.population.best_chromosome(config.fitness_ordering)
        {
            match self.is_better_chromosome(
                contending_chromosome,
                &config.fitness_ordering,
                config.replace_on_equal_fitness,
            ) {
                (true, true) => {
                    self.best_generation = self.current_generation;
                    self.best_fitness_score = contending_chromosome.fitness_score();
                    self.best_chromosome = Some(contending_chromosome.clone());
                    reporter.on_new_best_chromosome(genotype, self, config);
                    self.reset_stale_generations();
                }
                (true, false) => {
                    self.best_chromosome = Some(contending_chromosome.clone());
                    reporter.on_new_best_chromosome_equal_fitness(genotype, self, config);
                    self.increment_stale_generations();
                }
                _ => self.increment_stale_generations(),
            }
        } else {
            self.increment_stale_generations();
        }
        self.add_duration(StrategyAction::UpdateBestChromosome, now.elapsed());
    }
    fn scale(&mut self, genotype: &G, config: &EvolveConfig) {
        if let Some(current_scale_index) = self.current_scale_index {
            if let Some(max_stale_generations) = config.max_stale_generations {
                if let Some(max_scale_index) = genotype.max_scale_index() {
                    if self.stale_generations >= max_stale_generations
                        && current_scale_index < max_scale_index
                    {
                        self.current_scale_index = Some(current_scale_index + 1);
                        self.reset_scale_generation();
                        self.reset_stale_generations();
                    }
                }
            }
        }
    }

    fn population_filter_age(&mut self, _genotype: &G, config: &EvolveConfig) {
        if let Some(max_chromosome_age) = config.max_chromosome_age {
            // TODO: use something like partition_in_place when stable
            for i in (0..self.population.chromosomes.len()).rev() {
                if self.population.chromosomes[i].age() >= max_chromosome_age {
                    self.population.chromosomes.swap_remove(i);
                }
            }
        }
    }
    fn update_population_cardinality(&mut self, _genotype: &G, _config: &EvolveConfig) {
        // Note: genes_hashing() method not available in distributed genotype trait
        // self.population_cardinality = if genotype.genes_hashing() {
        //     self.population.genes_cardinality()
        // } else {
        self.population_cardinality = self.population.fitness_score_cardinality()
        // }
    }
}

impl<
        G: EvolveGenotype,
        M: Mutate<Genotype = G>,
        F: Fitness<Genotype = G>,
        S: Crossover<Genotype = G>,
        C: Select,
        E: Extension,
        SR: StrategyReporter<Genotype = G>,
    > TryFrom<EvolveBuilder<G, M, F, S, C, E, SR>> for Evolve<G, M, F, S, C, E, SR>
{
    type Error = TryFromEvolveBuilderError;

    fn try_from(builder: EvolveBuilder<G, M, F, S, C, E, SR>) -> Result<Self, Self::Error> {
        if builder.genotype.is_none() {
            Err(TryFromEvolveBuilderError(
                "Evolve requires a EvolveGenotype",
            ))
        } else if builder.fitness.is_none() {
            Err(TryFromEvolveBuilderError("Evolve requires a Fitness"))
        } else if builder.mutate.is_none() {
            Err(TryFromEvolveBuilderError(
                "Evolve requires a Mutate strategy",
            ))
        } else if builder.crossover.is_none() {
            Err(TryFromEvolveBuilderError(
                "Evolve requires a Crossover strategy",
            ))
        } else if builder.select.is_none() {
            Err(TryFromEvolveBuilderError(
                "Evolve requires a Select strategy",
            ))
        } else if builder
            .crossover
            .as_ref()
            .map(|o| o.require_crossover_indexes())
            .unwrap()
            && builder
                .genotype
                .as_ref()
                .map(|o| !o.has_crossover_indexes())
                .unwrap()
        {
            Err(TryFromEvolveBuilderError(
                "The provided Crossover strategy requires crossover_indexes, which the provided EvolveGenotype does not provide",
            ))
        } else if builder
            .crossover
            .as_ref()
            .map(|o| o.require_crossover_points())
            .unwrap()
            && builder
                .genotype
                .as_ref()
                .map(|o| !o.has_crossover_points())
                .unwrap()
        {
            Err(TryFromEvolveBuilderError(
                "The provided Crossover strategy requires crossover_points, which the provided EvolveGenotype does not provide",
            ))
        } else if builder.target_population_size == 0 {
            Err(TryFromEvolveBuilderError(
                "Evolve requires a target_population_size > 0",
            ))
        } else if builder.max_stale_generations.is_none()
            && builder.max_generations.is_none()
            && builder.target_fitness_score.is_none()
        {
            Err(TryFromEvolveBuilderError(
                "Evolve requires at least a max_stale_generations, max_generations or target_fitness_score ending condition",
            ))
        } else {
            let rng = builder.rng();
            let genotype = builder.genotype.unwrap();
            let state = EvolveState::new(&genotype);
            let target_population_size = builder.target_population_size;

            Ok(Self {
                genotype,
                fitness: builder.fitness.unwrap(),
                plugins: EvolvePlugins {
                    mutate: builder.mutate.unwrap(),
                    crossover: builder.crossover.unwrap(),
                    select: builder.select.unwrap(),
                    extension: builder.extension,
                },
                config: EvolveConfig {
                    target_population_size,
                    max_stale_generations: builder.max_stale_generations,
                    max_generations: builder.max_generations,
                    max_chromosome_age: builder.max_chromosome_age,
                    target_fitness_score: builder.target_fitness_score,
                    valid_fitness_score: builder.valid_fitness_score,
                    fitness_ordering: builder.fitness_ordering,
                    fitness_cache: builder.fitness_cache,
                    par_fitness: builder.par_fitness,
                    replace_on_equal_fitness: builder.replace_on_equal_fitness,
                    ..Default::default()
                },
                state,
                reporter: builder.reporter,
                rng,
            })
        }
    }
}

impl Default for EvolveConfig {
    fn default() -> Self {
        Self {
            variant: Default::default(),
            target_population_size: 0,
            max_stale_generations: None,
            max_generations: None,
            max_chromosome_age: None,
            target_fitness_score: None,
            valid_fitness_score: None,
            fitness_ordering: FitnessOrdering::Maximize,
            fitness_cache: None,
            par_fitness: false,
            replace_on_equal_fitness: false,
        }
    }
}
impl EvolveConfig {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<G: EvolveGenotype> EvolveState<G> {
    pub fn new(genotype: &G) -> Self {
        let base = Self {
            current_iteration: 0,
            current_generation: 0,
            stale_generations: 0,
            scale_generation: 0,
            current_scale_index: None,
            best_generation: 0,
            best_fitness_score: None,
            chromosome: None,
            population: Population::new_empty(),
            durations: HashMap::new(),
            population_cardinality: None,
            best_chromosome: None,
        };
        match genotype.mutation_type() {
            MutationType::Scaled => Self {
                current_scale_index: Some(0),
                ..base
            },
            MutationType::Relative => base,
            MutationType::Random => base,
        }
    }
}

impl<
        G: EvolveGenotype,
        M: Mutate<Genotype = G>,
        F: Fitness<Genotype = G>,
        S: Crossover<Genotype = G>,
        C: Select,
        E: Extension,
        SR: StrategyReporter<Genotype = G>,
    > fmt::Display for Evolve<G, M, F, S, C, E, SR>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "evolve:")?;
        writeln!(f, "  fitness: {:?}", self.fitness)?;
        writeln!(f)?;

        writeln!(f, "{}", self.plugins)?;
        writeln!(f, "{}", self.config)?;
        writeln!(f, "{}", self.state)?;
        writeln!(f, "{}", self.genotype)
    }
}

impl<M: Mutate, S: Crossover, C: Select, E: Extension> fmt::Display for EvolvePlugins<M, S, C, E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "evolve_plugins:")?;
        writeln!(f, "  mutate: {:?}", self.mutate)?;
        writeln!(f, "  crossover: {:?}", self.crossover)?;
        writeln!(f, "  select: {:?}", self.select)?;
        writeln!(f, "  extension: {:?}", self.extension)
    }
}

impl fmt::Display for EvolveConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "evolve_config:")?;
        writeln!(
            f,
            "  target_population_size: {}",
            self.target_population_size
        )?;
        writeln!(
            f,
            "  max_stale_generations: {:?}",
            self.max_stale_generations
        )?;
        writeln!(f, "  max_generations: {:?}", self.max_generations)?;
        writeln!(f, "  max_chromosome_age: {:?}", self.max_chromosome_age)?;
        writeln!(f, "  valid_fitness_score: {:?}", self.valid_fitness_score)?;
        writeln!(f, "  target_fitness_score: {:?}", self.target_fitness_score)?;
        writeln!(f, "  fitness_ordering: {:?}", self.fitness_ordering)?;
        writeln!(f, "  par_fitness: {:?}", self.par_fitness)
    }
}

impl<G: EvolveGenotype> fmt::Display for EvolveState<G> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "evolve_state:")?;
        writeln!(f, "  current iteration: {:?}", self.current_iteration)?;
        writeln!(f, "  current generation: {:?}", self.current_generation)?;
        writeln!(f, "  stale generations: {:?}", self.stale_generations)?;
        writeln!(f, "  current scale index: {:?}", self.current_scale_index)?;
        writeln!(
            f,
            "  population cardinality: {:?}",
            self.population_cardinality
        )?;
        writeln!(f, "  best fitness score: {:?}", self.best_fitness_score())
    }
}
