use super::builder::{Builder, TryFromGenotypeBuilderError};
use super::{Genotype, PermutableGenotype};
use crate::chromosome::Chromosome;
use crate::gene::Gene;
use rand::distributions::{Distribution, Uniform};
use rand::prelude::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Discrete<T: Gene> {
    pub gene_size: usize,
    pub gene_values: Vec<T>,
    gene_index_sampler: Uniform<usize>,
    gene_value_index_sampler: Uniform<usize>,
}

impl<T: Gene> TryFrom<Builder<Self>> for Discrete<T> {
    type Error = TryFromGenotypeBuilderError;

    fn try_from(builder: Builder<Self>) -> Result<Self, Self::Error> {
        if builder.gene_size.is_none() {
            Err(TryFromGenotypeBuilderError("Require gene_size"))
        } else if builder.gene_values.is_empty() {
            Err(TryFromGenotypeBuilderError(
                "Require gene_values not be empty",
            ))
        } else {
            Ok(Self {
                gene_size: builder.gene_size.unwrap(),
                gene_values: builder.gene_values.clone(),
                gene_index_sampler: Uniform::from(0..builder.gene_size.unwrap()),
                gene_value_index_sampler: Uniform::from(0..builder.gene_values.len()),
            })
        }
    }
}

impl<T: Gene> Genotype for Discrete<T> {
    type Gene = T;
    fn gene_size(&self) -> usize {
        self.gene_size
    }
    fn chromosome_factory<R: Rng>(&self, rng: &mut R) -> Chromosome<Self> {
        let genes: Vec<Self::Gene> = (0..self.gene_size)
            .map(|_| self.gene_values[self.gene_value_index_sampler.sample(rng)].clone())
            .collect();
        Chromosome::new(genes)
    }

    fn mutate_chromosome<R: Rng>(&self, chromosome: &mut Chromosome<Self>, rng: &mut R) {
        let index = self.gene_index_sampler.sample(rng);
        chromosome.genes[index] =
            self.gene_values[self.gene_value_index_sampler.sample(rng)].clone();
        chromosome.taint_fitness_score();
    }
}

impl<T: Gene> PermutableGenotype for Discrete<T> {
    fn gene_values(&self) -> Vec<Self::Gene> {
        self.gene_values.clone()
    }
}

impl<T: Gene> fmt::Display for Discrete<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "genotype:")?;
        writeln!(f, "  gene_size: {}", self.gene_size)?;
        writeln!(f, "  gene_values: {:?}", self.gene_values)?;
        writeln!(f, "  gene_index_sampler: {:?}", self.gene_index_sampler)?;
        writeln!(
            f,
            "  gene_value_index_sampler: {:?}",
            self.gene_value_index_sampler
        )
    }
}
