use super::builder::{Builder, TryFromBuilderError};
use super::{Allele, Genotype, IncrementalGenotype, PermutableGenotype};
use crate::chromosome::Chromosome;
use factorial::Factorial;
use itertools::Itertools;
use num::BigUint;
use rand::distributions::{Distribution, Uniform, WeightedIndex};
use rand::prelude::*;
use std::fmt;

pub type DefaultAllele = usize;

/// Genes are a concatinated list of sets of unique values, each set taken from its own
/// allele_list using clone(). The genes_size is derived to be the sum of the allele_list
/// lengths. All allele_list have to be of the same type, but can have different values and
/// lengths. On random initialization, the allele_list sets are internally shuffled and
/// concatinated to form the genes, but the order of the sets is always the same. Each unique set
/// has a weighted probability of mutating, depending on its allele_list length. If a set
/// mutates, the values for a pair of genes in the set are switched, ensuring the set remains
/// unique. Duplicate allele values are allowed. Defaults to usize as item.
///
/// # Example (usize, default):
/// ```
/// use genetic_algorithm::genotype::{Genotype, MultiUniqueGenotype};
///
/// let genotype = MultiUniqueGenotype::builder()
///     .with_allele_lists(vec![
///        (0..=3).collect(),
///        (4..=6).collect(),
///        (7..=9).collect(),
///        (0..=2).collect(),
///     ])
///     .build()
///     .unwrap();
///
/// // chromosome genes example: [1,2,3, 5,4,6, 9,8,7, 1,0]
/// // four unique sets internally shuffled
/// ```
///
/// # Example (struct, the limitation is that the type needs to be the same for all lists)
/// ```
/// use genetic_algorithm::genotype::{Allele, Genotype, MultiUniqueGenotype};
///
/// #[derive(Clone, Debug, PartialEq)]
/// struct Item(pub u16, pub u16);
/// impl Allele for Item {}
///
/// let genotype = MultiUniqueGenotype::builder()
///     .with_allele_lists(vec![
///       vec![Item(1, 505), Item(2, 352), Item(3, 458)],
///       vec![Item(4, 505), Item(5, 352)],
///       vec![Item(6, 352), Item(7, 458), Item(8, 123)],
///     ])
///     .build()
///     .unwrap();
/// ```
#[derive(Clone, Debug)]
pub struct MultiUnique<T: Allele = DefaultAllele> {
    genes_size: usize,
    allele_list_sizes: Vec<usize>,
    allele_list_index_offsets: Vec<usize>,
    pub allele_lists: Vec<Vec<T>>,
    allele_list_index_sampler: WeightedIndex<usize>,
    allele_list_index_samplers: Vec<Uniform<usize>>,
    pub seed_genes_list: Vec<Vec<T>>,
}

impl<T: Allele> TryFrom<Builder<Self>> for MultiUnique<T> {
    type Error = TryFromBuilderError;

    fn try_from(builder: Builder<Self>) -> Result<Self, Self::Error> {
        if builder.allele_lists.is_none() {
            Err(TryFromBuilderError(
                "MultiUniqueGenotype requires a allele_lists",
            ))
        } else if builder.allele_lists.as_ref().map(|o| o.is_empty()).unwrap() {
            Err(TryFromBuilderError(
                "MultiUniqueGenotype requires non-empty allele_lists",
            ))
        } else {
            let allele_lists = builder.allele_lists.unwrap();
            let allele_list_sizes: Vec<usize> = allele_lists.iter().map(|v| v.len()).collect();
            let allele_list_index_offsets =
                allele_list_sizes.iter().fold(vec![0], |mut acc, size| {
                    acc.push(*acc.last().unwrap() + size);
                    acc
                });
            Ok(Self {
                genes_size: allele_list_sizes.iter().sum(),
                allele_list_sizes: allele_list_sizes.clone(),
                allele_list_index_offsets,
                allele_lists: allele_lists.clone(),
                allele_list_index_sampler: WeightedIndex::new(allele_list_sizes.clone()).unwrap(),
                allele_list_index_samplers: allele_list_sizes
                    .iter()
                    .map(|allele_value_size| Uniform::from(0..*allele_value_size))
                    .collect(),
                seed_genes_list: builder.seed_genes_list,
            })
        }
    }
}

impl<T: Allele> Genotype for MultiUnique<T> {
    type Allele = T;
    fn genes_size(&self) -> usize {
        self.genes_size
    }
    fn crossover_points(&self) -> Vec<usize> {
        let mut crossover_points = self.allele_list_sizes.clone();
        crossover_points.pop();
        crossover_points
    }
    ///unique genotypes can't simply exchange genes without gene duplication issues
    fn crossover_indexes(&self) -> Vec<usize> {
        vec![]
    }
    fn random_genes_factory<R: Rng>(&self, rng: &mut R) -> Vec<Self::Allele> {
        if self.seed_genes_list.is_empty() {
            self.allele_lists
                .iter()
                .flat_map(|allele_list| {
                    let mut genes = allele_list.clone();
                    genes.shuffle(rng);
                    genes
                })
                .collect()
        } else {
            self.seed_genes_list.choose(rng).unwrap().clone()
        }
    }
    fn chromosome_factory<R: Rng>(&self, rng: &mut R) -> Chromosome<Self::Allele> {
        Chromosome::new(self.random_genes_factory(rng))
    }

    fn mutate_chromosome<R: Rng>(
        &self,
        chromosome: &mut Chromosome<Self::Allele>,
        _scale_index: Option<usize>,
        rng: &mut R,
    ) {
        let index = self.allele_list_index_sampler.sample(rng);
        let index_offset: usize = self.allele_list_index_offsets[index];
        let index1 = index_offset + self.allele_list_index_samplers[index].sample(rng);
        let index2 = index_offset + self.allele_list_index_samplers[index].sample(rng);
        chromosome.genes.swap(index1, index2);
        chromosome.taint_fitness_score();
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

impl<T: Allele> IncrementalGenotype for MultiUnique<T> {
    fn neighbouring_chromosomes<R: Rng>(
        &self,
        chromosome: &Chromosome<Self::Allele>,
        _scale_index: Option<usize>,
        _rng: &mut R,
    ) -> Vec<Chromosome<Self::Allele>> {
        self.allele_list_sizes
            .iter()
            .enumerate()
            .flat_map(|(index, allele_value_size)| {
                let index_offset: usize = self.allele_list_index_offsets[index];

                (0..*allele_value_size)
                    .tuple_combinations()
                    .map(|(first, second)| {
                        let mut new_genes = chromosome.genes.clone();
                        new_genes.swap(index_offset + first, index_offset + second);
                        new_genes
                    })
                    .map(Chromosome::new)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    }

    fn neighbouring_population_size(&self) -> BigUint {
        self.allele_list_sizes
            .iter()
            .filter(|allele_value_size| **allele_value_size > 1)
            .map(|allele_value_size| {
                let n = BigUint::from(*allele_value_size);
                let k = BigUint::from(2usize);

                n.factorial() / (k.factorial() * (n - k).factorial())
            })
            .sum()
    }
}

impl<T: Allele> PermutableGenotype for MultiUnique<T> {
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
            .map(|allele_list| {
                let size = allele_list.len();
                allele_list.into_iter().permutations(size)
            })
            .multi_cartesian_product()
            .map(|gene_sets| Chromosome::new(gene_sets.into_iter().concat()))
    }

    fn chromosome_permutations_size(&self) -> BigUint {
        self.allele_list_sizes
            .iter()
            .map(|v| BigUint::from(*v))
            .fold(BigUint::from(1u8), |acc, allele_list_size| {
                acc * allele_list_size.factorial()
            })
    }
}

impl<T: Allele> fmt::Display for MultiUnique<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "genotype:")?;
        writeln!(f, "  genes_size: {}", self.genes_size)?;
        writeln!(f, "  allele_lists: {:?}", self.allele_lists)?;
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
