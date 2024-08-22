//! A solution strategy for finding the best chromosome using evolution
mod builder;
pub mod prelude;
mod reporter;

pub use self::builder::{
    Builder as EvolveBuilder, TryFromBuilderError as TryFromEvolveBuilderError,
};

use super::{Strategy, StrategyConfig, StrategyState};
use crate::chromosome::Chromosome;
use crate::compete::Compete;
use crate::crossover::Crossover;
use crate::extension::{Extension, ExtensionNoop};
use crate::fitness::{Fitness, FitnessOrdering, FitnessValue};
use crate::genotype::{Allele, Genotype};
use crate::mutate::Mutate;
use crate::population::Population;
use rand::Rng;
use std::cell::RefCell;
use std::fmt;
use thread_local::ThreadLocal;

pub use self::reporter::Log as EvolveReporterLog;
pub use self::reporter::Noop as EvolveReporterNoop;
pub use self::reporter::Reporter as EvolveReporter;
pub use self::reporter::Simple as EvolveReporterSimple;

/// The Evolve strategy initializes with a random population of chromosomes (unless the genotype
/// seeds specific genes to start with).
/// Then the Evolve strategy runs through generations of chromosomes in a loop:
/// * [extension](crate::extension) an optional step (e.g. [MassExtinction](crate::extension::ExtensionMassExtinction))
/// * [crossover](crate::crossover) to produce new offspring with a mix of parents chromosome genes
/// * [mutate](crate::mutate) a subset of chromosomes to add some additional diversity
/// * calculate [fitness](crate::fitness) for all chromosomes
/// * [compete](crate::compete) to pair up chromosomes for crossover in next generation and drop excess chromosomes
/// * store best chromosome and check ending conditions
///
/// The ending conditions are one or more of the following:
/// * target_fitness_score: when the ultimate goal in terms of fitness score is known and reached
/// * max_stale_generations: when the ultimate goal in terms of fitness score is unknown and one depends on some convergion
///   threshold, or one wants a duration limitation next to the target_fitness_score
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
/// [EvolveReporter] (e.g. [EvolveReporterNoop], [EvolveReporterSimple]). But you are encouraged to
/// roll your own, see [EvolveReporter].
///
/// From the [EvolveBuilder] level, there are several calling mechanisms:
/// * [call](EvolveBuilder::call): this runs a single evolve strategy
/// * [call_repeatedly](EvolveBuilder::call_repeatedly): this runs multiple independent evolve
///   strategies and returns the best one (or short circuits when the target_fitness_score is
///   reached)
/// * [call_par_repeatedly](EvolveBuilder::call_par_repeatedly): this runs multiple independent
///   evolve strategies in parallel and returns the best one (or short circuits when the
///   target_fitness_score is reached). This is separate and independent from the
///   `with_multithreading()` flag on the builder, which determines multithreading inside the evolve
///   strategy. Both can be combined.
/// * [call_speciated](EvolveBuilder::call_speciated): this runs multiple independent
///   evolve strategies and then competes their best results against each other in one final evolve
///   strategy (or short circuits when the target_fitness_score is reached)
/// * [call_par_speciated](EvolveBuilder::call_par_speciated): this runs multiple independent
///   evolve strategies in parallel and then competes their best results against each other in one
///   final evolve strategy (or short circuits when the target_fitness_score is reached). This is
///   separate and independent from the `with_multithreading()` flag on the builder, which determines
///   multithreading inside the evolve strategy. Both can be combined.
///
/// All multithreading mechanisms are implemented using [rayon::iter] and [std::sync::mpsc].
///
/// See [EvolveBuilder] for initialization options.
///
/// Example:
/// ```
/// use genetic_algorithm::strategy::evolve::prelude::*;
/// use genetic_algorithm::fitness::placeholders::CountTrue;
/// use rand::prelude::*;
/// use rand::rngs::SmallRng;
///
/// // the search space
/// let genotype = BinaryGenotype::builder() // boolean alleles
///     .with_genes_size(100)                // 100 genes per chromosome
///     .build()
///     .unwrap();
///
/// // the search strategy
/// let mut rng = SmallRng::from_entropy(); // a randomness provider implementing Trait rand::Rng + Clone + Send + Sync
/// let evolve = Evolve::builder()
///     .with_genotype(genotype)
///     .with_target_population_size(100)                      // evolve with 100 chromosomes
///     .with_target_fitness_score(0)                          // ending condition if 0 times true in the best chromosome
///     .with_valid_fitness_score(10)                          // block ending conditions until at most a 10 times true in the best chromosome
///     .with_max_stale_generations(1000)                      // stop searching if there is no improvement in fitness score for 1000 generations
///     .with_max_chromosome_age(10)                           // kill chromosomes after 10 generations
///     .with_fitness(CountTrue)                               // count the number of true values in the chromosomes
///     .with_fitness_ordering(FitnessOrdering::Minimize)      // aim for the least true values
///     .with_multithreading(true)                             // optional, defaults to false, use all cores for calculating the fitness of the population
///     .with_replace_on_equal_fitness(true)                   // optional, defaults to false, maybe useful to avoid repeatedly seeding with the same best chromosomes after mass extinction events
///     .with_crossover(CrossoverUniform::new(true))           // crossover all individual genes between 2 chromosomes for offspring
///     .with_mutate(MutateSingleGene::new(0.2))               // mutate a single gene with a 20% probability per chromosome
///     .with_compete(CompeteElite::new())                     // sort the chromosomes by fitness to determine crossover order
///     .with_extension(ExtensionMassExtinction::new(10, 0.1)) // optional builder step, simulate cambrian explosion by mass extinction, when fitness score cardinality drops to 10, trim to 10% of population
///     .with_reporter(EvolveReporterSimple::new(100))         // optional builder step, report every 100 generations
///     .call(&mut rng)
///     .unwrap();
///
/// // it's all about the best chromosome after all
/// let best_chromosome = evolve.best_chromosome().unwrap();
/// assert_eq!(best_chromosome.genes, vec![false; 100])
/// ```
pub struct Evolve<
    G: Genotype,
    M: Mutate,
    F: Fitness<Allele = G::Allele>,
    S: Crossover,
    C: Compete,
    E: Extension,
    SR: EvolveReporter<Allele = G::Allele>,
> {
    pub genotype: G,
    pub fitness: F,
    pub plugins: EvolvePlugins<M, S, C, E>,
    pub config: EvolveConfig,
    pub state: EvolveState<G::Allele>,
    reporter: SR,
}

pub struct EvolvePlugins<M: Mutate, S: Crossover, C: Compete, E: Extension> {
    pub mutate: M,
    pub crossover: S,
    pub compete: C,
    pub extension: E,
}

pub struct EvolveConfig {
    pub target_population_size: usize,
    pub max_stale_generations: Option<usize>,
    pub max_chromosome_age: Option<usize>,
    pub target_fitness_score: Option<FitnessValue>,
    pub valid_fitness_score: Option<FitnessValue>,
    pub fitness_ordering: FitnessOrdering,
    pub multithreading: bool,
    pub replace_on_equal_fitness: bool,
}

/// Stores the state of the Evolve strategy. Next to the expected general fields, the following
/// strategy specific fields are added:
/// * population: the population of the current generation
#[derive(Clone)]
pub struct EvolveState<A: Allele> {
    pub current_iteration: usize,
    pub current_generation: usize,
    pub stale_generations: usize,
    pub best_generation: usize,
    pub best_chromosome: Option<Chromosome<A>>,

    pub current_scale_index: Option<usize>,
    pub max_scale_index: usize,
    pub population: Population<A>,
}

impl<
        G: Genotype,
        M: Mutate,
        F: Fitness<Allele = G::Allele>,
        S: Crossover,
        C: Compete,
        E: Extension,
        SR: EvolveReporter<Allele = G::Allele>,
    > Strategy<G> for Evolve<G, M, F, S, C, E, SR>
{
    fn call<R: Rng + Clone + Send + Sync>(&mut self, rng: &mut R) {
        self.state.population = self.population_factory(rng);

        // reuse same thread local variables for all loops
        let mut fitness_thread_local: Option<ThreadLocal<RefCell<F>>> = None;
        let mut rng_thread_local: Option<ThreadLocal<RefCell<R>>> = None;
        if self.config.multithreading {
            fitness_thread_local = Some(ThreadLocal::new());
            rng_thread_local = Some(ThreadLocal::new());
        }

        self.reporter
            .on_start(&self.genotype, &self.state, &self.config);
        while !self.is_finished() {
            self.state.current_generation += 1;
            self.state.population.increment_and_filter_age(&self.config);

            self.plugins.extension.call(
                &self.genotype,
                &mut self.state,
                &self.config,
                &mut self.reporter,
                rng,
            );
            self.plugins.crossover.call(
                &self.genotype,
                &mut self.state,
                &self.config,
                &mut self.reporter,
                rng,
            );
            self.plugins.mutate.call(
                &self.genotype,
                &mut self.state,
                &self.config,
                &mut self.reporter,
                rng,
            );
            self.fitness
                .call_for_population(&mut self.state.population, fitness_thread_local.as_ref());
            self.plugins.compete.call(
                &mut self.state,
                &self.config,
                &mut self.reporter,
                rng,
                rng_thread_local.as_ref(),
            );

            if let Some(contending_chromosome) = self
                .state
                .population
                .best_chromosome(self.config.fitness_ordering)
                .cloned()
            {
                self.state.update_best_chromosome_and_report(
                    &contending_chromosome,
                    &self.config,
                    &mut self.reporter,
                );
            } else {
                self.state.increment_stale_generations();
            }
            //self.ensure_best_chromosome(population);
            self.reporter.on_new_generation(&self.state, &self.config);
            self.state.scale(&self.config);
        }
        self.reporter.on_finish(&self.state, &self.config);
    }
    fn best_chromosome(&self) -> Option<Chromosome<G::Allele>> {
        self.state.best_chromosome()
    }
    fn best_generation(&self) -> usize {
        self.state.best_generation
    }
    fn best_fitness_score(&self) -> Option<FitnessValue> {
        self.state.best_fitness_score()
    }
}

impl<G: Genotype, M: Mutate, F: Fitness<Allele = G::Allele>, S: Crossover, C: Compete>
    Evolve<G, M, F, S, C, ExtensionNoop, EvolveReporterNoop<G::Allele>>
{
    pub fn builder() -> EvolveBuilder<G, M, F, S, C, ExtensionNoop, EvolveReporterNoop<G::Allele>> {
        EvolveBuilder::new()
    }
}

impl<
        G: Genotype,
        M: Mutate,
        F: Fitness<Allele = G::Allele>,
        S: Crossover,
        C: Compete,
        E: Extension,
        SR: EvolveReporter<Allele = G::Allele>,
    > Evolve<G, M, F, S, C, E, SR>
{
    #[allow(dead_code)]
    fn ensure_best_chromosome(&mut self, population: &mut Population<G::Allele>) {
        if let Some(best_chromosome) = &self.state.best_chromosome {
            if !population.fitness_score_present(best_chromosome.fitness_score) {
                population.chromosomes.push(best_chromosome.clone());
            }
        }
    }

    fn is_finished(&self) -> bool {
        self.allow_finished_by_valid_fitness_score()
            && (self.is_finished_by_max_stale_generations()
                || self.is_finished_by_target_fitness_score())
    }

    fn is_finished_by_max_stale_generations(&self) -> bool {
        if let Some(max_stale_generations) = self.config.max_stale_generations {
            self.state.stale_generations >= max_stale_generations
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

    pub fn population_factory<R: Rng + Clone + Send + Sync>(
        &mut self,
        rng: &mut R,
    ) -> Population<G::Allele> {
        (0..self.config.target_population_size)
            .map(|_| self.genotype.chromosome_factory(rng))
            .collect::<Vec<_>>()
            .into()
    }
}

impl StrategyConfig for EvolveConfig {
    fn fitness_ordering(&self) -> FitnessOrdering {
        self.fitness_ordering
    }
    fn multithreading(&self) -> bool {
        self.multithreading
    }
    fn replace_on_equal_fitness(&self) -> bool {
        self.replace_on_equal_fitness
    }
}

impl<A: Allele> StrategyState<A> for EvolveState<A> {
    fn best_chromosome(&self) -> Option<Chromosome<A>> {
        self.best_chromosome.clone()
    }
    fn best_chromosome_as_ref(&self) -> Option<&Chromosome<A>> {
        self.best_chromosome.as_ref()
    }
    fn best_fitness_score(&self) -> Option<FitnessValue> {
        self.best_chromosome.as_ref().and_then(|c| c.fitness_score)
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
    fn stale_generations(&self) -> usize {
        self.stale_generations
    }
    fn increment_stale_generations(&mut self) {
        self.stale_generations += 1;
    }
    fn reset_stale_generations(&mut self) {
        self.stale_generations = 0;
    }
    fn set_best_chromosome(
        &mut self,
        best_chromosome: &Chromosome<A>,
        improved_fitness: bool,
    ) -> (bool, bool) {
        self.best_chromosome = Some(best_chromosome.clone());
        if improved_fitness {
            self.best_generation = self.current_generation;
        }
        (true, improved_fitness)
    }
}
impl<A: Allele> EvolveState<A> {
    fn update_best_chromosome_and_report<SR: EvolveReporter<Allele = A>>(
        &mut self,
        contending_chromosome: &Chromosome<A>,
        config: &EvolveConfig,
        reporter: &mut SR,
    ) {
        match self.update_best_chromosome(
            contending_chromosome,
            &config.fitness_ordering,
            config.replace_on_equal_fitness,
        ) {
            (true, true) => {
                reporter.on_new_best_chromosome(self, config);
                self.reset_stale_generations();
            }
            (true, false) => {
                reporter.on_new_best_chromosome_equal_fitness(self, config);
                self.increment_stale_generations()
            }
            _ => self.increment_stale_generations(),
        }
    }
    fn scale(&mut self, config: &EvolveConfig) {
        if let Some(current_scale_index) = self.current_scale_index {
            if let Some(max_stale_generations) = config.max_stale_generations {
                if self.stale_generations >= max_stale_generations
                    && current_scale_index < self.max_scale_index
                {
                    self.current_scale_index = Some(current_scale_index + 1);
                    self.reset_stale_generations();
                }
            }
        }
    }
}

impl<
        G: Genotype,
        M: Mutate,
        F: Fitness<Allele = G::Allele>,
        S: Crossover,
        C: Compete,
        E: Extension,
        SR: EvolveReporter<Allele = G::Allele>,
    > TryFrom<EvolveBuilder<G, M, F, S, C, E, SR>> for Evolve<G, M, F, S, C, E, SR>
{
    type Error = TryFromEvolveBuilderError;

    fn try_from(builder: EvolveBuilder<G, M, F, S, C, E, SR>) -> Result<Self, Self::Error> {
        if builder.genotype.is_none() {
            Err(TryFromEvolveBuilderError("Evolve requires a Genotype"))
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
        } else if builder.compete.is_none() {
            Err(TryFromEvolveBuilderError(
                "Evolve requires a Compete strategy",
            ))
        } else if builder
            .crossover
            .as_ref()
            .map(|o| o.require_crossover_indexes())
            .unwrap()
            && builder
                .genotype
                .as_ref()
                .map(|o| o.crossover_indexes().is_empty())
                .unwrap()
        {
            Err(TryFromEvolveBuilderError(
                "The provided Crossover strategy requires crossover_indexes, which the provided Genotype does not provide",
            ))
        } else if builder
            .crossover
            .as_ref()
            .map(|o| o.require_crossover_points())
            .unwrap()
            && builder
                .genotype
                .as_ref()
                .map(|o| o.crossover_points().is_empty())
                .unwrap()
        {
            Err(TryFromEvolveBuilderError(
                "The provided Crossover strategy requires crossover_points, which the provided Genotype does not provide",
            ))
        } else if builder.target_population_size == 0 {
            Err(TryFromEvolveBuilderError(
                "Evolve requires a target_population_size > 0",
            ))
        } else if builder.max_stale_generations.is_none() && builder.target_fitness_score.is_none()
        {
            Err(TryFromEvolveBuilderError(
                "Evolve requires at least a max_stale_generations or target_fitness_score ending condition",
            ))
        } else {
            let genotype = builder.genotype.unwrap();
            let population = Population::new_empty();
            let state = EvolveState::new(&genotype, population);

            Ok(Self {
                genotype,
                fitness: builder.fitness.unwrap(),
                plugins: EvolvePlugins {
                    mutate: builder.mutate.unwrap(),
                    crossover: builder.crossover.unwrap(),
                    compete: builder.compete.unwrap(),
                    extension: builder.extension,
                },
                config: EvolveConfig {
                    target_population_size: builder.target_population_size,
                    max_stale_generations: builder.max_stale_generations,
                    max_chromosome_age: builder.max_chromosome_age,
                    target_fitness_score: builder.target_fitness_score,
                    valid_fitness_score: builder.valid_fitness_score,
                    fitness_ordering: builder.fitness_ordering,
                    multithreading: builder.multithreading,
                    replace_on_equal_fitness: builder.replace_on_equal_fitness,
                },
                state,
                reporter: builder.reporter,
            })
        }
    }
}

impl Default for EvolveConfig {
    fn default() -> Self {
        Self {
            target_population_size: 0,
            max_stale_generations: None,
            max_chromosome_age: None,
            target_fitness_score: None,
            valid_fitness_score: None,
            fitness_ordering: FitnessOrdering::Maximize,
            multithreading: false,
            replace_on_equal_fitness: false,
        }
    }
}
impl EvolveConfig {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<A: Allele> Default for EvolveState<A> {
    fn default() -> Self {
        Self {
            current_iteration: 0,
            current_generation: 0,
            stale_generations: 0,
            current_scale_index: None,
            max_scale_index: 0,
            best_generation: 0,
            best_chromosome: None,
            population: Population::new_empty(),
        }
    }
}
impl<A: Allele> EvolveState<A> {
    pub fn new<G: Genotype>(genotype: &G, population: Population<A>) -> Self {
        if let Some(max_scale_index) = genotype.max_scale_index() {
            Self {
                current_scale_index: Some(0),
                max_scale_index,
                population,
                ..Default::default()
            }
        } else {
            Self {
                population,
                ..Default::default()
            }
        }
    }
}

impl<
        G: Genotype,
        M: Mutate,
        F: Fitness<Allele = G::Allele>,
        S: Crossover,
        C: Compete,
        E: Extension,
        SR: EvolveReporter<Allele = G::Allele>,
    > fmt::Display for Evolve<G, M, F, S, C, E, SR>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "evolve:")?;
        writeln!(f, "  genotype: {:?}", self.genotype)?;
        writeln!(f, "  fitness: {:?}", self.fitness)?;

        writeln!(f, "{}", self.plugins)?;
        writeln!(f, "{}", self.config)?;
        writeln!(f, "{}", self.state)
    }
}

impl<M: Mutate, S: Crossover, C: Compete, E: Extension> fmt::Display for EvolvePlugins<M, S, C, E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "evolve_plugins:")?;
        writeln!(f, "  mutate: {:?}", self.mutate)?;
        writeln!(f, "  crossover: {:?}", self.crossover)?;
        writeln!(f, "  compete: {:?}", self.compete)?;
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
        writeln!(f, "  max_chromosome_age: {:?}", self.max_chromosome_age)?;
        writeln!(f, "  valid_fitness_score: {:?}", self.valid_fitness_score)?;
        writeln!(f, "  target_fitness_score: {:?}", self.target_fitness_score)?;
        writeln!(f, "  fitness_ordering: {:?}", self.fitness_ordering)?;
        writeln!(f, "  multithreading: {:?}", self.multithreading)
    }
}

impl<A: Allele> fmt::Display for EvolveState<A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "evolve_state:")?;
        writeln!(f, "  current iteration: {:?}", self.current_iteration)?;
        writeln!(f, "  current generation: {:?}", self.current_generation)?;
        writeln!(f, "  stale generations: {:?}", self.stale_generations)?;
        writeln!(
            f,
            "  scale index (current/max): {:?}/{}",
            self.current_scale_index, self.max_scale_index
        )?;
        writeln!(f, "  best fitness score: {:?}", self.best_fitness_score())?;
        writeln!(f, "  best_chromosome: {:?}", self.best_chromosome.as_ref())
    }
}
