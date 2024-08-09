//! A solution strategy for finding the best chromosome in case of small problem spaces (with a 100% guarantee)
mod builder;
pub mod prelude;
mod reporter;

pub use self::builder::{
    Builder as PermutateBuilder, TryFromBuilderError as TryFromPermutateBuilderError,
};

use super::{Strategy, StrategyConfig, StrategyState};
use crate::chromosome::Chromosome;
use crate::fitness::{Fitness, FitnessOrdering, FitnessValue};
use crate::genotype::PermutableGenotype;
use crossbeam::channel::bounded;
use num::BigUint;
use rand::Rng;
use std::fmt;

pub use self::reporter::Log as PermutateReporterLog;
pub use self::reporter::Noop as PermutateReporterNoop;
pub use self::reporter::Reporter as PermutateReporter;
pub use self::reporter::Simple as PermutateReporterSimple;

/// All possible combinations of genes are iterated over as chromosomes.
/// The fitness is calculated for each chromosome and the best is taken.
/// For efficiency reasons the full population is never instantiated as a whole.
///
/// The `chromosome_permutations_size` is subject to combinatorial explosion, so check the genotype
/// for practical values before using the [Permutate] strategy.
///
/// There are reporting hooks in the loop receiving the [PermutateState], which can by handled by an
/// [PermutateReporter] (e.g. [PermutateReporterNoop], [PermutateReporterSimple]). But you are encouraged to
/// roll your own, see [PermutateReporter].
///
/// See [PermutateBuilder] for initialization options.
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
/// let mut rng = rand::thread_rng(); // unused randomness provider implementing Trait rand::Rng
/// let permutate = Permutate::builder()
///     .with_genotype(genotype)
///     .with_fitness(CountTrue)                          // count the number of true values in the chromosomes
///     .with_fitness_ordering(FitnessOrdering::Minimize) // aim for the least true values
///     .with_reporter(PermutateReporterSimple::new(100)) // optional builder step, report every 100 generations
///     .with_multithreading(true)                        // use all cores
///     .call(&mut rng)
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
    genotype: G,
    fitness: F,
    pub config: PermutateConfig,
    pub state: PermutateState<G>,
    reporter: SR,
}

pub struct PermutateConfig {
    pub fitness_ordering: FitnessOrdering,
    pub multithreading: bool,
}

/// Stores the state of the Permutate strategy. Next to the expected general fields, the following
/// strategy specific fields are added:
/// * total_population_size: only the size as the full population is never instantiated simultaneously
pub struct PermutateState<G: PermutableGenotype> {
    pub current_iteration: usize,
    pub current_generation: usize,
    pub best_generation: usize,
    pub best_chromosome: Option<Chromosome<G>>,

    pub total_population_size: BigUint,
}

impl<G: PermutableGenotype, F: Fitness<Genotype = G>, SR: PermutateReporter<Genotype = G>>
    Strategy<G> for Permutate<G, F, SR>
{
    fn call<R: Rng>(&mut self, rng: &mut R) {
        self.reporter.on_start(&self.state);
        if self.config.multithreading {
            self.call_multi_thread(rng)
        } else {
            self.call_single_thread(rng)
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

impl<G: PermutableGenotype, F: Fitness<Genotype = G>> Permutate<G, F, PermutateReporterNoop<G>> {
    pub fn builder() -> PermutateBuilder<G, F, PermutateReporterNoop<G>> {
        PermutateBuilder::new()
    }
}

impl<G: PermutableGenotype, F: Fitness<Genotype = G>, SR: PermutateReporter<Genotype = G>>
    Permutate<G, F, SR>
{
    fn call_single_thread<R: Rng>(&mut self, _rng: &mut R) {
        for mut chromosome in self.genotype.clone().chromosome_permutations_into_iter() {
            self.state.current_generation += 1;
            self.fitness.call_for_chromosome(&mut chromosome);
            if self
                .state
                .update_best_chromosome(&chromosome, &self.config.fitness_ordering, false)
                .0
            {
                self.reporter.on_new_best_chromosome(&self.state);
            }
            self.reporter.on_new_generation(&self.state);
        }
    }
    fn call_multi_thread<R: Rng>(&mut self, _rng: &mut R) {
        crossbeam::scope(|s| {
            let number_of_threads = rayon::current_num_threads();
            let (unprocessed_chromosome_sender, unprocessed_chromosome_receiver) =
                bounded(number_of_threads * 100);
            let (processed_chromosome_sender, processed_chromosome_receiver) =
                bounded(number_of_threads * 100);

            let thread_genotype = self.genotype.clone();
            s.spawn(move |_| {
                for chromosome in thread_genotype.chromosome_permutations_into_iter() {
                    unprocessed_chromosome_sender.send(chromosome).unwrap();
                }
                drop(unprocessed_chromosome_sender);
            });

            for _i in 0..number_of_threads {
                let mut fitness = self.fitness.clone();
                let unprocessed_chromosome_receiver = unprocessed_chromosome_receiver.clone();
                let processed_chromosome_sender = processed_chromosome_sender.clone();
                s.spawn(move |_| {
                    for mut chromosome in unprocessed_chromosome_receiver {
                        fitness.call_for_chromosome(&mut chromosome);
                        processed_chromosome_sender.send(chromosome).unwrap();
                    }
                });
            }

            s.spawn(|_| {
                for chromosome in processed_chromosome_receiver {
                    self.state.current_generation += 1;
                    if self
                        .state
                        .update_best_chromosome(&chromosome, &self.config.fitness_ordering, false)
                        .0
                    {
                        self.reporter.on_new_best_chromosome(&self.state);
                    }
                    self.reporter.on_new_generation(&self.state);
                }
            });
        })
        .unwrap();
    }
}

impl StrategyConfig for PermutateConfig {
    fn fitness_ordering(&self) -> FitnessOrdering {
        self.fitness_ordering
    }
    fn multithreading(&self) -> bool {
        self.multithreading
    }
}

impl<G: PermutableGenotype> StrategyState<G> for PermutateState<G> {
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
            let total_population_size = genotype.chromosome_permutations_size();

            Ok(Self {
                genotype,
                fitness: builder.fitness.unwrap(),

                config: PermutateConfig {
                    fitness_ordering: builder.fitness_ordering,
                    multithreading: builder.multithreading,
                },
                state: PermutateState {
                    total_population_size,
                    ..Default::default()
                },
                reporter: builder.reporter,
            })
        }
    }
}

impl Default for PermutateConfig {
    fn default() -> Self {
        Self {
            fitness_ordering: FitnessOrdering::Maximize,
            multithreading: false,
        }
    }
}
impl PermutateConfig {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<G: PermutableGenotype> Default for PermutateState<G> {
    fn default() -> Self {
        Self {
            total_population_size: BigUint::default(),
            current_iteration: 0,
            current_generation: 0,
            best_generation: 0,
            best_chromosome: None,
        }
    }
}
impl<G: PermutableGenotype> PermutateState<G> {
    pub fn new() -> Self {
        Self::default()
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
        writeln!(f, "  multithreading: {:?}", self.multithreading)
    }
}

impl<G: PermutableGenotype> fmt::Display for PermutateState<G> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "permutate_state:")?;
        writeln!(f, "  total_population_size: {}", self.total_population_size)?;
        writeln!(f, "  current iteration: -")?;
        writeln!(f, "  current generation: {:?}", self.current_generation)?;
        writeln!(f, "  best fitness score: {:?}", self.best_fitness_score())?;
        writeln!(f, "  best_chromosome: {:?}", self.best_chromosome.as_ref())
    }
}
