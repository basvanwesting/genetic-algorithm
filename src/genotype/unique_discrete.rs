use super::builder::{Builder, TryFromBuilderError};
use super::{Genotype, PermutableGenotype};
use crate::chromosome::Chromosome;
use crate::gene::Gene;
use crate::population::Population;
use factorial::Factorial;
use itertools::Itertools;
use rand::distributions::{Distribution, Uniform};
use rand::prelude::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct UniqueDiscrete<T: Gene> {
    pub gene_values: Vec<T>,
    gene_index_sampler: Uniform<usize>,
}

impl<T: Gene> TryFrom<Builder<Self>> for UniqueDiscrete<T> {
    type Error = TryFromBuilderError;

    fn try_from(builder: Builder<Self>) -> Result<Self, Self::Error> {
        if builder.gene_values.is_empty() {
            Err(TryFromBuilderError(
                "UniqueDiscreteGenotype requires non-empty gene_values",
            ))
        } else {
            Ok(Self {
                gene_values: builder.gene_values.clone(),
                gene_index_sampler: Uniform::from(0..builder.gene_values.len()),
            })
        }
    }
}

impl<T: Gene> Genotype for UniqueDiscrete<T> {
    type Gene = T;
    fn gene_size(&self) -> usize {
        self.gene_values.len()
    }
    fn chromosome_factory<R: Rng>(&self, rng: &mut R) -> Chromosome<Self> {
        let mut genes = self.gene_values.clone();
        genes.shuffle(rng);
        Chromosome::new(genes)
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

impl<T: Gene> PermutableGenotype for UniqueDiscrete<T> {
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

impl<T: Gene> fmt::Display for UniqueDiscrete<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "genotype:")?;
        writeln!(f, "  gene_values: {:?}", self.gene_values)?;
        writeln!(f, "  gene_index_sampler: {:?}", self.gene_index_sampler)
    }
}
