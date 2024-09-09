use super::builder::{Builder, TryFromBuilderError};
use super::{Allele, Genotype};
use crate::chromosome::{Chromosome, ChromosomeManager, StaticMatrixChromosome};
use itertools::Itertools;
use rand::distributions::uniform::SampleUniform;
use rand::distributions::{Distribution, Uniform};
use rand::prelude::*;
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Bound, Range, RangeBounds, RangeInclusive};

#[derive(Copy, Clone, Debug)]
pub enum MutationType {
    Random,
    Relative,
    Scaled,
}

/// Genes (N) and Population (M) are a `N*M` matrix of numeric values, stored  on the stack as a
/// nested array '[[T; N]; M]'. The genes are contiguous in memory, with an N jump to the next
/// chromosome ([[T; N]; M] can be treated like [T; N*M] in memory). The genes are therefore not
/// stored on the Chromosomes themselves, which just point to the data (chromosome.row_id ==
/// row id of the matrix). The genes_size can be smaller than N, which would just leave a part of
/// the matrix unused at T::default(). This opens the possibility for linear algebra fitness
/// calculations on the whole population at once, possibly using the GPU in the future (if the data
/// is stored and mutated at a GPU readable memory location). The fitness would then implement
/// `call_for_population` instead of `calculate_for_chromosome`.
///
/// This is a simple stack based example implementation, which is
/// threrefore limited in size. Exceeding this size, will result in a "fatal runtime error: stack
/// overflow" panic, aborting the execution during the initialization of the Genotype.
///
/// The population size needs to be padded for 2 additional sets of genes. This is for storing the
/// working chromosome genes and best chromosome genes outside of the population itself. Failure
/// to provide this additional space will result in a "fatal runtime error: stack overflow"
///
/// The GenotypeBuilder `with_chromosome_recycling` is implicit and always enabled for this Genotype.
///
/// The rest is like [RangeGenotype](super::RangeGenotype):
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
/// # Example (f32):
/// ```
/// use genetic_algorithm::genotype::{Genotype, StaticMatrixGenotype};
///
/// const GENES_SIZE: usize = 100;
/// const POPULATION_SIZE: usize = 200;
///
/// let genotype = StaticMatrixGenotype::<f32, GENES_SIZE, { POPULATION_SIZE + 2 }>::builder()
///     .with_genes_size(100)
///     .with_allele_range(0.0..=1.0) // also default mutation range
///     .with_allele_mutation_range(-0.1..=0.1) // optional, restricts mutations to a smaller relative range
///     .with_allele_mutation_scaled_range(vec![-0.1..=0.1, -0.01..=0.01, -0.001..=0.001]) // optional, restricts mutations to relative start/end of each scale
///     .build()
///     .unwrap();
/// ```
///
/// # Example (isize):
/// ```
/// use genetic_algorithm::genotype::{Genotype, StaticMatrixGenotype};
///
/// const GENES_SIZE: usize = 100;
/// const POPULATION_SIZE: usize = 200;
///
/// let genotype = StaticMatrixGenotype::<isize, GENES_SIZE, { POPULATION_SIZE + 2 }>::builder()
///     .with_genes_size(100)
///     .with_allele_range(0..=100) // also default mutation range
///     .with_allele_mutation_range(-1..=1) // optional, restricts mutations to a smaller relative range
///     .with_allele_mutation_scaled_range(vec![-10..=10, -3..=3, -1..=1]) // optional, restricts mutations to relative start/end of each scale
///     .build()
///     .unwrap();
/// ```
pub struct StaticMatrix<
    T: Allele + Add<Output = T> + std::cmp::PartialOrd + Default,
    const N: usize,
    const M: usize,
> where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    pub data: [[T; N]; M],
    pub chromosome_bin: Vec<StaticMatrixChromosome>,
    pub genes_size: usize,
    pub allele_range: RangeInclusive<T>,
    pub allele_mutation_range: Option<RangeInclusive<T>>,
    pub allele_mutation_scaled_range: Option<Vec<RangeInclusive<T>>>,
    pub mutation_type: MutationType,
    gene_index_sampler: Uniform<usize>,
    allele_sampler: Uniform<T>,
    allele_relative_sampler: Option<Uniform<T>>,
    pub seed_genes_list: Vec<[T; N]>,
    pub best_genes: [T; N],
}

impl<
        T: Allele + Add<Output = T> + std::cmp::PartialOrd + Default,
        const N: usize,
        const M: usize,
    > TryFrom<Builder<Self>> for StaticMatrix<T, N, M>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    type Error = TryFromBuilderError;

    fn try_from(builder: Builder<Self>) -> Result<Self, Self::Error> {
        if builder.genes_size.is_none() {
            Err(TryFromBuilderError("RangeGenotype requires a genes_size"))
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
                data: [[T::default(); N]; M],
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
                best_genes: [T::default(); N],
            })
        }
    }
}

impl<
        T: Allele + Add<Output = T> + std::cmp::PartialOrd + Default,
        const N: usize,
        const M: usize,
    > StaticMatrix<T, N, M>
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

    // fn linear_id(&self, id: usize, index: usize) -> usize {
    //     id * N + index
    // }
    // pub fn linear_range<B: RangeBounds<usize>>(&self, id: usize, range: B) -> Range<usize> {
    //     let min_index = match range.start_bound() {
    //         Bound::Unbounded => 0,
    //         Bound::Included(&i) => i,
    //         Bound::Excluded(&i) => i + 1,
    //     }
    //     .max(0);
    //     let max_index = match range.end_bound() {
    //         Bound::Unbounded => self.genes_size,
    //         Bound::Included(&i) => i + 1,
    //         Bound::Excluded(&i) => i,
    //     }
    //     .min(N);
    //     (id * N + min_index)..(id * N + max_index)
    // }

    /// returns a slice of genes_size <= N
    pub fn get_genes(&self, chromosome: &StaticMatrixChromosome) -> &[T] {
        self.get_genes_by_id(chromosome.row_id)
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
    pub fn gene_slice_pair_mut(&mut self, ids: (usize, usize)) -> (&mut [T; N], &mut [T; N]) {
        match ids.0.cmp(&ids.1) {
            Ordering::Less => {
                let (x, y) = self.data.split_at_mut(ids.1);
                (&mut x[ids.0], &mut y[0])
            }
            Ordering::Greater => {
                let (x, y) = self.data.split_at_mut(ids.0);
                (&mut y[0], &mut x[ids.1])
            }
            Ordering::Equal => panic!("ids cannot be the same: {:?}", ids),
        }
    }
    pub fn gene_slice_pair_range<B: RangeBounds<usize>>(
        &self,
        range: B,
    ) -> (Range<usize>, Range<usize>) {
        let min_index = match range.start_bound() {
            Bound::Unbounded => 0,
            Bound::Included(&i) => i,
            Bound::Excluded(&i) => i + 1,
        }
        .max(0);
        let max_index = match range.end_bound() {
            Bound::Unbounded => self.genes_size,
            Bound::Included(&i) => i + 1,
            Bound::Excluded(&i) => i,
        }
        .min(N);
        (min_index..max_index, min_index..max_index)
    }
}

impl<
        T: Allele + Add<Output = T> + std::cmp::PartialOrd + Default,
        const N: usize,
        const M: usize,
    > Genotype for StaticMatrix<T, N, M>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    type Allele = T;
    type Genes = [T; N];
    type Chromosome = StaticMatrixChromosome;

    fn genes_size(&self) -> usize {
        self.genes_size
    }
    fn store_best_genes(&mut self, chromosome: &Self::Chromosome) {
        // let (x, _) = self.data.split_at_mut(chromosome.row_id);
        // self.best_genes.copy_from_slice(&x[0][..]);
        let x = self.data[chromosome.row_id].as_mut_slice();
        self.best_genes.copy_from_slice(x)
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
        self.allele_mutation_scaled_range
            .as_ref()
            .map(|r| r.len() - 1)
    }
}

impl<
        T: Allele + Add<Output = T> + std::cmp::PartialOrd + Default,
        const N: usize,
        const M: usize,
    > StaticMatrix<T, N, M>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    // pub fn reset_ids(&mut self) {
    //     (0..M).for_each(|i| {
    //         self.free_ids.insert(i);
    //     });
    // }
    // pub fn release_id(&mut self, id: usize) -> bool {
    //     self.free_ids.insert(id)
    // }
    // pub fn claim_id_forced(&mut self, id: usize) -> bool {
    //     self.free_ids.remove(&id)
    // }
    // pub fn claim_id(&mut self) -> Option<usize> {
    //     self.free_ids.pop_first()
    // }
}

impl<
        T: Allele + Add<Output = T> + std::cmp::PartialOrd + Default,
        const N: usize,
        const M: usize,
    > ChromosomeManager<Self> for StaticMatrix<T, N, M>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    fn random_genes_factory<R: Rng>(&self, rng: &mut R) -> [T; N] {
        std::array::from_fn(|_| self.allele_sampler.sample(rng))
    }
    fn chromosome_constructor_empty(&self) -> StaticMatrixChromosome {
        StaticMatrixChromosome::new(usize::MAX)
    }
    fn chromosome_is_empty(&self, chromosome: &StaticMatrixChromosome) -> bool {
        chromosome.row_id == usize::MAX
    }
    fn chromosome_recycling(&self) -> bool {
        true
    }
    fn chromosomes_init(&mut self) {
        self.chromosome_bin = (0..M)
            .rev()
            .map(|row_id| StaticMatrixChromosome::new(row_id))
            .collect();
    }
    fn chromosome_bin_push(&mut self, chromosome: StaticMatrixChromosome) {
        self.chromosome_bin.push(chromosome);
    }
    fn chromosome_bin_pop(&mut self) -> Option<StaticMatrixChromosome> {
        self.chromosome_bin.pop().or_else(|| {
            panic!("genetic_algorithm error: chromosome capacity exceeded");
        })
    }
    // FIXME: directly set genes
    fn chromosome_constructor<R: Rng>(&mut self, rng: &mut R) -> StaticMatrixChromosome {
        let chromosome = self.chromosome_bin_pop().unwrap();
        let genes = self.random_genes_factory(rng);
        // let (x, _) = self.data.split_at_mut(chromosome.row_id);
        let x = self.data[chromosome.row_id].as_mut_slice();
        x.copy_from_slice(&genes);
        chromosome
    }
    fn chromosome_cloner(&mut self, chromosome: &StaticMatrixChromosome) -> StaticMatrixChromosome {
        if self.chromosome_recycling() && !self.chromosome_is_empty(chromosome) {
            if let Some(mut new_chromosome) = self.chromosome_bin_pop() {
                self.copy_genes_by_id(chromosome.row_id, new_chromosome.row_id);
                new_chromosome.age = chromosome.age;
                new_chromosome.fitness_score = chromosome.fitness_score;
                new_chromosome
            } else {
                chromosome.clone()
            }
        } else {
            chromosome.clone()
        }
    }
}

impl<
        T: Allele + Add<Output = T> + std::cmp::PartialOrd + Default,
        const N: usize,
        const M: usize,
    > Clone for StaticMatrix<T, N, M>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    fn clone(&self) -> Self {
        Self {
            data: [[T::default(); N]; M],
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
            best_genes: [T::default(); N],
        }
    }
}

impl<
        T: Allele + Add<Output = T> + std::cmp::PartialOrd + Default,
        const N: usize,
        const M: usize,
    > fmt::Debug for StaticMatrix<T, N, M>
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

impl<
        T: Allele + Add<Output = T> + std::cmp::PartialOrd + Default,
        const N: usize,
        const M: usize,
    > fmt::Display for StaticMatrix<T, N, M>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "genotype:")?;
        writeln!(f, "  genes_size: {}", self.genes_size)?;
        writeln!(f, "  allele_range: {:?}", self.allele_range)?;
        writeln!(
            f,
            "  allele_mutation_range: {:?}",
            self.allele_mutation_range
        )?;
        writeln!(
            f,
            "  allele_mutation_scaled_range: {:?}",
            self.allele_mutation_scaled_range
        )?;
        writeln!(f, "  mutation_type: {:?}", self.mutation_type)?;
        writeln!(f, "  chromosome_permutations_size: uncountable")?;
        writeln!(f, "  seed_genes_list: {:?}", self.seed_genes_list)
    }
}
