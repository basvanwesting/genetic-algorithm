use super::Genotype;
use crate::chromosome::Chromosome;
use crate::gene::DiscreteGene;
use itertools::Itertools;
use rand::prelude::*;
use rand::seq::SliceRandom;
use std::fmt;

pub struct DiscreteUnique {
    pub gene_values: Vec<DiscreteGene>,
}

impl DiscreteUnique {
    pub fn new() -> Self {
        Self {
            gene_values: vec![],
        }
    }

    pub fn with_gene_values(mut self, gene_values: Vec<DiscreteGene>) -> Self {
        self.gene_values = gene_values;
        self
    }
}

impl Genotype<DiscreteGene> for DiscreteUnique {
    fn gene_size(&self) -> usize {
        self.gene_values.len()
    }
    fn gene_values(&self) -> Vec<DiscreteGene> {
        self.gene_values.clone()
    }

    fn chromosome_factory<R: Rng>(&self, rng: &mut R) -> Chromosome<DiscreteGene> {
        let mut genes = self.gene_values.clone();
        genes.shuffle(rng);
        Chromosome::new(genes)
    }

    fn mutate_chromosome<R: Rng>(&self, chromosome: &mut Chromosome<DiscreteGene>, rng: &mut R) {
        let index1 = rng.gen_range(0..self.gene_size());
        let index2 = rng.gen_range(0..self.gene_size());
        chromosome.genes.swap(index1, index2);
        chromosome.taint_fitness_score();
    }
}

impl fmt::Display for DiscreteUnique {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "genotype:\n")?;
        write!(f, "  gene_size: {}\n", self.gene_size())?;
        write!(f, "  gene_values: {}\n", self.gene_values.iter().join(","))
    }
}
