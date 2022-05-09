use super::builder::{Builder, TryFromBuilderError};
use super::{Genotype, PermutableGenotype};
use crate::chromosome::Chromosome;
use crate::gene::IndexGene;
use crate::population::Population;
use factorial::Factorial;
use itertools::Itertools;
use rand::distributions::{Distribution, Uniform};
use rand::prelude::*;
use std::fmt;

#[derive(Clone, Debug)]
pub struct UniqueIndex {
    pub gene_value_size: IndexGene,
    gene_index_sampler: Uniform<usize>,
    pub seed_genes: Option<Vec<IndexGene>>,
}

impl TryFrom<Builder<Self>> for UniqueIndex {
    type Error = TryFromBuilderError;

    fn try_from(builder: Builder<Self>) -> Result<Self, Self::Error> {
        if builder.gene_value_size.is_none() {
            Err(TryFromBuilderError(
                "UniqueIndexGenotype requires a gene_value_size",
            ))
        } else {
            Ok(Self {
                gene_value_size: builder.gene_value_size.unwrap(),
                gene_index_sampler: Uniform::from(0..builder.gene_value_size.unwrap()),
                seed_genes: builder.seed_genes,
            })
        }
    }
}

impl Genotype for UniqueIndex {
    type Gene = IndexGene;
    fn gene_size(&self) -> usize {
        self.gene_value_size
    }
    fn chromosome_factory<R: Rng>(&self, rng: &mut R) -> Chromosome<Self> {
        if let Some(seed_genes) = self.seed_genes.as_ref() {
            Chromosome::new(seed_genes.clone())
        } else {
            let mut genes: Vec<Self::Gene> = (0..self.gene_value_size).collect();
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

impl PermutableGenotype for UniqueIndex {
    fn gene_values(&self) -> Vec<Self::Gene> {
        (0..self.gene_value_size).collect()
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

impl fmt::Display for UniqueIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "genotype:")?;
        writeln!(f, "  gene_value_size: {}", self.gene_value_size)?;
        writeln!(f, "  gene_index_sampler: {:?}", self.gene_index_sampler)
    }
}
