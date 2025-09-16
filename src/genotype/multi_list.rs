use super::builder::{Builder, TryFromBuilderError};
use super::{EvolveGenotype, Genotype, HillClimbGenotype, PermutateGenotype};
use crate::allele::Allele;
use crate::chromosome::{Chromosome, Genes};
use crate::population::Population;
use itertools::Itertools;
use num::BigUint;
use rand::distributions::{Distribution, Uniform, WeightedIndex};
use rand::prelude::*;
use std::fmt;
use std::hash::Hash;

pub type DefaultAllele = usize;

/// Genes are a vector of values, each individually taken from its own allele_list using clone(). The
/// allele_lists are taken as unrelated, unorderable set with no concept op distance between the
/// values. The genes_size is derived to be the allele_lists length. All allele_list have to be of
/// the same type, but can have different values and lengths. On random initialization, each gene
/// gets a value from its own allele_list with a uniform probability. Each gene has a weighted
/// probability of mutating, depending on its allele_list length. If a gene mutates, a new values
/// is taken from its own allele_list with a uniform probability (regardless of current value,
/// which could therefore be assigned again, not mutating as a result). Duplicate allele values are
/// allowed. Defaults to usize as item.
///
/// This genotype is also used in the [meta analysis](https://github.com/basvanwesting/genetic-algorithm-meta.git), to hold the indices of the
/// different [Evolve](crate::strategy::evolve::Evolve) configuration values (defined outside of the genotype).
///
/// # Example (usize, default):
/// ```
/// use genetic_algorithm::genotype::{Genotype, MultiListGenotype};
///
/// let genotype = MultiListGenotype::builder()
///     .with_allele_lists(vec![
///        (0..=10).collect(),
///        (0..=20).collect(),
///        (0..=5).collect(),
///        (0..=30).collect(),
///     ])
///     .with_genes_hashing(false) // optional, defaults to false
///     .build()
///     .unwrap();
/// ```
///
/// # Example (usize, used to lookup external types of different kind):
/// ```
/// use genetic_algorithm::genotype::{Genotype, MultiListGenotype};
///
/// let cars = vec!["BMW X3", "Ford Mustang", "Chevrolet Camaro"];
/// let drivers = vec!["Louis", "Max", "Charles"];
/// let number_of_laps = vec![10, 20, 30, 40];
/// let rain_probabilities = vec![0.0, 0.2, 0.4, 0.6, 0.8, 1.0];
///
/// let genotype = MultiListGenotype::builder()
///     .with_allele_lists(vec![
///        (0..cars.len()).collect(),
///        (0..drivers.len()).collect(),
///        (0..number_of_laps.len()).collect(),
///        (0..rain_probabilities.len()).collect(),
///     ])
///     .with_genes_hashing(true) // optional, defaults to false
///     .build()
///     .unwrap();
///
/// // The fitness function will be provided the genes (e.g. [2,0,1,4]) and will then have to
/// // lookup the external types and implement some fitness logic for the combination (e.g.
/// // ["Chevrolet Camaro", "Louis", 20, 0.8])
/// ```
///
/// # Example (struct, the limitation is that the type needs to be the same for all lists)
/// ```
/// use genetic_algorithm::genotype::{Allele, Genotype, MultiListGenotype};
///
/// #[derive(Clone, Copy, PartialEq, Hash, Debug)]
/// struct Item(pub u16, pub u16);
/// genetic_algorithm::impl_allele!(Item);
///
/// let genotype = MultiListGenotype::builder()
///     .with_allele_lists(vec![
///       vec![Item(23, 505), Item(26, 352), Item(20, 458)],
///       vec![Item(23, 505), Item(26, 352)],
///       vec![Item(26, 352), Item(20, 458), Item(13, 123)],
///     ])
///     .with_genes_hashing(false) // optional, defaults to false
///     .build()
///     .unwrap();
/// ```
#[derive(Clone, Debug)]
pub struct MultiList<T: Allele + PartialEq + Hash = DefaultAllele> {
    pub genes_size: usize,
    pub allele_lists: Vec<Vec<T>>,
    pub allele_list_sizes: Vec<usize>,
    gene_index_sampler: Uniform<usize>,
    gene_weighted_index_sampler: WeightedIndex<usize>,
    allele_index_samplers: Vec<Uniform<usize>>,
    pub seed_genes_list: Vec<Vec<T>>,
}

impl<T: Allele + PartialEq + Hash> TryFrom<Builder<Self>> for MultiList<T> {
    type Error = TryFromBuilderError;

    fn try_from(builder: Builder<Self>) -> Result<Self, Self::Error> {
        if builder.allele_lists.is_none() {
            Err(TryFromBuilderError(
                "MultiListGenotype requires a allele_lists",
            ))
        } else if builder.allele_lists.as_ref().map(|o| o.is_empty()).unwrap() {
            Err(TryFromBuilderError(
                "MultiListGenotype requires non-empty allele_lists",
            ))
        } else {
            let allele_lists = builder.allele_lists.unwrap();
            let genes_size = allele_lists.len();
            let allele_list_sizes: Vec<usize> = allele_lists.iter().map(|v| v.len()).collect();
            Ok(Self {
                genes_size,
                allele_list_sizes: allele_list_sizes.clone(),
                allele_lists: allele_lists.clone(),
                gene_index_sampler: Uniform::from(0..genes_size),
                gene_weighted_index_sampler: WeightedIndex::new(allele_list_sizes.clone()).unwrap(),
                allele_index_samplers: allele_list_sizes
                    .iter()
                    .map(|allele_value_size| Uniform::from(0..*allele_value_size))
                    .collect(),
                seed_genes_list: builder.seed_genes_list,
            })
        }
    }
}

impl<T: Allele + PartialEq + Hash> MultiList<T> {
    pub fn sample_allele<R: Rng>(&self, index: usize, rng: &mut R) -> T {
        self.allele_lists[index][self.allele_index_samplers[index].sample(rng)]
    }
}

impl<T: Allele + PartialEq + Hash> Genotype for MultiList<T> {
    type Allele = T;

    fn genes_size(&self) -> usize {
        self.genes_size
    }
    fn genes_slice<'a>(&'a self, chromosome: &'a Chromosome<Self::Allele>) -> &'a [Self::Allele] {
        chromosome.genes.as_slice()
    }

    fn sample_gene_index<R: Rng>(&self, rng: &mut R) -> usize {
        self.gene_weighted_index_sampler.sample(rng)
    }
    fn sample_gene_indices<R: Rng>(
        &self,
        count: usize,
        allow_duplicates: bool,
        rng: &mut R,
    ) -> Vec<usize> {
        if allow_duplicates {
            (0..count)
                .map(|_| self.gene_weighted_index_sampler.sample(rng))
                .collect()
        } else {
            rand::seq::index::sample_weighted(
                rng,
                self.genes_size,
                |i| self.allele_list_sizes[i] as f64,
                count.min(self.genes_size),
            )
            .unwrap()
            .into_vec()
        }
    }

    fn mutate_chromosome_genes<R: Rng>(
        &self,
        number_of_mutations: usize,
        allow_duplicates: bool,
        chromosome: &mut Chromosome<Self::Allele>,
        _scale_index: Option<usize>,
        rng: &mut R,
    ) {
        if allow_duplicates {
            for _ in 0..number_of_mutations {
                let index = self.gene_weighted_index_sampler.sample(rng);
                chromosome.genes[index] =
                    self.allele_lists[index][self.allele_index_samplers[index].sample(rng)];
            }
        } else {
            rand::seq::index::sample_weighted(
                rng,
                self.genes_size,
                |i| self.allele_list_sizes[i] as f64,
                number_of_mutations.min(self.genes_size),
            )
            .unwrap()
            .iter()
            .for_each(|index| {
                chromosome.genes[index] =
                    self.allele_lists[index][self.allele_index_samplers[index].sample(rng)];
            });
        }
        chromosome.reset_state();
    }
    fn with_seed_genes_list(&self, seed_genes_list: Vec<Genes<Self::Allele>>) -> Self {
        let mut new = self.clone();
        new.seed_genes_list = seed_genes_list;
        new
    }
    fn seed_genes_list(&self) -> &Vec<Genes<Self::Allele>> {
        &self.seed_genes_list
    }
    fn max_scale_index(&self) -> Option<usize> {
        None
    }
    fn random_genes_factory<R: Rng>(&self, rng: &mut R) -> Vec<T> {
        if self.seed_genes_list.is_empty() {
            self.allele_lists
                .iter()
                .enumerate()
                .map(|(index, allele_list)| {
                    allele_list[self.allele_index_samplers[index].sample(rng)]
                })
                .collect()
        } else {
            self.seed_genes_list.choose(rng).unwrap().clone()
        }
    }
    fn genes_capacity(&self) -> usize {
        self.genes_size
    }
}

impl<T: Allele + PartialEq + Hash> EvolveGenotype for MultiList<T> {
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
        mother.reset_state();
        father.reset_state();
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
        mother.reset_state();
        father.reset_state();
    }

    fn has_crossover_indexes(&self) -> bool {
        true
    }
    fn has_crossover_points(&self) -> bool {
        true
    }
}
impl<T: Allele + PartialEq + Hash> HillClimbGenotype for MultiList<T> {
    fn fill_neighbouring_population<R: Rng>(
        &self,
        chromosome: &Chromosome<Self::Allele>,
        population: &mut Population<Self::Allele>,
        _scale_index: Option<usize>,
        _rng: &mut R,
    ) {
        for index in 0..self.genes_size() {
            for allele_value in self.allele_lists[index].clone() {
                if chromosome.genes[index] != allele_value {
                    let mut new_chromosome = population.get_or_create_chromosome(chromosome);
                    new_chromosome.genes[index] = allele_value;
                    new_chromosome.reset_state();
                    population.chromosomes.push(new_chromosome);
                }
            }
        }
    }

    fn neighbouring_population_size(&self) -> BigUint {
        BigUint::from(self.allele_list_sizes.iter().map(|v| *v - 1).sum::<usize>())
    }
}

impl<T: Allele + PartialEq + Hash> PermutateGenotype for MultiList<T> {
    fn chromosome_permutations_into_iter<'a>(
        &'a self,
        _chromosome: Option<&Chromosome<Self::Allele>>,
        _scale_index: Option<usize>,
    ) -> Box<dyn Iterator<Item = Chromosome<Self::Allele>> + Send + 'a> {
        if self.seed_genes_list.is_empty() {
            Box::new(
                self.allele_lists
                    .clone()
                    .into_iter()
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
            self.allele_list_sizes
                .iter()
                .map(|v| BigUint::from(*v))
                .product()
        } else {
            self.seed_genes_list.len().into()
        }
    }
    fn mutation_type_allows_permutation(&self) -> bool {
        true
    }
}

impl<T: Allele + PartialEq + Hash> fmt::Display for MultiList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "genotype:")?;
        writeln!(f, "  genes_size: {}", self.genes_size)?;
        writeln!(f, "  mutation_type: {:?}", self.mutation_type())?;
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
        writeln!(
            f,
            "  expected_number_of_sampled_index_duplicates: {}",
            self.expected_number_of_sampled_index_duplicates_report()
        )?;
        writeln!(f, "  seed_genes: {:?}", self.seed_genes_list.len())
    }
}
