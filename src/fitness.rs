use crate::chromosome::Chromosome;

pub fn count_true_values(chromosome: &Chromosome) -> usize {
    chromosome.genes.iter().filter(|&gene| gene.0).count()
}
