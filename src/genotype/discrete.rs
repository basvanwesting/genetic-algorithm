use super::{Genotype, PermutableGenotype};
use crate::chromosome::Chromosome;
use crate::gene::DiscreteGene;
use itertools::Itertools;
use rand::distributions::{Distribution, Uniform};
use rand::prelude::*;
use std::fmt;

pub struct Discrete {
    pub gene_size: usize,
    pub gene_values: Vec<DiscreteGene>,
    gene_index_sampler: Uniform<usize>,
    gene_value_index_sampler: Uniform<usize>,
}

impl Discrete {
    pub fn new() -> Self {
        Self {
            gene_size: 0,
            gene_values: vec![],
            gene_index_sampler: Uniform::from(0..=0),
            gene_value_index_sampler: Uniform::from(0..=0),
        }
    }

    pub fn with_gene_size(mut self, gene_size: usize) -> Self {
        self.gene_size = gene_size;
        self
    }

    pub fn with_gene_values(mut self, gene_values: Vec<DiscreteGene>) -> Self {
        self.gene_values = gene_values;
        self
    }

    pub fn build(mut self) -> Self {
        self.gene_index_sampler = Uniform::from(0..self.gene_size);
        self.gene_value_index_sampler = Uniform::from(0..self.gene_values.len());
        self
    }
}

impl Genotype<DiscreteGene> for Discrete {
    fn gene_size(&self) -> usize {
        self.gene_size
    }
    fn chromosome_factory<R: Rng>(&self, rng: &mut R) -> Chromosome<DiscreteGene> {
        let genes: Vec<DiscreteGene> = (0..self.gene_size)
            .map(|_| self.gene_values[self.gene_value_index_sampler.sample(rng)])
            .collect();
        Chromosome::new(genes)
    }

    fn mutate_chromosome<R: Rng>(&self, chromosome: &mut Chromosome<DiscreteGene>, rng: &mut R) {
        let index = self.gene_index_sampler.sample(rng);
        chromosome.genes[index] = self.gene_values[self.gene_value_index_sampler.sample(rng)];
        chromosome.taint_fitness_score();
    }
}

impl PermutableGenotype<DiscreteGene> for Discrete {
    fn gene_values(&self) -> Vec<DiscreteGene> {
        self.gene_values.clone()
    }
}

impl fmt::Display for Discrete {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "genotype:\n")?;
        write!(f, "  gene_size: {}\n", self.gene_size)?;
        write!(f, "  gene_values: {}\n", self.gene_values.iter().join(","))
    }
}
