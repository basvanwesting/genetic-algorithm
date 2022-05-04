use super::{Genotype, PermutableGenotype};
use crate::chromosome::Chromosome;
use crate::gene::IndexGene;
use rand::distributions::{Distribution, Uniform};
use rand::prelude::*;
use std::fmt;

#[derive(Clone, Debug)]
pub struct UniqueIndex {
    pub gene_value_size: IndexGene,
    gene_index_sampler: Uniform<usize>,
}

impl UniqueIndex {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_gene_value_size(mut self, gene_value_size: IndexGene) -> Self {
        self.gene_value_size = gene_value_size;
        self
    }

    pub fn build(mut self) -> Self {
        self.gene_index_sampler = Uniform::from(0..self.gene_value_size);
        self
    }
}

impl Default for UniqueIndex {
    fn default() -> Self {
        Self {
            gene_value_size: 0,
            gene_index_sampler: Uniform::from(0..=0),
        }
    }
}

impl Genotype for UniqueIndex {
    type Gene = IndexGene;
    fn gene_size(&self) -> usize {
        self.gene_value_size
    }
    fn chromosome_factory<R: Rng>(&self, rng: &mut R) -> Chromosome<Self> {
        let mut genes: Vec<Self::Gene> = (0..self.gene_value_size).collect();
        genes.shuffle(rng);
        Chromosome::new(genes)
    }

    fn mutate_chromosome<R: Rng>(&self, chromosome: &mut Chromosome<Self>, rng: &mut R) {
        let index1 = self.gene_index_sampler.sample(rng);
        let index2 = self.gene_index_sampler.sample(rng);
        chromosome.genes.swap(index1, index2);
        chromosome.taint_fitness_score();
    }

    fn is_unique(&self) -> bool {
        true
    }
}

impl PermutableGenotype for UniqueIndex {
    fn gene_values(&self) -> Vec<Self::Gene> {
        (0..self.gene_value_size).collect()
    }
}

impl fmt::Display for UniqueIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "genotype:")?;
        writeln!(f, "  gene_value_size: {}", self.gene_value_size)?;
        writeln!(f, "  gene_index_sampler: {:?}", self.gene_index_sampler)
    }
}
