use super::Genotype;
use crate::chromosome::Chromosome;
use crate::gene::ContinuousGene;
use rand::distributions::{Distribution, Uniform};
use rand::prelude::*;
use std::fmt;

pub struct Continuous {
    pub gene_size: usize,
    gene_index_sampler: Uniform<usize>,
}

impl Continuous {
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

impl Genotype<ContinuousGene> for Continuous {
    fn gene_size(&self) -> usize {
        self.gene_size
    }
    fn chromosome_factory<R: Rng>(&self, rng: &mut R) -> Chromosome<ContinuousGene> {
        let genes: Vec<ContinuousGene> = (0..self.gene_size).map(|_| rng.gen()).collect();
        Chromosome::new(genes)
    }

    fn mutate_chromosome<R: Rng>(&self, chromosome: &mut Chromosome<ContinuousGene>, rng: &mut R) {
        let index = self.gene_index_sampler.sample(rng);
        chromosome.genes[index] = rng.gen();
        chromosome.taint_fitness_score();
    }
}

impl fmt::Display for Continuous {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "genotype:\n")?;
        write!(f, "  gene_size: {}\n", self.gene_size)
    }
}
