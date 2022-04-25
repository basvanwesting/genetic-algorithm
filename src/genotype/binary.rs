use super::{Genotype, PermutableGenotype};
use crate::chromosome::Chromosome;
use crate::gene::BinaryGene;
use rand::distributions::{Bernoulli, Distribution, Uniform};
use rand::Rng;
use std::fmt;

pub struct Binary {
    pub gene_size: usize,
    pub gene_index_sampler: Uniform<usize>,
    gene_value_sampler: Bernoulli,
}

impl Binary {
    pub fn new() -> Self {
        Self {
            gene_size: 0,
            gene_index_sampler: Uniform::from(0..=0),
            gene_value_sampler: Bernoulli::new(0.0).unwrap(),
        }
    }

    pub fn with_gene_size(mut self, gene_size: usize) -> Self {
        self.gene_size = gene_size;
        self
    }

    pub fn build(mut self) -> Self {
        self.gene_index_sampler = Uniform::from(0..self.gene_size);
        self.gene_value_sampler = Bernoulli::new(0.5).unwrap();
        self
    }
}

impl Genotype<BinaryGene> for Binary {
    fn gene_size(&self) -> usize {
        self.gene_size
    }
    fn chromosome_factory<R: Rng>(&self, rng: &mut R) -> Chromosome<BinaryGene> {
        let genes: Vec<BinaryGene> = (0..self.gene_size)
            .map(|_| self.gene_value_sampler.sample(rng))
            .collect();
        Chromosome::new(genes)
    }

    fn mutate_chromosome<R: Rng>(&self, chromosome: &mut Chromosome<BinaryGene>, rng: &mut R) {
        let index = self.gene_index_sampler.sample(rng);
        chromosome.genes[index] = !chromosome.genes[index];
        chromosome.taint_fitness_score();
    }
}

impl PermutableGenotype<BinaryGene> for Binary {
    fn gene_values(&self) -> Vec<BinaryGene> {
        vec![true, false]
    }
}

impl fmt::Display for Binary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "genotype:\n")?;
        write!(f, "  gene_size: {}\n", self.gene_size)?;
        write!(f, "  gene_index_sampler: {:?}\n", self.gene_index_sampler)?;
        write!(f, "  gene_value_sampler: {:?}\n", self.gene_value_sampler)
    }
}
