//! The search space for the algorithm.
mod binary;
mod bit;
mod builder;
mod list;
mod matrix;
mod multi_list;
mod multi_range;
mod multi_unique;
mod range;
mod unique;

pub use self::binary::Binary as BinaryGenotype;
pub use self::bit::Bit as BitGenotype;
pub use self::builder::{
    Builder as GenotypeBuilder, TryFromBuilderError as TryFromGenotypeBuilderError,
};
pub use self::list::List as ListGenotype;
pub use self::matrix::Matrix as MatrixGenotype;
pub use self::multi_list::MultiList as MultiListGenotype;
pub use self::multi_range::MultiRange as MultiRangeGenotype;
pub use self::multi_unique::MultiUnique as MultiUniqueGenotype;
pub use self::range::Range as RangeGenotype;
pub use self::unique::Unique as UniqueGenotype;

use crate::chromosome::Chromosome;
use crate::population::Population;
use fixedbitset::FixedBitSet;
use impl_trait_for_tuples::impl_for_tuples;
use num::BigUint;
use rand::Rng;
use std::fmt;

/// Standard Allele, suitable for [Genotype]. Implemented for a set of primitives by default
#[impl_for_tuples(0, 12)]
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

/// Standard Genes, suitable for [Genotype]. Implemented for `Vec<Allele>` and [FixedBitSet]
pub trait Genes: Clone + Send + Sync + std::fmt::Debug {}
impl<T: Allele> Genes for Vec<T> {}
impl Genes for FixedBitSet {}

/// Standard genotype, suitable for [Evolve](crate::strategy::evolve::Evolve).
/// Each implemented genotype handles its own random genes initialization and mutation.
pub trait Genotype:
    Clone + Send + Sync + fmt::Debug + fmt::Display + TryFrom<GenotypeBuilder<Self>>
{
    type Allele: Allele;
    type Genes: Genes;

    fn genes_size(&self) -> usize;
    /// a chromosome factory to seed the initial population for [Evolve](crate::strategy::evolve::Evolve)
    /// random genes unless seed genes are provided
    fn chromosome_factory<R: Rng>(&self, rng: &mut R) -> Chromosome<Self>;
    /// a random genes factory (respecting seed genes)
    fn random_genes_factory<R: Rng>(&self, rng: &mut R) -> Self::Genes;
    // functionally invalid placeholder
    fn chromosome_factory_empty(&self) -> Chromosome<Self>;
    // test for functionally invalid placeholder
    fn chromosome_is_empty(&self, chromosome: &Chromosome<Self>) -> bool;
    /// Mutate the chromosome, the genotype determines whether this is random, relative or scaled.
    /// Choose between allowing duplicates or not (~2x slower).
    fn mutate_chromosome_genes<R: Rng>(
        &self,
        number_of_mutations: usize,
        allow_duplicates: bool,
        chromosome: &mut Chromosome<Self>,
        scale_index: Option<usize>,
        rng: &mut R,
    );

    /// Crossover genes between a pair of chromosomes.
    /// Choose between allowing duplicates or not (~2x slower).
    /// panics if there are no valid crossover indexes
    fn crossover_chromosome_genes<R: Rng>(
        &self,
        number_of_crossovers: usize,
        allow_duplicates: bool,
        father: &mut Chromosome<Self>,
        mother: &mut Chromosome<Self>,
        rng: &mut R,
    );
    /// Crossover points between a pair of chromosomes.
    /// Choose between allowing duplicates or not (not much slower)
    /// panics if there are no valid crossover points
    fn crossover_chromosome_points<R: Rng>(
        &self,
        number_of_crossovers: usize,
        allow_duplicates: bool,
        father: &mut Chromosome<Self>,
        mother: &mut Chromosome<Self>,
        rng: &mut R,
    );
    /// to guard against invalid crossover strategies which break the internal consistency
    /// of the genes, unique genotypes can't simply exchange genes without gene duplication issues
    fn has_crossover_indexes(&self) -> bool {
        false
    }
    /// to guard against invalid crossover strategies which break the internal consistency
    /// of the genes, unique genotypes can't simply exchange genes without gene duplication issues
    fn has_crossover_points(&self) -> bool {
        false
    }
    fn builder() -> GenotypeBuilder<Self> {
        GenotypeBuilder::<Self>::default()
    }
    fn set_seed_genes_list(&mut self, seed_genes_list: Vec<Self::Genes>);
    fn seed_genes_list(&self) -> &Vec<Self::Genes>;
    fn max_scale_index(&self) -> Option<usize>;
    // drop unreferenced IDs and handle cloned IDs (clone and new ID)
    fn population_sync(&mut self, _population: &mut Population<Self>) {}
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
