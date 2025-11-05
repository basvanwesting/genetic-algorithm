//! A solution strategy for finding the best chromosome, when search space is convex with little local optima or crossover is impossible or inefficient
mod builder;
pub mod prelude;
mod reporter;

pub use self::builder::{
    Builder as HillClimbBuilder, TryFromBuilderError as TryFromHillClimbBuilderError,
};

use super::{
    Strategy, StrategyAction, StrategyConfig, StrategyReporter, StrategyReporterNoop,
    StrategyState, StrategyVariant,
};
use crate::chromosome::{Chromosome, Genes};
use crate::fitness::{Fitness, FitnessCache, FitnessOrdering, FitnessValue};
use crate::genotype::HillClimbGenotype;
use crate::population::Population;
use rand::prelude::SliceRandom;
use rand::rngs::SmallRng;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::time::{Duration, Instant};
use thread_local::ThreadLocal;

pub use self::reporter::Simple as HillClimbReporterSimple;
pub use crate::strategy::reporter::Duration as HillClimbReporterDuration;
pub use crate::strategy::reporter::Noop as HillClimbReporterNoop;

#[derive(Copy, Clone, Debug, Default)]
pub enum HillClimbVariant {
    #[default]
    Stochastic,
    SteepestAscent,
}

/// The HillClimb strategy is an iterative algorithm that starts with a single arbitrary solution
/// to a problem (unless the genotype seeds specific genes to sample a single starting point from),
/// then attempts to find a better solution by making an incremental change to the solution
///
/// There are 2 variants:
/// * [HillClimbVariant::Stochastic]: does not examine all neighbors before deciding how to move.
///   Rather, it selects a neighbor at random, and decides (based on the improvement in that
///   neighbour) whether to move to that neighbor or to examine another
/// * [HillClimbVariant::SteepestAscent]: all neighbours are compared and the one with the best
///   improvement is chosen.
///
/// The ending conditions are one or more of the following:
/// * target_fitness_score: when the ultimate goal in terms of fitness score is known and reached
/// * max_stale_generations: when the ultimate goal in terms of fitness score is unknown and one depends on some convergion
///   threshold, or one wants a duration limitation next to the target_fitness_score.
///   * set to a high value for [HillClimbVariant::Stochastic]
///   * set to a low value for [HillClimbVariant::SteepestAscent], preferably even `1`, unless
///   there is a replace_on_equal_fitness consideration or some remaining randomness in the neighbouring population (see RangeGenotype
///   below)
/// * max_generations: when the ultimate goal in terms of fitness score is unknown and there is a effort constraint
///
/// There are optional mutation distance limitations for
/// [RangeGenotype](crate::genotype::RangeGenotype) and
/// [MultiRangeGenotype](crate::genotype::MultiRangeGenotype) neighbouring chromosomes, see [crate::genotype::MutationType].
/// * With MutationType::Scaled
///     * Mutation distance only on edges of current scale (e.g. -1 and +1 for -1..-1 scale)
///         * Pick random edge for [HillClimbVariant::Stochastic]
///         * Take both edges per gene for [HillClimbVariant::SteepestAscent]
///     * Scale down after max_stale_generations is reached and reset stale_generations to zero
///     * Only trigger max_stale_generations ending condition when already reached the smallest scale
///     * max_stale_generations could be set to 1, as there is no remaining randomness
/// * With MutationType::Relative
///     * Mutation distance taken uniformly from mutation range
///         * Sample single random value for [HillClimbVariant::Stochastic]
///         * Ensure to sample both a higer and lower value per gene for [HillClimbVariant::SteepestAscent]
///     * Standard max_stale_generations ending condition
///     * max_stale_generations should be set somewhat higher than 1 as there is some remaining randomness
/// * With MutationType::Random (not advised for hill climbing):
///     * Mutate uniformly over the complete allele range
///         * Sample single random value for [HillClimbVariant::Stochastic]
///         * Ensure to sample both a higer and lower value per gene for [HillClimbVariant::SteepestAscent]
///     * Standard max_stale_generations ending condition
///     * max_stale_generations should be set substantially higher than 1 as there is a lot remaining randomness
/// * With MutationType::Discrete
///     * All values are neighbours, just like ListGenotype
/// * With MutationType::Transition
///     * Behaves like Random and then slowly transitions to Relative
///
/// There are reporting hooks in the loop receiving the [HillClimbState], which can by handled by an
/// [StrategyReporter] (e.g. [HillClimbReporterDuration], [HillClimbReporterSimple]). But you are encouraged to
/// roll your own, see [StrategyReporter].
///
/// From the [HillClimbBuilder] level, there are several calling mechanisms:
/// * [call](HillClimbBuilder::call): this runs a single [HillClimb] strategy
/// * [call_repeatedly](HillClimbBuilder::call_repeatedly): this runs multiple independent [HillClimb]
///   strategies and returns the best one (or short circuits when the target_fitness_score is
///   reached)
/// * [call_par_repeatedly](HillClimbBuilder::call_par_repeatedly): this runs multiple independent
///   [HillClimb] strategies in parallel and returns the best one (or short circuits when the
///   target_fitness_score is reached). This is separate and independent from the
///   `with_par_fitness()` flag on the builder, which determines multithreading of the fitness
///   calculation inside the [HillClimb] strategy. Both can be combined.
///
/// Multithreading inside the [HillClimbVariant::Stochastic] using the `with_par_fitness()` builder
/// step does nothing, due to the sequential nature of the search. But
/// [call_par_repeatedly](HillClimbBuilder::call_par_repeatedly) still effectively multithreads for
/// these variants as the sequential nature is only internal to the [HillClimb] strategy.
///
/// All multithreading mechanisms are implemented using [rayon::iter] and [std::sync::mpsc].
///
/// See [HillClimbBuilder] for initialization options.
///
/// Example:
/// ```
/// use genetic_algorithm::strategy::hill_climb::prelude::*;
/// use genetic_algorithm::fitness::placeholders::SumGenes;
///
/// // the search space
/// let genotype = RangeGenotype::builder()     // f32 alleles
///     .with_genes_size(16)                    // 16 genes
///     .with_genes_hashing(false)              // store genes_hash on chromosome (required for fitness_cache and deduplication extension, both not useful here)
///     .with_chromosome_recycling(true)        // recycle genes memory allocations, maybe useful
///     .with_allele_range(0.0..=1.0)           // allow gene values between 0.0 and 1.0
///     .with_mutation_type(MutationType::Relative(-0.1..=0.1)) // optional, neighbouring step size randomly sampled from range
///     .with_mutation_type(MutationType::ScaledSteps(vec![0.1, 0.01, 0.001])) // optional, neighbouring step size equal to start/end of each scaled range
///     .build()
///     .unwrap();
///
/// // the search strategy
/// let hill_climb = HillClimb::builder()
///     .with_genotype(genotype)
///     .with_variant(HillClimbVariant::SteepestAscent)   // check all neighbours for each round
///     .with_fitness(SumGenes::new_with_precision(1e-5)) // sum the gene values of the chromosomes with precision 0.00001, which means multiply fitness score (isize) by 100_000
///     .with_fitness_ordering(FitnessOrdering::Minimize) // aim for the lowest sum
///     .with_fitness_cache(1000)                         // enable caching of fitness values (LRU size 1000), only works when genes_hash is stored in chromosome. Only useful for long stale runs
///     .with_par_fitness(true)                           // optional, defaults to false, use parallel fitness calculation
///     .with_target_fitness_score(0)                     // ending condition if sum of genes is <= 0.00001 in the best chromosome
///     .with_valid_fitness_score(100)                    // block ending conditions until at least the sum of genes <= 0.00100 is reached in the best chromosome
///     .with_max_stale_generations(1000)                 // stop searching if there is no improvement in fitness score for 1000 generations (per scaled_range)
///     .with_max_generations(1_000_000)                  // optional, stop searching after 1M generations
///     .with_replace_on_equal_fitness(true)              // optional, defaults to true, crucial for some type of problems with discrete fitness steps like nqueens
///     .with_reporter(HillClimbReporterSimple::new(100)) // optional, report every 100 generations
///     .with_rng_seed_from_u64(0)                        // for testing with deterministic results
///     .call()
///     .unwrap();
///
/// // it's all about the best genes after all
/// let (best_genes, best_fitness_score) = hill_climb.best_genes_and_fitness_score().unwrap();
/// assert_eq!(best_genes.into_iter().map(|v| v <= 1e-3).collect::<Vec<_>>(), vec![true; 16]);
/// assert_eq!(best_fitness_score, 0);
/// ```
pub struct HillClimb<
    G: HillClimbGenotype,
    F: Fitness<Genotype = G>,
    SR: StrategyReporter<Genotype = G>,
> {
    pub genotype: G,
    pub fitness: F,
    pub config: HillClimbConfig,
    pub state: HillClimbState<G>,
    pub reporter: SR,
    pub rng: SmallRng,
}

pub struct HillClimbConfig {
    pub variant: HillClimbVariant,
    pub fitness_ordering: FitnessOrdering,
    pub par_fitness: bool,
    pub replace_on_equal_fitness: bool,

    pub target_fitness_score: Option<FitnessValue>,
    pub max_stale_generations: Option<usize>,
    pub max_generations: Option<usize>,
    pub valid_fitness_score: Option<FitnessValue>,
    pub fitness_cache: Option<FitnessCache>,
}

/// Stores the state of the HillClimb strategy.
pub struct HillClimbState<G: HillClimbGenotype> {
    pub current_iteration: usize,
    pub current_generation: usize,
    pub stale_generations: usize,
    pub scale_generation: usize,
    pub best_generation: usize,
    pub best_fitness_score: Option<FitnessValue>,
    pub best_chromosome: Option<Chromosome<G::Allele>>,
    pub chromosome: Option<Chromosome<G::Allele>>,
    pub population: Population<G::Allele>,
    pub durations: HashMap<StrategyAction, Duration>,
}

impl<G: HillClimbGenotype, F: Fitness<Genotype = G>, SR: StrategyReporter<Genotype = G>> Strategy<G>
    for HillClimb<G, F, SR>
{
    fn call(&mut self) {
        let now = Instant::now();
        self.reporter
            .on_enter(&self.genotype, &self.state, &self.config);
        let mut fitness_thread_local: Option<ThreadLocal<RefCell<F>>> = None;
        if self.config.par_fitness {
            fitness_thread_local = Some(ThreadLocal::new());
        }

        self.setup();
        self.reporter
            .on_start(&self.genotype, &self.state, &self.config);
        while !self.is_finished() {
            self.genotype.increment_generation();
            self.state.increment_generation();
            match self.config.variant {
                HillClimbVariant::Stochastic => {
                    self.state
                        .chromosome
                        .clone_from(&self.state.best_chromosome);
                    self.genotype.mutate_chromosome_genes(
                        1,
                        true,
                        self.state.chromosome.as_mut().unwrap(),
                        &mut self.rng,
                    );
                    self.fitness.call_for_state_chromosome(
                        &self.genotype,
                        &mut self.state,
                        &self.config,
                    );
                    self.state.update_best_chromosome_from_state_chromosome(
                        &self.genotype,
                        &self.config,
                        &mut self.reporter,
                    );
                }
                HillClimbVariant::SteepestAscent => {
                    self.state
                        .chromosome
                        .clone_from(&self.state.best_chromosome);
                    self.state.population.truncate(0);
                    self.genotype.fill_neighbouring_population(
                        self.state.chromosome.as_ref().unwrap(),
                        &mut self.state.population,
                        &mut self.rng,
                    );
                    self.fitness.call_for_state_population(
                        &self.genotype,
                        &mut self.state,
                        &self.config,
                        fitness_thread_local.as_ref(),
                    );
                    self.state.update_best_chromosome_from_state_population(
                        &self.genotype,
                        &self.config,
                        &mut self.reporter,
                        &mut self.rng,
                    );
                }
            }
            self.reporter
                .on_generation_complete(&self.genotype, &self.state, &self.config);
            self.state.scale(&mut self.genotype, &self.config);
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
impl<G: HillClimbGenotype, F: Fitness<Genotype = G>, SR: StrategyReporter<Genotype = G>>
    HillClimb<G, F, SR>
{
    pub fn best_chromosome(&self) -> Option<Chromosome<G::Allele>> {
        if let Some(best_genes) = self.best_genes() {
            let mut chromosome = Chromosome::<G::Allele>::new(best_genes);
            chromosome.set_fitness_score(self.best_fitness_score());
            Some(chromosome)
        } else {
            None
        }
    }
}

impl<G: HillClimbGenotype, F: Fitness<Genotype = G>> HillClimb<G, F, StrategyReporterNoop<G>> {
    pub fn builder() -> HillClimbBuilder<G, F, StrategyReporterNoop<G>> {
        HillClimbBuilder::new()
    }
}
impl<G: HillClimbGenotype, F: Fitness<Genotype = G>, SR: StrategyReporter<Genotype = G>>
    HillClimb<G, F, SR>
{
    pub fn setup(&mut self) {
        let now = Instant::now();

        self.state.chromosome = Some(self.genotype.chromosome_constructor_random(&mut self.rng));
        self.state
            .add_duration(StrategyAction::SetupAndCleanup, now.elapsed());

        match self.config.variant {
            HillClimbVariant::Stochastic => {
                self.fitness.call_for_state_chromosome(
                    &self.genotype,
                    &mut self.state,
                    &self.config,
                );
                self.state.update_best_chromosome_from_state_chromosome(
                    &self.genotype,
                    &self.config,
                    &mut self.reporter,
                );
            }
            HillClimbVariant::SteepestAscent => {
                // init population with all seeds for first population if present, or just a single
                // random chromosome
                let population_size = self.genotype.seed_genes_list().len().max(1);
                self.state.population = self
                    .genotype
                    .population_constructor(population_size, &mut self.rng);

                self.fitness.call_for_state_population(
                    &self.genotype,
                    &mut self.state,
                    &self.config,
                    None,
                );
                self.state.update_best_chromosome_from_state_population(
                    &self.genotype,
                    &self.config,
                    &mut self.reporter,
                    &mut self.rng,
                );
            }
        }

        if self.state.best_fitness_score().is_none() {
            self.state.best_generation = self.state.current_generation;
            self.state
                .best_chromosome
                .clone_from(&self.state.chromosome);
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

impl StrategyConfig for HillClimbConfig {
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
        StrategyVariant::HillClimb(self.variant)
    }
}

impl<G: HillClimbGenotype> StrategyState<G> for HillClimbState<G> {
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
    fn best_fitness_score(&self) -> Option<FitnessValue> {
        self.best_fitness_score
    }
    fn best_generation(&self) -> usize {
        self.best_generation
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
    fn population_cardinality(&self) -> Option<usize> {
        None
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

impl<G: HillClimbGenotype> HillClimbState<G> {
    fn update_best_chromosome_from_state_chromosome<SR: StrategyReporter<Genotype = G>>(
        &mut self,
        genotype: &G,
        config: &HillClimbConfig,
        reporter: &mut SR,
    ) {
        if let Some(chromosome) = self.chromosome.as_ref() {
            let now = Instant::now();
            match self.is_better_chromosome(
                chromosome,
                &config.fitness_ordering,
                config.replace_on_equal_fitness,
            ) {
                (true, true) => {
                    self.best_generation = self.current_generation;
                    self.best_fitness_score = chromosome.fitness_score();
                    self.best_chromosome = Some(chromosome.clone());
                    reporter.on_new_best_chromosome(genotype, self, config);
                    self.reset_stale_generations();
                }
                (true, false) => {
                    self.best_chromosome = Some(chromosome.clone());
                    reporter.on_new_best_chromosome_equal_fitness(genotype, self, config);
                    self.increment_stale_generations()
                }
                _ => self.increment_stale_generations(),
            }
            self.add_duration(StrategyAction::UpdateBestChromosome, now.elapsed());
        }
    }
    fn update_best_chromosome_from_state_population<SR: StrategyReporter<Genotype = G>>(
        &mut self,
        genotype: &G,
        config: &HillClimbConfig,
        reporter: &mut SR,
        rng: &mut SmallRng,
    ) {
        let now = Instant::now();
        if config.replace_on_equal_fitness {
            // shuffle, so we don't repeatedly take the same best chromosome in sideways move
            self.population.chromosomes.shuffle(rng);
        }
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
                    self.increment_stale_generations()
                }
                _ => self.increment_stale_generations(),
            }
        } else {
            self.increment_stale_generations();
        }
        self.add_duration(StrategyAction::UpdateBestChromosome, now.elapsed());
    }
    fn scale(&mut self, genotype: &mut G, config: &HillClimbConfig) {
        if let Some(max_stale_generations) = config.max_stale_generations {
            if self.stale_generations >= max_stale_generations && genotype.increment_scale_index() {
                self.reset_scale_generation();
                self.reset_stale_generations();
            }
        }
    }
}

impl<G: HillClimbGenotype, F: Fitness<Genotype = G>, SR: StrategyReporter<Genotype = G>>
    TryFrom<HillClimbBuilder<G, F, SR>> for HillClimb<G, F, SR>
{
    type Error = TryFromHillClimbBuilderError;

    fn try_from(builder: HillClimbBuilder<G, F, SR>) -> Result<Self, Self::Error> {
        if builder.genotype.is_none() {
            Err(TryFromHillClimbBuilderError(
                "HillClimb requires a HillClimbGenotype",
            ))
        } else if builder.fitness.is_none() {
            Err(TryFromHillClimbBuilderError("HillClimb requires a Fitness"))
        } else if builder.max_stale_generations.is_none()
            && builder.max_generations.is_none()
            && builder.target_fitness_score.is_none()
        {
            Err(TryFromHillClimbBuilderError(
                "HillClimb requires at least a max_stale_generations, max_generations or target_fitness_score ending condition",
            ))
        } else {
            let rng = builder.rng();
            let genotype = builder.genotype.unwrap();
            let state = HillClimbState::new(&genotype);

            Ok(Self {
                genotype,
                fitness: builder.fitness.unwrap(),
                config: HillClimbConfig {
                    variant: builder.variant.unwrap_or_default(),
                    fitness_ordering: builder.fitness_ordering,
                    fitness_cache: builder.fitness_cache,
                    par_fitness: builder.par_fitness,
                    max_stale_generations: builder.max_stale_generations,
                    max_generations: builder.max_generations,
                    target_fitness_score: builder.target_fitness_score,
                    valid_fitness_score: builder.valid_fitness_score,
                    replace_on_equal_fitness: builder.replace_on_equal_fitness,
                },
                state,
                reporter: builder.reporter,
                rng,
            })
        }
    }
}

impl Default for HillClimbConfig {
    fn default() -> Self {
        Self {
            variant: Default::default(),
            fitness_ordering: FitnessOrdering::Maximize,
            fitness_cache: None,
            par_fitness: false,
            max_stale_generations: None,
            max_generations: None,
            target_fitness_score: None,
            valid_fitness_score: None,
            replace_on_equal_fitness: false,
        }
    }
}
impl HillClimbConfig {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<G: HillClimbGenotype> HillClimbState<G> {
    pub fn new(genotype: &G) -> Self {
        Self {
            current_iteration: 0,
            current_generation: 0,
            stale_generations: 0,
            scale_generation: 0,
            best_generation: 0,
            best_fitness_score: None,
            chromosome: None,
            population: Population::new_empty(genotype.chromosome_recycling()),
            durations: HashMap::new(),
            best_chromosome: None,
        }
    }
}

impl<G: HillClimbGenotype, F: Fitness<Genotype = G>, SR: StrategyReporter<Genotype = G>>
    fmt::Display for HillClimb<G, F, SR>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "hill_climb:")?;
        writeln!(f, "  fitness: {:?}", self.fitness)?;
        writeln!(f)?;

        writeln!(f, "{}", self.config)?;
        writeln!(f, "{}", self.state)?;
        writeln!(f, "{}", self.genotype)
    }
}

impl fmt::Display for HillClimbConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "hill_climb_config:")?;
        writeln!(f, "  variant: {:?}", self.variant)?;

        writeln!(
            f,
            "  max_stale_generations: {:?}",
            self.max_stale_generations
        )?;
        writeln!(f, "  max_generations: {:?}", self.max_generations)?;
        writeln!(f, "  valid_fitness_score: {:?}", self.valid_fitness_score)?;
        writeln!(f, "  target_fitness_score: {:?}", self.target_fitness_score)?;
        writeln!(f, "  fitness_ordering: {:?}", self.fitness_ordering)?;
        writeln!(f, "  par_fitness: {:?}", self.par_fitness)
    }
}

impl<G: HillClimbGenotype> fmt::Display for HillClimbState<G> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "hill_climb_state:")?;
        writeln!(f, "  current iteration: {:?}", self.current_iteration)?;
        writeln!(f, "  current generation: {:?}", self.current_generation)?;
        writeln!(f, "  stale generations: {:?}", self.stale_generations)?;
        writeln!(f, "  best fitness score: {:?}", self.best_fitness_score())
    }
}
