use super::Compete;
use crate::fitness::FitnessOrdering;
use crate::genotype::Genotype;
use crate::population::Population;
use rand::prelude::*;

#[derive(Clone, Debug)]
pub struct Elite;
impl Compete for Elite {
    fn call<T: Genotype, R: Rng>(
        &self,
        mut population: Population<T>,
        fitness_ordering: FitnessOrdering,
        target_population_size: usize,
        _rng: &mut R,
    ) -> Population<T> {
        population.sort(fitness_ordering);
        if population.size() > target_population_size {
            let to_drain_from_first = population.size() - target_population_size;
            population.chromosomes.drain(..to_drain_from_first);
        }
        population
    }
}
