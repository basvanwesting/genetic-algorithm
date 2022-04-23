use super::Genotype;
use crate::chromosome::Chromosome;
use crate::gene::BinaryGene;
use rand::prelude::*;
use std::fmt;

pub struct BinaryRandom {
    pub gene_size: usize,
}

impl BinaryRandom {
    pub fn new() -> Self {
        Self { gene_size: 0 }
    }

    pub fn with_gene_size(mut self, gene_size: usize) -> Self {
        self.gene_size = gene_size;
        self
    }
}

impl Genotype<BinaryGene> for BinaryRandom {
    fn gene_size(&self) -> usize {
        self.gene_size
    }
    fn gene_values(&self) -> Vec<BinaryGene> {
        vec![true, false]
    }
    fn chromosome_factory<R: Rng>(&self, rng: &mut R) -> Chromosome<BinaryGene> {
        let genes: Vec<BinaryGene> = (0..self.gene_size).map(|_| rng.gen()).collect();
        Chromosome::new(genes)
    }

    fn mutate_chromosome<R: Rng>(&self, chromosome: &mut Chromosome<BinaryGene>, rng: &mut R) {
        let index = rng.gen_range(0..self.gene_size);
        chromosome.genes[index] = !chromosome.genes[index];
        chromosome.taint_fitness_score();
    }
}

impl fmt::Display for BinaryRandom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "genotype:\n")?;
        write!(f, "  gene_size: {}\n", self.gene_size)
    }
}
