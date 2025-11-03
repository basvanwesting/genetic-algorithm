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
/// a value from its own allele_range with a uniform probability. Each gene has a equal probability
/// of mutating, regardless of its allele_range size. If a gene mutates, a new values is taken from
/// its own allele_range with a uniform probability. Duplicate allele values are allowed. Supports
/// heterogeneous chromosomes that mix different gene semantics (continuous values, discrete
/// choices, booleans) within a single numeric type `T`.
///
/// # Mutation types
///
/// Optionally the mutation range can be bound by relative `allele_mutation_ranges` or
/// `allele_mutation_scaled_ranges`. When `allele_mutation_ranges` are provided the mutation is
/// restricted to modify the existing value by a difference taken from `allele_mutation_ranges` with
/// a uniform probability. When `allele_mutation_scaled_ranges` are provided the mutation is
/// restricted to modify the existing value by a difference taken from start and end of the scaled
/// range (depending on current scale)
///
/// Mutation type is defined by the most recent builder setting, so these can overwrite:
/// * `with_mutation_types` → set directly (all types, mixed)
/// * `with_allele_mutation_scaled_ranges` → legacy setting, scaled for all genes
/// * `with_allele_mutation_ranges` → legacy setting, relative for all genes
/// * no setting → default, random for all genes
///
/// # Permutation
///
/// Supports Permutation for scaled and discrete mutations only. This approach implements a
/// increasingly localized grid search with increasing precision using the
/// `allele_mutation_scaled_ranges` to define the search scope and grid steps
/// * First scale (index = 0) traverses the whole `allele_ranges` with the
///   upper bound of the first scale as step size.
/// * Other scales (index > 0) center around the best chromosome of the previous
///   scale, traversing the previous scale bounds around the best chromosome with
///   the upper bound of the current scale as step size.
/// * Scale down and repeat after grid is fully traversed
///
/// The discrete mutations traverse all allowed values for every scale (see below)
///
/// # Heterogeneous Genotype Support
///
/// MultiRangeGenotype supports heterogeneous chromosomes that mix different gene semantics
/// (continuous values, discrete choices, booleans) within a single numeric type `T`.
///
/// Use `.with_mutation_types(vec![...])` to specify behavior for each gene individually:
///
/// * `MutationType::Random` - Samples uniformly from the full allele range
/// * `MutationType::Relative` - Mutates within a relative range around the current value
/// * `MutationType::Scaled` - Progressive refinement through multiple scale levels around the current value
/// * `MutationType::Discrete` - Rounded-to-integer values with uniform selection (like `ListGenotype`).
///   * Mutations ignore current value - all rounded-to-integer in range equally likely
///   * Range `0.0..=4.0` yields values: 0.0, 1.0, 2.0, 3.0, 4.0 (with equal probability)
///   * Useful for encoding: enums (0.0..=4.0), booleans (0.0..=1.0), or discrete choices
///   * Neighbours and permutations include all integer values in the allele range
///
/// Explicit `.with_mutation_types()` overrides any auto-detected mutation range settings.
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
/// # Example (isize, relative mutation):
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
///     .with_allele_mutation_ranges(vec![
///        -1..=1,
///        -2..=2,
///        -1..=1,
///        -3..=3,
///     ]) // optional, restricts mutations to a smaller relative range
///     .with_genes_hashing(true) // optional, defaults to true
///     .with_chromosome_recycling(true) // optional, defaults to true
///     .build()
///     .unwrap();
/// ```
///
/// # Example (f32, scaled mutation):
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
///     .with_allele_mutation_scaled_ranges(vec![
///        vec![-1.0..=1.0, -2.0..=2.0, -0.5..=0.5, -3.0..=3.0],
///        vec![-0.1..=0.1, -0.2..=0.2, -0.05..=0.05, -0.3..=0.3],
///     ]) // optional, restricts mutations to relative start/end of each scale
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
///         MutationType::Scaled(vec![-10.0..=10.0, -1.0..=1.0, -0.1..=0.1]),    // Continuous refinement
///     ])
///     .with_allele_mutation_scaled_ranges(vec![
///        vec![0.0..=0.0, 0.0..=0.0, -10.0..=10.0],
///        vec![0.0..=0.0, 0.0..=0.0, -1.0..=1.0],
///        vec![0.0..=0.0, 0.0..=0.0, -0.1..=0.1],
///     ]) // restricts mutations to relative start/end of each scale, the values for discrete genes are ignored
///     .with_genes_hashing(true) // optional, defaults to true
///     .with_chromosome_recycling(true) // optional, defaults to true
///     .build();
/// ```
pub struct MultiRange<T: RangeAllele + Into<f64> = DefaultAllele>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    pub genes_size: usize,
    pub allele_ranges: Vec<RangeInclusive<T>>,
    pub mutation_types: Vec<MutationType<T>>,
    gene_index_sampler: Uniform<usize>,
    allele_samplers: Vec<Uniform<T>>,
    allele_relative_samplers: Vec<Option<Uniform<T>>>,
    pub current_scale_index: usize,
    pub seed_genes_list: Vec<Vec<T>>,
    pub genes_hashing: bool,
    pub chromosome_recycling: bool,
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
            let mutation_types = builder
                .mutation_types
                .unwrap_or(vec![MutationType::Random; genes_size]);

            let allele_relative_samplers = mutation_types
                .iter()
                .map(|mutation_type| match &mutation_type {
                    MutationType::Random
                    | MutationType::Discrete
                    | MutationType::Scaled(_)
                    | MutationType::Transition(_, _, _) => None,
                    MutationType::Relative(relative_range) => {
                        Some(Uniform::from(relative_range.clone()))
                    }
                })
                .collect();

            Ok(Self {
                genes_size,
                allele_ranges: allele_ranges.clone(),
                mutation_types: mutation_types.clone(),
                gene_index_sampler: Uniform::from(0..genes_size),
                allele_samplers: allele_ranges
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
                    .collect(),
                allele_relative_samplers,
                current_scale_index: 0,
                seed_genes_list: builder.seed_genes_list,
                genes_hashing: builder.genes_hashing,
                chromosome_recycling: builder.chromosome_recycling,
            })
        }
    }
}

impl<T: RangeAllele + Into<f64>> MultiRange<T>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    fn mutation_types(&self) -> &[MutationType<T>] {
        &self.mutation_types
    }
    pub fn sample_allele<R: Rng>(&self, index: usize, rng: &mut R) -> T {
        match self.mutation_types[index] {
            MutationType::Discrete => self.allele_samplers[index].sample(rng).floor(),
            _ => self.allele_samplers[index].sample(rng),
        }
    }
    pub fn sample_gene_delta<R: Rng>(&self, index: usize, rng: &mut R) -> T {
        match &self.mutation_types[index] {
            MutationType::Scaled(scaled_ranges) => {
                let working_range = &scaled_ranges[self.current_scale_index];
                if rng.gen() {
                    *working_range.start()
                } else {
                    *working_range.end()
                }
            }
            MutationType::Relative(_) => self.allele_relative_samplers[index]
                .as_ref()
                .unwrap()
                .sample(rng),
            MutationType::Transition(_, _, _) => {
                todo!()
            }

            MutationType::Random => {
                panic!("RangeGenotype has no concept of gene delta for MutationType::Random")
            }
            MutationType::Discrete => {
                panic!("RangeGenotype has no concept of gene delta for MutationType::Discrete")
            }
        }
    }
    pub fn apply_gene_delta(&self, chromosome: &mut Chromosome<T>, index: usize, delta: T) {
        let allele_range = &self.allele_ranges[index];
        let new_value = chromosome.genes[index] + delta;
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
                match self.mutation_types[index] {
                    MutationType::Random | MutationType::Discrete => {
                        chromosome.genes[index] = self.sample_allele(index, rng);
                    }
                    MutationType::Transition(_, _, _) => {
                        todo!()
                    }
                    _ => {
                        let delta = self.sample_gene_delta(index, rng);
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
                match self.mutation_types[index] {
                    MutationType::Random | MutationType::Discrete => {
                        chromosome.genes[index] = self.sample_allele(index, rng);
                    }
                    MutationType::Transition(_, _, _) => {
                        todo!()
                    }
                    _ => {
                        let delta = self.sample_gene_delta(index, rng);
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
        self.mutation_types
            .iter()
            .find_map(|mutation_type| match mutation_type {
                MutationType::Scaled(scaled_ranges) => Some(scaled_ranges.len() - 1),
                _ => None,
            })
    }
    fn current_scale_index(&self) -> Option<usize> {
        self.mutation_types
            .iter()
            .find_map(|mutation_type| match mutation_type {
                MutationType::Scaled(_) => Some(self.current_scale_index),
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
                .map(|index| self.sample_allele(index, rng))
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
impl<T: RangeAllele + Into<f64>> HillClimbGenotype for MultiRange<T>
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
        self.mutation_types.iter().enumerate().for_each(
            |(index, mutation_type)| match mutation_type {
                MutationType::Scaled(scaled_ranges) => self.fill_neighbouring_population_scaled(
                    index,
                    chromosome,
                    population,
                    scaled_ranges,
                ),
                MutationType::Relative(relative_range) => self
                    .fill_neighbouring_population_relative(
                        index,
                        chromosome,
                        population,
                        relative_range,
                        rng,
                    ),
                MutationType::Transition(_, _, _) => {
                    todo!()
                }
                MutationType::Random => {
                    self.fill_neighbouring_population_random(index, chromosome, population, rng)
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

impl<T: RangeAllele + Into<f64>> MultiRange<T>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    fn fill_neighbouring_population_scaled(
        &self,
        index: usize,
        chromosome: &Chromosome<T>,
        population: &mut Population<T>,
        scaled_ranges: &Vec<RangeInclusive<T>>,
    ) {
        let allele_range_start = *self.allele_ranges[index].start();
        let allele_range_end = *self.allele_ranges[index].end();
        let working_range = &scaled_ranges[self.current_scale_index];
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
    }

    fn fill_neighbouring_population_relative<R: Rng>(
        &self,
        index: usize,
        chromosome: &Chromosome<T>,
        population: &mut Population<T>,
        relative_range: &RangeInclusive<T>,
        rng: &mut R,
    ) {
        let allele_range_start = *self.allele_ranges[index].start();
        let allele_range_end = *self.allele_ranges[index].end();
        let working_range_start = *relative_range.start();
        let working_range_end = *relative_range.end();

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
            working_value = working_value + T::one();
        }
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
    ) -> Box<dyn Iterator<Item = Chromosome<Self::Allele>> + Send + 'a> {
        if self.seed_genes_list.is_empty() {
            Box::new(
                self.mutation_types
                    .iter()
                    .enumerate()
                    .map(|(index, mutation_type)| match mutation_type {
                        MutationType::Scaled(scaled_ranges) => {
                            self.permutable_gene_values_scaled(index, chromosome, scaled_ranges)
                        }
                        MutationType::Relative(_) => {
                            panic!("RangeGenotype is not permutable for MutationType::Relative")
                        }
                        MutationType::Transition(_, _, _) => {
                            todo!()
                        }
                        MutationType::Random => {
                            panic!("RangeGenotype is not permutable for MutationType::Random")
                        }
                        MutationType::Discrete => {
                            self.permutable_gene_values_discrete(index, chromosome)
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
        if self.seed_genes_list.is_empty() {
            if let Some(max_scale_index) = self.max_scale_index() {
                (0..=max_scale_index)
                    .map(|scale_index| {
                        self.chromosome_permutations_size_for_scale_index(scale_index)
                    })
                    .sum()
            } else {
                panic!("MultiRangeGenotype is only permutable for MutationType::Scaled")
            }
        } else {
            self.seed_genes_list.len().into()
        }
    }
    fn chromosome_permutations_size_report(&self) -> String {
        if self.allows_permutation() {
            let size_per_scale: Vec<String> = (0..=self.max_scale_index().unwrap())
                .map(|scale_index| self.chromosome_permutations_size_for_scale_index(scale_index))
                .map(|scale_size| self.format_biguint_scientific(&scale_size))
                .collect();
            format!(
                "{}, per scale {:?}",
                self.format_biguint_scientific(&self.chromosome_permutations_size()),
                size_per_scale
            )
        } else {
            "uncountable".to_string()
        }
    }
    fn allows_permutation(&self) -> bool {
        !self
            .mutation_types
            .iter()
            .any(|mutation_type| match mutation_type {
                MutationType::Relative(_) | MutationType::Random => true,
                MutationType::Scaled(_) | MutationType::Discrete => false,
                MutationType::Transition(_, _, _) => {
                    todo!()
                }
            })
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
        index: usize,
        chromosome: Option<&Chromosome<T>>,
        scaled_ranges: &Vec<RangeInclusive<T>>,
    ) -> Vec<T> {
        let allele_range_start = *self.allele_ranges[index].start();
        let allele_range_end = *self.allele_ranges[index].end();

        let (allele_value_start, allele_value_end) = if let Some(chromosome) = chromosome {
            if let Some(previous_scale_index) = self.current_scale_index.checked_sub(1) {
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
                (allele_range_start, allele_range_end)
            }
        } else {
            (allele_range_start, allele_range_end)
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
                // FIXME: remove bounds check?
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

    pub fn permutable_allele_sizes_for_scale_index(&self, scale_index: usize) -> Vec<usize> {
        self.mutation_types
            .iter()
            .enumerate()
            .map(|(index, mutation_type)| match mutation_type {
                MutationType::Scaled(scaled_ranges) => {
                    let (allele_value_start, allele_value_end) = if let Some(previous_scale_index) =
                        scale_index.checked_sub(1)
                    {
                        let working_range = &scaled_ranges[previous_scale_index];
                        (*working_range.start(), *working_range.end())
                    } else {
                        (
                            *self.allele_ranges[index].start(),
                            *self.allele_ranges[index].end(),
                        )
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
                MutationType::Discrete => {
                  let start_f64: f64 = (*self.allele_ranges[index].start()).into();
                  let end_f64: f64 = (*self.allele_ranges[index].end() + T::one()).into();
                  (end_f64.floor() - start_f64.floor()) as usize
                }
                _ => {
                    panic!("MultiRangeGenotype is only permutable for MutationType::Scaled and MutationType::Discrete")
                }
            })
            .collect()
    }

    pub fn chromosome_permutations_size_for_scale_index(&self, scale_index: usize) -> BigUint {
        self.permutable_allele_sizes_for_scale_index(scale_index)
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
        let allele_relative_samplers = self
            .mutation_types
            .iter()
            .map(|mutation_type| match &mutation_type {
                MutationType::Random | MutationType::Discrete | MutationType::Scaled(_) => None,
                MutationType::Transition(_, _, _) => {
                    todo!()
                }
                MutationType::Relative(relative_range) => {
                    Some(Uniform::from(relative_range.clone()))
                }
            })
            .collect();

        Self {
            genes_size: self.genes_size,
            allele_ranges: self.allele_ranges.clone(),
            mutation_types: self.mutation_types.clone(),
            gene_index_sampler: self.gene_index_sampler,
            allele_samplers: self
                .allele_ranges
                .iter()
                .map(|allele_range| Uniform::from(allele_range.clone()))
                .collect(),
            allele_relative_samplers,
            current_scale_index: self.current_scale_index,
            seed_genes_list: self.seed_genes_list.clone(),
            genes_hashing: self.genes_hashing,
            chromosome_recycling: self.chromosome_recycling,
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
            .field("mutation_types", &self.mutation_types)
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
