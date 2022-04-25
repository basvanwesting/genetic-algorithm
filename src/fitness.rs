use crate::chromosome::Chromosome;
use crate::gene::{BinaryGene, ContinuousGene, DiscreteGene, Gene};
use crate::population::Population;

pub trait Fitness<T: Gene>: Clone + std::fmt::Debug {
    fn call_for_population(&self, mut population: Population<T>) -> Population<T> {
        population
            .chromosomes
            .iter_mut()
            .filter(|c| c.fitness_score.is_none())
            .for_each(|c| c.fitness_score = Some(self.call_for_chromosome(c)));
        population
    }
    fn call_for_chromosome(&self, chromosome: &Chromosome<T>) -> isize;
}

#[derive(Clone, Debug)]
pub struct SimpleSum;
impl Fitness<BinaryGene> for SimpleSum {
    fn call_for_chromosome(&self, chromosome: &Chromosome<BinaryGene>) -> isize {
        chromosome.genes.iter().filter(|&value| *value).count() as isize
    }
}

impl Fitness<DiscreteGene> for SimpleSum {
    fn call_for_chromosome(&self, chromosome: &Chromosome<DiscreteGene>) -> isize {
        chromosome.genes.iter().sum::<DiscreteGene>() as isize
    }
}

impl Fitness<ContinuousGene> for SimpleSum {
    fn call_for_chromosome(&self, chromosome: &Chromosome<ContinuousGene>) -> isize {
        chromosome.genes.iter().sum::<ContinuousGene>() as isize
    }
}
