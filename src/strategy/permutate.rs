//! A solution strategy for finding the best chromosome in case of small problem spaces (with a 100% guarantee)
mod builder;
pub mod prelude;

pub use self::builder::{
    Builder as PermutateBuilder, TryFromBuilderError as TryFromPermutateBuilderError,
};

use super::Strategy;
use crate::chromosome::Chromosome;
use crate::fitness::{Fitness, FitnessOrdering, FitnessValue};
use crate::genotype::PermutableGenotype;
use crossbeam::channel::bounded;
use num::BigUint;
use rand::Rng;
use std::fmt;

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
///     .with_fitness_threads(4)                          // use 4 threads
///     .call(&mut rng)
///     .unwrap();
///
/// // it's all about the best chromosome after all
/// let best_chromosome = permutate.best_chromosome().unwrap();
/// assert_eq!(best_chromosome.genes, vec![false; 16])
/// ```
pub struct Permutate<G: PermutableGenotype, F: Fitness<Genotype = G>> {
    genotype: G,
    fitness: F,
    fitness_ordering: FitnessOrdering,
    fitness_threads: usize,

    pub population_size: BigUint,
    best_chromosome: Option<Chromosome<G>>,
}

impl<G: PermutableGenotype, F: Fitness<Genotype = G>> Strategy<G> for Permutate<G, F> {
    fn call<R: Rng>(&mut self, rng: &mut R) {
        if self.fitness_threads > 1 {
            self.call_multi_thread(rng)
        } else {
            self.call_single_thread(rng)
        }
    }
    fn best_chromosome(&self) -> Option<Chromosome<G>> {
        self.best_chromosome.clone()
    }
}

impl<G: PermutableGenotype, F: Fitness<Genotype = G>> Permutate<G, F> {
    pub fn builder() -> PermutateBuilder<G, F> {
        PermutateBuilder::new()
    }
    fn call_single_thread<R: Rng>(&mut self, _rng: &mut R) {
        for mut chromosome in self.genotype.clone().chromosome_permutations_into_iter() {
            self.fitness.call_for_chromosome(&mut chromosome);
            self.update_best_chromosome(&chromosome);
        }
    }
    fn call_multi_thread<R: Rng>(&mut self, _rng: &mut R) {
        crossbeam::scope(|s| {
            let (unprocessed_chromosome_sender, unprocessed_chromosome_receiver) =
                bounded(self.fitness_threads * 100);
            let (processed_chromosome_sender, processed_chromosome_receiver) =
                bounded(self.fitness_threads * 100);

            let thread_genotype = self.genotype.clone();
            s.spawn(move |_| {
                for chromosome in thread_genotype.chromosome_permutations_into_iter() {
                    unprocessed_chromosome_sender.send(chromosome).unwrap();
                }
                drop(unprocessed_chromosome_sender);
            });

            for _i in 0..self.fitness_threads {
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
                    self.update_best_chromosome(&chromosome);
                }
            });
        })
        .unwrap();
    }
    fn update_best_chromosome(&mut self, contending_best_chromosome: &Chromosome<G>) {
        match self.best_chromosome.as_ref() {
            None => {
                self.best_chromosome = Some(contending_best_chromosome.clone());
            }
            Some(current_best_chromosome) => {
                match (
                    current_best_chromosome.fitness_score,
                    contending_best_chromosome.fitness_score,
                ) {
                    (None, None) => {}
                    (Some(_), None) => {}
                    (None, Some(_)) => {
                        self.best_chromosome = Some(contending_best_chromosome.clone());
                    }
                    (Some(current_fitness_score), Some(contending_fitness_score)) => {
                        match self.fitness_ordering {
                            FitnessOrdering::Maximize => {
                                if contending_fitness_score > current_fitness_score {
                                    self.best_chromosome = Some(contending_best_chromosome.clone());
                                }
                            }
                            FitnessOrdering::Minimize => {
                                if contending_fitness_score < current_fitness_score {
                                    self.best_chromosome = Some(contending_best_chromosome.clone());
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn best_fitness_score(&self) -> Option<FitnessValue> {
        self.best_chromosome.as_ref().and_then(|c| c.fitness_score)
    }
}

impl<G: PermutableGenotype, F: Fitness<Genotype = G>> TryFrom<PermutateBuilder<G, F>>
    for Permutate<G, F>
{
    type Error = TryFromPermutateBuilderError;

    fn try_from(builder: PermutateBuilder<G, F>) -> Result<Self, Self::Error> {
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
                genotype: genotype,
                fitness: builder.fitness.unwrap(),

                fitness_ordering: builder.fitness_ordering,
                fitness_threads: builder.fitness_threads,

                best_chromosome: None,
                population_size: population_size,
            })
        }
    }
}

impl<G: PermutableGenotype, F: Fitness<Genotype = G>> fmt::Display for Permutate<G, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "permutate:")?;
        writeln!(f, "  genotype: {:?}", self.genotype)?;
        writeln!(f, "  fitness: {:?}", self.fitness)?;

        writeln!(f, "  population_size: {}", self.population_size)?;
        writeln!(f, "  fitness_ordering: {:?}", self.fitness_ordering)?;
        writeln!(f, "  fitness_threads: {:?}", self.fitness_threads)?;

        writeln!(f, "  best fitness score: {:?}", self.best_fitness_score())?;
        writeln!(f, "  best_chromosome: {:?}", self.best_chromosome.as_ref())
    }
}
