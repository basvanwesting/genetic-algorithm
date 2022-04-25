use super::Genotype;
use crate::chromosome::Chromosome;
use crate::gene::BinaryGene;
use crate::permutate::PermutableGenotype;
use rand::prelude::*;
use std::fmt;

pub struct Binary {
    pub gene_size: usize,
}

impl Binary {
    pub fn new() -> Self {
        Self { gene_size: 0 }
    }

    pub fn with_gene_size(mut self, gene_size: usize) -> Self {
        self.gene_size = gene_size;
        self
    }
}

impl Genotype<BinaryGene> for Binary {
    fn gene_size(&self) -> usize {
        self.gene_size
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

impl PermutableGenotype<BinaryGene> for Binary {
    fn gene_values_to_permutate(&self) -> Vec<BinaryGene> {
        vec![true, false]
    }
}

impl fmt::Display for Binary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "genotype:\n")?;
        write!(f, "  gene_size: {}\n", self.gene_size)
    }
}
