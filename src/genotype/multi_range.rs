use super::builder::{Builder, TryFromBuilderError};
use super::{Allele, Genotype, IncrementalGenotype};
use crate::chromosome::Chromosome;
use itertools::Itertools;
use num::{BigUint, Zero};
use rand::distributions::uniform::SampleUniform;
use rand::distributions::{Bernoulli, Distribution, Uniform, WeightedIndex};
use rand::prelude::*;
use std::fmt;
use std::ops::{Add, RangeInclusive};

pub type DefaultAllele = f32;

/// Genes are a list of numberic values, each individually taken from its own allele_range. The genes_size is
/// derived to be the allele_ranges length. On random initialization, each gene gets a value
/// from its own allele_range with a uniform probability. Each gene has a weighted probability of
/// mutating, depending on its allele_range size. If a gene mutates, a new values is taken from its
/// own allele_range with a uniform probability. Duplicate allele values are allowed.
///
/// For (optional) neighbouring logic an allele_neighbour_ranges or allele_neighbour_scaled_ranges
/// must be provided.
/// When allele_neighbour_ranges is provided the mutation is restricted to modify
/// the existing value by a difference taken from allele_neighbour_ranges with a uniform
/// probability.
/// When allele_neighbour_scaled_ranges is provided the mutation is restricted to modify
/// the existing value by a difference taken from edges of the scaled ranges (depending on current scale)
///
/// # Example (f32, default):
/// ```
/// use genetic_algorithm::genotype::{Genotype, MultiRangeGenotype};
///
/// let genotype = MultiRangeGenotype::builder()
///     .with_allele_ranges(vec![
///        0.0..=10.0,
///        5.0..=20.0,
///        0.0..=5.0,
///        10.0..=30.0
///     ])
///     .with_allele_neighbour_ranges(vec![
///        -1.0..=1.0,
///        -2.0..=2.0,
///        -0.5..=0.5,
///        -3.0..=3.0
///     ]) // optional, only required for neighbouring logic
///     .with_allele_neighbour_scaled_ranges(vec![
///        vec![-1.0..=1.0, -2.0..=2.0, -0.5..=0.5, -3.0..=3.0],
///        vec![-0.1..=0.1, -0.2..=0.2, -0.05..=0.05, -0.3..=0.3],
///     ]) // optional, only required for neighbouring logic
///     .build()
///     .unwrap();
/// ```
///
/// # Example (isize):
/// ```
/// use genetic_algorithm::genotype::{Genotype, MultiRangeGenotype};
///
/// let genotype = MultiRangeGenotype::builder()
///     .with_allele_ranges(vec![
///        0..=10,
///        5..=20,
///        -5..=5,
///        10..=30,
///     ])
///     .with_allele_neighbour_ranges(vec![
///        -1..=1,
///        -1..=1,
///        -1..=1,
///        -2..=2,
///     ]) // optional, only required for neighbouring logic
///     .with_allele_neighbour_scaled_ranges(vec![
///        vec![-1..=1, -2..=2, -1..=1, -5..=5],
///        vec![-1..=1, -1..=1, -1..=1, -1..=1],
///     ]) // optional, only required for neighbouring logic
///     .build()
///     .unwrap();
/// ```
pub struct MultiRange<
    T: Allele + Copy + Default + Zero + Into<f64> + Add<Output = T> + std::cmp::PartialOrd = DefaultAllele,
> where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    genes_size: usize,
    pub allele_ranges: Vec<RangeInclusive<T>>,
    pub allele_neighbour_ranges: Option<Vec<RangeInclusive<T>>>,
    pub allele_neighbour_scaled_ranges: Option<Vec<Vec<RangeInclusive<T>>>>,
    gene_index_sampler: WeightedIndex<f64>,
    allele_samplers: Vec<Uniform<T>>,
    allele_neighbour_samplers: Option<Vec<Uniform<T>>>,
    sign_sampler: Bernoulli,
    pub seed_genes_list: Vec<Vec<T>>,
}

impl<T: Allele + Copy + Default + Zero + Into<f64> + Add<Output = T> + std::cmp::PartialOrd>
    TryFrom<Builder<Self>> for MultiRange<T>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    type Error = TryFromBuilderError;

    fn try_from(builder: Builder<Self>) -> Result<Self, Self::Error> {
        if builder.allele_ranges.is_none() {
            Err(TryFromBuilderError(
                "MultiRangeGenotype requires a allele_ranges",
            ))
        } else if builder
            .allele_ranges
            .as_ref()
            .map(|o| o.is_empty())
            .unwrap()
        {
            Err(TryFromBuilderError(
                "MultiRangeGenotype requires non-empty allele_ranges",
            ))
        } else {
            let allele_ranges = builder.allele_ranges.unwrap();
            let genes_size = allele_ranges.len();
            let index_weights: Vec<f64> = allele_ranges
                .iter()
                .map(|allele_range| (*allele_range.end()).into() - (*allele_range.start()).into())
                .collect();

            Ok(Self {
                genes_size,
                allele_ranges: allele_ranges.clone(),
                allele_neighbour_ranges: builder.allele_neighbour_ranges.clone(),
                allele_neighbour_scaled_ranges: builder.allele_neighbour_scaled_ranges.clone(),
                gene_index_sampler: WeightedIndex::new(index_weights).unwrap(),
                allele_samplers: allele_ranges
                    .iter()
                    .map(|allele_range| Uniform::from(allele_range.clone()))
                    .collect(),
                allele_neighbour_samplers: builder.allele_neighbour_ranges.map(
                    |allele_neighbour_ranges| {
                        allele_neighbour_ranges
                            .iter()
                            .map(|allele_neighbour_range| {
                                Uniform::from(allele_neighbour_range.clone())
                            })
                            .collect()
                    },
                ),
                sign_sampler: Bernoulli::new(0.5).unwrap(),
                seed_genes_list: builder.seed_genes_list,
            })
        }
    }
}

impl<T: Allele + Copy + Default + Zero + Into<f64> + Add<Output = T> + std::cmp::PartialOrd>
    Genotype for MultiRange<T>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    type Allele = T;
    fn genes_size(&self) -> usize {
        self.genes_size
    }
    fn random_genes_factory<R: Rng>(&self, rng: &mut R) -> Vec<Self::Allele> {
        if self.seed_genes_list.is_empty() {
            (0..self.genes_size)
                .map(|index| self.allele_samplers[index].sample(rng))
                .collect()
        } else {
            self.seed_genes_list.choose(rng).unwrap().clone()
        }
    }
    fn chromosome_factory<R: Rng>(&self, rng: &mut R) -> Chromosome<Self::Allele> {
        Chromosome::new(self.random_genes_factory(rng))
    }

    fn mutate_chromosome_random<R: Rng>(
        &self,
        chromosome: &mut Chromosome<Self::Allele>,
        rng: &mut R,
    ) {
        let index = self.gene_index_sampler.sample(rng);
        chromosome.genes[index] = self.allele_samplers[index].sample(rng);
        chromosome.taint_fitness_score();
    }
    fn mutate_chromosome_neighbour<R: Rng>(
        &self,
        chromosome: &mut Chromosome<Self::Allele>,
        scale_index: Option<usize>,
        rng: &mut R,
    ) {
        let index = self.gene_index_sampler.sample(rng);
        let allele_range = &self.allele_ranges[index];

        let value_diff = if let Some(scale_index) = scale_index {
            let working_range =
                &self.allele_neighbour_scaled_ranges.as_ref().unwrap()[scale_index][index];
            if self.sign_sampler.sample(rng) {
                *working_range.start()
            } else {
                *working_range.end()
            }
        } else {
            self.allele_neighbour_samplers.as_ref().unwrap()[index].sample(rng)
        };

        let new_value = chromosome.genes[index] + value_diff;
        if new_value < *allele_range.start() {
            chromosome.genes[index] = *allele_range.start();
        } else if new_value > *allele_range.end() {
            chromosome.genes[index] = *allele_range.end();
        } else {
            chromosome.genes[index] = new_value;
        }
        chromosome.taint_fitness_score();
    }
    fn set_seed_genes_list(&mut self, seed_genes_list: Vec<Vec<T>>) {
        self.seed_genes_list = seed_genes_list;
    }
    fn seed_genes_list(&self) -> &Vec<Vec<T>> {
        &self.seed_genes_list
    }
}

impl<T: Allele + Copy + Default + Zero + Into<f64> + Add<Output = T> + std::cmp::PartialOrd>
    IncrementalGenotype for MultiRange<T>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    fn neighbouring_chromosomes<R: Rng>(
        &self,
        chromosome: &Chromosome<Self::Allele>,
        scale_index: Option<usize>,
        rng: &mut R,
    ) -> Vec<Chromosome<Self::Allele>> {
        if let Some(scale_index) = scale_index {
            self.allele_ranges
                .iter()
                .enumerate()
                .flat_map(|(index, allele_range)| {
                    let allele_range_start = *allele_range.start();
                    let allele_range_end = *allele_range.end();
                    let working_range =
                        &self.allele_neighbour_scaled_ranges.as_ref().unwrap()[scale_index][index];
                    let working_range_start = *working_range.start();
                    let working_range_end = *working_range.end();

                    let base_value = chromosome.genes[index];
                    let value_start = if base_value + working_range_start < allele_range_start {
                        allele_range_start
                    } else {
                        base_value + working_range_start
                    };
                    let value_end = if base_value + working_range_end > allele_range_end {
                        allele_range_end
                    } else {
                        base_value + working_range_end
                    };

                    [
                        if value_start < base_value {
                            let mut genes = chromosome.genes.clone();
                            genes[index] = value_start;
                            Some(genes)
                        } else {
                            None
                        },
                        if base_value < value_end {
                            let mut genes = chromosome.genes.clone();
                            genes[index] = value_end;
                            Some(genes)
                        } else {
                            None
                        },
                    ]
                })
                .flatten()
                .dedup()
                .filter(|genes| *genes != chromosome.genes)
                .map(Chromosome::new)
                .collect::<Vec<_>>()
        } else {
            self.allele_ranges
                .iter()
                .enumerate()
                .flat_map(|(index, allele_range)| {
                    let allele_range_start = *allele_range.start();
                    let allele_range_end = *allele_range.end();
                    let working_range = &self.allele_neighbour_ranges.as_ref().unwrap()[index];
                    let working_range_start = *working_range.start();
                    let working_range_end = *working_range.end();

                    let base_value = chromosome.genes[index];
                    let range_start = if base_value + working_range_start < allele_range_start {
                        allele_range_start
                    } else {
                        base_value + working_range_start
                    };
                    let range_end = if base_value + working_range_end > allele_range_end {
                        allele_range_end
                    } else {
                        base_value + working_range_end
                    };

                    [
                        if range_start < base_value {
                            let mut genes = chromosome.genes.clone();
                            genes[index] = rng.gen_range(range_start..base_value);
                            Some(genes)
                        } else {
                            None
                        },
                        if base_value < range_end {
                            let mut genes = chromosome.genes.clone();
                            let mut new_value = rng.gen_range(base_value..=range_end);
                            // FIXME: ugly loop, goal is to have an exclusive below range
                            while new_value <= base_value {
                                new_value = rng.gen_range(base_value..=range_end);
                            }
                            genes[index] = new_value;
                            Some(genes)
                        } else {
                            None
                        },
                    ]
                })
                .flatten()
                .dedup()
                .filter(|genes| *genes != chromosome.genes)
                .map(Chromosome::new)
                .collect::<Vec<_>>()
        }
    }
    fn neighbouring_population_size(&self) -> BigUint {
        BigUint::from(2 * self.genes_size)
    }
    fn max_scale_index(&self) -> Option<usize> {
        self.allele_neighbour_scaled_ranges
            .as_ref()
            .map(|r| r.len() - 1)
    }
}

impl<T: Allele + Copy + Default + Zero + Into<f64> + Add<Output = T> + std::cmp::PartialOrd> Clone
    for MultiRange<T>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    fn clone(&self) -> Self {
        Self {
            genes_size: self.genes_size.clone(),
            allele_ranges: self.allele_ranges.clone(),
            allele_neighbour_ranges: self.allele_neighbour_ranges.clone(),
            allele_neighbour_scaled_ranges: self.allele_neighbour_scaled_ranges.clone(),
            gene_index_sampler: self.gene_index_sampler.clone(),
            allele_samplers: self
                .allele_ranges
                .iter()
                .map(|allele_range| Uniform::from(allele_range.clone()))
                .collect(),
            allele_neighbour_samplers: self.allele_neighbour_ranges.clone().map(
                |allele_neighbour_ranges| {
                    allele_neighbour_ranges
                        .iter()
                        .map(|allele_neighbour_range| Uniform::from(allele_neighbour_range.clone()))
                        .collect()
                },
            ),
            sign_sampler: Bernoulli::new(0.5).unwrap(),
            seed_genes_list: self.seed_genes_list.clone(),
        }
    }
}

impl<T: Allele + Copy + Default + Zero + Into<f64> + Add<Output = T> + std::cmp::PartialOrd>
    fmt::Debug for MultiRange<T>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Point")
            .field("genes_size", &self.genes_size)
            .field("allele_ranges", &self.allele_ranges)
            .field("allele_neighbour_ranges", &self.allele_neighbour_ranges)
            .field("gene_index_sampler", &self.gene_index_sampler)
            .field("seed_genes_list", &self.seed_genes_list)
            .finish()
    }
}

impl<T: Allele + Copy + Default + Zero + Into<f64> + Add<Output = T> + std::cmp::PartialOrd>
    fmt::Display for MultiRange<T>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "genotype:")?;
        writeln!(f, "  genes_size: {}", self.genes_size)?;
        writeln!(f, "  allele_ranges: {:?}", self.allele_ranges)?;
        writeln!(
            f,
            "  allele_neighbour_ranges: {:?}",
            self.allele_neighbour_ranges
        )?;
        writeln!(f, "  chromosome_permutations_size: uncountable")?;
        writeln!(
            f,
            "  neighbouring_population_size: {}",
            self.neighbouring_population_size()
        )?;
        writeln!(f, "  seed_genes_list: {:?}", self.seed_genes_list)
    }
}
