use super::{Genotype, PermutableGenotype};
use crate::chromosome::Chromosome;
use crate::gene::Gene;
use rand::distributions::{Distribution, Uniform};
use rand::prelude::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct UniqueDiscrete<T: Gene> {
    pub gene_values: Vec<T>,
    gene_index_sampler: Uniform<usize>,
}

impl<T: Gene> UniqueDiscrete<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_gene_values(mut self, gene_values: Vec<T>) -> Self {
        self.gene_values = gene_values;
        self
    }

    pub fn build(mut self) -> Self {
        self.gene_index_sampler = Uniform::from(0..self.gene_values.len());
        self
    }
}

impl<T: Gene> Default for UniqueDiscrete<T> {
    fn default() -> Self {
        Self {
            gene_values: vec![],
            gene_index_sampler: Uniform::from(0..=0),
        }
    }
}

impl<T: Gene> Genotype for UniqueDiscrete<T> {
    type Gene = T;
    fn gene_size(&self) -> usize {
        self.gene_values.len()
    }
    fn chromosome_factory<R: Rng>(&self, rng: &mut R) -> Chromosome<Self> {
        let mut genes = self.gene_values.clone();
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

impl<T: Gene> PermutableGenotype for UniqueDiscrete<T> {
    fn gene_values(&self) -> Vec<Self::Gene> {
        self.gene_values.clone()
    }
}

impl<T: Gene> fmt::Display for UniqueDiscrete<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "genotype:")?;
        writeln!(f, "  gene_values: {:?}", self.gene_values)?;
        writeln!(f, "  gene_index_sampler: {:?}", self.gene_index_sampler)
    }
}
