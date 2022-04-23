use crate::chromosome::Chromosome;
use crate::gene::Gene;
use crate::population::Population;
use itertools::Itertools;
use rand::prelude::*;
use std::fmt;

pub struct Context<T: Gene> {
    pub gene_size: usize,
    pub gene_values: Vec<T>,
    pub population_size: usize,
}

impl<T: Gene> Context<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_gene_size(mut self, gene_size: usize) -> Self {
        self.gene_size = gene_size;
        self
    }

    pub fn with_gene_values(mut self, gene_values: Vec<T>) -> Self {
        self.gene_values = gene_values;
        self
    }

    pub fn with_population_size(mut self, population_size: usize) -> Self {
        self.population_size = population_size;
        self
    }

    pub fn permutation_population_factory(&self) -> Population<T> {
        let chromosomes = (0..self.gene_size)
            .map(|_| self.gene_values.clone())
            .multi_cartesian_product()
            .map(|genes| Chromosome::new(genes))
            .collect();

        Population::new(chromosomes)
    }

    pub fn random_chromosome_factory<R: Rng>(&self, rng: &mut R) -> Chromosome<T> {
        Chromosome::random_factory(self, rng)
    }

    pub fn random_population_factory<R: Rng>(&self, rng: &mut R) -> Population<T> {
        let chromosomes = (0..self.population_size)
            .map(|_| self.random_chromosome_factory(rng))
            .collect();
        Population::new(chromosomes)
    }
}

impl<T: Gene> Default for Context<T> {
    fn default() -> Self {
        Context {
            gene_size: 10,
            gene_values: vec![],
            population_size: 100,
        }
    }
}

impl<T: Gene> fmt::Display for Context<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "context:\n")?;
        write!(f, "  gene_size: {}\n", self.gene_size)?;
        write!(f, "  gene_values: {}\n", self.gene_values.iter().join(","))?;
        write!(f, "  population_size: {}\n", self.population_size)
    }
}
