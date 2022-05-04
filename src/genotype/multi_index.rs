use super::{Genotype, PermutableGenotype};
use crate::chromosome::Chromosome;
use crate::gene::IndexGene;
use crate::population::Population;
use itertools::Itertools;
use rand::distributions::{Distribution, Uniform, WeightedIndex};
use rand::prelude::*;
use std::fmt;

#[derive(Clone, Debug)]
pub struct MultiIndex {
    gene_size: usize,
    pub gene_value_sizes: Vec<IndexGene>,
    gene_index_sampler: WeightedIndex<usize>,
    gene_value_samplers: Vec<Uniform<IndexGene>>,
}

impl MultiIndex {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_gene_value_sizes(mut self, gene_value_sizes: Vec<IndexGene>) -> Self {
        self.gene_value_sizes = gene_value_sizes;
        self
    }

    pub fn build(mut self) -> Self {
        self.gene_size = self.gene_value_sizes.len();
        self.gene_index_sampler = WeightedIndex::new(self.gene_value_sizes.clone()).unwrap();
        self.gene_value_samplers = self
            .gene_value_sizes
            .iter()
            .map(|gene_value_size| Uniform::from(0..*gene_value_size))
            .collect();
        self
    }
}

impl Default for MultiIndex {
    fn default() -> Self {
        Self {
            gene_size: 0,
            gene_value_sizes: vec![],
            gene_index_sampler: WeightedIndex::new(vec![1]).unwrap(),
            gene_value_samplers: vec![],
        }
    }
}

impl Genotype for MultiIndex {
    type Gene = IndexGene;
    fn gene_size(&self) -> usize {
        self.gene_size
    }
    fn chromosome_factory<R: Rng>(&self, rng: &mut R) -> Chromosome<Self> {
        let genes: Vec<Self::Gene> = (0..self.gene_size)
            .map(|index| self.gene_value_samplers[index].sample(rng))
            .collect();
        Chromosome::new(genes)
    }

    fn mutate_chromosome<R: Rng>(&self, chromosome: &mut Chromosome<Self>, rng: &mut R) {
        let index = self.gene_index_sampler.sample(rng);
        chromosome.genes[index] = self.gene_value_samplers[index].sample(rng);
        chromosome.taint_fitness_score();
    }
}

impl PermutableGenotype for MultiIndex {
    //noop
    fn gene_values(&self) -> Vec<Self::Gene> {
        vec![]
    }
    fn population_factory(&self) -> Population<Self> {
        let chromosomes = self
            .gene_value_sizes
            .iter()
            .map(|gene_value_size| (0..*gene_value_size).collect::<Vec<Self::Gene>>())
            .multi_cartesian_product()
            .map(Chromosome::new)
            .collect();

        Population::new(chromosomes)
    }
    fn population_factory_size(&self) -> usize {
        self.gene_value_sizes
            .iter()
            .fold(1, |acc, gene_value_size| acc * gene_value_size)
    }
}

impl fmt::Display for MultiIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "genotype:")?;
        writeln!(f, "  gene_value_sizes: {:?}", self.gene_value_sizes)?;
        writeln!(f, "  gene_index_sampler: {:?}", self.gene_index_sampler)?;
        writeln!(f, "  gene_value_samplers: {:?}", self.gene_value_samplers)
    }
}
