//! A solution strategy for finding the best chromosome, when crossover is impossible or inefficient
mod builder;
pub mod prelude;

pub use self::builder::{
    Builder as HillClimbBuilder, TryFromBuilderError as TryFromHillClimbBuilderError,
};

use super::Strategy;
use crate::chromosome::Chromosome;
use crate::fitness::{Fitness, FitnessOrdering, FitnessValue};
use crate::genotype::Genotype;
use rand::Rng;
use std::fmt;

/// Optimize by repeatedly mutating a single chromosome.
/// The fitness is calculated each round.
/// * If the fitness is worse, the mutation is undone and the next round is started
/// * If the fitness is equal or better, the mutated chromosome is taken for the next round.
///   It is important to update the best chromosome on equal fitness for diversity reasons
///
/// See [HillClimbBuilder] for initialization options.
///
/// Example:
/// ```
/// use genetic_algorithm::strategy::hill_climb::prelude::*;
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
/// let hill_climb = HillClimb::builder()
///     .with_genotype(genotype)
///     .with_fitness(CountTrue)           // count the number of true values in the chromosomes
///     .with_fitness_ordering(FitnessOrdering::Minimize) // aim for the least true values
///     .with_target_fitness_score(0)      // goal is 0 times true in the best chromosome
///     .with_max_stale_generations(1000)  // stop searching if there is no improvement in fitness score for 1000 generations
///     .call(&mut rng)
///     .unwrap();
///
/// // it's all about the best chromosome after all
/// let best_chromosome = hill_climb.best_chromosome().unwrap();
/// assert_eq!(best_chromosome.genes, vec![false; 16])
/// ```
pub struct HillClimb<G: Genotype, F: Fitness<Genotype = G>> {
    genotype: G,
    fitness: F,

    fitness_ordering: FitnessOrdering,
    max_stale_generations: Option<usize>,
    target_fitness_score: Option<FitnessValue>,

    current_generation: usize,
    best_chromosome: Option<Chromosome<G>>,
    pub best_generation: usize,
}

impl<G: Genotype, F: Fitness<Genotype = G>> Strategy<G> for HillClimb<G, F> {
    fn call<R: Rng>(&mut self, rng: &mut R) {
        self.current_generation = 0;
        self.best_generation = 0;
        self.best_chromosome = Some(self.genotype.chromosome_factory(rng));

        while !self.is_finished() {
            let working_chromosome = &mut self.best_chromosome().unwrap();
            self.genotype.mutate_chromosome(working_chromosome, rng);
            self.fitness.call_for_chromosome(working_chromosome);
            self.update_best_chromosome(working_chromosome);
            //self.report_round(working_chromosome);
            self.current_generation += 1;
        }
    }
    fn best_chromosome(&self) -> Option<Chromosome<G>> {
        self.best_chromosome.clone()
    }
}

impl<G: Genotype, F: Fitness<Genotype = G>> HillClimb<G, F> {
    pub fn builder() -> HillClimbBuilder<G, F> {
        HillClimbBuilder::new()
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
                        self.best_generation = self.current_generation;
                    }
                    (Some(current_fitness_score), Some(contending_fitness_score)) => {
                        match self.fitness_ordering {
                            FitnessOrdering::Maximize => {
                                if contending_fitness_score >= current_fitness_score {
                                    self.best_chromosome = Some(contending_best_chromosome.clone());
                                    if contending_fitness_score > current_fitness_score {
                                        self.best_generation = self.current_generation;
                                    }
                                }
                            }
                            FitnessOrdering::Minimize => {
                                if contending_fitness_score <= current_fitness_score {
                                    self.best_chromosome = Some(contending_best_chromosome.clone());
                                    if contending_fitness_score < current_fitness_score {
                                        self.best_generation = self.current_generation;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn is_finished(&self) -> bool {
        self.is_finished_by_max_stale_generations() || self.is_finished_by_target_fitness_score()
    }

    fn is_finished_by_max_stale_generations(&self) -> bool {
        if let Some(max_stale_generations) = self.max_stale_generations {
            self.current_generation - self.best_generation >= max_stale_generations
        } else {
            false
        }
    }

    fn is_finished_by_target_fitness_score(&self) -> bool {
        if let Some(target_fitness_score) = self.target_fitness_score {
            if let Some(fitness_score) = self.best_fitness_score() {
                match self.fitness_ordering {
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

    #[allow(dead_code)]
    fn report_round(&self, chromosome: &Chromosome<G>) {
        println!(
            "current generation: {}, best fitness score: {:?}, current fitness score: {:?}, genes: {:?}",
            self.current_generation,
            self.best_fitness_score(),
            chromosome.fitness_score,
            chromosome.genes,
        );
    }

    fn best_fitness_score(&self) -> Option<FitnessValue> {
        self.best_chromosome.as_ref().and_then(|c| c.fitness_score)
    }
}

impl<G: Genotype, F: Fitness<Genotype = G>> TryFrom<HillClimbBuilder<G, F>> for HillClimb<G, F> {
    type Error = TryFromHillClimbBuilderError;

    fn try_from(builder: HillClimbBuilder<G, F>) -> Result<Self, Self::Error> {
        if builder.genotype.is_none() {
            Err(TryFromHillClimbBuilderError(
                "HillClimb requires a Genotype",
            ))
        } else if builder.fitness.is_none() {
            Err(TryFromHillClimbBuilderError("HillClimb requires a Fitness"))
        } else if builder.max_stale_generations.is_none() && builder.target_fitness_score.is_none()
        {
            Err(TryFromHillClimbBuilderError(
                "HillClimb requires at least a max_stale_generations or target_fitness_score ending condition",
            ))
        } else {
            let genotype = builder.genotype.unwrap();

            Ok(Self {
                genotype: genotype,
                fitness: builder.fitness.unwrap(),

                fitness_ordering: builder.fitness_ordering,
                max_stale_generations: builder.max_stale_generations,
                target_fitness_score: builder.target_fitness_score,

                current_generation: 0,
                best_generation: 0,
                best_chromosome: None,
            })
        }
    }
}

impl<G: Genotype, F: Fitness<Genotype = G>> fmt::Display for HillClimb<G, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "hill_climb:")?;
        writeln!(f, "  genotype: {:?}", self.genotype)?;
        writeln!(f, "  fitness: {:?}", self.fitness)?;

        writeln!(
            f,
            "  max_stale_generations: {:?}",
            self.max_stale_generations
        )?;
        writeln!(f, "  target_fitness_score: {:?}", self.target_fitness_score)?;
        writeln!(f, "  fitness_ordering: {:?}", self.fitness_ordering)?;

        writeln!(f, "  current generation: {:?}", self.current_generation)?;
        writeln!(f, "  best fitness score: {:?}", self.best_fitness_score())?;
        writeln!(f, "  best_chromosome: {:?}", self.best_chromosome.as_ref())
    }
}
