use super::builder::{Builder, TryFromBuilderError};
use super::{EvolveGenotype, Genotype, HillClimbGenotype, MutationType};
use crate::centralized::allele::RangeAllele;
use crate::centralized::chromosome::{
    Chromosome, ChromosomeManager, GenesHash, GenesPointer, StaticMatrixChromosome,
};
use crate::centralized::fitness::FitnessValue;
use crate::centralized::population::Population;
use bytemuck::cast_slice;
use itertools::Itertools;
use num::BigUint;
use rand::distributions::uniform::SampleUniform;
use rand::distributions::{Distribution, Uniform};
use rand::prelude::*;
use rustc_hash::FxHasher;
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::{Bound, Range, RangeBounds, RangeInclusive};

/// Genes (`N`) and Population (`M`) are a fixed `N*M` matrix of numeric values, stored on the heap as a
/// nested array `Box<[[T; N]; M]>`. The genes are contiguous in memory, with an `N` jump to the next
/// chromosome (`[[T; N]; M]` can be treated like `[T; N * M]` in memory). The genes are therefore not
/// stored on the Chromosomes themselves, which just point to the data (chromosome.row_id ==
/// row id of the matrix). The genes_size can be smaller than N, which would just leave a part of
/// the matrix unused at T::default(). This opens the possibility for linear algebra fitness
/// calculations on the whole population at once, possibly using the GPU in the future (if the data
/// is stored and mutated at a GPU readable memory location). The fitness would then implement
/// [calculate_for_population](crate::fitness::Fitness::calculate_for_population) instead of
/// [calculate_for_chromosome](crate::fitness::Fitness::calculate_for_chromosome).
///
/// The rest is like [RangeGenotype](super::RangeGenotype), but it cannot be permutated:
///
/// The values are taken from the allele range. On random initialization, each gene gets a value
/// from the allele_range with a uniform probability. Each gene has an equal probability of
/// mutating. If a gene mutates, a new value is taken from allele_range with a uniform probability.
///
/// Optionally the mutation range can be bound by relative allele_mutation_range or
/// allele_mutation_scaled_range. When allele_mutation_range is provided the mutation is restricted
/// to modify the existing value by a difference taken from allele_mutation_range with a uniform
/// probability. When allele_mutation_scaled_range is provided the mutation is restricted to modify
/// the existing value by a difference taken from start and end of the scaled range (depending on
/// current scale)
///
/// # Panics
///
/// Will panic if more chromosomes are instantiated than the population (M) allows. M should
/// account for the target_population_size and the crossover selection_rate which adds offspring on
/// top of that.
///
/// # Example (f32):
/// ```
/// use genetic_algorithm::centralized::genotype::{Genotype, StaticMatrixGenotype};
///
/// const GENES_SIZE: usize = 100;
/// const POPULATION_SIZE: usize = 200;
///
/// let genotype = StaticMatrixGenotype::<f32, GENES_SIZE, POPULATION_SIZE>::builder()
///     .with_genes_size(100)
///     .with_allele_range(0.0..=1.0) // also default mutation range
///     .with_allele_mutation_range(-0.1..=0.1) // optional, restricts mutations to a smaller relative range
///     .with_allele_mutation_scaled_range(vec![-0.1..=0.1, -0.01..=0.01, -0.001..=0.001]) // optional, restricts mutations to relative start/end of each scale
///     .with_genes_hashing(false) // optional, defaults to false
///     .build()
///     .unwrap();
/// ```
///
/// # Example (isize):
/// ```
/// use genetic_algorithm::centralized::genotype::{Genotype, StaticMatrixGenotype};
///
/// const GENES_SIZE: usize = 100;
/// const POPULATION_SIZE: usize = 200;
///
/// let genotype = StaticMatrixGenotype::<isize, GENES_SIZE, POPULATION_SIZE>::builder()
///     .with_genes_size(100)
///     .with_allele_range(0..=100) // also default mutation range
///     .with_allele_mutation_range(-1..=1) // optional, restricts mutations to a smaller relative range
///     .with_allele_mutation_scaled_range(vec![-10..=10, -3..=3, -1..=1]) // optional, restricts mutations to relative start/end of each scale
///     .with_genes_hashing(true) // optional, defaults to false
///     .build()
///     .unwrap();
/// ```
pub struct StaticMatrix<T: RangeAllele, const N: usize, const M: usize>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    pub data: Box<[[T; N]; M]>,
    pub chromosome_bin: Vec<StaticMatrixChromosome>,
    pub genes_size: usize,
    pub allele_range: RangeInclusive<T>,
    pub allele_mutation_range: Option<RangeInclusive<T>>,
    pub allele_mutation_scaled_range: Option<Vec<RangeInclusive<T>>>,
    pub mutation_type: MutationType,
    gene_index_sampler: Uniform<usize>,
    allele_sampler: Uniform<T>,
    allele_relative_sampler: Option<Uniform<T>>,
    pub seed_genes_list: Vec<Box<[T; N]>>,
    pub best_genes: Box<[T; N]>,
    pub genes_hashing: bool,
}

impl<T: RangeAllele, const N: usize, const M: usize> TryFrom<Builder<Self>>
    for StaticMatrix<T, N, M>
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
            let mutation_type = if builder.allele_mutation_scaled_range.is_some() {
                MutationType::Scaled
            } else if builder.allele_mutation_range.is_some() {
                MutationType::Relative
            } else {
                MutationType::Random
            };

            Ok(Self {
                data: Box::new([[T::default(); N]; M]),
                chromosome_bin: Vec::with_capacity(M),
                genes_size,
                allele_range: allele_range.clone(),
                allele_mutation_range: builder.allele_mutation_range.clone(),
                allele_mutation_scaled_range: builder.allele_mutation_scaled_range.clone(),
                mutation_type,
                gene_index_sampler: Uniform::from(0..genes_size),
                allele_sampler: Uniform::from(allele_range.clone()),
                allele_relative_sampler: builder
                    .allele_mutation_range
                    .map(|allele_mutation_range| Uniform::from(allele_mutation_range.clone())),
                seed_genes_list: builder.seed_genes_list,
                best_genes: Box::new([T::default(); N]),
                genes_hashing: builder.genes_hashing,
            })
        }
    }
}

impl<T: RangeAllele, const N: usize, const M: usize> StaticMatrix<T, N, M>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    fn mutate_chromosome_index_random<R: Rng>(
        &mut self,
        index: usize,
        chromosome: &mut StaticMatrixChromosome,
        rng: &mut R,
    ) {
        self.set_gene_by_id(chromosome.row_id, index, self.allele_sampler.sample(rng));
    }
    fn mutate_chromosome_index_relative<R: Rng>(
        &mut self,
        index: usize,
        chromosome: &mut StaticMatrixChromosome,
        rng: &mut R,
    ) {
        let value_diff = self.allele_relative_sampler.as_ref().unwrap().sample(rng);
        let new_value = self.get_gene_by_id(chromosome.row_id, index) + value_diff;
        if new_value < *self.allele_range.start() {
            self.set_gene_by_id(chromosome.row_id, index, *self.allele_range.start());
        } else if new_value > *self.allele_range.end() {
            self.set_gene_by_id(chromosome.row_id, index, *self.allele_range.end());
        } else {
            self.set_gene_by_id(chromosome.row_id, index, new_value);
        }
    }
    fn mutate_chromosome_index_scaled<R: Rng>(
        &mut self,
        index: usize,
        chromosome: &mut StaticMatrixChromosome,
        scale_index: usize,
        rng: &mut R,
    ) {
        let working_range = &self.allele_mutation_scaled_range.as_ref().unwrap()[scale_index];
        let value_diff = if rng.gen() {
            *working_range.start()
        } else {
            *working_range.end()
        };
        let new_value = self.get_gene_by_id(chromosome.row_id, index) + value_diff;
        if new_value < *self.allele_range.start() {
            self.set_gene_by_id(chromosome.row_id, index, *self.allele_range.start());
        } else if new_value > *self.allele_range.end() {
            self.set_gene_by_id(chromosome.row_id, index, *self.allele_range.end());
        } else {
            self.set_gene_by_id(chromosome.row_id, index, new_value);
        }
    }

    /// returns a slice of genes_size <= N
    fn get_genes_by_id(&self, id: usize) -> &[T] {
        &self.data[id][..self.genes_size]
    }
    fn get_gene_by_id(&self, id: usize, index: usize) -> T {
        self.data[id][index]
    }
    fn set_gene_by_id(&mut self, id: usize, index: usize, value: T) {
        self.data[id][index] = value;
    }
    fn copy_genes_by_id(&mut self, source_id: usize, target_id: usize) {
        let (source, target) = self.gene_slice_pair_mut((source_id, target_id));
        (target).copy_from_slice(&source[..]);
    }
    fn swap_gene_by_id(&mut self, father_id: usize, mother_id: usize, index: usize) {
        let (father, mother) = self.gene_slice_pair_mut((father_id, mother_id));
        std::mem::swap(&mut father[index], &mut mother[index]);

        // // unsafe version, not much faster
        // // Can't take two mutable loans from one vector, so instead use raw pointers.
        // let pa = std::ptr::addr_of_mut!(self.matrix[father_id][index]);
        // let pb = std::ptr::addr_of_mut!(self.matrix[mother_id][index]);
        // // SAFETY: `pa` and `pb` have been created from safe mutable references and refer
        // // to elements in the slice and therefore are guaranteed to be valid and aligned.
        // // Note that accessing the elements behind `a` and `b` is checked and will
        // // panic when out of bounds.
        // unsafe {
        //     std::ptr::swap(pa, pb);
        // }
    }
    #[allow(dead_code)]
    fn copy_gene_range_by_id<B: RangeBounds<usize>>(
        &mut self,
        source_id: usize,
        target_id: usize,
        range: B,
    ) {
        let (source_range, target_range) = self.gene_slice_pair_range(range);
        let (source, target) = self.gene_slice_pair_mut((source_id, target_id));
        (target[target_range]).copy_from_slice(&source[source_range]);
    }
    fn swap_gene_range_by_id<B: RangeBounds<usize>>(
        &mut self,
        father_id: usize,
        mother_id: usize,
        range: B,
    ) {
        let (father_range, mother_range) = self.gene_slice_pair_range(range);
        let (father, mother) = self.gene_slice_pair_mut((father_id, mother_id));
        (mother[mother_range]).swap_with_slice(&mut father[father_range]);
    }
    fn gene_slice_pair_mut(&mut self, ids: (usize, usize)) -> (&mut [T; N], &mut [T; N]) {
        match ids.0.cmp(&ids.1) {
            Ordering::Less => {
                let (x, y) = self.data.split_at_mut(ids.1);
                (&mut x[ids.0], &mut y[0])
            }
            Ordering::Greater => {
                let (x, y) = self.data.split_at_mut(ids.0);
                (&mut y[0], &mut x[ids.1])
            }
            Ordering::Equal => unreachable!("ids cannot be the same: {:?}", ids),
        }
    }
    fn gene_slice_pair_range<B: RangeBounds<usize>>(
        &self,
        range: B,
    ) -> (Range<usize>, Range<usize>) {
        let min_index = match range.start_bound() {
            Bound::Unbounded => 0,
            Bound::Included(&i) => i,
            Bound::Excluded(&i) => i + 1,
        }
        .max(0);
        let max_index_excl = match range.end_bound() {
            Bound::Unbounded => self.genes_size,
            Bound::Included(&i) => i + 1,
            Bound::Excluded(&i) => i,
        }
        .min(N);
        (min_index..max_index_excl, min_index..max_index_excl)
    }
}

impl<T: RangeAllele, const N: usize, const M: usize> Genotype for StaticMatrix<T, N, M>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    type Allele = T;
    type Genes = Box<[T; N]>;
    type Chromosome = StaticMatrixChromosome;

    fn genes_size(&self) -> usize {
        self.genes_size
    }
    fn save_best_genes(&mut self, chromosome: &Self::Chromosome) {
        let x = self.data[chromosome.row_id].as_slice();
        self.best_genes.copy_from_slice(x)
    }
    fn load_best_genes(&mut self, chromosome: &mut Self::Chromosome) {
        let x = self.data[chromosome.row_id].as_mut_slice();
        x.copy_from_slice(self.best_genes.as_slice())
    }
    fn best_genes(&self) -> &Self::Genes {
        &self.best_genes
    }
    fn best_genes_slice(&self) -> &[Self::Allele] {
        self.best_genes.as_slice()
    }
    fn genes_slice<'a>(&'a self, chromosome: &'a Self::Chromosome) -> &'a [Self::Allele] {
        self.get_genes_by_id(chromosome.row_id)
    }
    fn genes_hashing(&self) -> bool {
        self.genes_hashing
    }
    fn calculate_genes_hash(&self, chromosome: &Self::Chromosome) -> Option<GenesHash> {
        if self.genes_hashing {
            let mut s = FxHasher::default();
            let bytes: &[u8] = cast_slice(self.genes_slice(chromosome));
            // unsafe {
            //     let (prefix, shorts, suffix) = bytes.align_to::<u64>();
            //     prefix.hash(&mut s);
            //     shorts.hash(&mut s);
            //     suffix.hash(&mut s);
            // }
            bytes.hash(&mut s);
            Some(s.finish())
        } else {
            None
        }
    }

    fn update_population_fitness_scores(
        &self,
        population: &mut Population<Self::Chromosome>,
        fitness_scores: Vec<Option<FitnessValue>>,
    ) {
        population
            .chromosomes
            .iter_mut()
            .filter(|c| c.fitness_score.is_none())
            .for_each(|chromosome| {
                if let Some(&fitness_score) = fitness_scores.get(chromosome.row_id) {
                    chromosome.set_fitness_score(fitness_score);
                }
            });
    }

    fn mutation_type(&self) -> MutationType {
        self.mutation_type
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
        self.reset_chromosome_state(chromosome);
    }

    fn set_seed_genes_list(&mut self, seed_genes_list: Vec<Self::Genes>) {
        self.seed_genes_list = seed_genes_list;
    }
    fn seed_genes_list(&self) -> &Vec<Self::Genes> {
        &self.seed_genes_list
    }
    fn max_scale_index(&self) -> Option<usize> {
        self.allele_mutation_scaled_range
            .as_ref()
            .map(|r| r.len() - 1)
    }
}

impl<T: RangeAllele, const N: usize, const M: usize> EvolveGenotype for StaticMatrix<T, N, M>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
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
                    self.swap_gene_by_id(father.row_id, mother.row_id, index);
                });
        } else {
            rand::seq::index::sample(
                rng,
                self.genes_size(),
                number_of_crossovers.min(self.genes_size()),
            )
            .iter()
            .for_each(|index| {
                self.swap_gene_by_id(father.row_id, mother.row_id, index);
            });
        }
        self.reset_chromosome_state(mother);
        self.reset_chromosome_state(father);
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
                    self.swap_gene_range_by_id(father.row_id, mother.row_id, index..);
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
                    self.swap_gene_range_by_id(
                        father.row_id,
                        mother.row_id,
                        start_index..end_index,
                    );
                }
                (Some(start_index), _) => {
                    self.swap_gene_range_by_id(father.row_id, mother.row_id, start_index..);
                }
                _ => (),
            });
        }
        self.reset_chromosome_state(mother);
        self.reset_chromosome_state(father);
    }

    fn has_crossover_indexes(&self) -> bool {
        true
    }
    fn has_crossover_points(&self) -> bool {
        true
    }
}
impl<T: RangeAllele, const N: usize, const M: usize> HillClimbGenotype for StaticMatrix<T, N, M>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    fn fill_neighbouring_population<R: Rng>(
        &mut self,
        chromosome: &Self::Chromosome,
        population: &mut Population<Self::Chromosome>,
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

impl<T: RangeAllele, const N: usize, const M: usize> StaticMatrix<T, N, M>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    fn fill_neighbouring_population_scaled(
        &mut self,
        chromosome: &StaticMatrixChromosome,
        population: &mut Population<StaticMatrixChromosome>,
        scale_index: usize,
    ) {
        let allele_range_start = *self.allele_range.start();
        let allele_range_end = *self.allele_range.end();

        let working_range = &self.allele_mutation_scaled_range.as_ref().unwrap()[scale_index];
        let working_range_start = *working_range.start();
        let working_range_end = *working_range.end();

        (0..self.genes_size).for_each(|index| {
            let base_value = self.get_gene_by_id(chromosome.row_id, index);
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
                let mut new_chromosome = self.chromosome_cloner(chromosome);
                self.set_gene_by_id(new_chromosome.row_id, index, value_low);
                self.reset_chromosome_state(&mut new_chromosome);
                population.chromosomes.push(new_chromosome)
            };
            if value_high > base_value {
                let mut new_chromosome = self.chromosome_cloner(chromosome);
                self.set_gene_by_id(new_chromosome.row_id, index, value_high);
                self.reset_chromosome_state(&mut new_chromosome);
                population.chromosomes.push(new_chromosome)
            };
        });
    }

    fn fill_neighbouring_population_relative<R: Rng>(
        &mut self,
        chromosome: &StaticMatrixChromosome,
        population: &mut Population<StaticMatrixChromosome>,
        rng: &mut R,
    ) {
        let allele_range_start = *self.allele_range.start();
        let allele_range_end = *self.allele_range.end();

        let working_range = &self.allele_mutation_range.as_ref().unwrap();
        let working_range_start = *working_range.start();
        let working_range_end = *working_range.end();

        (0..self.genes_size).for_each(|index| {
            let base_value = self.get_gene_by_id(chromosome.row_id, index);
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
                let mut new_chromosome = self.chromosome_cloner(chromosome);
                let new_value = rng.gen_range(range_start..base_value);
                self.set_gene_by_id(new_chromosome.row_id, index, new_value);
                self.reset_chromosome_state(&mut new_chromosome);
                population.chromosomes.push(new_chromosome)
            };
            if base_value < range_end {
                let mut new_chromosome = self.chromosome_cloner(chromosome);
                let new_value = rng.gen_range((base_value + T::smallest_increment())..=range_end);
                self.set_gene_by_id(new_chromosome.row_id, index, new_value);
                self.reset_chromosome_state(&mut new_chromosome);
                population.chromosomes.push(new_chromosome)
            };
        });
    }

    fn fill_neighbouring_population_random<R: Rng>(
        &mut self,
        chromosome: &StaticMatrixChromosome,
        population: &mut Population<StaticMatrixChromosome>,
        rng: &mut R,
    ) {
        let allele_range_start = *self.allele_range.start();
        let allele_range_end = *self.allele_range.end();

        (0..self.genes_size).for_each(|index| {
            let base_value = self.get_gene_by_id(chromosome.row_id, index);
            if allele_range_start < base_value {
                let mut new_chromosome = self.chromosome_cloner(chromosome);
                let new_value = rng.gen_range(allele_range_start..base_value);
                self.set_gene_by_id(new_chromosome.row_id, index, new_value);
                self.reset_chromosome_state(&mut new_chromosome);
                population.chromosomes.push(new_chromosome)
            };
            if base_value < allele_range_end {
                let mut new_chromosome = self.chromosome_cloner(chromosome);
                let new_value =
                    rng.gen_range((base_value + T::smallest_increment())..=allele_range_end);
                self.set_gene_by_id(new_chromosome.row_id, index, new_value);
                self.reset_chromosome_state(&mut new_chromosome);
                population.chromosomes.push(new_chromosome)
            };
        });
    }
}

impl<T: RangeAllele, const N: usize, const M: usize> ChromosomeManager<Self>
    for StaticMatrix<T, N, M>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    fn random_genes_factory<R: Rng>(&self, rng: &mut R) -> Box<[T; N]> {
        if self.seed_genes_list.is_empty() {
            Box::new(std::array::from_fn(|_| self.allele_sampler.sample(rng)))
        } else {
            self.seed_genes_list.choose(rng).unwrap().clone()
        }
    }
    fn set_genes(&mut self, chromosome: &mut StaticMatrixChromosome, genes: &Box<[T; N]>) {
        let x = self.data[chromosome.row_id].as_mut_slice();
        x.copy_from_slice(genes.as_slice());
        self.reset_chromosome_state(chromosome);
    }
    fn get_genes(&self, chromosome: &StaticMatrixChromosome) -> Box<[T; N]> {
        Box::new(self.data[chromosome.row_id])
    }
    fn copy_genes(&mut self, source: &StaticMatrixChromosome, target: &mut StaticMatrixChromosome) {
        self.copy_genes_by_id(source.row_id, target.row_id);
        self.copy_chromosome_state(source, target);
    }
    fn chromosomes_setup(&mut self) {
        self.chromosome_bin = (0..M).rev().map(StaticMatrixChromosome::new).collect();
    }
    fn chromosome_bin_push(&mut self, chromosome: StaticMatrixChromosome) {
        self.chromosome_bin.push(chromosome);
    }
    fn chromosome_bin_find_or_create(&mut self) -> StaticMatrixChromosome {
        self.chromosome_bin.pop().unwrap_or_else(|| {
            panic!("genetic_algorithm error: chromosome capacity exceeded");
        })
    }
    fn chromosomes_cleanup(&mut self) {
        std::mem::take(&mut self.chromosome_bin);
        // FIXME: does this leave an empty box?
        // let _ = *self.data;
    }
}

impl<T: RangeAllele, const N: usize, const M: usize> Clone for StaticMatrix<T, N, M>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    fn clone(&self) -> Self {
        Self {
            data: Box::new([[T::default(); N]; M]),
            chromosome_bin: Vec::with_capacity(M),
            genes_size: self.genes_size,
            allele_range: self.allele_range.clone(),
            allele_mutation_range: self.allele_mutation_range.clone(),
            allele_mutation_scaled_range: self.allele_mutation_scaled_range.clone(),
            mutation_type: self.mutation_type,
            gene_index_sampler: self.gene_index_sampler,
            allele_sampler: Uniform::from(self.allele_range.clone()),
            allele_relative_sampler: self
                .allele_mutation_range
                .clone()
                .map(|allele_mutation_range| Uniform::from(allele_mutation_range.clone())),
            seed_genes_list: self.seed_genes_list.clone(),
            best_genes: Box::new([T::default(); N]),
            genes_hashing: self.genes_hashing,
        }
    }
}

impl<T: RangeAllele, const N: usize, const M: usize> fmt::Debug for StaticMatrix<T, N, M>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Point")
            .field("genes_size", &self.genes_size)
            .field("allele_range", &self.allele_range)
            .field("allele_mutation_range", &self.allele_mutation_range)
            .field(
                "allele_mutation_scaled_range",
                &self.allele_mutation_scaled_range,
            )
            .field("mutation_type", &self.mutation_type)
            .field("seed_genes_list", &self.seed_genes_list)
            .finish()
    }
}

impl<T: RangeAllele, const N: usize, const M: usize> fmt::Display for StaticMatrix<T, N, M>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "genotype:")?;
        writeln!(f, "  genes_size: {}", self.genes_size)?;
        writeln!(f, "  mutation_type: {:?}", self.mutation_type)?;
        writeln!(f, "  chromosome_permutations_size: uncountable")?;
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
