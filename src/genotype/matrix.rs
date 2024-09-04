use super::builder::{Builder, TryFromBuilderError};
use super::{Allele, Genotype, IncrementalGenotype};
use crate::chromosome::Chromosome;
use itertools::Itertools;
use nalgebra::SMatrix;
use num::{BigUint, Zero};
use rand::distributions::uniform::SampleUniform;
use rand::distributions::{Distribution, Uniform};
use rand::prelude::*;
use std::collections::HashSet;
use std::fmt;
use std::ops::{Add, RangeInclusive};

#[derive(Copy, Clone, Debug)]
pub enum MutationType {
    Random,
    Relative,
    Scaled,
}

/// All matrices of nalgebra are stored in column-major order. This means that any two consecutive elements of a single matrix column will be contiguous in memory as well. Therefore a column will store a chromosome's genes
/// N = R = genes_size
/// M = C = population_size
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
///     .build()
///     .unwrap();
/// ```
pub struct Matrix<
    T: Allele + Add<Output = T> + std::cmp::PartialOrd + Zero + 'static,
    const N: usize,
    const M: usize,
> where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    pub matrix: SMatrix<T, N, M>,
    pub free_ids: HashSet<usize>,
    pub genes_size: usize,
    pub allele_range: RangeInclusive<T>,
    pub allele_mutation_range: Option<RangeInclusive<T>>,
    pub allele_mutation_scaled_range: Option<Vec<RangeInclusive<T>>>,
    pub mutation_type: MutationType,
    gene_index_sampler: Uniform<usize>,
    allele_sampler: Uniform<T>,
    allele_relative_sampler: Option<Uniform<T>>,
    pub seed_genes_list: Vec<()>,
}

impl<
        T: Allele + Add<Output = T> + std::cmp::PartialOrd + Zero + 'static,
        const N: usize,
        const M: usize,
    > TryFrom<Builder<Self>> for Matrix<T, N, M>
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
                matrix: SMatrix::<T, N, M>::zeros(),
                free_ids: HashSet::from_iter(0..N),
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
            })
        }
    }
}

impl<
        T: Allele + Add<Output = T> + std::cmp::PartialOrd + Zero + 'static,
        const N: usize,
        const M: usize,
    > Matrix<T, N, M>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    fn mutate_chromosome_index_random<R: Rng>(
        &mut self,
        index: usize,
        chromosome: &mut Chromosome<Self>,
        rng: &mut R,
    ) {
        self.matrix[(index, chromosome.reference_id)] = self.allele_sampler.sample(rng);
    }
    fn mutate_chromosome_index_relative<R: Rng>(
        &mut self,
        index: usize,
        chromosome: &mut Chromosome<Self>,
        rng: &mut R,
    ) {
        let value_diff = self.allele_relative_sampler.as_ref().unwrap().sample(rng);
        let new_value = self.matrix[(index, chromosome.reference_id)] + value_diff;
        if new_value < *self.allele_range.start() {
            self.matrix[(index, chromosome.reference_id)] = *self.allele_range.start();
        } else if new_value > *self.allele_range.end() {
            self.matrix[(index, chromosome.reference_id)] = *self.allele_range.end();
        } else {
            self.matrix[(index, chromosome.reference_id)] = new_value;
        }
    }
    fn mutate_chromosome_index_scaled<R: Rng>(
        &mut self,
        index: usize,
        chromosome: &mut Chromosome<Self>,
        scale_index: usize,
        rng: &mut R,
    ) {
        let working_range = &self.allele_mutation_scaled_range.as_ref().unwrap()[scale_index];
        let value_diff = if rng.gen() {
            *working_range.start()
        } else {
            *working_range.end()
        };
        let new_value = self.matrix[(index, chromosome.reference_id)] + value_diff;
        if new_value < *self.allele_range.start() {
            self.matrix[(index, chromosome.reference_id)] = *self.allele_range.start();
        } else if new_value > *self.allele_range.end() {
            self.matrix[(index, chromosome.reference_id)] = *self.allele_range.end();
        } else {
            self.matrix[(index, chromosome.reference_id)] = new_value;
        }
    }
    pub fn inspect_genes(&self, chromosome: &Chromosome<Self>) -> Vec<T> {
        (0..self.genes_size)
            .map(|i| self.matrix[(i, chromosome.reference_id)])
            .collect()
    }
}

impl<
        T: Allele + Add<Output = T> + std::cmp::PartialOrd + Zero + 'static,
        const N: usize,
        const M: usize,
    > Genotype for Matrix<T, N, M>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    type Allele = T;
    type Genes = ();

    fn genes_size(&self) -> usize {
        self.genes_size
    }

    fn chromosome_factory<R: Rng>(&mut self, rng: &mut R) -> Chromosome<Self> {
        let free_id = *self.free_ids.iter().next().unwrap();
        self.free_ids.remove(&free_id);

        (0..self.genes_size)
            .for_each(|i| self.matrix[(i, free_id)] = self.allele_sampler.sample(rng));

        Chromosome {
            reference_id: free_id,
            genes: (),
            fitness_score: None,
            age: 0,
        }
    }
    fn chromosome_factory_empty(&self) -> Chromosome<Self> {
        Chromosome {
            reference_id: usize::MAX,
            genes: (),
            fitness_score: None,
            age: 0,
        }
    }
    fn chromosome_is_empty(&self, chromosome: &Chromosome<Self>) -> bool {
        chromosome.reference_id == usize::MAX
    }

    fn mutate_chromosome_genes<R: Rng>(
        &mut self,
        number_of_mutations: usize,
        allow_duplicates: bool,
        chromosome: &mut Chromosome<Self>,
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
        father: &mut Chromosome<Self>,
        mother: &mut Chromosome<Self>,
        rng: &mut R,
    ) {
        if allow_duplicates {
            rng.sample_iter(self.gene_index_sampler)
                .take(number_of_crossovers)
                .for_each(|index| {
                    self.matrix
                        .swap((index, father.reference_id), (index, mother.reference_id));
                });
        } else {
            rand::seq::index::sample(
                rng,
                self.genes_size(),
                number_of_crossovers.min(self.genes_size()),
            )
            .iter()
            .for_each(|index| {
                self.matrix
                    .swap((index, father.reference_id), (index, mother.reference_id));
            });
        }
        mother.taint_fitness_score();
        father.taint_fitness_score();
    }
    fn crossover_chromosome_points<R: Rng>(
        &mut self,
        number_of_crossovers: usize,
        allow_duplicates: bool,
        father: &mut Chromosome<Self>,
        mother: &mut Chromosome<Self>,
        rng: &mut R,
    ) {
        if allow_duplicates {
            rng.sample_iter(self.gene_index_sampler)
                .take(number_of_crossovers)
                .for_each(|index| {
                    // let mother_back =
                    //     &mut self.matrix.column_mut(mother.reference_id).as_mut_slice()[index..];
                    // let father_back =
                    //     &mut self.matrix.column_mut(father.reference_id).as_mut_slice()[index..];
                    // father_back.swap_with_slice(mother_back);
                    for i in index..self.genes_size() {
                        self.matrix
                            .swap((i, father.reference_id), (i, mother.reference_id));
                    }
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
                    for i in start_index..end_index {
                        self.matrix
                            .swap((i, father.reference_id), (i, mother.reference_id));
                    }
                }
                (Some(start_index), _) => {
                    for i in start_index..self.genes_size() {
                        self.matrix
                            .swap((i, father.reference_id), (i, mother.reference_id));
                    }
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
        T: Allele + Add<Output = T> + std::cmp::PartialOrd + Zero + 'static,
        const N: usize,
        const M: usize,
    > Clone for Matrix<T, N, M>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    fn clone(&self) -> Self {
        Self {
            matrix: SMatrix::<T, N, M>::zeros(),
            free_ids: HashSet::from_iter(0..N),
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
        }
    }
}

impl<
        T: Allele + Add<Output = T> + std::cmp::PartialOrd + Zero + 'static,
        const N: usize,
        const M: usize,
    > fmt::Debug for Matrix<T, N, M>
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
        T: Allele + Add<Output = T> + std::cmp::PartialOrd + Zero + 'static,
        const N: usize,
        const M: usize,
    > fmt::Display for Matrix<T, N, M>
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
