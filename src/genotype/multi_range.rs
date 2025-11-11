use super::builder::{Builder, TryFromBuilderError};
use super::{EvolveGenotype, Genotype, HillClimbGenotype, MutationType, PermutateGenotype};
use crate::allele::RangeAllele;
use crate::chromosome::{Chromosome, Genes};
use crate::population::Population;
use itertools::Itertools;
use num::BigUint;
use rand::distributions::{Distribution, Uniform};
use rand::prelude::*;
use std::fmt;
use std::ops::RangeInclusive;

pub type DefaultAllele = f32;

/// Genes are a vector of numberic values, each individually taken from its own allele_range. The
/// genes_size is derived to be the allele_ranges length. On random initialization, each gene gets
/// a value from its own allele_range with a uniform probability. Each gene has a equal probability
/// of mutating, regardless of its allele_range size. If a gene mutates, a new values is taken from
/// its own allele_range with a uniform probability. Duplicate allele values are allowed. Supports
/// heterogeneous chromosomes that mix different gene semantics (continuous values, discrete
/// choices, booleans) within a single numeric type `T`.
///
/// # Mutation types
/// See [MutationType]
///
/// # Permutation
///
/// Supports Permutation for scaled and discrete mutations only. This approach implements a
/// increasingly localized grid search with increasing precision using the
/// [MutationType::StepScaled]
/// to define the search scope and grid steps
/// * First scale (index = 0) traverses the whole `allele_ranges` with the
///   upper bound of the first scale as step size.
/// * Other scales (index > 0) center around the best chromosome of the previous
///   scale, traversing the previous scale bounds around the best chromosome with
///   the upper bound of the current scale as step size.
/// * Scale down and repeat after grid is fully traversed
///
/// The discrete mutations traverse all allowed values for every scale (see
/// [MutationType::Discrete])
///
/// ** Note: ** When all parameters are discrete, prefer
/// [MultiListGenotype](crate::genotype::MultiListGenotype) as this is more optimized and also
/// balance the mutation probablity per allowed value, not per gene.
///
/// # Heterogeneous Genotype Support
///
/// MultiRangeGenotype supports heterogeneous chromosomes that mix different gene semantics
/// (continuous values, discrete choices, booleans) within a single numeric type `T`.
///
/// Use `.with_mutation_types(vec![...])` to specify behavior for each gene individually:
///
/// * `MutationType::Random` - Samples uniformly from the full allele range
/// * `MutationType::Range` - Mutates within a relative range around the current value
/// * `MutationType::StepScaled` - Progressive refinement through multiple scale levels around the current value
/// * `MutationType::Discrete` - Rounded-to-integer values with uniform selection (like `ListGenotype`).
///   * Mutations ignore current value - all rounded-to-integer in range equally likely
///   * Range `0.0..=4.0` yields values: 0.0, 1.0, 2.0, 3.0, 4.0 (with equal probability)
///   * Useful for encoding: enums (0.0..=4.0), booleans (0.0..=1.0), or discrete choices
///   * Neighbours and permutations include all integer values in the allele range
///
/// # Example (f32, default, random mutation):
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
///     .with_genes_hashing(true) // optional, defaults to true
///     .with_chromosome_recycling(true) // optional, defaults to true
///     .build()
///     .unwrap();
/// ```
///
/// # Example (isize, relative range mutation):
/// ```
/// use genetic_algorithm::genotype::{Genotype, MultiRangeGenotype, MutationType};
///
/// let genotype = MultiRangeGenotype::builder()
///     .with_allele_ranges(vec![
///        0..=10,
///        5..=20,
///        -5..=5,
///        10..=30,
///     ])
///     .with_mutation_types(vec![
///        MutationType::Range(1),
///        MutationType::Range(2),
///        MutationType::Range(1),
///        MutationType::Range(3),
///     ]) // optional, restricts mutations to a smaller relative range
///     .with_genes_hashing(true) // optional, defaults to true
///     .with_chromosome_recycling(true) // optional, defaults to true
///     .build()
///     .unwrap();
/// ```
///
/// # Example (f32, scaled mutation):
/// ```
/// use genetic_algorithm::genotype::{Genotype, MultiRangeGenotype, MutationType};
///
/// let genotype = MultiRangeGenotype::builder()
///     .with_allele_ranges(vec![
///        0.0..=10.0,
///        5.0..=20.0,
///        0.0..=5.0,
///        10.0..=30.0
///     ])
///     .with_mutation_types(vec![
///        MutationType::StepScaled(vec![1.0,0.1]),
///        MutationType::StepScaled(vec![2.0,0.2]),
///        MutationType::StepScaled(vec![0.5,0.05]),
///        MutationType::StepScaled(vec![3.0,0.3]),
///     ]) // optional, restricts mutations to relative step up or down of each scale
///     .with_genes_hashing(true) // optional, defaults to true
///     .with_chromosome_recycling(true) // optional, defaults to true
///     .build()
///     .unwrap();
/// ```
///
/// # Example (f32, heterogeneous mutation):
/// ```
/// use genetic_algorithm::genotype::{Genotype, MultiRangeGenotype, MutationType};
///
/// let genotype = MultiRangeGenotype::builder()
///     .with_allele_ranges(vec![
///         0.0..=1.0,    // Gene 0: Boolean flag
///         0.0..=4.0,    // Gene 1: Algorithm choice (5 options)
///         0.0..=100.0,  // Gene 2: Speed percentage
///     ])
///     .with_mutation_types(vec![
///         MutationType::Discrete,  // Boolean as 0 or 1
///         MutationType::Discrete,  // One of 5 algorithms
///         MutationType::StepScaled(vec![10.0, 1.0, 0.1]), // Continuous refinement
///     ])
///     .with_genes_hashing(true) // optional, defaults to true
///     .with_chromosome_recycling(true) // optional, defaults to true
///     .build();
/// ```
pub struct MultiRange<T: RangeAllele = DefaultAllele>
where
    Uniform<T>: Send + Sync,
{
    pub genes_size: usize,
    pub allele_ranges: Vec<RangeInclusive<T>>,
    pub mutation_types: Vec<MutationType<T>>,
    gene_index_sampler: Uniform<usize>,
    allele_samplers: Vec<Uniform<T>>,
    // post-clamped sampler, always positive to support unsigned
    allele_bandwidth_samplers: Vec<Option<Uniform<T>>>,
    pub current_scale_index: usize,
    pub seed_genes_list: Vec<Vec<T>>,
    pub genes_hashing: bool,
    pub chromosome_recycling: bool,
}

impl<T: RangeAllele> TryFrom<Builder<Self>> for MultiRange<T>
where
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
            let mutation_types = builder
                .mutation_types
                .unwrap_or(vec![MutationType::Random; genes_size]);
            let allele_samplers = allele_ranges
                .iter()
                .zip(&mutation_types)
                .map(|(allele_range, mutation_type)| {
                    match mutation_type {
                        MutationType::Discrete => {
                            // [start, end+1) for uniform floor() sampling
                            Uniform::new(*allele_range.start(), *allele_range.end() + T::one())
                        }
                        _ => {
                            // [start, end] for uniform sampling
                            Uniform::from(allele_range.clone())
                        }
                    }
                })
                .collect();
            let allele_bandwidth_samplers = mutation_types
                .iter()
                .map(|mutation_type| match &mutation_type {
                    MutationType::Range(bandwidth) => {
                        if *bandwidth >= T::smallest_increment() {
                            Some(Uniform::new_inclusive(T::smallest_increment(), bandwidth))
                        } else {
                            None
                        }
                    }
                    MutationType::RangeScaled(bandwidths) => {
                        let bandwidth = bandwidths.last().unwrap();
                        if *bandwidth >= T::smallest_increment() {
                            Some(Uniform::new_inclusive(T::smallest_increment(), bandwidth))
                        } else {
                            None
                        }
                    }
                    _ => None,
                })
                .collect();

            Ok(Self {
                genes_size,
                allele_ranges: allele_ranges.clone(),
                mutation_types: mutation_types.clone(),
                gene_index_sampler: Uniform::from(0..genes_size),
                allele_samplers,
                allele_bandwidth_samplers,
                current_scale_index: 0,
                seed_genes_list: builder.seed_genes_list,
                genes_hashing: builder.genes_hashing,
                chromosome_recycling: builder.chromosome_recycling,
            })
        }
    }
}

impl<T: RangeAllele> MultiRange<T>
where
    Uniform<T>: Send + Sync,
{
    fn mutation_types(&self) -> &[MutationType<T>] {
        &self.mutation_types
    }
    pub fn sample_gene_random<R: Rng>(&self, index: usize, rng: &mut R) -> T {
        match self.mutation_types[index] {
            MutationType::Discrete => self.allele_samplers[index].sample(rng).floor(),
            _ => self.allele_samplers[index].sample(rng),
        }
    }
    // all delta's are positive, because we support unsigned integers as RangeAllele
    // quite the overhead to make this work, but I think it is worth it
    pub fn mutate_gene<R: Rng>(&self, chromosome: &mut Chromosome<T>, index: usize, rng: &mut R) {
        match &self.mutation_types[index] {
            MutationType::Random => {
                chromosome.genes[index] = self.allele_samplers[index].sample(rng);
            }
            MutationType::Discrete => {
                chromosome.genes[index] = self.allele_samplers[index].sample(rng).floor();
            }
            MutationType::Range(_) => {
                // post-clamp
                let current_value = chromosome.genes[index];
                if let Some(sampler) = self.allele_bandwidth_samplers[index].as_ref() {
                    let delta = sampler.sample(rng);
                    if rng.gen() {
                        chromosome.genes[index] =
                            T::clamped_add(current_value, delta, *self.allele_ranges[index].end());
                    } else {
                        chromosome.genes[index] = T::clamped_sub(
                            current_value,
                            delta,
                            *self.allele_ranges[index].start(),
                        );
                    }
                }
            }
            MutationType::RangeScaled(bandwidths) => {
                if self.current_scale_index >= bandwidths.len().saturating_sub(1) {
                    // post-clamp
                    let current_value = chromosome.genes[index];
                    if let Some(sampler) = self.allele_bandwidth_samplers[index].as_ref() {
                        let delta = sampler.sample(rng);
                        if rng.gen() {
                            chromosome.genes[index] = T::clamped_add(
                                current_value,
                                delta,
                                *self.allele_ranges[index].end(),
                            );
                        } else {
                            chromosome.genes[index] = T::clamped_sub(
                                current_value,
                                delta,
                                *self.allele_ranges[index].start(),
                            );
                        }
                    }
                } else {
                    // pre-clamp
                    let bandwidth = bandwidths[self.current_scale_index];
                    let allele_range_start = *self.allele_ranges[index].start();
                    let allele_range_end = *self.allele_ranges[index].end();
                    if allele_range_end - allele_range_start <= bandwidth {
                        // Random, leverage existing sampler
                        chromosome.genes[index] = self.allele_samplers[index].sample(rng);
                    } else {
                        // Bandwidth
                        let current_value = chromosome.genes[index];
                        if rng.gen() {
                            let max_delta_up = allele_range_end - current_value;
                            let working_delta_up = T::min(bandwidth, max_delta_up);
                            if working_delta_up >= T::smallest_increment() {
                                let delta =
                                    rng.gen_range(T::smallest_increment()..=working_delta_up);
                                chromosome.genes[index] += delta; // no need to check again
                            }
                        } else {
                            let max_delta_down = current_value - allele_range_start;
                            let working_delta_down = T::min(bandwidth, max_delta_down);
                            if working_delta_down >= T::smallest_increment() {
                                let delta =
                                    rng.gen_range(T::smallest_increment()..=working_delta_down);
                                chromosome.genes[index] -= delta; // no need to check again
                            }
                        }
                    }
                }
            }
            MutationType::Step(step) => {
                // post-clamp
                let current_value = chromosome.genes[index];
                if rng.gen() {
                    chromosome.genes[index] =
                        T::clamped_add(current_value, *step, *self.allele_ranges[index].end());
                } else {
                    chromosome.genes[index] =
                        T::clamped_sub(current_value, *step, *self.allele_ranges[index].start());
                }
            }
            MutationType::StepScaled(steps) => {
                // post-clamp
                let current_value = chromosome.genes[index];
                let delta = steps[self.current_scale_index];
                if rng.gen() {
                    chromosome.genes[index] =
                        T::clamped_add(current_value, delta, *self.allele_ranges[index].end());
                } else {
                    chromosome.genes[index] =
                        T::clamped_sub(current_value, delta, *self.allele_ranges[index].start());
                }
            }
        }
    }
}

impl<T: RangeAllele> Genotype for MultiRange<T>
where
    Uniform<T>: Send + Sync,
{
    type Allele = T;

    fn genes_size(&self) -> usize {
        self.genes_size
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
                self.mutate_gene(chromosome, index, rng);
            }
        } else {
            rand::seq::index::sample(
                rng,
                self.genes_size,
                number_of_mutations.min(self.genes_size),
            )
            .iter()
            .for_each(|index| {
                self.mutate_gene(chromosome, index, rng);
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
        self.mutation_types
            .iter()
            .find_map(|mutation_type| match mutation_type {
                MutationType::RangeScaled(scales) | MutationType::StepScaled(scales) => {
                    Some(scales.len().saturating_sub(1))
                }
                _ => None,
            })
    }
    fn current_scale_index(&self) -> Option<usize> {
        self.mutation_types
            .iter()
            .find_map(|mutation_type| match mutation_type {
                MutationType::RangeScaled(_) | MutationType::StepScaled(_) => {
                    Some(self.current_scale_index)
                }
                _ => None,
            })
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
                .map(|index| self.sample_gene_random(index, rng))
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

impl<T: RangeAllele> EvolveGenotype for MultiRange<T>
where
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
impl<T: RangeAllele> HillClimbGenotype for MultiRange<T>
where
    Uniform<T>: Send + Sync,
{
    fn fill_neighbouring_population<R: Rng>(
        &self,
        chromosome: &Chromosome<Self::Allele>,
        population: &mut Population<Self::Allele>,
        rng: &mut R,
    ) {
        self.mutation_types.iter().enumerate().for_each(
            |(index, mutation_type)| match mutation_type {
                MutationType::Random => {
                    self.fill_neighbouring_population_random(index, chromosome, population, rng)
                }
                MutationType::Step(step) => {
                    self.fill_neighbouring_population_step(index, chromosome, population, *step)
                }
                MutationType::StepScaled(steps) => {
                    let step = steps[self.current_scale_index];
                    self.fill_neighbouring_population_step(index, chromosome, population, step)
                }
                MutationType::Range(_) => {
                    // post-clamp
                    self.fill_neighbouring_population_range_post_clamp(
                        index, chromosome, population, rng,
                    )
                }
                MutationType::RangeScaled(bandwidths) => {
                    if self.current_scale_index >= bandwidths.len().saturating_sub(1) {
                        // final scale, post-clamp
                        self.fill_neighbouring_population_range_post_clamp(
                            index, chromosome, population, rng,
                        )
                    } else {
                        // pre-clamp, no need for leveraging random implementation as it is basically the same
                        let bandwidth = bandwidths[self.current_scale_index];
                        self.fill_neighbouring_population_range_pre_clamp(
                            index, chromosome, population, bandwidth, rng,
                        )
                    }
                }
                MutationType::Discrete => {
                    self.fill_neighbouring_population_discrete(index, chromosome, population)
                }
            },
        );
    }

    fn neighbouring_population_size(&self) -> BigUint {
        BigUint::from(2 * self.genes_size)
    }
}

impl<T: RangeAllele> MultiRange<T>
where
    Uniform<T>: Send + Sync,
{
    fn fill_neighbouring_population_step(
        &self,
        index: usize,
        chromosome: &Chromosome<T>,
        population: &mut Population<T>,
        step: T,
    ) {
        let allele_range_start = *self.allele_ranges[index].start();
        let allele_range_end = *self.allele_ranges[index].end();

        let current_value = chromosome.genes[index];
        if allele_range_start < current_value {
            let mut new_chromosome = population.new_chromosome(chromosome);
            new_chromosome.genes[index] = T::clamped_sub(current_value, step, allele_range_start);
            new_chromosome.reset_metadata(self.genes_hashing);
            population.chromosomes.push(new_chromosome);
        };
        if current_value < allele_range_end {
            let mut new_chromosome = population.new_chromosome(chromosome);
            new_chromosome.genes[index] = T::clamped_add(current_value, step, allele_range_end);
            new_chromosome.reset_metadata(self.genes_hashing);
            population.chromosomes.push(new_chromosome);
        };
    }
    fn fill_neighbouring_population_range_post_clamp<R: Rng>(
        &self,
        index: usize,
        chromosome: &Chromosome<T>,
        population: &mut Population<T>,
        rng: &mut R,
    ) {
        if let Some(sampler) = self.allele_bandwidth_samplers[index].as_ref() {
            let delta = sampler.sample(rng);
            let allele_range_start = *self.allele_ranges[index].start();
            let allele_range_end = *self.allele_ranges[index].end();
            let current_value = chromosome.genes[index];
            if allele_range_start < current_value {
                let mut new_chromosome = population.new_chromosome(chromosome);
                new_chromosome.genes[index] =
                    T::clamped_sub(current_value, delta, allele_range_start);
                new_chromosome.reset_metadata(self.genes_hashing);
                population.chromosomes.push(new_chromosome);
            };
            if current_value < allele_range_end {
                let mut new_chromosome = population.new_chromosome(chromosome);
                new_chromosome.genes[index] =
                    T::clamped_add(current_value, delta, allele_range_end);
                new_chromosome.reset_metadata(self.genes_hashing);
                population.chromosomes.push(new_chromosome);
            };
        }
    }

    fn fill_neighbouring_population_range_pre_clamp<R: Rng>(
        &self,
        index: usize,
        chromosome: &Chromosome<T>,
        population: &mut Population<T>,
        bandwidth: T,
        rng: &mut R,
    ) {
        let allele_range_start = *self.allele_ranges[index].start();
        let allele_range_end = *self.allele_ranges[index].end();

        let current_value = chromosome.genes[index];
        if allele_range_start < current_value {
            let mut new_chromosome = population.new_chromosome(chromosome);
            let max_delta_down = current_value - allele_range_start;
            let working_delta_down = T::min(bandwidth, max_delta_down);
            if working_delta_down >= T::smallest_increment() {
                let delta = rng.gen_range(T::smallest_increment()..=working_delta_down);
                new_chromosome.genes[index] -= delta; // no need to check again
                new_chromosome.reset_metadata(self.genes_hashing);
                population.chromosomes.push(new_chromosome);
            }
        };
        if current_value < allele_range_end {
            let mut new_chromosome = population.new_chromosome(chromosome);
            let max_delta_up = allele_range_end - current_value;
            let working_delta_up = T::min(bandwidth, max_delta_up);
            if working_delta_up >= T::smallest_increment() {
                let delta = rng.gen_range(T::smallest_increment()..=working_delta_up);
                new_chromosome.genes[index] += delta; // no need to check again
                new_chromosome.reset_metadata(self.genes_hashing);
                population.chromosomes.push(new_chromosome);
            }
        };
    }
    fn fill_neighbouring_population_random<R: Rng>(
        &self,
        index: usize,
        chromosome: &Chromosome<T>,
        population: &mut Population<T>,
        rng: &mut R,
    ) {
        let allele_range_start = *self.allele_ranges[index].start();
        let allele_range_end = *self.allele_ranges[index].end();

        let current_value = chromosome.genes[index];
        if allele_range_start < current_value {
            let mut new_chromosome = population.new_chromosome(chromosome);
            new_chromosome.genes[index] = rng.gen_range(allele_range_start..current_value);
            new_chromosome.reset_metadata(self.genes_hashing);
            population.chromosomes.push(new_chromosome);
        };
        if current_value < allele_range_end {
            let mut new_chromosome = population.new_chromosome(chromosome);
            let new_value =
                rng.gen_range((current_value + T::smallest_increment())..=allele_range_end);
            new_chromosome.genes[index] = new_value;
            new_chromosome.reset_metadata(self.genes_hashing);
            population.chromosomes.push(new_chromosome);
        };
    }

    fn fill_neighbouring_population_discrete(
        &self,
        index: usize,
        chromosome: &Chromosome<T>,
        population: &mut Population<T>,
    ) {
        let mut working_value = self.allele_ranges[index].start().floor();
        let ending_value = self.allele_ranges[index].end().floor();
        let current_value = chromosome.genes[index].floor();

        while working_value <= ending_value {
            if working_value != current_value {
                let mut new_chromosome = population.new_chromosome(chromosome);
                new_chromosome.genes[index] = working_value;
                new_chromosome.reset_metadata(self.genes_hashing);
                population.chromosomes.push(new_chromosome);
            }
            working_value += T::one();
        }
    }
}

impl<T: RangeAllele> PermutateGenotype for MultiRange<T>
where
    Uniform<T>: Send + Sync,
{
    fn chromosome_permutations_into_iter<'a>(
        &'a self,
        chromosome: Option<&Chromosome<Self::Allele>>,
    ) -> Box<dyn Iterator<Item = Chromosome<Self::Allele>> + Send + 'a> {
        if self.seed_genes_list.is_empty() {
            Box::new(
                self.mutation_types
                    .iter()
                    .enumerate()
                    .map(|(index, mutation_type)| match mutation_type {
                        MutationType::Step(step) => self.permutable_gene_values_step(index, *step),
                        MutationType::StepScaled(steps) => {
                            self.permutable_gene_values_step_scaled(index, chromosome, steps)
                        }
                        MutationType::Discrete => {
                            self.permutable_gene_values_discrete(index, chromosome)
                        }
                        _ => {
                            panic!(
                                "MultiRangeGenotype is not permutable for {:?}",
                                mutation_type
                            )
                        }
                    })
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
        self.chromosome_permutations_size_per_scale().iter().sum()
    }

    fn chromosome_permutations_size_report(&self) -> String {
        if self.allows_permutation() {
            let sizes = self.chromosome_permutations_size_per_scale();
            let total = sizes.iter().sum();
            if sizes.len() > 1 {
                format!(
                    "{}, per scale {:?}",
                    self.format_biguint_scientific(&total),
                    sizes
                )
            } else {
                self.format_biguint_scientific(&total).to_string()
            }
        } else {
            "uncountable".to_string()
        }
    }
    fn allows_permutation(&self) -> bool {
        self.mutation_types.iter().all(|mutation_type| {
            matches!(
                mutation_type,
                MutationType::Step(_) | MutationType::StepScaled(_) | MutationType::Discrete
            )
        })
    }
}

impl<T: RangeAllele> MultiRange<T>
where
    Uniform<T>: Send + Sync,
{
    pub fn permutable_gene_values_step(&self, index: usize, step: T) -> Vec<T> {
        let allele_range_start = *self.allele_ranges[index].start();
        let allele_range_end = *self.allele_ranges[index].end();
        std::iter::successors(Some(allele_range_start), |value| {
            if *value < allele_range_end {
                let next_value = *value + step;
                if next_value > allele_range_end {
                    Some(allele_range_end)
                } else {
                    Some(next_value)
                }
            } else {
                None
            }
        })
        .collect()
    }
    pub fn permutable_gene_values_step_scaled(
        &self,
        index: usize,
        chromosome: Option<&Chromosome<T>>,
        steps: &[T],
    ) -> Vec<T> {
        let allele_range_start = *self.allele_ranges[index].start();
        let allele_range_end = *self.allele_ranges[index].end();
        let (allele_value_start, allele_value_end) = if let Some(chromosome) = chromosome {
            if let Some(previous_scale_index) = self.current_scale_index.checked_sub(1) {
                let working_step = steps[previous_scale_index];
                let current_value = chromosome.genes[index];
                let value_start = T::clamped_sub(current_value, working_step, allele_range_start);
                let value_end = T::clamped_add(current_value, working_step, allele_range_end);
                (value_start, value_end)
            } else {
                (allele_range_start, allele_range_end)
            }
        } else {
            (allele_range_start, allele_range_end)
        };

        let working_step = steps[self.current_scale_index];
        std::iter::successors(Some(allele_value_start), |value| {
            if *value < allele_value_end {
                let next_value = *value + working_step;
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
    }

    pub fn permutable_gene_values_discrete(
        &self,
        index: usize,
        _chromosome: Option<&Chromosome<T>>,
    ) -> Vec<T> {
        let allele_value_start = self.allele_ranges[index].start().floor();
        let allele_value_end = self.allele_ranges[index].end().floor();
        std::iter::successors(Some(allele_value_start), |value| {
            if *value < allele_value_end {
                let next_value = *value + T::one();
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
    }

    pub fn chromosome_permutations_size_per_scale(&self) -> Vec<BigUint> {
        // first scale is affected by seed_genes_list
        let mut results = vec![];
        if self.seed_genes_list.is_empty() {
            results.push(self.chromosome_permutations_size_for_scale_index(0));
        } else {
            results.push(self.seed_genes_list.len().into());
        };
        // next scales are not
        if let Some(max_scale_index) = self.max_scale_index() {
            (1..=max_scale_index).for_each(|scale_index| {
                results.push(self.chromosome_permutations_size_for_scale_index(scale_index));
            })
        }
        results
    }

    pub fn chromosome_permutations_size_for_scale_index(&self, scale_index: usize) -> BigUint {
        self.mutation_types
            .iter()
            .enumerate()
            .map(|(index, mutation_type)| match mutation_type {
                MutationType::Step(step) => {
                    let allele_value_start = *self.allele_ranges[index].start();
                    let allele_value_end = *self.allele_ranges[index].end();
                    std::iter::successors(Some(allele_value_start), |value| {
                        if *value < allele_value_end {
                            let next_value = *value + *step;
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
                MutationType::StepScaled(steps) => {
                    let (allele_value_start, allele_value_end) =
                        if let Some(previous_scale_index) = scale_index.checked_sub(1) {
                            let working_step = steps[previous_scale_index];
                            (T::zero(), working_step + working_step)
                        } else {
                            (
                                *self.allele_ranges[index].start(),
                                *self.allele_ranges[index].end(),
                            )
                        };

                    let working_step = steps[scale_index];
                    std::iter::successors(Some(allele_value_start), |value| {
                        if *value < allele_value_end {
                            let next_value = *value + working_step;
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
                MutationType::Discrete => {
                    let allele_value_start = self.allele_ranges[index].start().floor();
                    let allele_value_end = self.allele_ranges[index].end().floor();

                    std::iter::successors(Some(allele_value_start), |value| {
                        if *value < allele_value_end {
                            let next_value = *value + T::one();
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
                _ => {
                    panic!(
                        "MultiRangeGenotype is not permutable for {:?}",
                        mutation_type
                    )
                }
            })
            .map(BigUint::from)
            .product()
    }
}

impl<T: RangeAllele> Clone for MultiRange<T>
where
    Uniform<T>: Send + Sync,
{
    fn clone(&self) -> Self {
        let allele_samplers = self
            .allele_ranges
            .iter()
            .zip(&self.mutation_types)
            .map(|(allele_range, mutation_type)| {
                match mutation_type {
                    MutationType::Discrete => {
                        // [start, end+1) for uniform floor() sampling
                        Uniform::new(*allele_range.start(), *allele_range.end() + T::one())
                    }
                    _ => {
                        // [start, end] for uniform sampling
                        Uniform::from(allele_range.clone())
                    }
                }
            })
            .collect();
        let allele_bandwidth_samplers = self
            .mutation_types
            .iter()
            .map(|mutation_type| match &mutation_type {
                MutationType::Range(bandwidth) => {
                    if *bandwidth >= T::smallest_increment() {
                        Some(Uniform::new_inclusive(T::smallest_increment(), bandwidth))
                    } else {
                        None
                    }
                }
                MutationType::RangeScaled(bandwidths) => {
                    let bandwidth = bandwidths.last().unwrap();
                    if *bandwidth >= T::smallest_increment() {
                        Some(Uniform::new_inclusive(T::smallest_increment(), bandwidth))
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .collect();

        Self {
            genes_size: self.genes_size,
            allele_ranges: self.allele_ranges.clone(),
            mutation_types: self.mutation_types.clone(),
            gene_index_sampler: self.gene_index_sampler,
            allele_samplers,
            allele_bandwidth_samplers,
            current_scale_index: self.current_scale_index,
            seed_genes_list: self.seed_genes_list.clone(),
            genes_hashing: self.genes_hashing,
            chromosome_recycling: self.chromosome_recycling,
        }
    }
}

impl<T: RangeAllele> fmt::Debug for MultiRange<T>
where
    Uniform<T>: Send + Sync,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Point")
            .field("genes_size", &self.genes_size)
            .field("allele_ranges", &self.allele_ranges)
            .field("mutation_types", &self.mutation_types)
            .field("seed_genes_list", &self.seed_genes_list)
            .finish()
    }
}

impl<T: RangeAllele> fmt::Display for MultiRange<T>
where
    Uniform<T>: Send + Sync,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "genotype:")?;
        writeln!(f, "  genes_size: {}", self.genes_size)?;
        writeln!(f, "  mutation_types: {:?}", self.mutation_types())?;

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
