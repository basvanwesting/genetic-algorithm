use super::builder::{Builder, TryFromBuilderError};
use super::{Genotype, IncrementalGenotype, PermutableGenotype};
use crate::chromosome::Chromosome;
use crate::population::Population;
use factorial::Factorial;
use itertools::Itertools;
use num::BigUint;
use rand::distributions::{Distribution, Uniform};
use rand::prelude::*;
use std::fmt;

pub type DefaultAllele = usize;

/// Genes are a list of unique values, taken from the allele_list using clone(), each value occurs
/// exactly once. The genes_size is derived to be the same as allele_list length. On random
/// initialization, the allele_list are suffled to form the genes. Each pair of genes has an equal
/// probability of mutating. If a pair of genes mutates, the values are switched, ensuring the list
/// of alleles remains unique. Defaults to usize as item.
///
/// # Example (usize, default):
/// ```
/// use genetic_algorithm::genotype::{Genotype, UniqueGenotype};
///
/// let genotype = UniqueGenotype::builder()
///     .with_allele_list((0..100).collect())
///     .build()
///     .unwrap();
/// ```
///
/// # Example (struct)
/// ```
/// use genetic_algorithm::genotype::{Genotype, UniqueGenotype};
///
/// #[derive(Clone, Debug)]
/// struct Item(pub u16, pub u16);
///
/// let genotype = UniqueGenotype::builder()
///     .with_allele_list(vec![
///         Item(23, 505),
///         Item(26, 352),
///         Item(20, 458),
///     ])
///     .build()
///     .unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct Unique<T: Clone + Send + std::fmt::Debug = DefaultAllele> {
    pub allele_list: Vec<T>,
    gene_index_sampler: Uniform<usize>,
    pub seed_genes: Option<Vec<T>>,
}

impl<T: Clone + Send + std::fmt::Debug> TryFrom<Builder<Self>> for Unique<T> {
    type Error = TryFromBuilderError;

    fn try_from(builder: Builder<Self>) -> Result<Self, Self::Error> {
        if builder.allele_list.is_none() {
            Err(TryFromBuilderError("UniqueGenotype requires allele_list"))
        } else if builder.allele_list.as_ref().map(|o| o.is_empty()).unwrap() {
            Err(TryFromBuilderError(
                "UniqueGenotype requires non-empty allele_list",
            ))
        } else {
            let allele_list = builder.allele_list.unwrap();
            Ok(Self {
                allele_list: allele_list.clone(),
                gene_index_sampler: Uniform::from(0..allele_list.len()),
                seed_genes: builder.seed_genes,
            })
        }
    }
}

impl<T: Clone + Send + std::fmt::Debug> Genotype for Unique<T> {
    type Allele = T;
    fn genes_size(&self) -> usize {
        self.allele_list.len()
    }
    ///unique genotypes can't simply exchange genes without gene duplication issues
    fn crossover_points(&self) -> Vec<usize> {
        vec![]
    }
    ///unique genotypes can't simply exchange genes without gene duplication issues
    fn crossover_indexes(&self) -> Vec<usize> {
        vec![]
    }
    fn chromosome_seed<R: Rng>(&self, rng: &mut R) -> Chromosome<Self> {
        if let Some(seed_genes) = self.seed_genes.as_ref() {
            Chromosome::new(seed_genes.clone())
        } else {
            self.chromosome_factory(rng)
        }
    }
    fn chromosome_factory<R: Rng>(&self, rng: &mut R) -> Chromosome<Self> {
        if let Some(seed_genes) = self.seed_genes.as_ref() {
            let mut chromosome = Chromosome::new(seed_genes.clone());
            self.mutate_chromosome_random(&mut chromosome, rng);
            chromosome
        } else {
            let mut genes = self.allele_list.clone();
            genes.shuffle(rng);
            Chromosome::new(genes)
        }
    }

    fn mutate_chromosome_random<R: Rng>(&self, chromosome: &mut Chromosome<Self>, rng: &mut R) {
        let index1 = self.gene_index_sampler.sample(rng);
        let index2 = self.gene_index_sampler.sample(rng);
        chromosome.genes.swap(index1, index2);
        chromosome.taint_fitness_score();
    }
}

impl<T: Clone + Send + std::fmt::Debug> IncrementalGenotype for Unique<T> {
    fn mutate_chromosome_neighbour<R: Rng>(
        &self,
        chromosome: &mut Chromosome<Self>,
        _scale: Option<f32>,
        rng: &mut R,
    ) {
        self.mutate_chromosome_random(chromosome, rng);
    }

    fn neighbouring_population(
        &self,
        chromosome: &Chromosome<Self>,
        _scale: Option<f32>,
    ) -> Population<Self> {
        (0..self.genes_size())
            .tuple_combinations()
            .map(|(first, second)| {
                let mut new_genes = chromosome.genes.clone();
                new_genes.swap(first, second);
                new_genes
            })
            .map(|genes| Chromosome::new(genes))
            .collect::<Vec<_>>()
            .into()
    }

    fn neighbouring_population_size(&self) -> BigUint {
        let n = BigUint::from(self.allele_list.len());
        let k = BigUint::from(2usize);

        n.factorial() / (k.factorial() * (n - k).factorial())
    }
}

impl<T: Clone + Send + std::fmt::Debug> PermutableGenotype for Unique<T> {
    //noop
    fn allele_list_for_chromosome_permutations(&self) -> Vec<Self::Allele> {
        vec![]
    }

    fn chromosome_permutations_into_iter<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = Chromosome<Self>> + 'a> {
        Box::new(
            self.allele_list
                .clone()
                .into_iter()
                .permutations(self.genes_size())
                .map(|genes| Chromosome::new(genes)),
        )
    }

    fn chromosome_permutations_size(&self) -> BigUint {
        BigUint::from(self.allele_list.len()).factorial()
    }
}

impl<T: Clone + Send + std::fmt::Debug> fmt::Display for Unique<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "genotype:")?;
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
        writeln!(f, "  seed_genes: {:?}", self.seed_genes)
    }
}
