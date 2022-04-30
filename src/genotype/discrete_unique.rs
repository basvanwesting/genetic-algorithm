use super::{Genotype, PermutableGenotype};
use crate::chromosome::Chromosome;
use crate::gene::Gene;
use rand::distributions::{Distribution, Uniform};
use rand::prelude::*;
use rand::seq::SliceRandom;
use std::fmt;

#[derive(Debug)]
pub struct DiscreteUnique<T: Gene> {
    pub gene_values: Vec<T>,
    gene_index_sampler: Uniform<usize>,
}

impl<T: Gene> DiscreteUnique<T> {
    pub fn new() -> Self {
        Self {
            gene_values: vec![],
            gene_index_sampler: Uniform::from(0..=0),
        }
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

impl<T: Gene> Genotype for DiscreteUnique<T> {
    type Gene = T;
    fn gene_size(&self) -> usize {
        self.gene_values.len()
    }
    fn chromosome_factory<R: Rng>(&self, rng: &mut R) -> Chromosome<DiscreteUnique<T>> {
        let mut genes = self.gene_values.clone();
        genes.shuffle(rng);
        Chromosome::new(genes)
    }

    fn mutate_chromosome<R: Rng>(
        &self,
        chromosome: &mut Chromosome<DiscreteUnique<T>>,
        rng: &mut R,
    ) {
        let index1 = self.gene_index_sampler.sample(rng);
        let index2 = self.gene_index_sampler.sample(rng);
        chromosome.genes.swap(index1, index2);
        chromosome.taint_fitness_score();
    }
}

impl<T: Gene> PermutableGenotype for DiscreteUnique<T> {
    fn gene_values(&self) -> Vec<T> {
        self.gene_values.clone()
    }
}

impl<T: Gene> fmt::Display for DiscreteUnique<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "genotype:\n")?;
        write!(f, "  gene_size: {}\n", self.gene_size())?;
        write!(f, "  gene_values: {:?}\n", self.gene_values)
    }
}
