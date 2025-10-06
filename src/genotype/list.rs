use super::builder::{Builder, TryFromBuilderError};
use super::{EvolveGenotype, Genotype, HillClimbGenotype, PermutateGenotype};
use crate::allele::Allele;
use crate::chromosome::{Chromosome, Genes};
use crate::population::Population;
use itertools::Itertools;
use num::BigUint;
use rand::distributions::{Distribution, Uniform};
use rand::prelude::*;
use std::fmt;
use std::hash::Hash;

pub type DefaultAllele = usize;

/// Genes are a vector of values, each taken from the allele_list using clone(). The allele_list is
/// taken as unrelated, unorderable set with no concept op distance between the values. On random
/// initialization, each gene gets a value from the allele_list with a uniform probability. Each
/// gene has an equal probability of mutating. If a gene mutates, a new values is taken from the
/// allele_list with a uniform probability (regardless of current value, which could therefore be
/// assigned again, not mutating as a result). Duplicate allele values are allowed. Defaults to
/// usize as item.
///
/// # Example (usize, default):
/// ```
/// use genetic_algorithm::genotype::{Genotype, ListGenotype};
///
/// let genotype = ListGenotype::builder()
///     .with_genes_size(100)
///     .with_allele_list((0..10).collect())
///     .with_genes_hashing(false) // optional, defaults to false
///     .build()
///     .unwrap();
/// ```
///
/// # Example (struct, manual impl Allele)
/// ```
/// use genetic_algorithm::genotype::{Allele, Genotype, ListGenotype};
/// use std::hash::{Hash, Hasher};
///
/// #[derive(Clone, Copy, PartialEq, Hash, Debug)]
/// struct Item(pub u16, pub u16);
/// impl Allele for Item {
///     fn hash_slice(slice: &[Self], hasher: &mut impl Hasher) {
///         slice.hash(hasher);
///     }
/// }
///
/// let genotype = ListGenotype::builder()
///     .with_genes_size(100)
///     .with_allele_list(vec![
///         Item(23, 505),
///         Item(26, 352),
///         Item(20, 458),
///     ])
///     .with_genes_hashing(true) // optional, defaults to false
///     .build()
///     .unwrap();
/// ```
///
/// # Example (struct, macro impl Allele)
/// ```
/// use genetic_algorithm::genotype::{Allele, Genotype, ListGenotype};
///
/// #[derive(Clone, Copy, PartialEq, Hash, Debug)]
/// struct Item(pub u16, pub u16);
/// genetic_algorithm::impl_allele!(Item);
///
/// let genotype = ListGenotype::builder()
///     .with_genes_size(100)
///     .with_allele_list(vec![
///         Item(23, 505),
///         Item(26, 352),
///         Item(20, 458),
///     ])
///     .with_genes_hashing(true) // optional, defaults to false
///     .build()
///     .unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct List<T: Allele + PartialEq + Hash = DefaultAllele> {
    pub genes_size: usize,
    pub allele_list: Vec<T>,
    gene_index_sampler: Uniform<usize>,
    allele_index_sampler: Uniform<usize>,
    pub seed_genes_list: Vec<Vec<T>>,
    pub genes_hashing: bool,
}

impl<T: Allele + PartialEq + Hash> TryFrom<Builder<Self>> for List<T> {
    type Error = TryFromBuilderError;

    fn try_from(builder: Builder<Self>) -> Result<Self, Self::Error> {
        if !builder.genes_size.is_some_and(|x| x > 0) {
            Err(TryFromBuilderError(
                "ListGenotype requires a genes_size > 0",
            ))
        } else if builder.allele_list.is_none() {
            Err(TryFromBuilderError("ListGenotype requires allele_list"))
        } else if builder.allele_list.as_ref().map(|o| o.is_empty()).unwrap() {
            Err(TryFromBuilderError(
                "ListGenotype requires non-empty allele_list",
            ))
        } else {
            let allele_list = builder.allele_list.unwrap();
            Ok(Self {
                genes_size: builder.genes_size.unwrap(),
                allele_list: allele_list.clone(),
                gene_index_sampler: Uniform::from(0..builder.genes_size.unwrap()),
                allele_index_sampler: Uniform::from(0..allele_list.len()),
                seed_genes_list: builder.seed_genes_list,
                genes_hashing: builder.genes_hashing,
            })
        }
    }
}

impl<T: Allele + PartialEq + Hash> List<T> {
    pub fn sample_allele<R: Rng>(&self, rng: &mut R) -> T {
        self.allele_list[self.allele_index_sampler.sample(rng)]
    }
}

impl<T: Allele + PartialEq + Hash> Genotype for List<T> {
    type Allele = T;

    fn genes_size(&self) -> usize {
        self.genes_size
    }
    fn genes_slice<'a>(&'a self, chromosome: &'a Chromosome<Self::Allele>) -> &'a [Self::Allele] {
        chromosome.genes.as_slice()
    }

    fn sample_gene_index<R: Rng>(&self, rng: &mut R) -> usize {
        self.gene_index_sampler.sample(rng)
    }
    fn sample_gene_indices<R: Rng>(
        &self,
        count: usize,
        allow_duplicates: bool,
        rng: &mut R,
    ) -> Vec<usize> {
        if allow_duplicates {
            rng.sample_iter(self.gene_index_sampler)
                .take(count)
                .collect()
        } else {
            rand::seq::index::sample(rng, self.genes_size, count.min(self.genes_size)).into_vec()
        }
    }

    fn mutate_chromosome_genes<R: Rng>(
        &self,
        number_of_mutations: usize,
        allow_duplicates: bool,
        chromosome: &mut Chromosome<Self::Allele>,
        rng: &mut R,
    ) {
        if allow_duplicates {
            for _ in 0..number_of_mutations {
                let index = self.gene_index_sampler.sample(rng);
                chromosome.genes[index] = self.allele_list[self.allele_index_sampler.sample(rng)];
            }
        } else {
            rand::seq::index::sample(
                rng,
                self.genes_size,
                number_of_mutations.min(self.genes_size),
            )
            .iter()
            .for_each(|index| {
                chromosome.genes[index] = self.allele_list[self.allele_index_sampler.sample(rng)];
            });
        }
        chromosome.reset_metadata(self.genes_hashing);
    }
    fn with_seed_genes_list(&self, seed_genes_list: Vec<Genes<Self::Allele>>) -> Self {
        let mut new = self.clone();
        new.seed_genes_list = seed_genes_list;
        new
    }
    fn seed_genes_list(&self) -> &Vec<Genes<Self::Allele>> {
        &self.seed_genes_list
    }
    fn random_genes_factory<R: Rng>(&self, rng: &mut R) -> Vec<T> {
        if self.seed_genes_list.is_empty() {
            (0..self.genes_size)
                .map(|_| self.allele_list[self.allele_index_sampler.sample(rng)])
                .collect()
        } else {
            self.seed_genes_list.choose(rng).unwrap().clone()
        }
    }
    fn genes_capacity(&self) -> usize {
        self.genes_size
    }
    fn genes_hashing(&self) -> bool {
        self.genes_hashing
    }
}

impl<T: Allele + PartialEq + Hash> EvolveGenotype for List<T> {
    fn crossover_chromosome_genes<R: Rng>(
        &self,
        number_of_crossovers: usize,
        allow_duplicates: bool,
        father: &mut Chromosome<Self::Allele>,
        mother: &mut Chromosome<Self::Allele>,
        rng: &mut R,
    ) {
        if allow_duplicates {
            rng.sample_iter(self.gene_index_sampler)
                .take(number_of_crossovers)
                .for_each(|index| {
                    std::mem::swap(&mut father.genes[index], &mut mother.genes[index]);
                });
        } else {
            rand::seq::index::sample(
                rng,
                self.genes_size(),
                number_of_crossovers.min(self.genes_size()),
            )
            .iter()
            .for_each(|index| {
                std::mem::swap(&mut father.genes[index], &mut mother.genes[index]);
            });
        }
        mother.reset_metadata(self.genes_hashing);
        father.reset_metadata(self.genes_hashing);
    }
    fn crossover_chromosome_points<R: Rng>(
        &self,
        number_of_crossovers: usize,
        allow_duplicates: bool,
        father: &mut Chromosome<Self::Allele>,
        mother: &mut Chromosome<Self::Allele>,
        rng: &mut R,
    ) {
        if allow_duplicates {
            rng.sample_iter(self.gene_index_sampler)
                .take(number_of_crossovers)
                .for_each(|index| {
                    let mother_back = &mut mother.genes[index..];
                    let father_back = &mut father.genes[index..];
                    father_back.swap_with_slice(mother_back);
                });
        } else {
            rand::seq::index::sample(
                rng,
                self.genes_size(),
                number_of_crossovers.min(self.genes_size()),
            )
            .iter()
            .sorted_unstable()
            .chunks(2)
            .into_iter()
            .for_each(|mut chunk| match (chunk.next(), chunk.next()) {
                (Some(start_index), Some(end_index)) => {
                    let mother_back = &mut mother.genes[start_index..end_index];
                    let father_back = &mut father.genes[start_index..end_index];
                    father_back.swap_with_slice(mother_back);
                }
                (Some(start_index), _) => {
                    let mother_back = &mut mother.genes[start_index..];
                    let father_back = &mut father.genes[start_index..];
                    father_back.swap_with_slice(mother_back);
                }
                _ => (),
            });
        }
        mother.reset_metadata(self.genes_hashing);
        father.reset_metadata(self.genes_hashing);
    }

    fn has_crossover_indexes(&self) -> bool {
        true
    }
    fn has_crossover_points(&self) -> bool {
        true
    }
}
impl<T: Allele + PartialEq + Hash> HillClimbGenotype for List<T> {
    fn fill_neighbouring_population<R: Rng>(
        &self,
        chromosome: &Chromosome<Self::Allele>,
        population: &mut Population<Self::Allele>,
        _rng: &mut R,
    ) {
        for index in 0..self.genes_size() {
            for allele_value in self.allele_list.clone() {
                if chromosome.genes[index] != allele_value {
                    let mut new_chromosome = population.new_chromosome(chromosome);
                    new_chromosome.genes[index] = allele_value;
                    new_chromosome.reset_metadata(self.genes_hashing);
                    population.chromosomes.push(new_chromosome);
                }
            }
        }
    }

    fn neighbouring_population_size(&self) -> BigUint {
        BigUint::from((self.allele_list.len() - 1) * self.genes_size)
    }
}

impl<T: Allele + PartialEq + Hash> PermutateGenotype for List<T> {
    fn chromosome_permutations_into_iter<'a>(
        &'a self,
        _chromosome: Option<&Chromosome<Self::Allele>>,
    ) -> Box<dyn Iterator<Item = Chromosome<Self::Allele>> + Send + 'a> {
        if self.seed_genes_list.is_empty() {
            Box::new(
                (0..self.genes_size())
                    .map(|_| self.allele_list.clone())
                    .multi_cartesian_product()
                    .map(Chromosome::new),
            )
        } else {
            Box::new(
                self.seed_genes_list
                    .clone()
                    .into_iter()
                    .map(Chromosome::new),
            )
        }
    }

    fn chromosome_permutations_size(&self) -> BigUint {
        if self.seed_genes_list.is_empty() {
            BigUint::from(self.allele_list.len()).pow(self.genes_size() as u32)
        } else {
            self.seed_genes_list.len().into()
        }
    }
    fn mutation_type_allows_permutation(&self) -> bool {
        true
    }
}

impl<T: Allele + PartialEq + Hash> fmt::Display for List<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "genotype:")?;
        writeln!(f, "  genes_size: {}", self.genes_size)?;
        writeln!(f, "  mutation_type: {:?}", self.mutation_type())?;
        writeln!(
            f,
            "  chromosome_permutations_size: {}",
            self.chromosome_permutations_size_report()
        )?;
        writeln!(
            f,
            "  neighbouring_population_size: {}",
            self.neighbouring_population_size_report()
        )?;
        writeln!(
            f,
            "  expected_number_of_sampled_index_duplicates: {}",
            self.expected_number_of_sampled_index_duplicates_report()
        )?;
        writeln!(f, "  seed_genes: {:?}", self.seed_genes_list.len())
    }
}
