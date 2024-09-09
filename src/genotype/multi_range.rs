use super::builder::{Builder, TryFromBuilderError};
use super::{Allele, Genotype, IncrementalGenotype};
use crate::chromosome::{Chromosome, ChromosomeManager, LegacyChromosome, MultiRangeChromosome};
use itertools::Itertools;
use num::BigUint;
use rand::distributions::uniform::SampleUniform;
use rand::distributions::{Distribution, Uniform, WeightedIndex};
use rand::prelude::*;
use std::fmt;
use std::ops::{Add, RangeInclusive};

pub type DefaultAllele = f32;

#[derive(Copy, Clone, Debug)]
pub enum MutationType {
    Random,
    Relative,
    Scaled,
}

/// Genes are a vector of numberic values, each individually taken from its own allele_range. The
/// genes_size is derived to be the allele_ranges length. On random initialization, each gene gets
/// a value from its own allele_range with a uniform probability. Each gene has a weighted
/// probability of mutating, depending on its allele_range size. If a gene mutates, a new values is
/// taken from its own allele_range with a uniform probability. Duplicate allele values are
/// allowed.
///
/// Optionally the mutation range can be bound by relative allele_mutation_ranges or
/// allele_mutation_scaled_ranges. When allele_mutation_ranges are provided the mutation is
/// restricted to modify the existing value by a difference taken from allele_mutation_ranges with
/// a uniform probability. When allele_mutation_scaled_ranges are provided the mutation is
/// restricted to modify the existing value by a difference taken from start and end of the scaled
/// range (depending on current scale)
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
///     ]) // also default mutation range
///     .with_allele_mutation_ranges(vec![
///        -1.0..=1.0,
///        -2.0..=2.0,
///        -0.5..=0.5,
///        -3.0..=3.0
///     ]) // optional, restricts mutations to a smaller relative range
///     .with_allele_mutation_scaled_ranges(vec![
///        vec![-1.0..=1.0, -2.0..=2.0, -0.5..=0.5, -3.0..=3.0],
///        vec![-0.1..=0.1, -0.2..=0.2, -0.05..=0.05, -0.3..=0.3],
///     ]) // optional, restricts mutations to relative start/end of each scale
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
///     ]) // also default mutation range
///     .with_allele_mutation_ranges(vec![
///        -1..=1,
///        -1..=1,
///        -1..=1,
///        -2..=2,
///     ]) // optional, restricts mutations to a smaller relative range
///     .with_allele_mutation_scaled_ranges(vec![
///        vec![-1..=1, -2..=2, -1..=1, -5..=5],
///        vec![-1..=1, -1..=1, -1..=1, -1..=1],
///     ]) // optional, restricts mutations to relative start/end of each scale
///     .build()
///     .unwrap();
/// ```
pub struct MultiRange<
    T: Allele + Into<f64> + Add<Output = T> + std::cmp::PartialOrd = DefaultAllele,
> where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    pub genes_size: usize,
    pub allele_ranges: Vec<RangeInclusive<T>>,
    pub allele_mutation_ranges: Option<Vec<RangeInclusive<T>>>,
    pub allele_mutation_scaled_ranges: Option<Vec<Vec<RangeInclusive<T>>>>,
    pub mutation_type: MutationType,
    gene_index_sampler: Uniform<usize>,
    gene_weighted_index_sampler: WeightedIndex<f64>,
    allele_samplers: Vec<Uniform<T>>,
    allele_relative_samplers: Option<Vec<Uniform<T>>>,
    pub index_weights: Vec<f64>,
    pub seed_genes_list: Vec<Vec<T>>,
    pub chromosome_recycling: bool,
    pub chromosome_bin: Vec<MultiRangeChromosome<T>>,
    pub best_genes: Vec<T>,
}

impl<T: Allele + Into<f64> + Add<Output = T> + std::cmp::PartialOrd> TryFrom<Builder<Self>>
    for MultiRange<T>
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
            let mutation_type = if builder.allele_mutation_scaled_ranges.is_some() {
                MutationType::Scaled
            } else if builder.allele_mutation_ranges.is_some() {
                MutationType::Relative
            } else {
                MutationType::Random
            };

            Ok(Self {
                genes_size,
                allele_ranges: allele_ranges.clone(),
                allele_mutation_ranges: builder.allele_mutation_ranges.clone(),
                allele_mutation_scaled_ranges: builder.allele_mutation_scaled_ranges.clone(),
                mutation_type,
                gene_index_sampler: Uniform::from(0..genes_size),
                gene_weighted_index_sampler: WeightedIndex::new(index_weights.clone()).unwrap(),
                allele_samplers: allele_ranges
                    .iter()
                    .map(|allele_range| Uniform::from(allele_range.clone()))
                    .collect(),
                allele_relative_samplers: builder.allele_mutation_ranges.map(
                    |allele_mutation_ranges| {
                        allele_mutation_ranges
                            .iter()
                            .map(|allele_mutation_range| {
                                Uniform::from(allele_mutation_range.clone())
                            })
                            .collect()
                    },
                ),
                index_weights,
                seed_genes_list: builder.seed_genes_list,
                chromosome_recycling: builder.chromosome_recycling,
                chromosome_bin: vec![],
                best_genes: allele_ranges.iter().map(|a| *a.start()).collect(),
            })
        }
    }
}

impl<T: Allele + Into<f64> + Add<Output = T> + std::cmp::PartialOrd> MultiRange<T>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    fn mutate_chromosome_index_random<R: Rng>(
        &self,
        index: usize,
        chromosome: &mut MultiRangeChromosome<T>,
        rng: &mut R,
    ) {
        chromosome.genes[index] = self.allele_samplers[index].sample(rng);
    }
    fn mutate_chromosome_index_relative<R: Rng>(
        &self,
        index: usize,
        chromosome: &mut MultiRangeChromosome<T>,
        rng: &mut R,
    ) {
        let allele_range = &self.allele_ranges[index];
        let value_diff = self.allele_relative_samplers.as_ref().unwrap()[index].sample(rng);
        let new_value = chromosome.genes[index] + value_diff;
        if new_value < *allele_range.start() {
            chromosome.genes[index] = *allele_range.start();
        } else if new_value > *allele_range.end() {
            chromosome.genes[index] = *allele_range.end();
        } else {
            chromosome.genes[index] = new_value;
        }
    }
    fn mutate_chromosome_index_scaled<R: Rng>(
        &self,
        index: usize,
        chromosome: &mut MultiRangeChromosome<T>,
        scale_index: usize,
        rng: &mut R,
    ) {
        let allele_range = &self.allele_ranges[index];
        let working_range =
            &self.allele_mutation_scaled_ranges.as_ref().unwrap()[scale_index][index];
        let value_diff = if rng.gen() {
            *working_range.start()
        } else {
            *working_range.end()
        };

        let new_value = chromosome.genes[index] + value_diff;
        if new_value < *allele_range.start() {
            chromosome.genes[index] = *allele_range.start();
        } else if new_value > *allele_range.end() {
            chromosome.genes[index] = *allele_range.end();
        } else {
            chromosome.genes[index] = new_value;
        }
    }
}

impl<T: Allele + Into<f64> + Add<Output = T> + std::cmp::PartialOrd> Genotype for MultiRange<T>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    type Allele = T;
    type Genes = Vec<Self::Allele>;
    type Chromosome = MultiRangeChromosome<Self::Allele>;

    fn genes_size(&self) -> usize {
        self.genes_size
    }
    fn store_best_genes(&mut self, chromosome: &Self::Chromosome) {
        self.best_genes.clone_from(&chromosome.genes);
    }
    fn get_best_genes(&self) -> &Self::Genes {
        &self.best_genes
    }

    fn mutate_chromosome_genes<R: Rng>(
        &mut self,
        number_of_mutations: usize,
        allow_duplicates: bool,
        chromosome: &mut Self::Chromosome,
        scale_index: Option<usize>,
        rng: &mut R,
    ) {
        if allow_duplicates {
            for _ in 0..number_of_mutations {
                let index = self.gene_weighted_index_sampler.sample(rng);
                match self.mutation_type {
                    MutationType::Scaled => self.mutate_chromosome_index_scaled(
                        index,
                        chromosome,
                        scale_index.unwrap(),
                        rng,
                    ),
                    MutationType::Relative => {
                        self.mutate_chromosome_index_relative(index, chromosome, rng)
                    }
                    MutationType::Random => {
                        self.mutate_chromosome_index_random(index, chromosome, rng)
                    }
                };
            }
        } else {
            rand::seq::index::sample_weighted(
                rng,
                self.genes_size,
                |i| self.index_weights[i],
                number_of_mutations.min(self.genes_size),
            )
            .unwrap()
            .iter()
            .for_each(|index| {
                match self.mutation_type {
                    MutationType::Scaled => self.mutate_chromosome_index_scaled(
                        index,
                        chromosome,
                        scale_index.unwrap(),
                        rng,
                    ),
                    MutationType::Relative => {
                        self.mutate_chromosome_index_relative(index, chromosome, rng)
                    }
                    MutationType::Random => {
                        self.mutate_chromosome_index_random(index, chromosome, rng)
                    }
                };
            });
        }
        chromosome.taint_fitness_score();
    }

    fn crossover_chromosome_genes<R: Rng>(
        &mut self,
        number_of_crossovers: usize,
        allow_duplicates: bool,
        father: &mut Self::Chromosome,
        mother: &mut Self::Chromosome,
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
        mother.taint_fitness_score();
        father.taint_fitness_score();
    }
    fn crossover_chromosome_points<R: Rng>(
        &mut self,
        number_of_crossovers: usize,
        allow_duplicates: bool,
        father: &mut Self::Chromosome,
        mother: &mut Self::Chromosome,
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
        mother.taint_fitness_score();
        father.taint_fitness_score();
    }

    fn has_crossover_indexes(&self) -> bool {
        true
    }
    fn has_crossover_points(&self) -> bool {
        true
    }
    fn set_seed_genes_list(&mut self, seed_genes_list: Vec<Self::Genes>) {
        self.seed_genes_list = seed_genes_list;
    }
    fn seed_genes_list(&self) -> &Vec<Self::Genes> {
        &self.seed_genes_list
    }
    fn max_scale_index(&self) -> Option<usize> {
        self.allele_mutation_scaled_ranges
            .as_ref()
            .map(|r| r.len() - 1)
    }
}

impl<T: Allele + Into<f64> + Add<Output = T> + std::cmp::PartialOrd> IncrementalGenotype
    for MultiRange<T>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    fn neighbouring_chromosomes<R: Rng>(
        &self,
        chromosome: &MultiRangeChromosome<T>,
        scale_index: Option<usize>,
        rng: &mut R,
    ) -> Vec<MultiRangeChromosome<T>> {
        if let Some(scale_index) = scale_index {
            self.allele_ranges
                .iter()
                .enumerate()
                .flat_map(|(index, allele_range)| {
                    let allele_range_start = *allele_range.start();
                    let allele_range_end = *allele_range.end();
                    let working_range =
                        &self.allele_mutation_scaled_ranges.as_ref().unwrap()[scale_index][index];
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
                .map(MultiRangeChromosome::new)
                .collect::<Vec<_>>()
        } else {
            self.allele_ranges
                .iter()
                .enumerate()
                .flat_map(|(index, allele_range)| {
                    let allele_range_start = *allele_range.start();
                    let allele_range_end = *allele_range.end();
                    let working_range = &self.allele_mutation_ranges.as_ref().unwrap()[index];
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
                .map(MultiRangeChromosome::new)
                .collect::<Vec<_>>()
        }
    }
    fn neighbouring_population_size(&self) -> BigUint {
        BigUint::from(2 * self.genes_size)
    }
}

impl<T: Allele + Into<f64> + Add<Output = T> + std::cmp::PartialOrd> ChromosomeManager<Self>
    for MultiRange<T>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    fn random_genes_factory<R: Rng>(&self, rng: &mut R) -> <Self as Genotype>::Genes {
        if self.seed_genes_list.is_empty() {
            (0..self.genes_size)
                .map(|index| self.allele_samplers[index].sample(rng))
                .collect()
        } else {
            self.seed_genes_list.choose(rng).unwrap().clone()
        }
    }
    fn chromosome_constructor_empty(&self) -> MultiRangeChromosome<T> {
        MultiRangeChromosome::new(vec![])
    }
    fn chromosome_is_empty(&self, chromosome: &MultiRangeChromosome<T>) -> bool {
        chromosome.genes.is_empty()
    }
    fn chromosome_recycling(&self) -> bool {
        self.chromosome_recycling
    }
    fn chromosome_bin_push(&mut self, chromosome: MultiRangeChromosome<T>) {
        self.chromosome_bin.push(chromosome);
    }
    fn chromosome_bin_pop(&mut self) -> Option<MultiRangeChromosome<T>> {
        self.chromosome_bin.pop()
    }
}

impl<T: Allele + Into<f64> + Add<Output = T> + std::cmp::PartialOrd> Clone for MultiRange<T>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    fn clone(&self) -> Self {
        Self {
            genes_size: self.genes_size,
            allele_ranges: self.allele_ranges.clone(),
            allele_mutation_ranges: self.allele_mutation_ranges.clone(),
            allele_mutation_scaled_ranges: self.allele_mutation_scaled_ranges.clone(),
            mutation_type: self.mutation_type,
            gene_index_sampler: self.gene_index_sampler,
            gene_weighted_index_sampler: self.gene_weighted_index_sampler.clone(),
            allele_samplers: self
                .allele_ranges
                .iter()
                .map(|allele_range| Uniform::from(allele_range.clone()))
                .collect(),
            allele_relative_samplers: self.allele_mutation_ranges.clone().map(
                |allele_mutation_ranges| {
                    allele_mutation_ranges
                        .iter()
                        .map(|allele_mutation_range| Uniform::from(allele_mutation_range.clone()))
                        .collect()
                },
            ),
            index_weights: self.index_weights.clone(),
            seed_genes_list: self.seed_genes_list.clone(),
            chromosome_recycling: self.chromosome_recycling,
            chromosome_bin: vec![],
            best_genes: self.allele_ranges.iter().map(|a| *a.start()).collect(),
        }
    }
}

impl<T: Allele + Into<f64> + Add<Output = T> + std::cmp::PartialOrd> fmt::Debug for MultiRange<T>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Point")
            .field("genes_size", &self.genes_size)
            .field("allele_ranges", &self.allele_ranges)
            .field("allele_mutation_ranges", &self.allele_mutation_ranges)
            .field(
                "allele_mutation_scaled_ranges",
                &self.allele_mutation_scaled_ranges,
            )
            .field("mutation_type", &self.mutation_type)
            .field("index_weights", &self.index_weights)
            .field("seed_genes_list", &self.seed_genes_list)
            .finish()
    }
}

impl<T: Allele + Into<f64> + Add<Output = T> + std::cmp::PartialOrd> fmt::Display for MultiRange<T>
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
            "  allele_mutation_ranges: {:?}",
            self.allele_mutation_ranges
        )?;
        writeln!(
            f,
            "  allele_mutation_scaled_ranges: {:?}",
            self.allele_mutation_scaled_ranges
        )?;
        writeln!(f, "  mutation_type: {:?}", self.mutation_type)?;
        writeln!(f, "  chromosome_permutations_size: uncountable")?;
        writeln!(
            f,
            "  neighbouring_population_size: {}",
            self.neighbouring_population_size()
        )?;
        writeln!(f, "  seed_genes_list: {:?}", self.seed_genes_list)
    }
}
