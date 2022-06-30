use super::builder::{Builder, TryFromBuilderError};
use super::{Genotype, IncrementalGenotype, PermutableGenotype};
use crate::chromosome::Chromosome;
use factorial::Factorial;
use itertools::Itertools;
use num::BigUint;
use rand::distributions::{Distribution, Uniform, WeightedIndex};
use rand::prelude::*;
use std::fmt;

pub type DefaultAllele = usize;

/// Genes are a concatinated list of sets of unique values, each set taken from its own
/// allele_values using clone(). The genes_size is derived to be the sum of the allele_values
/// lengths. All allele_values have to be of the same type, but can have different values and
/// lengths. On random initialization, the allele_values sets are internally suffled and
/// concatinated to form the genes, but the order of the sets is always the same. Each unique set
/// has a weighted probability of mutating, depending on its allele_values length. If a set
/// mutates, the values for a pair of genes in the set are switched, ensuring the set remains
/// unique. Duplicate allele values are allowed. Defaults to usize as item.
///
/// # Example (usize, default):
/// ```
/// use genetic_algorithm::genotype::{Genotype, MultiUniqueGenotype};
///
/// let genotype = MultiUniqueGenotype::builder()
///     .with_allele_multi_values(vec![
///        (0..3).collect(),
///        (4..6).collect(),
///        (7..9).collect(),
///        (0..2).collect(),
///     ])
///     .build()
///     .unwrap();
///
/// // chromosome genes example: [1,2,3, 5,4,6, 9,8,7, 1,0]
/// // four unique sets internally suffled
/// ```
///
/// # Example (struct, the limitation is that the type needs to be the same for all lists)
/// ```
/// use genetic_algorithm::genotype::{Genotype, MultiUniqueGenotype};
///
/// #[derive(Clone, Debug)]
/// struct Item(pub u16, pub u16);
///
/// let genotype = MultiUniqueGenotype::builder()
///     .with_allele_multi_values(vec![
///       vec![Item(1, 505), Item(2, 352), Item(3, 458)],
///       vec![Item(4, 505), Item(5, 352)],
///       vec![Item(6, 352), Item(7, 458), Item(8, 123)],
///     ])
///     .build()
///     .unwrap();
/// ```
#[derive(Clone, Debug)]
pub struct MultiUnique<T: Clone + std::fmt::Debug = DefaultAllele> {
    genes_size: usize,
    allele_values_sizes: Vec<usize>,
    allele_values_index_offsets: Vec<usize>,
    pub allele_multi_values: Vec<Vec<T>>,
    allele_values_index_sampler: WeightedIndex<usize>,
    allele_values_index_samplers: Vec<Uniform<usize>>,
    pub seed_genes: Option<Vec<T>>,
}

impl<T: Clone + std::fmt::Debug> TryFrom<Builder<Self>> for MultiUnique<T> {
    type Error = TryFromBuilderError;

    fn try_from(builder: Builder<Self>) -> Result<Self, Self::Error> {
        if builder.allele_multi_values.is_none() {
            Err(TryFromBuilderError(
                "MultiUniqueGenotype requires a allele_multi_values",
            ))
        } else if builder
            .allele_multi_values
            .as_ref()
            .map(|o| o.is_empty())
            .unwrap()
        {
            Err(TryFromBuilderError(
                "MultiUniqueGenotype requires non-empty allele_multi_values",
            ))
        } else {
            let allele_multi_values = builder.allele_multi_values.unwrap();
            let allele_values_sizes: Vec<usize> =
                allele_multi_values.iter().map(|v| v.len()).collect();
            let allele_values_index_offsets =
                allele_values_sizes.iter().fold(vec![0], |mut acc, size| {
                    acc.push(*acc.last().unwrap() + size);
                    acc
                });
            Ok(Self {
                genes_size: allele_values_sizes.iter().sum(),
                allele_values_sizes: allele_values_sizes.clone(),
                allele_values_index_offsets: allele_values_index_offsets,
                allele_multi_values: allele_multi_values.clone(),
                allele_values_index_sampler: WeightedIndex::new(allele_values_sizes.clone())
                    .unwrap(),
                allele_values_index_samplers: allele_values_sizes
                    .iter()
                    .map(|allele_value_size| Uniform::from(0..*allele_value_size))
                    .collect(),
                seed_genes: builder.seed_genes,
            })
        }
    }
}

impl<T: Clone + std::fmt::Debug> Genotype for MultiUnique<T> {
    type Allele = T;
    fn genes_size(&self) -> usize {
        self.genes_size
    }
    fn crossover_points(&self) -> Vec<usize> {
        let mut crossover_points = self.allele_values_sizes.clone();
        crossover_points.pop();
        crossover_points
    }
    ///unique genotypes can't simply exchange genes without gene duplication issues
    fn crossover_indexes(&self) -> Vec<usize> {
        vec![]
    }
    fn chromosome_factory<R: Rng>(&self, rng: &mut R) -> Chromosome<Self> {
        if let Some(seed_genes) = self.seed_genes.as_ref() {
            Chromosome::new(seed_genes.clone())
        } else {
            let genes: Vec<Self::Allele> = self
                .allele_multi_values
                .iter()
                .flat_map(|allele_values| {
                    let mut genes = allele_values.clone();
                    genes.shuffle(rng);
                    genes
                })
                .collect();
            Chromosome::new(genes)
        }
    }

    fn mutate_chromosome_random<R: Rng>(&self, chromosome: &mut Chromosome<Self>, rng: &mut R) {
        let index = self.allele_values_index_sampler.sample(rng);
        let index_offset: usize = self.allele_values_index_offsets[index];
        let index1 = index_offset + self.allele_values_index_samplers[index].sample(rng);
        let index2 = index_offset + self.allele_values_index_samplers[index].sample(rng);
        chromosome.genes.swap(index1, index2);
        chromosome.taint_fitness_score();
    }
}

impl<T: Clone + std::fmt::Debug> IncrementalGenotype for MultiUnique<T> {
    fn mutate_chromosome_neighbour<R: Rng>(&self, chromosome: &mut Chromosome<Self>, rng: &mut R) {
        self.mutate_chromosome_random(chromosome, rng);
    }

    fn chromosome_neighbours(
        &self,
        chromosome: &Chromosome<Self>,
        _scale: f32,
    ) -> Vec<Chromosome<Self>> {
        self.allele_values_sizes
            .iter()
            .enumerate()
            .flat_map(|(index, allele_value_size)| {
                let index_offset: usize = self.allele_values_index_offsets[index];

                (0..*allele_value_size)
                    .combinations(2)
                    .map(|pair| {
                        let mut new_genes = chromosome.genes.clone();
                        new_genes.swap(index_offset + pair[0], index_offset + pair[1]);
                        new_genes
                    })
                    .map(|genes| Chromosome::new(genes))
                    .collect::<Vec<Chromosome<Self>>>()
            })
            .collect::<Vec<Chromosome<Self>>>()
    }

    fn chromosome_neighbours_size(&self) -> BigUint {
        self.allele_values_sizes
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

impl<T: Clone + std::fmt::Debug> PermutableGenotype for MultiUnique<T> {
    //noop
    fn allele_values_for_chromosome_permutations(&self) -> Vec<Self::Allele> {
        vec![]
    }

    fn chromosome_permutations_into_iter<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = Chromosome<Self>> + 'a> {
        Box::new(
            self.allele_multi_values
                .clone()
                .into_iter()
                .map(|allele_values| {
                    let size = allele_values.len();
                    allele_values.into_iter().permutations(size)
                })
                .multi_cartesian_product()
                .map(|gene_sets| Chromosome::new(gene_sets.into_iter().concat())),
        )
    }

    fn chromosome_permutations_size(&self) -> BigUint {
        self.allele_values_sizes
            .iter()
            .map(|v| BigUint::from(*v))
            .fold(BigUint::from(1u8), |acc, allele_values_size| {
                acc * allele_values_size.factorial()
            })
    }
}

impl<T: Clone + std::fmt::Debug> fmt::Display for MultiUnique<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "genotype:")?;
        writeln!(f, "  genes_size: {}\n", self.genes_size)?;
        writeln!(f, "  allele_values_sizes: {:?}", self.allele_values_sizes)?;
        writeln!(f, "  allele_multi_values: {:?}", self.allele_multi_values)?;
        writeln!(
            f,
            "  chromosome_permutations_size: {}",
            self.chromosome_permutations_size()
        )?;
        writeln!(
            f,
            "  chromosome_neighbours_size: {}",
            self.chromosome_neighbours_size()
        )?;
        writeln!(f, "  seed_genes: {:?}", self.seed_genes)
    }
}
