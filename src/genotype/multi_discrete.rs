use super::builder::{Builder, TryFromBuilderError};
use super::{Genotype, PermutableGenotype};
use crate::chromosome::Chromosome;
use crate::population::Population;
use itertools::Itertools;
use rand::distributions::{Distribution, Uniform, WeightedIndex};
use rand::prelude::*;
use std::fmt;

#[derive(Clone, Debug)]
pub struct MultiDiscrete<T: Clone + std::fmt::Debug> {
    gene_size: usize,
    gene_value_sizes: Vec<usize>,
    pub gene_multi_values: Vec<Vec<T>>,
    gene_index_sampler: WeightedIndex<usize>,
    gene_value_index_samplers: Vec<Uniform<usize>>,
    pub seed_genes: Option<Vec<T>>,
}

impl<T: Clone + std::fmt::Debug> TryFrom<Builder<Self>> for MultiDiscrete<T> {
    type Error = TryFromBuilderError;

    fn try_from(builder: Builder<Self>) -> Result<Self, Self::Error> {
        if builder.gene_multi_values.is_none() {
            Err(TryFromBuilderError(
                "MultiDiscreteGenotype requires a gene_multi_values",
            ))
        } else if builder
            .gene_multi_values
            .as_ref()
            .map(|o| o.is_empty())
            .unwrap()
        {
            Err(TryFromBuilderError(
                "UniqueDiscreteGenotype requires non-empty gene_multi_values",
            ))
        } else {
            let gene_multi_values = builder.gene_multi_values.unwrap();
            let gene_value_sizes: Vec<usize> = gene_multi_values.iter().map(|v| v.len()).collect();
            Ok(Self {
                gene_size: gene_multi_values.len(),
                gene_value_sizes: gene_value_sizes.clone(),
                gene_multi_values: gene_multi_values.clone(),
                gene_index_sampler: WeightedIndex::new(gene_value_sizes.clone()).unwrap(),
                gene_value_index_samplers: gene_value_sizes
                    .iter()
                    .map(|gene_value_size| Uniform::from(0..*gene_value_size))
                    .collect(),
                seed_genes: builder.seed_genes,
            })
        }
    }
}

impl<T: Clone + std::fmt::Debug> Genotype for MultiDiscrete<T> {
    type Gene = T;
    fn gene_size(&self) -> usize {
        self.gene_size
    }
    fn chromosome_factory<R: Rng>(&self, rng: &mut R) -> Chromosome<Self> {
        if let Some(seed_genes) = self.seed_genes.as_ref() {
            Chromosome::new(seed_genes.clone())
        } else {
            let genes: Vec<Self::Gene> = self
                .gene_multi_values
                .iter()
                .enumerate()
                .map(|(index, gene_values)| {
                    gene_values[self.gene_value_index_samplers[index].sample(rng)].clone()
                })
                .collect();
            Chromosome::new(genes)
        }
    }

    fn mutate_chromosome<R: Rng>(&self, chromosome: &mut Chromosome<Self>, rng: &mut R) {
        let index = self.gene_index_sampler.sample(rng);
        chromosome.genes[index] = self.gene_multi_values[index]
            [self.gene_value_index_samplers[index].sample(rng)]
        .clone();
        chromosome.taint_fitness_score();
    }
}

impl<T: Clone + std::fmt::Debug> PermutableGenotype for MultiDiscrete<T> {
    //noop
    fn gene_values(&self) -> Vec<Self::Gene> {
        vec![]
    }
    fn population_factory(&self) -> Population<Self> {
        let chromosomes = self
            .gene_multi_values
            .iter()
            .map(|gene_values| gene_values.clone())
            .multi_cartesian_product()
            .map(Chromosome::new)
            .collect();

        Population::new(chromosomes)
    }
    fn population_factory_size(&self) -> usize {
        self.gene_value_sizes.iter().product()
    }
}

impl<T: Clone + std::fmt::Debug> fmt::Display for MultiDiscrete<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "genotype:")?;
        writeln!(f, "  gene_size: {}\n", self.gene_size)?;
        writeln!(f, "  gene_value_sizes: {:?}", self.gene_value_sizes)?;
        writeln!(f, "  gene_multi_values: {:?}", self.gene_multi_values)?;
        writeln!(f, "  seed_genes: {:?}", self.seed_genes)
    }
}
