use super::Compete;
use crate::fitness::{FitnessOrdering, FitnessValue};
use crate::genotype::Genotype;
use crate::population::Population;
use rand::prelude::*;
use std::cmp::Reverse;

/// Simply sort the chromosomes with fittest first. This approach has the risk of locking in to a local optimum.
///
/// Excess chromosomes, beyond the target_population_size, are dropped.
#[derive(Clone, Debug)]
pub struct Elite;
impl Compete for Elite {
    fn call<T: Genotype, R: Rng>(
        &self,
        population: &mut Population<T>,
        fitness_ordering: FitnessOrdering,
        target_population_size: usize,
        _rng: &mut R,
    ) {
        match fitness_ordering {
            FitnessOrdering::Maximize => population
                .chromosomes
                .sort_unstable_by_key(|c| c.fitness_score),
            FitnessOrdering::Minimize => {
                population
                    .chromosomes
                    .sort_unstable_by_key(|c| match c.fitness_score {
                        Some(fitness_score) => Reverse(fitness_score),
                        None => Reverse(FitnessValue::MAX),
                    })
            }
        }
        if population.size() > target_population_size {
            let to_drain_from_first = population.size() - target_population_size;
            population.chromosomes.drain(..to_drain_from_first);
        }
    }
}
