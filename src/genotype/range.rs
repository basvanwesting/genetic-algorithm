use super::Genotype;
use crate::chromosome::Chromosome;
use crate::gene::{ContinuousGene, DiscreteGene, Gene};
use rand::prelude::*;
use std::fmt;

pub struct Range<T: Gene> {
    pub gene_size: usize,
    pub gene_range: std::ops::RangeInclusive<T>,
}

impl<T: Gene> Range<T> {
    pub fn new() -> Self {
        Self {
            gene_size: 0,
            gene_range: std::ops::RangeInclusive::<T>::new(T::default(), T::default()),
        }
    }

    pub fn with_gene_size(mut self, gene_size: usize) -> Self {
        self.gene_size = gene_size;
        self
    }

    pub fn with_gene_range(mut self, gene_range: std::ops::RangeInclusive<T>) -> Self {
        self.gene_range = gene_range;
        self
    }
}

impl Genotype<DiscreteGene> for Range<DiscreteGene> {
    fn gene_size(&self) -> usize {
        self.gene_size
    }
    fn gene_values(&self) -> Vec<DiscreteGene> {
        self.gene_range.clone().collect()
    }

    fn chromosome_factory<R: Rng>(&self, rng: &mut R) -> Chromosome<DiscreteGene> {
        let genes: Vec<DiscreteGene> = (0..self.gene_size)
            .map(|_| rng.gen_range(self.gene_range.clone()))
            .collect();
        Chromosome::new(genes)
    }

    fn mutate_chromosome<R: Rng>(&self, chromosome: &mut Chromosome<DiscreteGene>, rng: &mut R) {
        let index = rng.gen_range(0..self.gene_size);
        chromosome.genes[index] = rng.gen_range(self.gene_range.clone());
        chromosome.taint_fitness_score();
    }
}

impl Genotype<ContinuousGene> for Range<ContinuousGene> {
    fn gene_size(&self) -> usize {
        self.gene_size
    }
    fn gene_values(&self) -> Vec<ContinuousGene> {
        vec![]
    }

    fn chromosome_factory<R: Rng>(&self, rng: &mut R) -> Chromosome<ContinuousGene> {
        let genes: Vec<ContinuousGene> = (0..self.gene_size)
            .map(|_| rng.gen_range(self.gene_range.clone()))
            .collect();
        Chromosome::new(genes)
    }

    fn mutate_chromosome<R: Rng>(&self, chromosome: &mut Chromosome<ContinuousGene>, rng: &mut R) {
        let index = rng.gen_range(0..self.gene_size);
        chromosome.genes[index] = rng.gen_range(self.gene_range.clone());
        chromosome.taint_fitness_score();
    }
}

impl<T: Gene> fmt::Display for Range<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "genotype:\n")?;
        write!(f, "  gene_size: {}\n", self.gene_size)?;
        write!(f, "  gene_range: {:?}\n", self.gene_range)
    }
}
