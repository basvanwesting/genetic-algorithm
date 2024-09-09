//! A solution strategy for finding the best chromosome in case of small problem spaces (with a 100% guarantee)
mod builder;
pub mod prelude;
mod reporter;

pub use self::builder::{
    Builder as PermutateBuilder, TryFromBuilderError as TryFromPermutateBuilderError,
};

use super::{Strategy, StrategyAction, StrategyConfig, StrategyState};
use crate::fitness::{Fitness, FitnessOrdering, FitnessValue};
use crate::genotype::PermutableGenotype;
use crate::population::Population;
use num::BigUint;
use rayon::prelude::*;
use std::collections::HashMap;
use std::fmt;
use std::sync::mpsc::sync_channel;
use std::time::{Duration, Instant};

pub use self::reporter::Log as PermutateReporterLog;
pub use self::reporter::Noop as PermutateReporterNoop;
pub use self::reporter::Reporter as PermutateReporter;
pub use self::reporter::Simple as PermutateReporterSimple;

/// All possible combinations of genes are iterated over as chromosomes.
/// The fitness is calculated for each chromosome and the best is taken.
/// For efficiency reasons the full population is never instantiated as a whole.
///
/// The `chromosome_permutations_size` is subject to combinatorial explosion, so check the genotype
/// for practical values before using the [Permutate] strategy. This will not pose any memory
/// issues, as the permutations are not instantiated at the same time, just iterated over. But it
/// will take forever...
///
/// There are reporting hooks in the loop receiving the [PermutateState], which can by handled by an
/// [PermutateReporter] (e.g. [PermutateReporterNoop], [PermutateReporterSimple]). But you are encouraged to
/// roll your own, see [PermutateReporter].
///
/// See [PermutateBuilder] for initialization options.
///
/// All multithreading mechanisms are implemented using [rayon::iter] and [std::sync::mpsc].
///
/// Example:
/// ```
/// use genetic_algorithm::strategy::permutate::prelude::*;
/// use genetic_algorithm::fitness::placeholders::CountTrue;
///
/// // the search space
/// let genotype = BinaryGenotype::builder() // boolean alleles
///     .with_genes_size(16)                 // 16 genes per chromosome
///     .build()
///     .unwrap();
///
/// // the search strategy
/// let permutate = Permutate::builder()
///     .with_genotype(genotype)
///     .with_fitness(CountTrue)                          // count the number of true values in the chromosomes
///     .with_fitness_ordering(FitnessOrdering::Minimize) // aim for the least true values
///     .with_par_fitness(true)                           // optional, defaults to false, use parallel fitness calculation
///     .with_reporter(PermutateReporterSimple::new(100)) // optional builder step, report every 100 generations
///     .call()
///     .unwrap();
///
/// // it's all about the best chromosome after all
/// let best_chromosome = permutate.best_chromosome().unwrap();
/// assert_eq!(best_chromosome.genes, vec![false; 16])
/// ```
pub struct Permutate<
    G: PermutableGenotype,
    F: Fitness<Genotype = G>,
    SR: PermutateReporter<Genotype = G>,
> {
    pub genotype: G,
    pub fitness: F,
    pub config: PermutateConfig,
    pub state: PermutateState<G>,
    pub reporter: SR,
}

pub struct PermutateConfig {
    pub fitness_ordering: FitnessOrdering,
    pub par_fitness: bool,
    pub replace_on_equal_fitness: bool,
}

/// Stores the state of the Permutate strategy. Next to the expected general fields, the following
/// strategy specific fields are added:
/// * total_population_size: only the size as the full population is never instantiated simultaneously
pub struct PermutateState<G: PermutableGenotype> {
    pub current_iteration: usize,
    pub current_generation: usize,
    pub stale_generations: usize,
    pub best_generation: usize,
    pub best_chromosome: G::Chromosome,
    pub chromosome: G::Chromosome,
    pub population: Population<G::Chromosome>,
    pub durations: HashMap<StrategyAction, Duration>,

    pub total_population_size: BigUint,
}

impl<G: PermutableGenotype, F: Fitness<Genotype = G>, SR: PermutateReporter<Genotype = G>>
    Strategy<G> for Permutate<G, F, SR>
{
    fn call(&mut self) {
        let now = Instant::now();
        self.init();
        self.reporter.on_start(&self.state, &self.config);
        if self.config.par_fitness {
            self.call_parallel()
        } else {
            self.call_sequential()
        }
        self.state.close_duration(now.elapsed());
        self.reporter.on_finish(&self.state, &self.config);
    }
    fn best_chromosome(&self) -> Option<G::Chromosome> {
        if self
            .genotype
            .chromosome_is_empty(&self.state.best_chromosome)
        {
            None
        } else {
            Some(self.state.best_chromosome.clone())
        }
    }
    fn best_generation(&self) -> usize {
        self.state.best_generation
    }
    fn best_fitness_score(&self) -> Option<FitnessValue> {
        self.state.best_fitness_score()
    }
    fn best_genes(&self) -> Option<G::Genes> {
        if self.state.best_fitness_score().is_some() {
            Some(self.genotype.get_best_genes().clone())
        } else {
            None
        }
    }
}

impl<G: PermutableGenotype, F: Fitness<Genotype = G>> Permutate<G, F, PermutateReporterNoop<G>> {
    pub fn builder() -> PermutateBuilder<G, F, PermutateReporterNoop<G>> {
        PermutateBuilder::new()
    }
}

impl<G: PermutableGenotype, F: Fitness<Genotype = G>, SR: PermutateReporter<Genotype = G>>
    Permutate<G, F, SR>
{
    pub fn init(&mut self) {
        let now = Instant::now();
        self.reporter
            .on_init(&self.genotype, &self.state, &self.config);
        self.state.chromosome = self
            .genotype
            .chromosome_permutations_into_iter()
            .next()
            .unwrap();
        self.state.add_duration(StrategyAction::Init, now.elapsed());
        self.fitness
            .call_for_state_chromosome(&mut self.state, &self.genotype);
        self.state.best_generation = self.state.current_generation;
        self.state.best_chromosome = self.state.chromosome.clone();
        self.reporter
            .on_new_best_chromosome(&self.state, &self.config);
    }
    fn call_sequential(&mut self) {
        self.genotype
            .chromosome_permutations_into_iter()
            .for_each(|chromosome| {
                self.state.current_generation += 1;
                self.state.chromosome = chromosome;
                self.fitness
                    .call_for_state_chromosome(&mut self.state, &self.genotype);
                self.state
                    .update_best_chromosome_and_report(&self.config, &mut self.reporter);
                self.reporter.on_new_generation(&self.state, &self.config);
            });
    }
    fn call_parallel(&mut self) {
        rayon::scope(|s| {
            let genotype = &self.genotype;
            let fitness = self.fitness.clone();
            let (sender, receiver) = sync_channel(1000);

            s.spawn(move |_| {
                genotype
                    .chromosome_permutations_into_iter()
                    .par_bridge()
                    .for_each_with((sender, fitness), |(sender, fitness), mut chromosome| {
                        let now = Instant::now();
                        fitness.call_for_chromosome(&mut chromosome, genotype);
                        sender.send((chromosome, now.elapsed())).unwrap();
                    });
            });

            receiver.iter().for_each(|(chromosome, fitness_duration)| {
                self.state.current_generation += 1;
                self.state.chromosome = chromosome;
                self.state
                    .update_best_chromosome_and_report(&self.config, &mut self.reporter);
                self.state
                    .add_duration(StrategyAction::Fitness, fitness_duration);
            });
        });
    }
}

impl StrategyConfig for PermutateConfig {
    fn fitness_ordering(&self) -> FitnessOrdering {
        self.fitness_ordering
    }
    fn par_fitness(&self) -> bool {
        self.par_fitness
    }
    fn replace_on_equal_fitness(&self) -> bool {
        self.replace_on_equal_fitness
    }
}

impl<G: PermutableGenotype> StrategyState<G> for PermutateState<G> {
    fn chromosome_as_ref(&self) -> &G::Chromosome {
        &self.chromosome
    }
    fn chromosome_as_mut(&mut self) -> &mut G::Chromosome {
        &mut self.chromosome
    }
    fn population_as_ref(&self) -> &Population<G::Chromosome> {
        &self.population
    }
    fn population_as_mut(&mut self) -> &mut Population<G::Chromosome> {
        &mut self.population
    }
    fn best_chromosome_as_ref(&self) -> &G::Chromosome {
        &self.best_chromosome
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
    fn add_duration(&mut self, action: StrategyAction, duration: Duration) {
        *self.durations.entry(action).or_default() += duration;
    }
    fn total_duration(&self) -> Duration {
        self.durations.values().sum()
    }
}

impl<G: PermutableGenotype> PermutateState<G> {
    fn update_best_chromosome_and_report<SR: PermutateReporter<Genotype = G>>(
        &mut self,
        config: &PermutateConfig,
        reporter: &mut SR,
    ) {
        let now = Instant::now();
        match self.is_better_chromosome(
            &self.chromosome,
            &config.fitness_ordering,
            config.replace_on_equal_fitness,
        ) {
            (true, true) => {
                self.best_generation = self.current_generation;
                std::mem::swap(&mut self.chromosome, &mut self.best_chromosome);
                reporter.on_new_best_chromosome(self, config);
                self.reset_stale_generations();
            }
            (true, false) => {
                std::mem::swap(&mut self.chromosome, &mut self.best_chromosome);
                reporter.on_new_best_chromosome_equal_fitness(self, config);
                self.increment_stale_generations()
            }
            _ => self.increment_stale_generations(),
        }
        self.add_duration(StrategyAction::UpdateBestChromosome, now.elapsed());
    }
}

impl<G: PermutableGenotype, F: Fitness<Genotype = G>, SR: PermutateReporter<Genotype = G>>
    TryFrom<PermutateBuilder<G, F, SR>> for Permutate<G, F, SR>
{
    type Error = TryFromPermutateBuilderError;

    fn try_from(builder: PermutateBuilder<G, F, SR>) -> Result<Self, Self::Error> {
        if builder.genotype.is_none() {
            Err(TryFromPermutateBuilderError(
                "Permutate requires a Genotype",
            ))
        } else if builder.fitness.is_none() {
            Err(TryFromPermutateBuilderError("Permutate requires a Fitness"))
        } else {
            let genotype = builder.genotype.unwrap();
            let state = PermutateState::new(&genotype);

            Ok(Self {
                genotype,
                fitness: builder.fitness.unwrap(),

                config: PermutateConfig {
                    fitness_ordering: builder.fitness_ordering,
                    par_fitness: builder.par_fitness,
                    replace_on_equal_fitness: builder.replace_on_equal_fitness,
                },
                state,
                reporter: builder.reporter,
            })
        }
    }
}

impl Default for PermutateConfig {
    fn default() -> Self {
        Self {
            fitness_ordering: FitnessOrdering::Maximize,
            par_fitness: false,
            replace_on_equal_fitness: false,
        }
    }
}
impl PermutateConfig {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<G: PermutableGenotype> PermutateState<G> {
    pub fn new(genotype: &G) -> Self {
        let total_population_size = genotype.chromosome_permutations_size();
        Self {
            total_population_size,
            current_iteration: 0,
            current_generation: 0,
            stale_generations: 0,
            best_generation: 0,
            best_chromosome: genotype.chromosome_constructor_empty(), //invalid, temporary
            chromosome: genotype.chromosome_constructor_empty(),      //invalid, temporary
            population: Population::new_empty(),
            durations: HashMap::new(),
        }
    }
}

impl<G: PermutableGenotype, F: Fitness<Genotype = G>, SR: PermutateReporter<Genotype = G>>
    fmt::Display for Permutate<G, F, SR>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "permutate:")?;
        writeln!(f, "  genotype: {:?}", self.genotype)?;
        writeln!(f, "  fitness: {:?}", self.fitness)?;

        writeln!(f, "{}", self.config)?;
        writeln!(f, "{}", self.state)
    }
}

impl fmt::Display for PermutateConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "permutate_config:")?;
        writeln!(f, "  fitness_ordering: {:?}", self.fitness_ordering)?;
        writeln!(f, "  par_fitness: {:?}", self.par_fitness)
    }
}

impl<G: PermutableGenotype> fmt::Display for PermutateState<G> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "permutate_state:")?;
        writeln!(f, "  total_population_size: {}", self.total_population_size)?;
        writeln!(f, "  current iteration: -")?;
        writeln!(f, "  current generation: {:?}", self.current_generation)?;
        writeln!(f, "  best fitness score: {:?}", self.best_fitness_score())
    }
}
