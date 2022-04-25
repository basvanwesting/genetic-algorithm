use super::{Genotype, PermutableGenotype};
use crate::chromosome::Chromosome;
use crate::gene::DiscreteGene;
use itertools::Itertools;
use rand::distributions::{Distribution, Uniform};
use rand::prelude::*;
use rand::seq::SliceRandom;
use std::fmt;

pub struct DiscreteUnique {
    pub gene_values: Vec<DiscreteGene>,
    gene_index_sampler: Uniform<usize>,
}

impl DiscreteUnique {
    pub fn new() -> Self {
        Self {
            gene_values: vec![],
            gene_index_sampler: Uniform::from(0..=0),
        }
    }

    pub fn with_gene_values(mut self, gene_values: Vec<DiscreteGene>) -> Self {
        self.gene_values = gene_values;
        self
    }

    pub fn build(mut self) -> Self {
        self.gene_index_sampler = Uniform::from(0..self.gene_values.len());
        self
    }
}

impl Genotype<DiscreteGene> for DiscreteUnique {
    fn gene_size(&self) -> usize {
        self.gene_values.len()
    }
    fn chromosome_factory<R: Rng>(&self, rng: &mut R) -> Chromosome<DiscreteGene> {
        let mut genes = self.gene_values.clone();
        genes.shuffle(rng);
        Chromosome::new(genes)
    }

    fn mutate_chromosome<R: Rng>(&self, chromosome: &mut Chromosome<DiscreteGene>, rng: &mut R) {
        let index1 = self.gene_index_sampler.sample(rng);
        let index2 = self.gene_index_sampler.sample(rng);
        chromosome.genes.swap(index1, index2);
        chromosome.taint_fitness_score();
    }
}

impl PermutableGenotype<DiscreteGene> for DiscreteUnique {
    fn gene_values(&self) -> Vec<DiscreteGene> {
        self.gene_values.clone()
    }
}

impl fmt::Display for DiscreteUnique {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "genotype:\n")?;
        write!(f, "  gene_size: {}\n", self.gene_size())?;
        write!(f, "  gene_values: {}\n", self.gene_values.iter().join(","))
    }
}
