use crate::chromosome::Chromosome;

pub fn count_true_values(chromosome: &Chromosome<bool>) -> usize {
    chromosome.genes.iter().filter(|&gene| gene.0).count()
}

pub fn sum_values(chromosome: &Chromosome<u8>) -> usize {
    chromosome.genes.iter().map(|&gene| gene.0 as usize).sum()
}
