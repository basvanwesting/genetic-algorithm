use super::builder::{Builder, TryFromBuilderError};
use super::{EvolveGenotype, Genotype, HillClimbGenotype, MutationType, PermutateGenotype};
use crate::allele::RangeAllele;
use crate::chromosome::{Chromosome, Genes};
use crate::population::Population;
use itertools::Itertools;
use num::BigUint;
use rand::distributions::uniform::SampleUniform;
use rand::distributions::{Distribution, Uniform};
use rand::prelude::*;
use std::fmt;
use std::ops::RangeInclusive;

pub type DefaultAllele = f32;

/// Genes are a vector of numberic values, each individually taken from its own allele_range. The
/// genes_size is derived to be the allele_ranges length. On random initialization, each gene gets
/// a value from its own allele_range with a uniform probability. Each gene has a equal
/// probability of mutating, regardless of its allele_range size. If a gene mutates, a new values is
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
///     .with_genes_hashing(true) // optional, defaults to false
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
///     .with_genes_hashing(false) // optional, defaults to false
///     .build()
///     .unwrap();
/// ```
pub struct MultiRange<T: RangeAllele + Into<f64> = DefaultAllele>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    pub genes_size: usize,
    pub allele_ranges: Vec<RangeInclusive<T>>,
    pub allele_mutation_ranges: Option<Vec<RangeInclusive<T>>>,
    pub allele_mutation_scaled_ranges: Option<Vec<Vec<RangeInclusive<T>>>>,
    pub mutation_type: MutationType,
    gene_index_sampler: Uniform<usize>,
    allele_samplers: Vec<Uniform<T>>,
    allele_relative_samplers: Option<Vec<Uniform<T>>>,
    pub seed_genes_list: Vec<Vec<T>>,
}

impl<T: RangeAllele + Into<f64>> TryFrom<Builder<Self>> for MultiRange<T>
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
                seed_genes_list: builder.seed_genes_list,
            })
        }
    }
}

impl<T: RangeAllele + Into<f64>> MultiRange<T>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    fn mutate_chromosome_index_random<R: Rng>(
        &self,
        index: usize,
        chromosome: &mut Chromosome<T>,
        rng: &mut R,
    ) {
        chromosome.genes[index] = self.allele_samplers[index].sample(rng);
    }
    fn mutate_chromosome_index_relative<R: Rng>(
        &self,
        index: usize,
        chromosome: &mut Chromosome<T>,
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
        chromosome: &mut Chromosome<T>,
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

impl<T: RangeAllele + Into<f64>> Genotype for MultiRange<T>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
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

    fn mutation_type(&self) -> MutationType {
        self.mutation_type
    }
    fn mutate_chromosome_genes<R: Rng>(
        &self,
        number_of_mutations: usize,
        allow_duplicates: bool,
        chromosome: &mut Chromosome<Self::Allele>,
        scale_index: Option<usize>,
        rng: &mut R,
    ) {
        if allow_duplicates {
            for _ in 0..number_of_mutations {
                let index = self.gene_index_sampler.sample(rng);
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
            rand::seq::index::sample(
                rng,
                self.genes_size,
                number_of_mutations.min(self.genes_size),
            )
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
        self.allele_mutation_scaled_ranges
            .as_ref()
            .map(|r| r.len() - 1)
    }
    fn random_genes_factory<R: Rng>(&self, rng: &mut R) -> Vec<T> {
        if self.seed_genes_list.is_empty() {
            (0..self.genes_size)
                .map(|index| self.allele_samplers[index].sample(rng))
                .collect()
        } else {
            self.seed_genes_list.choose(rng).unwrap().clone()
        }
    }
    fn genes_capacity(&self) -> usize {
        self.genes_size
    }
}

impl<T: RangeAllele + Into<f64>> EvolveGenotype for MultiRange<T>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
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
impl<T: RangeAllele + Into<f64>> HillClimbGenotype for MultiRange<T>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    fn fill_neighbouring_population<R: Rng>(
        &self,
        chromosome: &Chromosome<Self::Allele>,
        population: &mut Population<Self::Allele>,
        scale_index: Option<usize>,
        rng: &mut R,
    ) {
        match self.mutation_type {
            MutationType::Scaled => self.fill_neighbouring_population_scaled(
                chromosome,
                population,
                scale_index.unwrap(),
            ),
            MutationType::Relative => {
                self.fill_neighbouring_population_relative(chromosome, population, rng)
            }
            MutationType::Random => {
                self.fill_neighbouring_population_random(chromosome, population, rng)
            }
        }
    }

    fn neighbouring_population_size(&self) -> BigUint {
        BigUint::from(2 * self.genes_size)
    }
}

impl<T: RangeAllele + Into<f64>> MultiRange<T>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    fn fill_neighbouring_population_scaled(
        &self,
        chromosome: &Chromosome<T>,
        population: &mut Population<T>,
        scale_index: usize,
    ) {
        self.allele_ranges
            .clone()
            .into_iter()
            .enumerate()
            .for_each(|(index, allele_range)| {
                let allele_range_start = *allele_range.start();
                let allele_range_end = *allele_range.end();
                let working_range =
                    &self.allele_mutation_scaled_ranges.as_ref().unwrap()[scale_index][index];
                let working_range_start = *working_range.start();
                let working_range_end = *working_range.end();

                let base_value = chromosome.genes[index];
                let value_low = if base_value + working_range_start < allele_range_start {
                    allele_range_start
                } else {
                    base_value + working_range_start
                };
                let value_high = if base_value + working_range_end > allele_range_end {
                    allele_range_end
                } else {
                    base_value + working_range_end
                };

                if value_low < base_value {
                    let mut new_chromosome = chromosome.clone();
                    new_chromosome.genes[index] = value_low;
                    new_chromosome.reset_state();
                    population.chromosomes.push(new_chromosome);
                };
                if value_high > base_value {
                    let mut new_chromosome = chromosome.clone();
                    new_chromosome.genes[index] = value_high;
                    new_chromosome.reset_state();
                    population.chromosomes.push(new_chromosome);
                };
            });
    }

    fn fill_neighbouring_population_relative<R: Rng>(
        &self,
        chromosome: &Chromosome<T>,
        population: &mut Population<T>,
        rng: &mut R,
    ) {
        self.allele_ranges
            .clone()
            .into_iter()
            .enumerate()
            .for_each(|(index, allele_range)| {
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

                if range_start < base_value {
                    let mut new_chromosome = chromosome.clone();
                    new_chromosome.genes[index] = rng.gen_range(range_start..base_value);
                    new_chromosome.reset_state();
                    population.chromosomes.push(new_chromosome);
                };
                if base_value < range_end {
                    let mut new_chromosome = chromosome.clone();
                    let new_value =
                        rng.gen_range((base_value + T::smallest_increment())..=range_end);
                    new_chromosome.genes[index] = new_value;
                    new_chromosome.reset_state();
                    population.chromosomes.push(new_chromosome);
                };
            });
    }

    fn fill_neighbouring_population_random<R: Rng>(
        &self,
        chromosome: &Chromosome<T>,
        population: &mut Population<T>,
        rng: &mut R,
    ) {
        self.allele_ranges
            .clone()
            .into_iter()
            .enumerate()
            .for_each(|(index, allele_range)| {
                let allele_range_start = *allele_range.start();
                let allele_range_end = *allele_range.end();

                let base_value = chromosome.genes[index];
                if allele_range_start < base_value {
                    let mut new_chromosome = chromosome.clone();
                    new_chromosome.genes[index] = rng.gen_range(allele_range_start..base_value);
                    new_chromosome.reset_state();
                    population.chromosomes.push(new_chromosome);
                };
                if base_value < allele_range_end {
                    let mut new_chromosome = chromosome.clone();
                    let new_value =
                        rng.gen_range((base_value + T::smallest_increment())..=allele_range_end);
                    new_chromosome.genes[index] = new_value;
                    new_chromosome.reset_state();
                    population.chromosomes.push(new_chromosome);
                };
            });
    }
}

impl<T: RangeAllele + Into<f64>> PermutateGenotype for MultiRange<T>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    fn chromosome_permutations_into_iter<'a>(
        &'a self,
        chromosome: Option<&Chromosome<Self::Allele>>,
        scale_index: Option<usize>,
    ) -> Box<dyn Iterator<Item = Chromosome<Self::Allele>> + Send + 'a> {
        if self.seed_genes_list.is_empty() {
            match self.mutation_type {
                MutationType::Scaled => Box::new(
                    self.permutable_gene_values_scaled(chromosome, scale_index.unwrap())
                        .into_iter()
                        .multi_cartesian_product()
                        .map(Chromosome::new),
                ),
                MutationType::Relative => {
                    panic!("RangeGenotype is not permutable for MutationType::Relative")
                }
                MutationType::Random => {
                    panic!("RangeGenotype is not permutable for MutationType::Random")
                }
            }
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
            match self.mutation_type {
                MutationType::Scaled => (0..=self.max_scale_index().unwrap())
                    .map(|scale_index| self.chromosome_permutations_size_scaled(scale_index))
                    .sum(),
                MutationType::Relative => {
                    panic!("RangeGenotype is not permutable for MutationType::Relative")
                }
                MutationType::Random => {
                    panic!("RangeGenotype is not permutable for MutationType::Random")
                }
            }
        } else {
            self.seed_genes_list.len().into()
        }
    }
    fn mutation_type_allows_permutation(&self) -> bool {
        match self.mutation_type {
            MutationType::Scaled => true,
            MutationType::Relative => false,
            MutationType::Random => false,
        }
    }
}

impl<T: RangeAllele + Into<f64>> MultiRange<T>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    // scales should be symmetrical, so the step is simply the scale end
    pub fn permutable_gene_values_scaled(
        &self,
        chromosome: Option<&Chromosome<T>>,
        scale_index: usize,
    ) -> Vec<Vec<T>> {
        self.allele_ranges
            .clone()
            .into_iter()
            .enumerate()
            .map(|(index, allele_range)| {
                let allele_range_start = *allele_range.start();
                let allele_range_end = *allele_range.end();

                let (allele_value_start, allele_value_end) = if let Some(chromosome) = chromosome {
                    if let Some(previous_scale_index) = scale_index.checked_sub(1) {
                        let working_range = &self.allele_mutation_scaled_ranges.as_ref().unwrap()
                            [previous_scale_index][index];

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

                        (value_start, value_end)
                    } else {
                        (allele_range_start, allele_range_end)
                    }
                } else {
                    (allele_range_start, allele_range_end)
                };

                let working_range =
                    &self.allele_mutation_scaled_ranges.as_ref().unwrap()[scale_index][index];
                let working_range_step = *working_range.end();

                std::iter::successors(Some(allele_value_start), |value| {
                    if *value < allele_value_end {
                        let next_value = *value + working_range_step;
                        if next_value > allele_value_end {
                            Some(allele_value_end)
                        } else {
                            Some(next_value)
                        }
                    } else {
                        None
                    }
                })
                .collect()
            })
            .collect()
    }

    pub fn permutable_allele_sizes_scaled(&self, scale_index: usize) -> Vec<usize> {
        self.allele_ranges
            .clone()
            .into_iter()
            .enumerate()
            .map(|(index, allele_range)| {
                let (allele_value_start, allele_value_end) =
                    if let Some(previous_scale_index) = scale_index.checked_sub(1) {
                        let working_range = &self.allele_mutation_scaled_ranges.as_ref().unwrap()
                            [previous_scale_index][index];

                        (*working_range.start(), *working_range.end())
                    } else {
                        (*allele_range.start(), *allele_range.end())
                    };

                let working_range =
                    &self.allele_mutation_scaled_ranges.as_ref().unwrap()[scale_index][index];
                let working_range_step = *working_range.end();

                std::iter::successors(Some(allele_value_start), |value| {
                    if *value < allele_value_end {
                        let next_value = *value + working_range_step;
                        if next_value > allele_value_end {
                            Some(allele_value_end)
                        } else {
                            Some(next_value)
                        }
                    } else {
                        None
                    }
                })
                .count()
            })
            .collect()
    }

    pub fn chromosome_permutations_size_scaled(&self, scale_index: usize) -> BigUint {
        self.permutable_allele_sizes_scaled(scale_index)
            .iter()
            .map(|v| BigUint::from(*v))
            .product()
    }
}

impl<T: RangeAllele + Into<f64>> Clone for MultiRange<T>
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
            seed_genes_list: self.seed_genes_list.clone(),
        }
    }
}

impl<T: RangeAllele + Into<f64>> fmt::Debug for MultiRange<T>
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
            .field("seed_genes_list", &self.seed_genes_list)
            .finish()
    }
}

impl<T: RangeAllele + Into<f64>> fmt::Display for MultiRange<T>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "genotype:")?;
        writeln!(f, "  genes_size: {}", self.genes_size)?;
        writeln!(f, "  mutation_type: {:?}", self.mutation_type)?;

        if self.mutation_type_allows_permutation() {
            let size_per_scale: Vec<BigUint> = (0..=self.max_scale_index().unwrap())
                .map(|scale_index| self.chromosome_permutations_size_scaled(scale_index))
                .collect();
            writeln!(
                f,
                "  chromosome_permutations_size: {}, per scale {:?}",
                self.chromosome_permutations_size(),
                size_per_scale
            )?;
        } else {
            writeln!(f, "  chromosome_permutations_size: uncountable")?;
        }

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
