use super::Genotype;
use crate::chromosome::Chromosome;
use crate::gene::ContinuousGene;
use rand::prelude::*;
use std::fmt;

pub struct ContinuousRandom {
    pub gene_size: usize,
}

impl ContinuousRandom {
    pub fn new() -> Self {
        Self { gene_size: 0 }
    }

    pub fn with_gene_size(mut self, gene_size: usize) -> Self {
        self.gene_size = gene_size;
        self
    }
}

impl Genotype<ContinuousGene> for ContinuousRandom {
    fn gene_size(&self) -> usize {
        self.gene_size
    }
    fn gene_values(&self) -> Vec<ContinuousGene> {
        vec![]
    }

    fn chromosome_factory<R: Rng>(&self, rng: &mut R) -> Chromosome<ContinuousGene> {
        let genes: Vec<ContinuousGene> = (0..self.gene_size).map(|_| rng.gen()).collect();
        Chromosome::new(genes)
    }

    fn mutate_chromosome<R: Rng>(&self, chromosome: &mut Chromosome<ContinuousGene>, rng: &mut R) {
        let index = rng.gen_range(0..self.gene_size);
        chromosome.genes[index] = rng.gen();
        chromosome.taint_fitness_score();
    }
}

impl fmt::Display for ContinuousRandom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "genotype:\n")?;
        write!(f, "  gene_size: {}\n", self.gene_size)
    }
}
