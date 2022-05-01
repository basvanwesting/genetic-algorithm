use super::{Genotype, PermutableGenotype};
use crate::chromosome::Chromosome;
use crate::gene::DiscreteGene;
use rand::distributions::{Distribution, Uniform};
use rand::prelude::*;
use std::fmt;

#[derive(Debug)]
pub struct UniqueIndex {
    pub gene_size: usize,
    gene_index_sampler: Uniform<usize>,
}

impl UniqueIndex {
    pub fn new() -> Self {
        Self {
            gene_size: 0,
            gene_index_sampler: Uniform::from(0..=0),
        }
    }

    pub fn with_gene_size(mut self, gene_size: usize) -> Self {
        self.gene_size = gene_size;
        self
    }

    pub fn build(mut self) -> Self {
        self.gene_index_sampler = Uniform::from(0..self.gene_size);
        self
    }
}

impl Genotype for UniqueIndex {
    type Gene = DiscreteGene;
    fn gene_size(&self) -> usize {
        self.gene_size
    }
    fn chromosome_factory<R: Rng>(&self, rng: &mut R) -> Chromosome<UniqueIndex> {
        let mut genes: Vec<DiscreteGene> = (0..self.gene_size as Self::Gene).collect();
        genes.shuffle(rng);
        Chromosome::new(genes)
    }

    fn mutate_chromosome<R: Rng>(&self, chromosome: &mut Chromosome<UniqueIndex>, rng: &mut R) {
        let index1 = self.gene_index_sampler.sample(rng);
        let index2 = self.gene_index_sampler.sample(rng);
        chromosome.genes.swap(index1, index2);
        chromosome.taint_fitness_score();
    }
}

impl PermutableGenotype for UniqueIndex {
    fn gene_values(&self) -> Vec<Self::Gene> {
        (0..self.gene_size as Self::Gene).collect()
    }
}

impl fmt::Display for UniqueIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "genotype:\n")?;
        write!(f, "  gene_size: {}\n", self.gene_size)?;
        write!(f, "  gene_index_sampler: {:?}\n", self.gene_index_sampler)
    }
}
