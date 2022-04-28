use super::{Genotype, PermutableGenotype};
use crate::chromosome::Chromosome;
use crate::gene::{DiscreteGene, Gene};
use rand::distributions::uniform::SampleUniform;
use rand::distributions::{Distribution, Uniform};
use rand::prelude::*;
use std::fmt;

pub struct Range<T: Gene + SampleUniform> {
    pub gene_size: usize,
    pub gene_range: std::ops::Range<T>,
    gene_index_sampler: Uniform<usize>,
    gene_value_sampler: Uniform<T>,
}

impl<T: Gene + SampleUniform> Range<T> {
    pub fn new() -> Self {
        Self {
            gene_size: 0,
            gene_range: std::ops::Range::<T>::default(),
            gene_index_sampler: Uniform::from(0..=0),
            gene_value_sampler: Uniform::from(T::default()..=T::default()),
        }
    }

    pub fn with_gene_size(mut self, gene_size: usize) -> Self {
        self.gene_size = gene_size;
        self
    }

    pub fn with_gene_range(mut self, gene_range: std::ops::Range<T>) -> Self {
        self.gene_range = gene_range;
        self
    }

    pub fn build(mut self) -> Self {
        self.gene_index_sampler = Uniform::from(0..self.gene_size);
        self.gene_value_sampler = Uniform::from(self.gene_range.clone());
        self
    }
}

impl<T: Gene + SampleUniform> Genotype for Range<T> {
    type Gene = T;
    fn gene_size(&self) -> usize {
        self.gene_size
    }
    fn chromosome_factory<R: Rng>(&self, rng: &mut R) -> Chromosome<Range<T>> {
        let genes: Vec<T> = (0..self.gene_size)
            .map(|_| self.gene_value_sampler.sample(rng))
            .collect();
        Chromosome::new(genes)
    }

    fn mutate_chromosome<R: Rng>(&self, chromosome: &mut Chromosome<Range<T>>, rng: &mut R) {
        let index = self.gene_index_sampler.sample(rng);
        chromosome.genes[index] = self.gene_value_sampler.sample(rng);
        chromosome.taint_fitness_score();
    }
}

impl PermutableGenotype for Range<DiscreteGene> {
    fn gene_values(&self) -> Vec<DiscreteGene> {
        self.gene_range.clone().collect()
    }
}

impl<T: Gene + SampleUniform> fmt::Display for Range<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "genotype:\n")?;
        write!(f, "  gene_size: {}\n", self.gene_size)?;
        write!(f, "  gene_range: {:?}\n", self.gene_range)
    }
}
