use super::builder::{Builder, TryFromBuilderError};
use super::{Genotype, PermutableGenotype};
use crate::chromosome::Chromosome;
use crate::population::Population;
use factorial::Factorial;
use itertools::Itertools;
use rand::distributions::{Distribution, Uniform};
use rand::prelude::*;
use std::fmt;

// trait alias, experimental
//pub trait Gene = Default + Clone + std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct UniqueDiscrete<T: Default + Clone + std::fmt::Debug> {
    pub gene_values: Vec<T>,
    gene_index_sampler: Uniform<usize>,
    pub seed_genes: Option<Vec<T>>,
}

impl<T: Default + Clone + std::fmt::Debug> TryFrom<Builder<Self>> for UniqueDiscrete<T> {
    type Error = TryFromBuilderError;

    fn try_from(builder: Builder<Self>) -> Result<Self, Self::Error> {
        if builder.gene_values.is_none() {
            Err(TryFromBuilderError(
                "UniqueDiscreteGenotype requires gene_values",
            ))
        } else if builder.gene_values.as_ref().map(|o| o.is_empty()).unwrap() {
            Err(TryFromBuilderError(
                "UniqueDiscreteGenotype requires non-empty gene_values",
            ))
        } else {
            let gene_values = builder.gene_values.unwrap();
            Ok(Self {
                gene_values: gene_values.clone(),
                gene_index_sampler: Uniform::from(0..gene_values.len()),
                seed_genes: builder.seed_genes,
            })
        }
    }
}

impl<T: Default + Clone + std::fmt::Debug> Genotype for UniqueDiscrete<T> {
    type Gene = T;
    fn gene_size(&self) -> usize {
        self.gene_values.len()
    }
    fn chromosome_factory<R: Rng>(&self, rng: &mut R) -> Chromosome<Self> {
        if let Some(seed_genes) = self.seed_genes.as_ref() {
            Chromosome::new(seed_genes.clone())
        } else {
            let mut genes = self.gene_values.clone();
            genes.shuffle(rng);
            Chromosome::new(genes)
        }
    }

    fn mutate_chromosome<R: Rng>(&self, chromosome: &mut Chromosome<Self>, rng: &mut R) {
        let index1 = self.gene_index_sampler.sample(rng);
        let index2 = self.gene_index_sampler.sample(rng);
        chromosome.genes.swap(index1, index2);
        chromosome.taint_fitness_score();
    }

    fn is_unique(&self) -> bool {
        true
    }
}

impl<T: Default + Clone + std::fmt::Debug> PermutableGenotype for UniqueDiscrete<T> {
    fn gene_values(&self) -> Vec<Self::Gene> {
        self.gene_values.clone()
    }
    fn population_factory(&self) -> Population<Self> {
        let chromosomes = self
            .gene_values()
            .iter()
            .permutations(self.gene_size())
            .map(|genes| Chromosome::new(genes.into_iter().cloned().collect()))
            .collect();

        Population::new(chromosomes)
    }
    fn population_factory_size(&self) -> usize {
        self.gene_values().len().factorial()
    }
}

impl<T: Default + Clone + std::fmt::Debug> fmt::Display for UniqueDiscrete<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "genotype:")?;
        writeln!(f, "  gene_values: {:?}", self.gene_values)?;
        writeln!(f, "  gene_index_sampler: {:?}", self.gene_index_sampler)
    }
}
