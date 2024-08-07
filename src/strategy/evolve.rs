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
use crate::extension::Extension;
use crate::fitness::{Fitness, FitnessOrdering, FitnessValue};
use crate::genotype::Genotype;
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
/// * [crossover](crate::crossover) to produce new offspring with a mix of parents chromosome genes
/// * [mutate](crate::mutate) a subset of chromosomes to add some additional diversity
/// * calculate [fitness](crate::fitness) for all chromosomes
/// * [compete](crate::compete) to pair up chromosomes for crossover in next generation and drop excess chromosomes
/// * store best chromosome
/// * check ending conditions
/// * [extension](crate::extension) an optional additional step (e.g. [MassExtinction](crate::extension::ExtensionMassExtinction))
///
/// The ending conditions are one or more of the following:
/// * target_fitness_score: when the ultimate goal in terms of fitness score is known and reached
/// * max_stale_generations: when the ultimate goal in terms of fitness score is unknown and one depends on some convergion
///   threshold, or one wants a duration limitation next to the target_fitness_score
///
/// There are reporting hooks in the loop receiving the [EvolveState], which can by handled by an
/// [EvolveReporter] (e.g. [EvolveReporterNoop], [EvolveReporterSimple]). But you are encouraged to
/// roll your own, see [EvolveReporter].
///
/// At the [EvolveBuilder] level, there are two additional mechanisms:
/// * [call_repeatedly](EvolveBuilder::call_repeatedly): this runs multiple independent evolve
///   strategies and returns the best one (or short circuits when the target_fitness_score is
///   reached)
/// * [call_speciated](EvolveBuilder::call_speciated): this runs multiple independent
///   evolve strategies and then competes their best results against each other in one final evolve
///   strategy
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
///     .build()
///     .unwrap();
///
/// // the search strategy
/// let mut rng = rand::thread_rng(); // a randomness provider implementing Trait rand::Rng
/// let evolve = Evolve::builder()
///     .with_genotype(genotype)
///     .with_target_population_size(100) // evolve with 100 chromosomes
///     .with_target_fitness_score(0)     // ending condition if 0 times true in the best chromosome
///     .with_valid_fitness_score(10)     // block ending conditions until at most a 10 times true in the best chromosome
///     .with_max_stale_generations(1000) // stop searching if there is no improvement in fitness score for 1000 generations
///     .with_max_chromosome_age(10)      // kill chromosomes after 10 generations
///     .with_fitness(CountTrue)          // count the number of true values in the chromosomes
///     .with_fitness_ordering(FitnessOrdering::Minimize) // aim for the least true values
///     .with_multithreading(true)              // use all cores for calculating the fitness of the population
///     .with_crossover(CrossoverUniform::new(true)) // crossover all individual genes between 2 chromosomes for offspring
///     .with_mutate(MutateSingleGeneRandom::new(0.2))      // mutate a single gene with a 20% probability per chromosome
///     .with_compete(CompeteElite::new())      // sort the chromosomes by fitness to determine crossover order
///     .with_extension(ExtensionMassExtinction::new(10, 0.1)) // simulate cambrian explosion by mass extinction, when fitness score cardinality drops to 10, trim to 10% of population
///     .with_reporter(EvolveReporterNoop::default()) // no reporting
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
    F: Fitness<Genotype = G>,
    S: Crossover,
    C: Compete,
    E: Extension,
    SR: EvolveReporter<Genotype = G>,
> {
    pub genotype: G,
    pub fitness: F,
    pub plugins: EvolvePlugins<M, S, C, E>,
    pub config: EvolveConfig,
    pub state: EvolveState<G>,
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
}

/// Stores the state of the Evolve strategy. Next to the expected general fields, the following
/// strategy specific fields are added:
/// * population: the population of the current generation
pub struct EvolveState<G: Genotype> {
    pub current_iteration: usize,
    pub current_generation: usize,
    pub best_generation: usize,
    pub best_chromosome: Option<Chromosome<G>>,

    pub population: Population<G>,
}

impl<
        G: Genotype,
        M: Mutate,
        F: Fitness<Genotype = G>,
        S: Crossover,
        C: Compete,
        E: Extension,
        SR: EvolveReporter<Genotype = G>,
    > Strategy<G> for Evolve<G, M, F, S, C, E, SR>
{
    fn call<R: Rng>(&mut self, rng: &mut R) {
        self.state = EvolveState::new(self.population_factory(rng));

        let mut fitness_thread_local: Option<ThreadLocal<RefCell<F>>> = None;
        if self.config.multithreading {
            fitness_thread_local = Some(ThreadLocal::new());
        }

        self.reporter.on_start(&self.state);
        while !self.is_finished() {
            self.state.current_generation += 1;
            self.state.population.increment_and_filter_age(&self.config);

            self.plugins.extension.call(
                &self.genotype,
                &self.config,
                &mut self.state.population,
                &mut self.reporter,
                rng,
            );
            self.plugins
                .crossover
                .call(&self.genotype, &mut self.state.population, rng);
            self.plugins.mutate.call(
                &self.genotype,
                &mut self.state.population,
                &mut self.reporter,
                rng,
            );
            self.fitness
                .call_for_population(&mut self.state.population, fitness_thread_local.as_ref());
            self.plugins
                .compete
                .call(&mut self.state.population, &self.config, rng);

            if let Some(contending_chromosome) = self
                .state
                .population
                .best_chromosome(self.config.fitness_ordering)
                .cloned()
            {
                if self
                    .state
                    .update_best_chromosome(
                        &contending_chromosome,
                        &self.config.fitness_ordering,
                        false,
                    )
                    .0
                {
                    self.reporter.on_new_best_chromosome(&self.state);
                }
            }
            //self.ensure_best_chromosome(population);
            self.reporter.on_new_generation(&self.state);
        }
        self.reporter.on_finish(&self.state);
    }
    fn best_chromosome(&self) -> Option<Chromosome<G>> {
        self.state.best_chromosome()
    }
    fn best_generation(&self) -> usize {
        self.state.best_generation
    }
    fn best_fitness_score(&self) -> Option<FitnessValue> {
        self.state.best_fitness_score()
    }
}

impl<
        G: Genotype,
        M: Mutate,
        F: Fitness<Genotype = G>,
        S: Crossover,
        C: Compete,
        E: Extension,
        SR: EvolveReporter<Genotype = G>,
    > Evolve<G, M, F, S, C, E, SR>
{
    pub fn builder() -> EvolveBuilder<G, M, F, S, C, E, SR> {
        EvolveBuilder::new()
    }

    #[allow(dead_code)]
    fn ensure_best_chromosome(&mut self, population: &mut Population<G>) {
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
            self.state.current_generation - self.state.best_generation >= max_stale_generations
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

    pub fn population_factory<R: Rng>(&mut self, rng: &mut R) -> Population<G> {
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
}

impl<G: Genotype> StrategyState<G> for EvolveState<G> {
    fn best_chromosome(&self) -> Option<Chromosome<G>> {
        self.best_chromosome.clone()
    }
    fn best_chromosome_as_ref(&self) -> Option<&Chromosome<G>> {
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
    fn set_best_chromosome(
        &mut self,
        best_chromosome: &Chromosome<G>,
        improved_fitness: bool,
    ) -> (bool, bool) {
        self.best_chromosome = Some(best_chromosome.clone());
        if improved_fitness {
            self.best_generation = self.current_generation;
        }
        (true, improved_fitness)
    }
}

impl<
        G: Genotype,
        M: Mutate,
        F: Fitness<Genotype = G>,
        S: Crossover,
        C: Compete,
        E: Extension,
        SR: EvolveReporter<Genotype = G>,
    > TryFrom<EvolveBuilder<G, M, F, S, C, E, SR>> for Evolve<G, M, F, S, C, E, SR>
{
    type Error = TryFromEvolveBuilderError;

    fn try_from(builder: EvolveBuilder<G, M, F, S, C, E, SR>) -> Result<Self, Self::Error> {
        if builder.genotype.is_none() {
            Err(TryFromEvolveBuilderError("Evolve requires a Genotype"))
        } else if builder.fitness.is_none() {
            Err(TryFromEvolveBuilderError("Evolve requires a Fitness"))
        } else if builder.reporter.is_none() {
            Err(TryFromEvolveBuilderError("Evolve requires a Reporter"))
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
        } else if !(builder.target_population_size > 0) {
            Err(TryFromEvolveBuilderError(
                "Evolve requires a target_population_size > 0",
            ))
        } else if builder.max_stale_generations.is_none() && builder.target_fitness_score.is_none()
        {
            Err(TryFromEvolveBuilderError(
                "Evolve requires at least a max_stale_generations or target_fitness_score ending condition",
            ))
        } else {
            Ok(Self {
                genotype: builder.genotype.unwrap(),
                fitness: builder.fitness.unwrap(),
                plugins: EvolvePlugins {
                    mutate: builder.mutate.unwrap(),
                    crossover: builder.crossover.unwrap(),
                    compete: builder.compete.unwrap(),
                    extension: builder.extension.unwrap(),
                },
                config: EvolveConfig {
                    target_population_size: builder.target_population_size,
                    max_stale_generations: builder.max_stale_generations,
                    max_chromosome_age: builder.max_chromosome_age,
                    target_fitness_score: builder.target_fitness_score,
                    valid_fitness_score: builder.valid_fitness_score,
                    fitness_ordering: builder.fitness_ordering,
                    multithreading: builder.multithreading,
                },
                state: EvolveState::default(),
                reporter: builder.reporter.unwrap(),
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
        }
    }
}

impl<G: Genotype> Default for EvolveState<G> {
    fn default() -> Self {
        Self {
            current_iteration: 0,
            current_generation: 0,
            best_generation: 0,
            best_chromosome: None,
            population: Population::new_empty(),
        }
    }
}

impl<G: Genotype> EvolveState<G> {
    pub fn new(population: Population<G>) -> Self {
        Self {
            population,
            ..Default::default()
        }
    }
}

impl<
        G: Genotype,
        M: Mutate,
        F: Fitness<Genotype = G>,
        S: Crossover,
        C: Compete,
        E: Extension,
        SR: EvolveReporter<Genotype = G>,
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

impl<G: Genotype> fmt::Display for EvolveState<G> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "evolve_state:")?;
        writeln!(f, "  current iteration: {:?}", self.current_iteration)?;
        writeln!(f, "  current generation: {:?}", self.current_generation)?;
        writeln!(f, "  best fitness score: {:?}", self.best_fitness_score())?;
        writeln!(f, "  best_chromosome: {:?}", self.best_chromosome.as_ref())
    }
}
