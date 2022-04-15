use crate::chromosome::Chromosome;
use crate::gene::{BinaryGene, ContinuousGene, DiscreteGene, Gene};
use crate::population::Population;

pub trait Fitness<T: Gene> {
    fn call_for_population(&self, population: &mut Population<T>) {
        population
            .chromosomes
            .iter_mut()
            .for_each(|c| c.fitness = Some(self.call_for_chromosome(c)));
    }
    fn call_for_chromosome(&self, chromosome: &Chromosome<T>) -> usize;
}

//pub struct Null;
//impl<T: Gene> Fitness<T> for Null {
//fn call_for_chromosome(&self, _chromosome: &Chromosome<T>) -> usize {
//0
//}
//}

pub struct FitnessSimpleSum;
impl Fitness<BinaryGene> for FitnessSimpleSum {
    fn call_for_chromosome(&self, chromosome: &Chromosome<BinaryGene>) -> usize {
        chromosome.genes.iter().filter(|&value| *value).count()
    }
}

impl Fitness<DiscreteGene> for FitnessSimpleSum {
    fn call_for_chromosome(&self, chromosome: &Chromosome<DiscreteGene>) -> usize {
        chromosome.genes.iter().map(|&value| value as usize).sum()
    }
}

impl Fitness<ContinuousGene> for FitnessSimpleSum {
    fn call_for_chromosome(&self, chromosome: &Chromosome<ContinuousGene>) -> usize {
        chromosome.genes.iter().sum::<f32>() as usize
    }
}
