use super::{Genotype, PermutableGenotype};
use crate::chromosome::Chromosome;
use crate::gene::DiscreteGene;
use rand::prelude::*;
use std::fmt;

pub struct RangeUnique {
    pub gene_range: std::ops::RangeInclusive<DiscreteGene>,
}

impl RangeUnique {
    pub fn new() -> Self {
        Self {
            gene_range: std::ops::RangeInclusive::new(0, 0),
        }
    }

    pub fn with_gene_range(mut self, gene_range: std::ops::RangeInclusive<DiscreteGene>) -> Self {
        self.gene_range = gene_range;
        self
    }
}

impl Genotype<DiscreteGene> for RangeUnique {
    fn gene_size(&self) -> usize {
        self.gene_range.len()
    }
    fn chromosome_factory<R: Rng>(&self, rng: &mut R) -> Chromosome<DiscreteGene> {
        let mut genes: Vec<DiscreteGene> = self.gene_range.clone().collect();
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

impl PermutableGenotype<DiscreteGene> for RangeUnique {
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
