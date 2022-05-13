//! A solution strategy for finding the best chromosomes in case of small problem spaces.
mod builder;
pub mod prelude;

pub use self::builder::{
    Builder as PermutateBuilder, TryFromBuilderError as TryFromPermutateBuilderError,
};

use crate::chromosome::Chromosome;
use crate::fitness::{Fitness, FitnessOrdering, FitnessValue};
use crate::genotype::PermutableGenotype;
use num::BigUint;
use std::fmt;

/// All possible combinations of genes are iterated over as chromosomes.
/// The fitness is calculated for each chromosome and the best is taken.
/// For efficiency reasons the full population is never instantiated as a whole.
///
/// See [PermutateBuilder] for initialization options.
///
/// Example:
/// ```
/// use genetic_algorithm::permutate::prelude::*;
/// use genetic_algorithm::fitness::placeholders::CountTrue;
///
/// // the search space
/// let genotype = BinaryGenotype::builder() // boolean genes
///     .with_gene_size(16)                  // 16 of them
///     .build()
///     .unwrap();
///
/// // the search strategy
/// let permutate = Permutate::builder()
///     .with_genotype(genotype)
///     .with_fitness(CountTrue)                          // count the number of true values in the chromosomes
///     .with_fitness_ordering(FitnessOrdering::Minimize) // aim for the least true values
///     .build()
///     .unwrap()
///     .call();
///
/// // it's all about the best chromosome after all
/// let best_chromosome = permutate.best_chromosome.unwrap();
/// assert_eq!(best_chromosome.genes, vec![false; 16])
/// ```
pub struct Permutate<G: PermutableGenotype, F: Fitness<Genotype = G>> {
    pub genotype: G,
    pub fitness: F,

    pub fitness_ordering: FitnessOrdering,

    pub best_chromosome: Option<Chromosome<G>>,
    pub population_size: BigUint,
}

impl<G: PermutableGenotype, F: Fitness<Genotype = G>> Permutate<G, F> {
    pub fn builder() -> PermutateBuilder<G, F> {
        PermutateBuilder::new()
    }

    pub fn call(mut self) -> Self {
        for mut chromosome in self.genotype.clone().chromosome_permutations_into_iter() {
            self.fitness.call_for_chromosome(&mut chromosome);
            self.update_best_chromosome(&chromosome);
        }
        self
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

        writeln!(f, "  best fitness score: {:?}", self.best_fitness_score())?;
        writeln!(f, "  best_chromosome: {:?}", self.best_chromosome.as_ref())
    }
}
