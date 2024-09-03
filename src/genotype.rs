//! The search space for the algorithm.
mod binary;
mod builder;
mod list;
mod multi_list;
mod multi_range;
mod multi_unique;
mod range;
mod unique;

pub use self::binary::{Binary as BinaryGenotype, BinaryAllele};
pub use self::builder::{
    Builder as GenotypeBuilder, TryFromBuilderError as TryFromGenotypeBuilderError,
};
pub use self::list::List as ListGenotype;
pub use self::multi_list::MultiList as MultiListGenotype;
pub use self::multi_range::MultiRange as MultiRangeGenotype;
pub use self::multi_unique::MultiUnique as MultiUniqueGenotype;
pub use self::range::Range as RangeGenotype;
pub use self::unique::Unique as UniqueGenotype;

use crate::chromosome::Chromosome;
use crate::population::Population;
use fixedbitset::FixedBitSet;
use impl_trait_for_tuples::impl_for_tuples;
use itertools::Itertools;
use num::BigUint;
use rand::distributions::{Distribution, Uniform};
use rand::Rng;
use std::fmt;

/// Standard Allele, suitable for [Genotype]. Implemented for a set of primitives by default
#[impl_for_tuples(1, 12)]
pub trait Allele: Clone + Copy + Send + Sync + std::fmt::Debug
// use rand::distributions::uniform::SampleUniform;
// + SampleUniform
// Copy
// + Default
{
}
// pub trait ListAllele: Allele + PartialEq {}
// pub trait RangeAllele: Allele + PartialEq + PartialOrd + Add<Output = Self> {}

impl Allele for bool {}
impl Allele for char {}
impl Allele for f32 {}
impl Allele for f64 {}
impl Allele for i128 {}
impl Allele for i16 {}
impl Allele for i32 {}
impl Allele for i64 {}
impl Allele for i8 {}
impl Allele for isize {}
impl Allele for u128 {}
impl Allele for u16 {}
impl Allele for u32 {}
impl Allele for u64 {}
impl Allele for u8 {}
impl Allele for usize {}

pub trait Genes: Clone + Send + Sync + std::fmt::Debug {}
impl<T: Allele> Genes for Vec<T> {}
impl Genes for FixedBitSet {}

/// Standard genotype, suitable for [Evolve](crate::strategy::evolve::Evolve).
/// Each implemented genotype handles its own random genes initialization and mutation.
pub trait Genotype:
    Clone + Send + Sync + fmt::Debug + fmt::Display + TryFrom<GenotypeBuilder<Self>>
{
    type Allele: Allele;
    fn genes_size(&self) -> usize;
    /// a chromosome factory to seed the initial population for [Evolve](crate::strategy::evolve::Evolve)
    /// random genes unless seed genes are provided
    fn chromosome_factory<R: Rng>(&self, rng: &mut R) -> Chromosome<Self>;
    /// a random genes factory (respecting seed genes)
    fn random_genes_factory<R: Rng>(&self, rng: &mut R) -> Vec<Self::Allele>;
    /// a single mutation of the chromosome, the genotype determines whether this is random, relative or scaled.
    fn mutate_chromosome_single<R: Rng>(
        &self,
        chromosome: &mut Chromosome<Self>,
        scale_index: Option<usize>,
        rng: &mut R,
    );
    /// multiple mutations of the chromosome, the genotype determines whether this is random, relative or scaled.
    fn mutate_chromosome_multi<R: Rng>(
        &self,
        number_of_mutations: usize,
        allow_duplicates: bool,
        chromosome: &mut Chromosome<Self>,
        scale_index: Option<usize>,
        rng: &mut R,
    );

    /// a crossover of a single gene between a pair of chromosomes
    /// panics if there are no valid crossover indexes
    fn crossover_chromosome_pair_single_gene<R: Rng>(
        &self,
        father: &mut Chromosome<Self>,
        mother: &mut Chromosome<Self>,
        rng: &mut R,
    ) {
        let index = self.crossover_index_sampler().unwrap().sample(rng);
        std::mem::swap(&mut father.genes[index], &mut mother.genes[index]);
        mother.taint_fitness_score();
        father.taint_fitness_score();
    }
    /// a crossover of a multi gene between a pair of chromosomes.
    /// Choose between allowing duplicates or not (~2x slower).
    /// panics if there are no valid crossover indexes
    fn crossover_chromosome_pair_multi_gene<R: Rng>(
        &self,
        number_of_crossovers: usize,
        allow_duplicates: bool,
        father: &mut Chromosome<Self>,
        mother: &mut Chromosome<Self>,
        rng: &mut R,
    ) {
        let sampler = self.crossover_index_sampler().unwrap(); // trigger panic for no duplicates branch
        if allow_duplicates {
            rng.sample_iter(sampler)
                .take(number_of_crossovers)
                .for_each(|index| {
                    std::mem::swap(&mut father.genes[index], &mut mother.genes[index]);
                });
        } else {
            // assume all genes are valid indexes, handle otherwise in trait implmentaiton
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
    /// a crossover of a single point between a pair of chromosomes
    /// panics if there are no valid crossover points
    fn crossover_chromosome_pair_single_point<R: Rng>(
        &self,
        father: &mut Chromosome<Self>,
        mother: &mut Chromosome<Self>,
        rng: &mut R,
    ) {
        let index = self.crossover_point_sampler().unwrap().sample(rng);

        let mother_back = &mut mother.genes[index..];
        let father_back = &mut father.genes[index..];
        father_back.swap_with_slice(mother_back);

        mother.taint_fitness_score();
        father.taint_fitness_score();
    }
    /// a crossover of a multi point between a pair of chromosomes.
    /// Choose between allowing duplicates or not (not much slower)
    /// panics if there are no valid crossover points
    fn crossover_chromosome_pair_multi_point<R: Rng>(
        &self,
        number_of_crossovers: usize,
        allow_duplicates: bool,
        father: &mut Chromosome<Self>,
        mother: &mut Chromosome<Self>,
        rng: &mut R,
    ) {
        let sampler = self.crossover_point_sampler().unwrap(); // trigger panic for no duplicates branch
        if allow_duplicates {
            rng.sample_iter(sampler)
                .take(number_of_crossovers)
                .for_each(|index| {
                    let mother_back = &mut mother.genes[index..];
                    let father_back = &mut father.genes[index..];
                    father_back.swap_with_slice(mother_back);
                });
        } else {
            // assume all genes are valid points, handle otherwise in trait implmentaiton
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
    fn crossover_index_sampler(&self) -> Option<&Uniform<usize>> {
        None
    }
    fn crossover_point_sampler(&self) -> Option<&Uniform<usize>> {
        None
    }
    /// to guard against invalid crossover strategies which break the internal consistency
    /// of the genes, unique genotypes can't simply exchange genes without gene duplication issues
    fn has_crossover_indexes(&self) -> bool {
        self.crossover_index_sampler().is_some()
    }
    /// to guard against invalid crossover strategies which break the internal consistency
    /// of the genes, unique genotypes can't simply exchange genes without gene duplication issues
    fn has_crossover_points(&self) -> bool {
        self.crossover_point_sampler().is_some()
    }
    fn builder() -> GenotypeBuilder<Self> {
        GenotypeBuilder::<Self>::default()
    }
    fn set_seed_genes_list(&mut self, seed_genes_list: Vec<Vec<Self::Allele>>);
    fn seed_genes_list(&self) -> &Vec<Vec<Self::Allele>>;
    fn max_scale_index(&self) -> Option<usize>;
    fn expected_number_of_sampled_index_collisions(&self, number_of_samples: usize) -> usize {
        number_of_samples * (number_of_samples - 1) / (2 * self.genes_size())
    }
}

//Evolvable is implicit, until proven otherwise
//pub trait EvolvableGenotype: Genotype {}

/// Genotype suitable for [HillClimb](crate::strategy::hill_climb::HillClimb).
pub trait IncrementalGenotype: Genotype {
    /// all neighbouring mutations of the chromosome
    /// used in HillClimbVariant::SteepestAscent and SteepestAscentSecondary
    fn neighbouring_chromosomes<R: Rng>(
        &self,
        _chromosome: &Chromosome<Self>,
        _scale_index: Option<usize>,
        _rng: &mut R,
    ) -> Vec<Chromosome<Self>>;

    fn neighbouring_population<R: Rng>(
        &self,
        chromosome: &Chromosome<Self>,
        scale_index: Option<usize>,
        rng: &mut R,
    ) -> Population<Self> {
        self.neighbouring_chromosomes(chromosome, scale_index, rng)
            .into()
    }
    /// chromosome neighbours size for the all possible neighbouring mutation combinations
    fn neighbouring_population_size(&self) -> BigUint;
}

/// Genotype suitable for [Permutate](crate::strategy::permutate::Permutate).
/// Not all genotypes are permutable, only countable ones (e.g. range genotypes cannot be permutated).
pub trait PermutableGenotype: Genotype {
    /// chromosome iterator for the all possible gene combinations for [Permutate](crate::strategy::permutate::Permutate)
    fn chromosome_permutations_into_iter(&self) -> impl Iterator<Item = Chromosome<Self>> + Send;

    /// chromosome iterator size for the all possible gene combinations for [Permutate](crate::strategy::permutate::Permutate)
    fn chromosome_permutations_size(&self) -> BigUint;
}
