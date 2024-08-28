use super::builder::{Builder, TryFromBuilderError};
use super::{Allele, Genotype, IncrementalGenotype, PermutableGenotype};
use crate::chromosome::Chromosome;
use itertools::Itertools;
use num::BigUint;
use rand::distributions::{Distribution, Uniform, WeightedIndex};
use rand::prelude::*;
use std::fmt;

pub type DefaultAllele = usize;

/// Genes are a list of values, each individually taken from its own allele_list using clone(). The
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
/// #[derive(Clone, Copy, PartialEq, Debug)]
/// struct Item(pub u16, pub u16);
/// impl Allele for Item {}
///
/// let genotype = MultiListGenotype::builder()
///     .with_allele_lists(vec![
///       vec![Item(23, 505), Item(26, 352), Item(20, 458)],
///       vec![Item(23, 505), Item(26, 352)],
///       vec![Item(26, 352), Item(20, 458), Item(13, 123)],
///     ])
///     .build()
///     .unwrap();
/// ```
#[derive(Clone, Debug)]
pub struct MultiList<T: Allele + PartialEq = DefaultAllele> {
    genes_size: usize,
    pub allele_lists: Vec<Vec<T>>,
    gene_index_sampler: Uniform<usize>,
    gene_weighted_index_sampler: WeightedIndex<usize>,
    allele_index_samplers: Vec<Uniform<usize>>,
    allele_list_sizes: Vec<usize>,
    pub seed_genes_list: Vec<Vec<T>>,
}

impl<T: Allele + PartialEq> TryFrom<Builder<Self>> for MultiList<T> {
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

impl<T: Allele + PartialEq> Genotype for MultiList<T> {
    type Allele = T;
    fn genes_size(&self) -> usize {
        self.genes_size
    }
    fn random_genes_factory<R: Rng>(&self, rng: &mut R) -> Vec<Self::Allele> {
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
    fn chromosome_factory<R: Rng>(&self, rng: &mut R) -> Chromosome<Self::Allele> {
        Chromosome::new(self.random_genes_factory(rng))
    }

    fn mutate_chromosome_single<R: Rng>(
        &self,
        chromosome: &mut Chromosome<Self::Allele>,
        _scale_index: Option<usize>,
        rng: &mut R,
    ) {
        let index = self.gene_weighted_index_sampler.sample(rng);
        chromosome.genes[index] =
            self.allele_lists[index][self.allele_index_samplers[index].sample(rng)];
        chromosome.taint_fitness_score();
    }

    fn mutate_chromosome_multi<R: Rng>(
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
        chromosome.taint_fitness_score();
    }

    fn crossover_index_sampler(&self) -> Option<&Uniform<usize>> {
        Some(&self.gene_index_sampler)
    }
    fn crossover_point_sampler(&self) -> Option<&Uniform<usize>> {
        Some(&self.gene_index_sampler)
    }
    fn set_seed_genes_list(&mut self, seed_genes_list: Vec<Vec<T>>) {
        self.seed_genes_list = seed_genes_list;
    }
    fn seed_genes_list(&self) -> &Vec<Vec<T>> {
        &self.seed_genes_list
    }
    fn max_scale_index(&self) -> Option<usize> {
        None
    }
}

impl<T: Allele + PartialEq> IncrementalGenotype for MultiList<T> {
    fn neighbouring_chromosomes<R: Rng>(
        &self,
        chromosome: &Chromosome<Self::Allele>,
        _scale_index: Option<usize>,
        _rng: &mut R,
    ) -> Vec<Chromosome<Self::Allele>> {
        (0..self.genes_size)
            .flat_map(|index| {
                self.allele_lists[index]
                    .iter()
                    .filter_map(move |allele_value| {
                        if chromosome.genes[index] == *allele_value {
                            None
                        } else {
                            let mut genes = chromosome.genes.clone();
                            genes[index] = *allele_value;
                            Some(Chromosome::new(genes))
                        }
                    })
            })
            .collect::<Vec<_>>()
    }

    fn neighbouring_population_size(&self) -> BigUint {
        BigUint::from(self.allele_list_sizes.iter().map(|v| *v - 1).sum::<usize>())
    }
}

impl<T: Allele + PartialEq> PermutableGenotype for MultiList<T> {
    //noop
    fn allele_list_for_chromosome_permutations(&self) -> Vec<Self::Allele> {
        vec![]
    }

    fn chromosome_permutations_into_iter(
        &self,
    ) -> impl Iterator<Item = Chromosome<Self::Allele>> + Send {
        self.allele_lists
            .clone()
            .into_iter()
            .multi_cartesian_product()
            .map(Chromosome::new)
    }

    fn chromosome_permutations_size(&self) -> BigUint {
        self.allele_list_sizes
            .iter()
            .map(|v| BigUint::from(*v))
            .product()
    }
}

impl<T: Allele + PartialEq> fmt::Display for MultiList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "genotype:")?;
        writeln!(f, "  genes_size: {}", self.genes_size)?;
        //writeln!(f, "  allele_lists: {:?}", self.allele_lists)?;
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
