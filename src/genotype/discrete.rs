use super::builder::{Builder, TryFromBuilderError};
use super::{Genotype, PermutableGenotype};
use crate::chromosome::Chromosome;
use rand::distributions::{Distribution, Uniform};
use rand::prelude::*;
use std::fmt;

pub type DefaultDiscreteGene = usize;

/// Genes are a list of values, each taken from the gene_values using clone(). On random
/// initialization, each gene gets a value from the gene_values with a uniform probability. Each
/// gene has an equal probability of mutating. If a gene mutates, a new values is taken from the
/// gene_values with a uniform probability (regardless of current value, which could therefore be
/// assigned again, not mutating as a result). Duplicate gene values are allowed. Defaults to usize
/// as item.
///
/// # Example (usize, default):
/// ```
/// use genetic_algorithm::genotype::{Genotype, DiscreteGenotype};
///
/// let genotype = DiscreteGenotype::builder()
///     .with_gene_size(100)
///     .with_gene_values((0..10).collect())
///     .build()
///     .unwrap();
/// ```
///
/// # Example (struct)
/// ```
/// use genetic_algorithm::genotype::{Genotype, DiscreteGenotype};
///
/// #[derive(Clone, Debug)]
/// struct Item(pub u16, pub u16);
///
/// let genotype = DiscreteGenotype::builder()
///     .with_gene_size(100)
///     .with_gene_values(vec![
///         Item(23, 505),
///         Item(26, 352),
///         Item(20, 458),
///     ])
///     .build()
///     .unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct Discrete<T: Clone + std::fmt::Debug = DefaultDiscreteGene> {
    pub gene_size: usize,
    pub gene_values: Vec<T>,
    gene_index_sampler: Uniform<usize>,
    gene_value_index_sampler: Uniform<usize>,
    pub seed_genes: Option<Vec<T>>,
}

impl<T: Clone + std::fmt::Debug> TryFrom<Builder<Self>> for Discrete<T> {
    type Error = TryFromBuilderError;

    fn try_from(builder: Builder<Self>) -> Result<Self, Self::Error> {
        if builder.gene_size.is_none() {
            Err(TryFromBuilderError("DiscreteGenotype requires a gene_size"))
        } else if builder.gene_values.is_none() {
            Err(TryFromBuilderError("DiscreteGenotype requires gene_values"))
        } else if builder.gene_values.as_ref().map(|o| o.is_empty()).unwrap() {
            Err(TryFromBuilderError(
                "DiscreteGenotype requires non-empty gene_values",
            ))
        } else {
            let gene_values = builder.gene_values.unwrap();
            Ok(Self {
                gene_size: builder.gene_size.unwrap(),
                gene_values: gene_values.clone(),
                gene_index_sampler: Uniform::from(0..builder.gene_size.unwrap()),
                gene_value_index_sampler: Uniform::from(0..gene_values.len()),
                seed_genes: builder.seed_genes,
            })
        }
    }
}

impl<T: Clone + std::fmt::Debug> Genotype for Discrete<T> {
    type Gene = T;
    fn gene_size(&self) -> usize {
        self.gene_size
    }
    fn chromosome_factory<R: Rng>(&self, rng: &mut R) -> Chromosome<Self> {
        if let Some(seed_genes) = self.seed_genes.as_ref() {
            Chromosome::new(seed_genes.clone())
        } else {
            let genes: Vec<Self::Gene> = (0..self.gene_size)
                .map(|_| self.gene_values[self.gene_value_index_sampler.sample(rng)].clone())
                .collect();
            Chromosome::new(genes)
        }
    }

    fn mutate_chromosome<R: Rng>(&self, chromosome: &mut Chromosome<Self>, rng: &mut R) {
        let index = self.gene_index_sampler.sample(rng);
        chromosome.genes[index] =
            self.gene_values[self.gene_value_index_sampler.sample(rng)].clone();
        chromosome.taint_fitness_score();
    }
}

impl<T: Clone + std::fmt::Debug> PermutableGenotype for Discrete<T> {
    fn gene_values(&self) -> Vec<Self::Gene> {
        self.gene_values.clone()
    }
}

impl<T: Clone + std::fmt::Debug> fmt::Display for Discrete<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "genotype:")?;
        writeln!(f, "  gene_size: {}", self.gene_size)?;
        writeln!(f, "  gene_values: {:?}", self.gene_values)?;
        writeln!(f, "  seed_genes: {:?}", self.seed_genes)?;
        writeln!(
            f,
            "  chromosome_permutations_size: {:?}",
            self.chromosome_permutations_size()
        )
    }
}
