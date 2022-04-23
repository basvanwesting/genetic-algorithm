use crate::chromosome::Chromosome;
use crate::gene::Gene;
use itertools::Itertools;
use rand::prelude::*;
use std::fmt;

pub struct Context<T: Gene> {
    pub gene_size: usize,
    pub gene_values: Vec<T>,
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

    pub fn random_chromosome_factory<R: Rng>(&self, rng: &mut R) -> Chromosome<T> {
        Chromosome::random_factory(self, rng)
    }
}

impl<T: Gene> Default for Context<T> {
    fn default() -> Self {
        Context {
            gene_size: 10,
            gene_values: vec![],
        }
    }
}

impl<T: Gene> fmt::Display for Context<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "context:\n")?;
        write!(f, "  gene_size: {}\n", self.gene_size)?;
        write!(f, "  gene_values: {}\n", self.gene_values.iter().join(","))
    }
}
