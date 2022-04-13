use crate::chromosome::Chromosome;

pub fn simple_sum(chromosome: &Chromosome) -> usize {
    chromosome.genes.iter().filter(|&gene| gene.0).count()
}
