use super::builder::{Builder, TryFromBuilderError};
use super::{Genotype, IncrementalGenotype, PermutableGenotype};
use crate::chromosome::Chromosome;
use num::BigUint;
use rand::distributions::{Distribution, Uniform};
use rand::prelude::*;
use std::fmt;

pub type DefaultAllele = usize;

/// Genes are a list of values, each taken from the allele_list using clone(). On random
/// initialization, each gene gets a value from the allele_list with a uniform probability. Each
/// gene has an equal probability of mutating. If a gene mutates, a new values is taken from the
/// allele_list with a uniform probability (regardless of current value, which could therefore be
/// assigned again, not mutating as a result). Duplicate allele values are allowed. Defaults to usize
/// as item.
///
/// # Example (usize, default):
/// ```
/// use genetic_algorithm::genotype::{Genotype, DiscreteGenotype};
///
/// let genotype = DiscreteGenotype::builder()
///     .with_genes_size(100)
///     .with_allele_list((0..10).collect())
///     .build()
///     .unwrap();
/// ```
///
/// # Example (struct)
/// ```
/// use genetic_algorithm::genotype::{Genotype, DiscreteGenotype};
///
/// #[derive(PartialEq, Clone, Debug)]
/// struct Item(pub u16, pub u16);
///
/// let genotype = DiscreteGenotype::builder()
///     .with_genes_size(100)
///     .with_allele_list(vec![
///         Item(23, 505),
///         Item(26, 352),
///         Item(20, 458),
///     ])
///     .build()
///     .unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct Discrete<T: PartialEq + Clone + Send + std::fmt::Debug = DefaultAllele> {
    pub genes_size: usize,
    pub allele_list: Vec<T>,
    gene_index_sampler: Uniform<usize>,
    allele_index_sampler: Uniform<usize>,
    pub seed_genes_list: Vec<Vec<T>>,
}

impl<T: PartialEq + Clone + Send + std::fmt::Debug> TryFrom<Builder<Self>> for Discrete<T> {
    type Error = TryFromBuilderError;

    fn try_from(builder: Builder<Self>) -> Result<Self, Self::Error> {
        if builder.genes_size.is_none() {
            Err(TryFromBuilderError(
                "DiscreteGenotype requires a genes_size",
            ))
        } else if builder.allele_list.is_none() {
            Err(TryFromBuilderError("DiscreteGenotype requires allele_list"))
        } else if builder.allele_list.as_ref().map(|o| o.is_empty()).unwrap() {
            Err(TryFromBuilderError(
                "DiscreteGenotype requires non-empty allele_list",
            ))
        } else {
            let allele_list = builder.allele_list.unwrap();
            Ok(Self {
                genes_size: builder.genes_size.unwrap(),
                allele_list: allele_list.clone(),
                gene_index_sampler: Uniform::from(0..builder.genes_size.unwrap()),
                allele_index_sampler: Uniform::from(0..allele_list.len()),
                seed_genes_list: builder.seed_genes_list,
            })
        }
    }
}

impl<T: PartialEq + Clone + Send + std::fmt::Debug> Genotype for Discrete<T> {
    type Allele = T;
    fn genes_size(&self) -> usize {
        self.genes_size
    }
    fn random_genes_factory<R: Rng>(&self, rng: &mut R) -> Vec<Self::Allele> {
        (0..self.genes_size)
            .map(|_| self.allele_list[self.allele_index_sampler.sample(rng)].clone())
            .collect()
    }
    fn chromosome_factory<R: Rng>(&self, rng: &mut R) -> Chromosome<Self> {
        if self.seed_genes_list.is_empty() {
            Chromosome::new(self.random_genes_factory(rng))
        } else {
            Chromosome::new(self.seed_genes_list.choose(rng).unwrap().clone())
        }
    }

    fn mutate_chromosome_random<R: Rng>(&self, chromosome: &mut Chromosome<Self>, rng: &mut R) {
        let index = self.gene_index_sampler.sample(rng);
        chromosome.genes[index] = self.allele_list[self.allele_index_sampler.sample(rng)].clone();
        chromosome.taint_fitness_score();
    }
    fn set_seed_genes_list(&mut self, seed_genes_list: Vec<Vec<T>>) {
        self.seed_genes_list = seed_genes_list;
    }
}

impl<T: PartialEq + Clone + Send + std::fmt::Debug> IncrementalGenotype for Discrete<T> {
    fn mutate_chromosome_neighbour<R: Rng>(
        &self,
        chromosome: &mut Chromosome<Self>,
        _scale: Option<f32>,
        rng: &mut R,
    ) {
        self.mutate_chromosome_random(chromosome, rng);
    }

    fn neighbouring_chromosomes(
        &self,
        chromosome: &Chromosome<Self>,
        _scale: Option<f32>,
    ) -> Vec<Chromosome<Self>> {
        (0..self.genes_size)
            .flat_map(|index| {
                self.allele_list.iter().filter_map(move |allele_value| {
                    if chromosome.genes[index] == *allele_value {
                        None
                    } else {
                        let mut genes = chromosome.genes.clone();
                        genes[index] = allele_value.clone();
                        Some(Chromosome::new(genes))
                    }
                })
            })
            .collect::<Vec<_>>()
    }

    fn neighbouring_population_size(&self) -> BigUint {
        BigUint::from((self.allele_list.len() - 1) * self.genes_size)
    }
}

impl<T: PartialEq + Clone + Send + std::fmt::Debug> PermutableGenotype for Discrete<T> {
    fn allele_list_for_chromosome_permutations(&self) -> Vec<Self::Allele> {
        self.allele_list.clone()
    }
}

impl<T: PartialEq + Clone + Send + std::fmt::Debug> fmt::Display for Discrete<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "genotype:")?;
        writeln!(f, "  genes_size: {}", self.genes_size)?;
        writeln!(f, "  allele_list: {:?}", self.allele_list)?;
        writeln!(
            f,
            "  chromosome_permutations_size: {}",
            self.chromosome_permutations_size()
        )?;
        writeln!(
            f,
            "  neighbouring_population_size: {}",
            self.neighbouring_population_size()
        )?;
        writeln!(f, "  seed_genes_list: {:?}", self.seed_genes_list)
    }
}
