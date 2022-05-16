use super::builder::{Builder, TryFromBuilderError};
use super::{Genotype, PermutableGenotype};
use crate::chromosome::Chromosome;
use factorial::Factorial;
use itertools::Itertools;
use num::BigUint;
use rand::distributions::{Distribution, Uniform};
use rand::prelude::*;
use std::fmt;

pub type DefaultDiscreteAllele = usize;

/// Genes are a list of unique values, taken from the allele_values using clone(), each value occurs
/// exactly once. The gene_size is derived to be the same as allele_values length. On random
/// initialization, the allele_values are suffled to form the genes. Each pair of genes has an equal
/// probability of mutating. If a pair of genes mutates, the values are switched, ensuring the list
/// of genes remains unique. Defaults to usize as item.
///
/// # Example (usize, default):
/// ```
/// use genetic_algorithm::genotype::{Genotype, UniqueDiscreteGenotype};
///
/// let genotype = UniqueDiscreteGenotype::builder()
///     .with_allele_values((0..100).collect())
///     .build()
///     .unwrap();
/// ```
///
/// # Example (struct)
/// ```
/// use genetic_algorithm::genotype::{Genotype, UniqueDiscreteGenotype};
///
/// #[derive(Clone, Debug)]
/// struct Item(pub u16, pub u16);
///
/// let genotype = UniqueDiscreteGenotype::builder()
///     .with_allele_values(vec![
///         Item(23, 505),
///         Item(26, 352),
///         Item(20, 458),
///     ])
///     .build()
///     .unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct UniqueDiscrete<T: Clone + std::fmt::Debug = DefaultDiscreteAllele> {
    pub allele_values: Vec<T>,
    gene_index_sampler: Uniform<usize>,
    pub seed_genes: Option<Vec<T>>,
}

impl<T: Clone + std::fmt::Debug> TryFrom<Builder<Self>> for UniqueDiscrete<T> {
    type Error = TryFromBuilderError;

    fn try_from(builder: Builder<Self>) -> Result<Self, Self::Error> {
        if builder.allele_values.is_none() {
            Err(TryFromBuilderError(
                "UniqueDiscreteGenotype requires allele_values",
            ))
        } else if builder
            .allele_values
            .as_ref()
            .map(|o| o.is_empty())
            .unwrap()
        {
            Err(TryFromBuilderError(
                "UniqueDiscreteGenotype requires non-empty allele_values",
            ))
        } else {
            let allele_values = builder.allele_values.unwrap();
            Ok(Self {
                allele_values: allele_values.clone(),
                gene_index_sampler: Uniform::from(0..allele_values.len()),
                seed_genes: builder.seed_genes,
            })
        }
    }
}

impl<T: Clone + std::fmt::Debug> Genotype for UniqueDiscrete<T> {
    type Allele = T;
    fn gene_size(&self) -> usize {
        self.allele_values.len()
    }
    fn chromosome_factory<R: Rng>(&self, rng: &mut R) -> Chromosome<Self> {
        if let Some(seed_genes) = self.seed_genes.as_ref() {
            Chromosome::new(seed_genes.clone())
        } else {
            let mut genes = self.allele_values.clone();
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

impl<T: Clone + std::fmt::Debug> PermutableGenotype for UniqueDiscrete<T> {
    fn allele_values(&self) -> Vec<Self::Allele> {
        self.allele_values.clone()
    }

    fn chromosome_permutations_into_iter<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = Chromosome<Self>> + 'a> {
        Box::new(
            self.allele_values
                .clone()
                .into_iter()
                .permutations(self.gene_size())
                .map(|genes| Chromosome::new(genes)),
        )
    }

    fn chromosome_permutations_size(&self) -> BigUint {
        BigUint::from(self.allele_values.len()).factorial()
    }
}

impl<T: Clone + std::fmt::Debug> fmt::Display for UniqueDiscrete<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "genotype:")?;
        writeln!(f, "  allele_values: {:?}", self.allele_values)?;
        writeln!(
            f,
            "  chromosome_permutations_size: {}",
            self.chromosome_permutations_size()
        )?;
        writeln!(f, "  seed_genes: {:?}", self.seed_genes)
    }
}
