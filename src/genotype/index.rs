use super::{Genotype, PermutableGenotype};
use crate::chromosome::Chromosome;
use crate::gene::IndexGene;
use rand::distributions::{Distribution, Uniform};
use rand::prelude::*;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Index {
    pub gene_size: usize,
    pub gene_value_size: IndexGene,
    gene_index_sampler: Uniform<usize>,
    gene_value_sampler: Uniform<IndexGene>,
}

impl Index {
    pub fn new() -> Self {
        Self {
            gene_size: 0,
            gene_value_size: 0,
            gene_index_sampler: Uniform::from(0..=0),
            gene_value_sampler: Uniform::from(0..=0),
        }
    }

    pub fn with_gene_size(mut self, gene_size: usize) -> Self {
        self.gene_size = gene_size;
        self
    }

    pub fn with_gene_value_size(mut self, gene_value_size: IndexGene) -> Self {
        self.gene_value_size = gene_value_size;
        self
    }

    pub fn build(mut self) -> Self {
        self.gene_index_sampler = Uniform::from(0..self.gene_size);
        self.gene_value_sampler = Uniform::from(0..self.gene_value_size);
        self
    }
}

impl Genotype for Index {
    type Gene = IndexGene;
    fn gene_size(&self) -> usize {
        self.gene_size
    }
    fn chromosome_factory<R: Rng>(&self, rng: &mut R) -> Chromosome<Self> {
        let genes: Vec<Self::Gene> = (0..self.gene_size)
            .map(|_| self.gene_value_sampler.sample(rng))
            .collect();
        Chromosome::new(genes)
    }

    fn mutate_chromosome<R: Rng>(&self, chromosome: &mut Chromosome<Self>, rng: &mut R) {
        let index = self.gene_index_sampler.sample(rng);
        chromosome.genes[index] = self.gene_value_sampler.sample(rng);
        chromosome.taint_fitness_score();
    }
}

impl PermutableGenotype for Index {
    fn gene_values(&self) -> Vec<Self::Gene> {
        (0..self.gene_value_size).collect()
    }
}

impl fmt::Display for Index {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "genotype:")?;
        writeln!(f, "  gene_size: {}", self.gene_size)?;
        writeln!(f, "  gene_value_size: {:?}", self.gene_value_size)?;
        writeln!(f, "  gene_index_sampler: {:?}", self.gene_index_sampler)?;
        writeln!(f, "  gene_value_sampler: {:?}", self.gene_value_sampler)
    }
}
