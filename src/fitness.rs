use crate::chromosome::Chromosome;
use crate::gene::{BinaryGene, DiscreteGene, Gene};

pub fn null<T: Gene>(_chromosome: &Chromosome<T>) -> usize {
    0
}

pub fn count_true_values(chromosome: &Chromosome<BinaryGene>) -> usize {
    chromosome.genes.iter().filter(|&value| *value).count()
}

pub fn sum_values(chromosome: &Chromosome<DiscreteGene>) -> usize {
    chromosome.genes.iter().map(|&value| value as usize).sum()
}
