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

/// Genes are a vector of numeric values, each taken from the allele_range. On random initialization,
/// each gene gets a value from the allele_range with a uniform probability. Each gene has an equal
/// probability of mutating. If a gene mutates, a new value is taken from allele_range with a
/// uniform probability.
///
/// # Mutation types
///
/// Optionally the mutation range can be bound by relative `allele_mutation_range` or
/// `allele_mutation_scaled_range`. When `allele_mutation_range` is provided the mutation is restricted
/// to modify the existing value by a difference taken from `allele_mutation_range` with a uniform
/// probability. When `allele_mutation_scaled_range` is provided the mutation is restricted to modify
/// the existing value by a difference taken from start and end of the scaled range (depending on
/// current scale)
///
/// Mutation type is defined by the most recent builder setting, so these can overwrite:
/// * `with_mutation_type` → set directly (any type)
/// * `with_allele_mutation_scaled_range` → legacy setting, scaled for all genes
/// * `with_allele_mutation_range` → legacy setting, relative for all genes
/// * no setting → default, random for all genes
///
/// # Permutation
///
/// Supports Permutation for scaled mutations only. This approach implements a increasingly localized
/// grid search with increasing precision using the `allele_mutation_scaled_range` to define the
/// search scope and grid steps
/// * First scale (index = 0) traverses the whole `allele_range` with the
///   upper bound of the first scale as step size.
/// * Other scales (index > 0) center around the best chromosome of the previous
///   scale, traversing the previous scale bounds around the best chromosome with
///   the upper bound of the current scale as step size.
/// * Scale down and repeat after grid is fully traversed
///
/// # Example (f32, default):
/// ```
/// use genetic_algorithm::genotype::{Genotype, RangeGenotype};
///
/// let genotype = RangeGenotype::builder()
///     .with_genes_size(100)
///     .with_allele_range(0.0..=1.0) // also default mutation range
///     .with_allele_mutation_range(-0.1..=0.1) // optional, restricts mutations to a smaller relative range
///     .with_allele_mutation_scaled_range(vec![-0.1..=0.1, -0.01..=0.01, -0.001..=0.001]) // optional, restricts mutations to relative start/end of each scale
///     .with_genes_hashing(true) // optional, defaults to true
///     .with_chromosome_recycling(true) // optional, defaults to true
///     .build()
///     .unwrap();
/// ```
///
/// # Example (isize):
/// ```
/// use genetic_algorithm::genotype::{Genotype, RangeGenotype};
///
/// let genotype = RangeGenotype::<isize>::builder()
///     .with_genes_size(100)
///     .with_allele_range(0..=100) // also default mutation range
///     .with_allele_mutation_range(-1..=1) // optional, restricts mutations to a smaller relative range
///     .with_allele_mutation_scaled_range(vec![-10..=10, -3..=3, -1..=1]) // optional, restricts mutations to relative start/end of each scale
///     .with_genes_hashing(true) // optional, defaults to true
///     .with_chromosome_recycling(true) // optional, defaults to true
///     .build()
///     .unwrap();
/// ```
pub struct Range<T: RangeAllele = DefaultAllele>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    pub genes_size: usize,
    pub allele_range: RangeInclusive<T>,
    pub mutation_type: MutationType<T>,
    gene_index_sampler: Uniform<usize>,
    allele_sampler: Uniform<T>,
    allele_relative_sampler: Option<Uniform<T>>,
    pub current_scale_index: usize,
    pub seed_genes_list: Vec<Vec<T>>,
    pub genes_hashing: bool,
    pub chromosome_recycling: bool,
}

impl<T: RangeAllele> TryFrom<Builder<Self>> for Range<T>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    type Error = TryFromBuilderError;

    fn try_from(builder: Builder<Self>) -> Result<Self, Self::Error> {
        if !builder.genes_size.is_some_and(|x| x > 0) {
            Err(TryFromBuilderError(
                "RangeGenotype requires a genes_size > 0",
            ))
        } else if builder.allele_range.is_none() {
            Err(TryFromBuilderError("RangeGenotype requires a allele_range"))
        } else {
            let genes_size = builder.genes_size.unwrap();
            let allele_range = builder.allele_range.unwrap();
            let mutation_type = builder.mutation_type.unwrap_or(MutationType::Random);
            let mut allele_relative_sampler = None;

            match &mutation_type {
                MutationType::Random | MutationType::Discrete | MutationType::Scaled(_) => {}
                MutationType::Relative(relative_range) => {
                    allele_relative_sampler = Some(Uniform::from(relative_range.clone()));
                }
            }

            Ok(Self {
                genes_size,
                allele_range: allele_range.clone(),
                mutation_type,
                gene_index_sampler: Uniform::from(0..genes_size),
                allele_sampler: Uniform::from(allele_range.clone()),
                allele_relative_sampler,
                current_scale_index: 0,
                seed_genes_list: builder.seed_genes_list,
                genes_hashing: builder.genes_hashing,
                chromosome_recycling: builder.chromosome_recycling,
            })
        }
    }
}

impl<T: RangeAllele> Range<T>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    fn mutation_type(&self) -> &MutationType<T> {
        &self.mutation_type
    }
    pub fn sample_allele<R: Rng>(&self, rng: &mut R) -> T {
        self.allele_sampler.sample(rng)
    }
    pub fn sample_gene_delta<R: Rng>(&self, rng: &mut R) -> T {
        match &self.mutation_type {
            MutationType::Scaled(scaled_range) => {
                let working_range = &scaled_range[self.current_scale_index];
                if rng.gen() {
                    *working_range.start()
                } else {
                    *working_range.end()
                }
            }
            MutationType::Relative(_) => self.allele_relative_sampler.as_ref().unwrap().sample(rng),
            MutationType::Random => {
                panic!("RangeGenotype has no concept of gene delta for MutationType::Random")
            }
            MutationType::Discrete => {
                panic!("RangeGenotype has no implementation for MutationType::Discrete")
            }
        }
    }
    pub fn apply_gene_delta(&self, chromosome: &mut Chromosome<T>, index: usize, delta: T) {
        let new_value = chromosome.genes[index] + delta;
        if new_value < *self.allele_range.start() {
            chromosome.genes[index] = *self.allele_range.start();
        } else if new_value > *self.allele_range.end() {
            chromosome.genes[index] = *self.allele_range.end();
        } else {
            chromosome.genes[index] = new_value;
        }
    }
}

impl<T: RangeAllele> Genotype for Range<T>
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
                match self.mutation_type {
                    MutationType::Random => {
                        chromosome.genes[index] = self.sample_allele(rng);
                    }
                    MutationType::Discrete => {
                        panic!("RangeGenotype has no implementation for MutationType::Discrete")
                    }
                    _ => {
                        let delta = self.sample_gene_delta(rng);
                        self.apply_gene_delta(chromosome, index, delta);
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
                    MutationType::Random => {
                        chromosome.genes[index] = self.sample_allele(rng);
                    }
                    MutationType::Discrete => {
                        panic!("RangeGenotype has no implementation for MutationType::Discrete")
                    }
                    _ => {
                        let delta = self.sample_gene_delta(rng);
                        self.apply_gene_delta(chromosome, index, delta);
                    }
                };
            });
        }
        chromosome.reset_metadata(self.genes_hashing);
    }
    fn set_seed_genes_list(&mut self, seed_genes_list: Vec<Genes<Self::Allele>>) {
        self.seed_genes_list = seed_genes_list;
    }
    fn seed_genes_list(&self) -> &Vec<Genes<Self::Allele>> {
        &self.seed_genes_list
    }
    fn max_scale_index(&self) -> Option<usize> {
        match &self.mutation_type {
            MutationType::Scaled(scaled_ranges) => Some(scaled_ranges.len() - 1),
            _ => None,
        }
    }
    fn current_scale_index(&self) -> Option<usize> {
        match self.mutation_type {
            MutationType::Scaled(_) => Some(self.current_scale_index),
            _ => None,
        }
    }
    fn reset_scale_index(&mut self) {
        self.current_scale_index = 0;
    }
    fn increment_scale_index(&mut self) -> bool {
        if let Some(max_scale_index) = self.max_scale_index() {
            if self.current_scale_index < max_scale_index {
                self.current_scale_index += 1;
                true
            } else {
                false
            }
        } else {
            false
        }
    }
    fn random_genes_factory<R: Rng>(&self, rng: &mut R) -> Vec<T> {
        if self.seed_genes_list.is_empty() {
            (0..self.genes_size)
                .map(|_| self.sample_allele(rng))
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
    fn chromosome_recycling(&self) -> bool {
        self.chromosome_recycling
    }
}

impl<T: RangeAllele> EvolveGenotype for Range<T>
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
impl<T: RangeAllele> HillClimbGenotype for Range<T>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    fn fill_neighbouring_population<R: Rng>(
        &self,
        chromosome: &Chromosome<Self::Allele>,
        population: &mut Population<Self::Allele>,
        rng: &mut R,
    ) {
        match &self.mutation_type {
            MutationType::Scaled(scaled_ranges) => {
                self.fill_neighbouring_population_scaled(chromosome, population, scaled_ranges)
            }
            MutationType::Relative(relative_range) => self.fill_neighbouring_population_relative(
                chromosome,
                population,
                relative_range,
                rng,
            ),
            MutationType::Random => {
                self.fill_neighbouring_population_random(chromosome, population, rng)
            }
            MutationType::Discrete => {
                panic!("RangeGenotype has no implementation for MutationType::Discrete")
            }
        }
    }

    fn neighbouring_population_size(&self) -> BigUint {
        BigUint::from(2 * self.genes_size)
    }
}

impl<T: RangeAllele> Range<T>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    fn fill_neighbouring_population_scaled(
        &self,
        chromosome: &Chromosome<T>,
        population: &mut Population<T>,
        scaled_ranges: &Vec<RangeInclusive<T>>,
    ) {
        let allele_range_start = *self.allele_range.start();
        let allele_range_end = *self.allele_range.end();

        let working_range = &scaled_ranges[self.current_scale_index];
        let working_range_start = *working_range.start();
        let working_range_end = *working_range.end();

        (0..self.genes_size).for_each(|index| {
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
                let mut new_chromosome = population.new_chromosome(chromosome);
                new_chromosome.genes[index] = value_low;
                new_chromosome.reset_metadata(self.genes_hashing);
                population.chromosomes.push(new_chromosome);
            };
            if value_high > base_value {
                let mut new_chromosome = population.new_chromosome(chromosome);
                new_chromosome.genes[index] = value_high;
                new_chromosome.reset_metadata(self.genes_hashing);
                population.chromosomes.push(new_chromosome);
            };
        });
    }

    fn fill_neighbouring_population_relative<R: Rng>(
        &self,
        chromosome: &Chromosome<T>,
        population: &mut Population<T>,
        relative_range: &RangeInclusive<T>,
        rng: &mut R,
    ) {
        let allele_range_start = *self.allele_range.start();
        let allele_range_end = *self.allele_range.end();
        let working_range_start = *relative_range.start();
        let working_range_end = *relative_range.end();

        (0..self.genes_size).for_each(|index| {
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
                let mut new_chromosome = population.new_chromosome(chromosome);
                new_chromosome.genes[index] = rng.gen_range(range_start..base_value);
                new_chromosome.reset_metadata(self.genes_hashing);
                population.chromosomes.push(new_chromosome);
            };
            if base_value < range_end {
                let mut new_chromosome = population.new_chromosome(chromosome);
                let new_value = rng.gen_range((base_value + T::smallest_increment())..=range_end);
                new_chromosome.genes[index] = new_value;
                new_chromosome.reset_metadata(self.genes_hashing);
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
        let allele_range_start = *self.allele_range.start();
        let allele_range_end = *self.allele_range.end();

        (0..self.genes_size).for_each(|index| {
            let base_value = chromosome.genes[index];
            if allele_range_start < base_value {
                let mut new_chromosome = population.new_chromosome(chromosome);
                new_chromosome.genes[index] = rng.gen_range(allele_range_start..base_value);
                new_chromosome.reset_metadata(self.genes_hashing);
                population.chromosomes.push(new_chromosome);
            };
            if base_value < allele_range_end {
                let mut new_chromosome = population.new_chromosome(chromosome);
                let new_value =
                    rng.gen_range((base_value + T::smallest_increment())..=allele_range_end);
                new_chromosome.genes[index] = new_value;
                new_chromosome.reset_metadata(self.genes_hashing);
                population.chromosomes.push(new_chromosome);
            };
        });
    }
}

impl<T: RangeAllele> PermutateGenotype for Range<T>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    fn chromosome_permutations_into_iter<'a>(
        &'a self,
        chromosome: Option<&Chromosome<Self::Allele>>,
    ) -> Box<dyn Iterator<Item = Chromosome<Self::Allele>> + Send + 'a> {
        if self.seed_genes_list.is_empty() {
            match &self.mutation_type {
                MutationType::Scaled(scaled_ranges) => Box::new(
                    self.permutable_gene_values_scaled(chromosome, scaled_ranges)
                        .into_iter()
                        .multi_cartesian_product()
                        .map(Chromosome::new),
                ),
                MutationType::Relative(_) => {
                    panic!("RangeGenotype is not permutable for MutationType::Relative")
                }
                MutationType::Random => {
                    panic!("RangeGenotype is not permutable for MutationType::Random")
                }
                MutationType::Discrete => {
                    panic!("RangeGenotype has no implementation for MutationType::Discrete")
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
            match &self.mutation_type {
                MutationType::Scaled(scaled_ranges) => (0..=self.max_scale_index().unwrap())
                    .map(|scale_index| {
                        self.chromosome_permutations_size_scaled(scale_index, scaled_ranges)
                    })
                    .sum(),
                MutationType::Relative(_) => {
                    panic!("RangeGenotype is not permutable for MutationType::Relative")
                }
                MutationType::Random => {
                    panic!("RangeGenotype is not permutable for MutationType::Random")
                }
                MutationType::Discrete => {
                    panic!("RangeGenotype has no implementation for MutationType::Discrete")
                }
            }
        } else {
            self.seed_genes_list.len().into()
        }
    }

    fn chromosome_permutations_size_report(&self) -> String {
        match &self.mutation_type {
            MutationType::Scaled(scaled_ranges) => {
                let size_per_scale: Vec<String> = (0..=self.max_scale_index().unwrap())
                    .map(|scale_index| {
                        self.chromosome_permutations_size_scaled(scale_index, scaled_ranges)
                    })
                    .map(|scale_size| self.format_biguint_scientific(&scale_size))
                    .collect();
                format!(
                    "{}, per scale {:?}",
                    self.format_biguint_scientific(&self.chromosome_permutations_size()),
                    size_per_scale
                )
            }
            _ => "uncountable".to_string(),
        }
    }

    fn allows_permutation(&self) -> bool {
        match self.mutation_type {
            MutationType::Scaled(_) => true,
            MutationType::Relative(_) => false,
            MutationType::Random => false,
            MutationType::Discrete => false, // can implement, but acts as inefficient ListGenotype
        }
    }
}

impl<T: RangeAllele> Range<T>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    // scales should be symmetrical, so the step is simply the scale end
    pub fn permutable_gene_values_scaled(
        &self,
        chromosome: Option<&Chromosome<T>>,
        scaled_ranges: &Vec<RangeInclusive<T>>,
    ) -> Vec<Vec<T>> {
        (0..self.genes_size())
            .map(|index| {
                let (allele_value_start, allele_value_end) = if let Some(chromosome) = chromosome {
                    if let Some(previous_scale_index) = self.current_scale_index.checked_sub(1) {
                        let allele_range_start = *self.allele_range.start();
                        let allele_range_end = *self.allele_range.end();

                        let working_range = &scaled_ranges[previous_scale_index];
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
                        (*self.allele_range.start(), *self.allele_range.end())
                    }
                } else {
                    (*self.allele_range.start(), *self.allele_range.end())
                };

                let working_range = &scaled_ranges[self.current_scale_index];
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

    pub fn permutable_allele_size_scaled(
        &self,
        scale_index: usize,
        scaled_ranges: &Vec<RangeInclusive<T>>,
    ) -> usize {
        let (allele_value_start, allele_value_end) =
            if let Some(previous_scale_index) = scale_index.checked_sub(1) {
                let working_range = &scaled_ranges[previous_scale_index];
                (*working_range.start(), *working_range.end())
            } else {
                (*self.allele_range.start(), *self.allele_range.end())
            };

        let working_range = &scaled_ranges[scale_index];
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
    }

    pub fn chromosome_permutations_size_scaled(
        &self,
        scale_index: usize,
        scaled_ranges: &Vec<RangeInclusive<T>>,
    ) -> BigUint {
        BigUint::from(self.permutable_allele_size_scaled(scale_index, scaled_ranges))
            .pow(self.genes_size() as u32)
    }
}

impl<T: RangeAllele> Clone for Range<T>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    fn clone(&self) -> Self {
        let allele_relative_sampler = match &self.mutation_type {
            MutationType::Relative(relative_range) => Some(Uniform::from(relative_range.clone())),
            _ => None,
        };
        Self {
            genes_size: self.genes_size,
            allele_range: self.allele_range.clone(),
            mutation_type: self.mutation_type.clone(),
            gene_index_sampler: self.gene_index_sampler,
            allele_sampler: Uniform::from(self.allele_range.clone()),
            allele_relative_sampler,
            current_scale_index: self.current_scale_index,
            seed_genes_list: self.seed_genes_list.clone(),
            genes_hashing: self.genes_hashing,
            chromosome_recycling: self.chromosome_recycling,
        }
    }
}

impl<T: RangeAllele> fmt::Debug for Range<T>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Point")
            .field("genes_size", &self.genes_size)
            .field("allele_range", &self.allele_range)
            .field("mutation_type", &self.mutation_type)
            .field("seed_genes_list", &self.seed_genes_list)
            .finish()
    }
}

impl<T: RangeAllele> fmt::Display for Range<T>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
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
        writeln!(f, "  current scale index: {:?}", self.current_scale_index)?;
        writeln!(f, "  seed_genes: {:?}", self.seed_genes_list.len())
    }
}
