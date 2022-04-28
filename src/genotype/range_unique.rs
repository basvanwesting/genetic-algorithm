use super::{Genotype, PermutableGenotype};
use crate::chromosome::Chromosome;
use crate::gene::DiscreteGene;
use rand::distributions::{Distribution, Uniform};
use rand::prelude::*;
use std::fmt;

pub struct RangeUnique {
    pub gene_range: std::ops::Range<DiscreteGene>,
    gene_index_sampler: Uniform<usize>,
}

impl RangeUnique {
    pub fn new() -> Self {
        Self {
            gene_range: std::ops::Range::default(),
            gene_index_sampler: Uniform::from(0..=0),
        }
    }

    pub fn with_gene_range(mut self, gene_range: std::ops::Range<DiscreteGene>) -> Self {
        self.gene_range = gene_range;
        self
    }

    pub fn build(mut self) -> Self {
        self.gene_index_sampler = Uniform::from(0..self.gene_range.len());
        self
    }
}

impl Genotype for RangeUnique {
    type Gene = DiscreteGene;
    fn gene_size(&self) -> usize {
        self.gene_range.len()
    }
    fn chromosome_factory<R: Rng>(&self, rng: &mut R) -> Chromosome<RangeUnique> {
        let mut genes: Vec<DiscreteGene> = self.gene_range.clone().collect();
        genes.shuffle(rng);
        Chromosome::new(genes)
    }

    fn mutate_chromosome<R: Rng>(&self, chromosome: &mut Chromosome<RangeUnique>, rng: &mut R) {
        let index1 = self.gene_index_sampler.sample(rng);
        let index2 = self.gene_index_sampler.sample(rng);
        chromosome.genes.swap(index1, index2);
        chromosome.taint_fitness_score();
    }
}

impl PermutableGenotype for RangeUnique {
    fn gene_values(&self) -> Vec<DiscreteGene> {
        self.gene_range.clone().collect()
    }
}

impl fmt::Display for RangeUnique {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "genotype:\n")?;
        write!(f, "  gene_range: {:?}\n", self.gene_range)
    }
}
