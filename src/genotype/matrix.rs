use super::builder::{Builder, TryFromBuilderError};
use super::{Allele, Genotype, IncrementalGenotype};
use crate::chromosome::Chromosome;
use crate::population::Population;
use itertools::Itertools;
use num::BigUint;
use rand::distributions::uniform::SampleUniform;
use rand::distributions::{Distribution, Uniform};
use rand::prelude::*;
use std::collections::HashSet;
use std::fmt;
use std::ops::{Add, Bound, RangeBounds, RangeInclusive};

#[derive(Copy, Clone, Debug)]
pub enum MutationType {
    Random,
    Relative,
    Scaled,
}

/// All matrices of nalgebra are stored in column-major order. This means that any two consecutive elements of a single matrix column will be contiguous in memory as well. Therefore a column will store a chromosome's genes
/// N = R = genes_size
/// M = C = population_size
pub struct Matrix<
    T: Allele + Add<Output = T> + std::cmp::PartialOrd + Default + 'static,
    const N: usize,
    const M: usize,
> where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    pub matrix: [[T; N]; M],
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
        T: Allele + Add<Output = T> + std::cmp::PartialOrd + Default + 'static,
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
                matrix: [[T::default(); N]; M],
                free_ids: HashSet::from_iter(0..M),
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
        T: Allele + Add<Output = T> + std::cmp::PartialOrd + Default + 'static,
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
        self.set_gene(
            chromosome.reference_id,
            index,
            self.allele_sampler.sample(rng),
        );
    }
    fn mutate_chromosome_index_relative<R: Rng>(
        &mut self,
        index: usize,
        chromosome: &mut Chromosome<Self>,
        rng: &mut R,
    ) {
        let value_diff = self.allele_relative_sampler.as_ref().unwrap().sample(rng);
        let new_value = self.get_gene(chromosome.reference_id, index) + value_diff;
        if new_value < *self.allele_range.start() {
            self.set_gene(chromosome.reference_id, index, *self.allele_range.start());
        } else if new_value > *self.allele_range.end() {
            self.set_gene(chromosome.reference_id, index, *self.allele_range.end());
        } else {
            self.set_gene(chromosome.reference_id, index, new_value);
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
        let new_value = self.get_gene(chromosome.reference_id, index) + value_diff;
        if new_value < *self.allele_range.start() {
            self.set_gene(chromosome.reference_id, index, *self.allele_range.start());
        } else if new_value > *self.allele_range.end() {
            self.set_gene(chromosome.reference_id, index, *self.allele_range.end());
        } else {
            self.set_gene(chromosome.reference_id, index, new_value);
        }
    }
    pub fn inspect_genes(&self, chromosome: &Chromosome<Self>) -> Vec<T> {
        (0..self.genes_size)
            .map(|i| self.get_gene(chromosome.reference_id, i))
            .collect()
    }

    pub fn reset_ids(&mut self) {
        self.free_ids.clear();
        (0..M).for_each(|i| {
            self.free_ids.insert(i);
        });
    }

    pub fn claim_id_forced(&mut self, id: usize) -> bool {
        self.free_ids.remove(&id)
    }
    pub fn claim_id(&mut self) -> usize {
        let id = *self.free_ids.iter().next().unwrap();
        self.free_ids.remove(&id);
        id
    }

    pub fn get_gene(&self, id: usize, index: usize) -> T {
        self.matrix[id][index]
    }
    pub fn set_gene(&mut self, id: usize, index: usize, value: T) {
        self.matrix[id][index] = value;
    }
    pub fn swap_gene(&mut self, father_id: usize, mother_id: usize, index: usize) {
        let (first, second) = (
            std::cmp::min(father_id, mother_id),
            std::cmp::max(father_id, mother_id),
        );
        let (_, tmp) = self.matrix.split_at_mut(first);
        let (x, rest) = tmp.split_at_mut(1);
        let (_, y) = rest.split_at_mut(second - first - 1);

        std::mem::swap(&mut x[0][index], &mut y[0][index]);
    }
    pub fn swap_gene_range<B: RangeBounds<usize>>(
        &mut self,
        father_id: usize,
        mother_id: usize,
        range: B,
    ) {
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

        let (first, second) = (
            std::cmp::min(father_id, mother_id),
            std::cmp::max(father_id, mother_id),
        );
        let (_, tmp) = self.matrix.split_at_mut(first);
        let (x, rest) = tmp.split_at_mut(1);
        let (_, y) = rest.split_at_mut(second - first - 1);

        (&mut x[0][min_index..max_index]).swap_with_slice(&mut y[0][min_index..max_index]);

        // let mother_back =
        //     &mut self.matrix.column_mut(mother.reference_id).as_mut_slice()[index..];
        // let father_back =
        //     &mut self.matrix.column_mut(father.reference_id).as_mut_slice()[index..];
        // father_back.swap_with_slice(mother_back);

        // let temp_slice = &mut self.vector.as_mut_slice();
        // let mother_col = &self.matrix.column(mother.reference_id).as_slice();
        // let mother_slice = &mother_col[index..];
        // temp_slice.copy_from_slice(mother_slice);
        //
        // let mother_slice =
        //     &mut self.matrix.column_mut(mother.reference_id).as_mut_slice()[index..];
        // let father_slice = &self.matrix.column(father.reference_id).as_slice()[index..];
        // mother_slice.copy_from_slice(father_slice);
        //
        // let father_slice =
        //     &mut self.matrix.column_mut(father.reference_id).as_mut_slice()[index..];
        // let temp_slice = &self.vector.as_slice();
        // father_slice.copy_from_slice(temp_slice);

        // for i in min_index..max_index {
        //     self.swap_gene(father_id, mother_id, i);
        // }
    }
    pub fn copy_gene_range<B: RangeBounds<usize>>(
        &mut self,
        source_id: usize,
        target_id: usize,
        range: B,
    ) {
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
        for i in min_index..max_index {
            self.set_gene(target_id, i, self.get_gene(source_id, i));
        }
    }
}

impl<
        T: Allele + Add<Output = T> + std::cmp::PartialOrd + Default + 'static,
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

    fn population_sync(&mut self, population: &mut Population<Self>) {
        // recycle ids
        self.reset_ids();
        population.chromosomes.iter_mut().for_each(|c| {
            if self.claim_id_forced(c.reference_id) {
                // first occurence, claim ID, use existing data
            } else {
                // it is a clone, copy data to new ID
                let new_id = self.claim_id();
                self.copy_gene_range(c.reference_id, new_id, ..);
                c.reference_id = new_id;
            }
        });
    }

    fn chromosome_factory<R: Rng>(&mut self, rng: &mut R) -> Chromosome<Self> {
        let id = self.claim_id();

        (0..self.genes_size).for_each(|i| self.set_gene(id, i, self.allele_sampler.sample(rng)));

        Chromosome {
            reference_id: id,
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
                    self.swap_gene(father.reference_id, mother.reference_id, index);
                });
        } else {
            rand::seq::index::sample(
                rng,
                self.genes_size(),
                number_of_crossovers.min(self.genes_size()),
            )
            .iter()
            .for_each(|index| {
                self.swap_gene(father.reference_id, mother.reference_id, index);
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
                    self.swap_gene_range(father.reference_id, mother.reference_id, index..);
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
                    self.swap_gene_range(
                        father.reference_id,
                        mother.reference_id,
                        start_index..end_index,
                    );
                }
                (Some(start_index), _) => {
                    self.swap_gene_range(father.reference_id, mother.reference_id, start_index..);
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
        T: Allele + Add<Output = T> + std::cmp::PartialOrd + Default + 'static,
        const N: usize,
        const M: usize,
    > Clone for Matrix<T, N, M>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    fn clone(&self) -> Self {
        Self {
            matrix: [[T::default(); N]; M],
            free_ids: HashSet::from_iter(0..M),
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
        T: Allele + Add<Output = T> + std::cmp::PartialOrd + Default + 'static,
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
        T: Allele + Add<Output = T> + std::cmp::PartialOrd + Default + 'static,
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
