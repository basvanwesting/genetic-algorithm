use super::builder::{Builder, TryFromBuilderError};
use super::{EvolveGenotype, Genotype, HillClimbGenotype};
use crate::centralized::chromosome::{
    Chromosome, ChromosomeManager, GenesHash, GenesPointer, StaticBinaryChromosome,
};
use crate::centralized::fitness::FitnessValue;
use crate::centralized::population::Population;
use itertools::Itertools;
use num::BigUint;
use rand::distributions::Uniform;
use rand::prelude::*;
use rustc_hash::FxHasher;
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::{Bound, Range, RangeBounds};

/// Genes (`N`) and Population (`M`) are a fixed `N*M` matrix of boolean values, stored on the heap as a
/// nested array `Box<[[bool; N]; M]>`. The genes are contiguous in memory, with an `N` jump to the next
/// chromosome (`[[bool; N]; M]` can be treated like `[bool; N * M]` in memory). The genes are therefore not
/// stored on the Chromosomes themselves, which just point to the data (chromosome.row_id ==
/// row id of the matrix). The genes_size can be smaller than N, which would just leave a part of
/// the matrix unused at false. This opens the possibility for linear algebra fitness
/// calculations on the whole population at once, possibly using the GPU in the future (if the data
/// is stored and mutated at a GPU readable memory location). The fitness would then implement
/// [calculate_for_population](crate::fitness::Fitness::calculate_for_population) instead of
/// [calculate_for_chromosome](crate::fitness::Fitness::calculate_for_chromosome).
///
/// The rest is like [BinaryGenotype](super::BinaryGenotype).
///
/// On random initialization, each gene has a 50% probability of becoming true or false.
/// Each gene has an equal probability of mutating. If a gene mutates, its value is flipped.
///
/// # Panics
///
/// Will panic if more chromosomes are instantiated than the population (M) allows. M should
/// account for the target_population_size and the crossover selection_rate which adds offspring on
/// top of that.
///
/// # Example:
/// ```
/// use genetic_algorithm::centralized::genotype::{Genotype, StaticBinaryGenotype};
///
/// const GENES_SIZE: usize = 100;
/// const POPULATION_SIZE: usize = 200;
///
/// let genotype = StaticBinaryGenotype::<GENES_SIZE, POPULATION_SIZE>::builder()
///     .with_genes_size(100)
///     .with_genes_hashing(false) // optional, defaults to false
///     .build()
///     .unwrap();
/// ```
pub struct StaticBinary<const N: usize, const M: usize> {
    pub data: Box<[[bool; N]; M]>,
    pub genes_size: usize,
    gene_index_sampler: Uniform<usize>,
    pub seed_genes_list: Vec<Box<[bool; N]>>,
    pub chromosome_bin: Vec<StaticBinaryChromosome>,
    pub best_genes: Box<[bool; N]>,
    pub genes_hashing: bool,
}

impl<const N: usize, const M: usize> TryFrom<Builder<Self>> for StaticBinary<N, M> {
    type Error = TryFromBuilderError;

    fn try_from(builder: Builder<Self>) -> Result<Self, Self::Error> {
        if !builder.genes_size.is_some_and(|x| x > 0) {
            Err(TryFromBuilderError(
                "StaticBinaryGenotype requires a genes_size > 0",
            ))
        } else {
            let genes_size = builder.genes_size.unwrap();

            Ok(Self {
                data: Box::new([[false; N]; M]),
                genes_size,
                gene_index_sampler: Uniform::from(0..genes_size),
                seed_genes_list: builder.seed_genes_list,
                chromosome_bin: Vec::with_capacity(M),
                best_genes: Box::new([false; N]),
                genes_hashing: builder.genes_hashing,
            })
        }
    }
}

impl<const N: usize, const M: usize> StaticBinary<N, M> {
    /// returns a slice of genes_size <= N
    fn get_genes_by_id(&self, id: usize) -> &[bool] {
        &self.data[id][..self.genes_size]
    }

    // fn get_gene_by_id(&self, id: usize, index: usize) -> bool {
    //     self.data[id][index]
    // }
    //
    // fn set_gene_by_id(&mut self, id: usize, index: usize, value: bool) {
    //     self.data[id][index] = value;
    // }

    fn flip_gene_by_id(&mut self, id: usize, index: usize) {
        self.data[id][index] = !self.data[id][index];
    }

    fn copy_genes_by_id(&mut self, source_id: usize, target_id: usize) {
        let (source, target) = self.gene_slice_pair_mut((source_id, target_id));
        (target).copy_from_slice(&source[..]);
    }

    fn swap_gene_by_id(&mut self, father_id: usize, mother_id: usize, index: usize) {
        let (father, mother) = self.gene_slice_pair_mut((father_id, mother_id));
        std::mem::swap(&mut father[index], &mut mother[index]);
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

    fn gene_slice_pair_mut(&mut self, ids: (usize, usize)) -> (&mut [bool; N], &mut [bool; N]) {
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

impl<const N: usize, const M: usize> Genotype for StaticBinary<N, M> {
    type Allele = bool;
    type Genes = Box<[bool; N]>;
    type Chromosome = StaticBinaryChromosome;

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
            self.genes_slice(chromosome).hash(&mut s);
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

    fn mutate_chromosome_genes<R: Rng>(
        &mut self,
        number_of_mutations: usize,
        allow_duplicates: bool,
        chromosome: &mut Self::Chromosome,
        _scale_index: Option<usize>,
        rng: &mut R,
    ) {
        if allow_duplicates {
            for _ in 0..number_of_mutations {
                let index = self.gene_index_sampler.sample(rng);
                self.flip_gene_by_id(chromosome.row_id, index);
            }
        } else {
            rand::seq::index::sample(
                rng,
                self.genes_size,
                number_of_mutations.min(self.genes_size),
            )
            .iter()
            .for_each(|index| {
                self.flip_gene_by_id(chromosome.row_id, index);
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
        None
    }
}

impl<const N: usize, const M: usize> EvolveGenotype for StaticBinary<N, M> {
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

impl<const N: usize, const M: usize> HillClimbGenotype for StaticBinary<N, M> {
    fn fill_neighbouring_population<R: Rng>(
        &mut self,
        chromosome: &Self::Chromosome,
        population: &mut Population<Self::Chromosome>,
        _scale_index: Option<usize>,
        _rng: &mut R,
    ) {
        (0..self.genes_size).for_each(|index| {
            let mut new_chromosome = self.chromosome_cloner(chromosome);
            self.flip_gene_by_id(new_chromosome.row_id, index);
            self.reset_chromosome_state(&mut new_chromosome);
            population.chromosomes.push(new_chromosome);
        });
    }

    fn neighbouring_population_size(&self) -> BigUint {
        BigUint::from(self.genes_size)
    }
}

impl<const N: usize, const M: usize> ChromosomeManager<Self> for StaticBinary<N, M> {
    fn random_genes_factory<R: Rng>(&self, rng: &mut R) -> Box<[bool; N]> {
        if self.seed_genes_list.is_empty() {
            Box::new(std::array::from_fn(|_| rng.gen()))
        } else {
            self.seed_genes_list.choose(rng).unwrap().clone()
        }
    }

    fn set_genes(&mut self, chromosome: &mut StaticBinaryChromosome, genes: &Box<[bool; N]>) {
        let x = self.data[chromosome.row_id].as_mut_slice();
        x.copy_from_slice(genes.as_slice());
        self.reset_chromosome_state(chromosome);
    }

    fn get_genes(&self, chromosome: &StaticBinaryChromosome) -> Box<[bool; N]> {
        Box::new(self.data[chromosome.row_id])
    }

    fn copy_genes(&mut self, source: &StaticBinaryChromosome, target: &mut StaticBinaryChromosome) {
        self.copy_genes_by_id(source.row_id, target.row_id);
        self.copy_chromosome_state(source, target);
    }

    fn chromosomes_setup(&mut self) {
        self.chromosome_bin = (0..M).rev().map(StaticBinaryChromosome::new).collect();
    }

    fn chromosome_bin_push(&mut self, chromosome: StaticBinaryChromosome) {
        self.chromosome_bin.push(chromosome);
    }

    fn chromosome_bin_find_or_create(&mut self) -> StaticBinaryChromosome {
        self.chromosome_bin.pop().unwrap_or_else(|| {
            panic!("genetic_algorithm error: chromosome capacity exceeded");
        })
    }

    fn chromosomes_cleanup(&mut self) {
        std::mem::take(&mut self.chromosome_bin);
    }
}

impl<const N: usize, const M: usize> Clone for StaticBinary<N, M> {
    fn clone(&self) -> Self {
        Self {
            data: Box::new([[false; N]; M]),
            chromosome_bin: Vec::with_capacity(M),
            genes_size: self.genes_size,
            gene_index_sampler: self.gene_index_sampler,
            seed_genes_list: self.seed_genes_list.clone(),
            best_genes: Box::new([false; N]),
            genes_hashing: self.genes_hashing,
        }
    }
}

impl<const N: usize, const M: usize> fmt::Debug for StaticBinary<N, M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StaticBinary")
            .field("genes_size", &self.genes_size)
            .field("seed_genes_list", &self.seed_genes_list)
            .finish()
    }
}

impl<const N: usize, const M: usize> fmt::Display for StaticBinary<N, M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "genotype:")?;
        writeln!(f, "  genes_size: {}", self.genes_size)?;
        writeln!(f, "  mutation_type: {:?}", self.mutation_type())?;
        writeln!(f, "  chromosome_permutations_size: unsupported")?;
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
