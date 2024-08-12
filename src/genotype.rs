//! The search space for the algorithm.
mod binary;
mod builder;
// mod continuous_f32;
mod continuous_t;
mod discrete;
mod multi_continuous;
mod multi_discrete;
mod multi_unique;
mod unique;

pub use self::binary::{Binary as BinaryGenotype, BinaryAllele};
pub use self::builder::{
    Builder as GenotypeBuilder, TryFromBuilderError as TryFromGenotypeBuilderError,
};
// pub use self::continuous_f32::{Continuous as ContinuousGenotype, ContinuousAllele};
pub use self::continuous_t::Continuous as ContinuousGenotype;
pub use self::discrete::Discrete as DiscreteGenotype;
pub use self::multi_continuous::MultiContinuous as MultiContinuousGenotype;
pub use self::multi_discrete::MultiDiscrete as MultiDiscreteGenotype;
pub use self::multi_unique::MultiUnique as MultiUniqueGenotype;
pub use self::unique::Unique as UniqueGenotype;

// pub use self::continuous::ContinuousAllele as ContinuousGenotypeAllele;
pub use self::multi_continuous::ContinuousAllele as MultiContinuousGenotypeAllele;

use crate::chromosome::Chromosome;
use crate::population::Population;
use itertools::Itertools;
use num::BigUint;
use rand::Rng;
use std::fmt;

/// Standard Allele, suitable for [Genotype]. Implemented for a set of primitives by default
pub trait Allele: Clone + Send + Sync + PartialEq + std::fmt::Debug
// use rand::distributions::uniform::SampleUniform;
// + SampleUniform
// Copy
// + Default
{
}
// pub trait DiscreteAllele: Allele + PartialEq {}
// pub trait ContinuousAllele: Allele + PartialEq + PartialOrd + Add<Output = Self> {}

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
impl Allele for String {}

/// Standard genotype, suitable for [Evolve](crate::strategy::evolve::Evolve).
/// Each implemented genotype handles its own random genes initialization and mutation.
pub trait Genotype:
    Clone + Send + Sync + fmt::Debug + fmt::Display + TryFrom<GenotypeBuilder<Self>>
{
    type Allele: Allele;
    fn genes_size(&self) -> usize;
    /// a chromosome factory to seed the initial population for [Evolve](crate::strategy::evolve::Evolve)
    /// random genes unless seed genes are provided
    fn chromosome_factory<R: Rng>(&self, rng: &mut R) -> Chromosome<Self::Allele>;
    /// a random genes factory (respecting seed genes)
    fn random_genes_factory<R: Rng>(&self, rng: &mut R) -> Vec<Self::Allele>;
    /// a random mutation of the chromosome
    fn mutate_chromosome_random<R: Rng>(
        &self,
        chromosome: &mut Chromosome<Self::Allele>,
        rng: &mut R,
    );
    /// a blanket neighbouring mutation fallback random mutation
    fn mutate_chromosome_neighbour<R: Rng>(
        &self,
        chromosome: &mut Chromosome<Self::Allele>,
        _scale: Option<f32>,
        rng: &mut R,
    ) {
        self.mutate_chromosome_random(chromosome, rng);
    }

    /// to guard against invalid crossover strategies which break the internal consistency
    /// of the genes, unique genotypes can't simply exchange genes without gene duplication issues
    fn crossover_points(&self) -> Vec<usize> {
        (0..self.genes_size()).collect()
    }
    /// to guard against invalid crossover strategies which break the internal consistency
    /// of the genes, unique genotypes can't simply exchange genes without gene duplication issues
    fn crossover_indexes(&self) -> Vec<usize> {
        (0..self.genes_size()).collect()
    }
    fn builder() -> GenotypeBuilder<Self> {
        GenotypeBuilder::<Self>::default()
    }
    fn set_seed_genes_list(&mut self, seed_genes_list: Vec<Vec<Self::Allele>>);
    fn seed_genes_list(&self) -> &Vec<Vec<Self::Allele>>;
}

//Evolvable is implicit, until proven otherwise
//pub trait EvolvableGenotype: Genotype {}

/// Genotype suitable for [HillClimb](crate::strategy::hill_climb::HillClimb). Need to implement a
/// neighbouring mutation and override the blanket mutate_chromosome_neighbour from the supertrait
/// Genotype
pub trait IncrementalGenotype: Genotype {
    /// all neighbouring mutations of the chromosome
    fn neighbouring_chromosomes(
        &self,
        _chromosome: &Chromosome<Self::Allele>,
        _scale: Option<f32>,
    ) -> Vec<Chromosome<Self::Allele>>;

    fn neighbouring_population(
        &self,
        chromosome: &Chromosome<Self::Allele>,
        scale: Option<f32>,
    ) -> Population<Self::Allele> {
        self.neighbouring_chromosomes(chromosome, scale).into()
    }
    /// chromosome neighbours size for the all possible neighbouring mutation combinations
    fn neighbouring_population_size(&self) -> BigUint;
}

/// Genotype suitable for [Permutate](crate::strategy::permutate::Permutate).
/// Not all genotypes are permutable, only countable ones (e.g. continuous genotypes cannot be permutated).
pub trait PermutableGenotype: Genotype {
    /// used for default chromosome_permutations_into_iter implementation
    fn allele_list_for_chromosome_permutations(&self) -> Vec<Self::Allele>;

    /// chromosome iterator for the all possible gene combinations for [Permutate](crate::strategy::permutate::Permutate)
    fn chromosome_permutations_into_iter<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = Chromosome<Self::Allele>> + 'a> {
        Box::new(
            (0..self.genes_size())
                .map(|_| self.allele_list_for_chromosome_permutations())
                .multi_cartesian_product()
                .map(|genes| Chromosome::new(genes)),
        )
    }

    /// chromosome iterator size for the all possible gene combinations for [Permutate](crate::strategy::permutate::Permutate)
    fn chromosome_permutations_size(&self) -> BigUint {
        BigUint::from(self.allele_list_for_chromosome_permutations().len())
            .pow(self.genes_size() as u32)
    }
}
