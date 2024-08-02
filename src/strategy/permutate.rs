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

pub use self::reporter::Noop as PermutateReporterNoop;
pub use self::reporter::Simple as PermutateReporterSimple;

/// All possible combinations of genes are iterated over as chromosomes.
/// The fitness is calculated for each chromosome and the best is taken.
/// For efficiency reasons the full population is never instantiated as a whole.
///
/// The `chromosome_permutations_size` is subject to combinatorial explosion, so check the genotype
/// for practical values before using the [Permutate] strategy.
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
///     .with_reporter(PermutateReporterNoop::default())  // no reporting
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
    pub population_size: BigUint,
    pub fitness_ordering: FitnessOrdering,
    pub multithreading: bool,
}

#[derive(Clone)]
pub struct PermutateState<G: PermutableGenotype> {
    pub current_iteration: usize,
    pub current_generation: usize,
    pub best_generation: usize,
    pub best_chromosome: Option<Chromosome<G>>,
}

pub trait PermutateReporter: Clone + Send {
    type Genotype: PermutableGenotype;

    fn on_start(&mut self, _state: &PermutateState<Self::Genotype>) {}
    fn on_finish(&mut self, _state: &PermutateState<Self::Genotype>) {}
    fn on_new_generation(&mut self, _state: &PermutateState<Self::Genotype>) {}
    fn on_new_best_chromosome(&mut self, _state: &PermutateState<Self::Genotype>) {}
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

impl<G: PermutableGenotype, F: Fitness<Genotype = G>, SR: PermutateReporter<Genotype = G>>
    Permutate<G, F, SR>
{
    pub fn builder() -> PermutateBuilder<G, F, SR> {
        PermutateBuilder::new()
    }
    fn call_single_thread<R: Rng>(&mut self, _rng: &mut R) {
        for mut chromosome in self.genotype.clone().chromosome_permutations_into_iter() {
            self.state.current_generation += 1;
            self.fitness.call_for_chromosome(&mut chromosome);
            self.state
                .update_best_chromosome(&chromosome, &self.config.fitness_ordering, false);
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
                    self.state.update_best_chromosome(
                        &chromosome,
                        &self.config.fitness_ordering,
                        false,
                    );
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
    fn set_best_chromosome(
        &mut self,
        best_chromosome: &Chromosome<G>,
        set_best_generation: bool,
    ) -> bool {
        self.best_chromosome = Some(best_chromosome.clone());
        if set_best_generation {
            self.best_generation = self.current_generation;
        }
        true
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
            let population_size = genotype.chromosome_permutations_size();

            Ok(Self {
                genotype,
                fitness: builder.fitness.unwrap(),

                config: PermutateConfig {
                    population_size,
                    fitness_ordering: builder.fitness_ordering,
                    multithreading: builder.multithreading,
                },
                state: PermutateState::default(),
                reporter: builder.reporter.unwrap(),
            })
        }
    }
}

impl Default for PermutateConfig {
    fn default() -> Self {
        Self {
            population_size: BigUint::default(),
            fitness_ordering: FitnessOrdering::Maximize,
            multithreading: false,
        }
    }
}

impl<G: PermutableGenotype> Default for PermutateState<G> {
    fn default() -> Self {
        Self {
            current_iteration: 0,
            current_generation: 0,
            best_generation: 0,
            best_chromosome: None,
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
        writeln!(f, "  population_size: {}", self.population_size)?;
        writeln!(f, "  fitness_ordering: {:?}", self.fitness_ordering)?;
        writeln!(f, "  multithreading: {:?}", self.multithreading)
    }
}

impl<G: PermutableGenotype> fmt::Display for PermutateState<G> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "permutate_state:")?;
        writeln!(f, "  current iteration: -")?;
        writeln!(f, "  current generation: {:?}", self.current_generation)?;
        writeln!(f, "  best fitness score: {:?}", self.best_fitness_score())?;
        writeln!(f, "  best_chromosome: {:?}", self.best_chromosome.as_ref())
    }
}
