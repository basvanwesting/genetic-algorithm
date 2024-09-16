//! The search space for the algorithm.
mod binary;
mod bit;
mod builder;
mod dynamic_matrix;
mod list;
mod multi_list;
mod multi_range;
mod multi_unique;
mod range;
mod static_matrix;
mod unique;

pub use self::binary::Binary as BinaryGenotype;
pub use self::bit::Bit as BitGenotype;
pub use self::builder::{
    Builder as GenotypeBuilder, TryFromBuilderError as TryFromGenotypeBuilderError,
};
pub use self::dynamic_matrix::DynamicMatrix as DynamicMatrixGenotype;
pub use self::list::List as ListGenotype;
pub use self::multi_list::MultiList as MultiListGenotype;
pub use self::multi_range::MultiRange as MultiRangeGenotype;
pub use self::multi_unique::MultiUnique as MultiUniqueGenotype;
pub use self::range::Range as RangeGenotype;
pub use self::static_matrix::StaticMatrix as StaticMatrixGenotype;
pub use self::unique::Unique as UniqueGenotype;

pub use crate::allele::{Allele, RangeAllele};
use crate::chromosome::{Chromosome, ChromosomeManager};
use crate::fitness::FitnessValue;
use crate::population::Population;
use fixedbitset::FixedBitSet;
use itertools::Itertools;
use num::BigUint;
use rand::Rng;
use std::fmt;

/// Standard Genes, suitable for [Genotype]. Implemented for `Vec<Allele>` and [FixedBitSet]
pub trait Genes: Clone + Send + Sync + std::fmt::Debug {}
impl<T: Allele> Genes for Vec<T> {}
impl Genes for FixedBitSet {}
impl Genes for () {}
impl<T: Allele, const N: usize> Genes for [T; N] {}
impl<T: Allele, const N: usize> Genes for Box<[T; N]> {}

#[derive(Copy, Clone, Debug, Default)]
pub enum MutationType {
    #[default]
    Random,
    Relative,
    Scaled,
}

/// Standard genotype, suitable for [Evolve](crate::strategy::evolve::Evolve).
/// Each implemented genotype handles its own random genes initialization and mutation.
pub trait Genotype:
    ChromosomeManager<Self>
    + Clone
    + Send
    + Sync
    + fmt::Debug
    + fmt::Display
    + TryFrom<GenotypeBuilder<Self>>
{
    type Allele: Allele;
    type Genes: Genes;
    type Chromosome: Chromosome;

    fn genes_size(&self) -> usize;
    fn save_best_genes(&mut self, chromosome: &Self::Chromosome);
    fn load_best_genes(&mut self, chromosome: &mut Self::Chromosome);
    fn best_genes(&self) -> &Self::Genes;
    fn best_genes_slice(&self) -> &[Self::Allele];
    fn genes_slice<'a>(&'a self, chromosome: &'a Self::Chromosome) -> &'a [Self::Allele];

    fn update_population_fitness_scores(
        &self,
        _population: &mut Population<Self::Chromosome>,
        _fitness_scores: Vec<Option<FitnessValue>>,
    ) {
        // TODO: we could default to the assumption that population and fitness_scores just align in
        // length and order. But we don't want to encourage this usage. This should be used for
        // GenesPointer chromosomes only
        // population
        //     .chromosomes
        //     .iter_mut()
        //     .zip(fitness_scores)
        //     .for_each(|(chromosome, fitness_score)| {
        //         chromosome.set_fitness_score(fitness_score);
        //     });
        panic!("The genotype does not suppport overwriting the Fitness::calculate_for_population implementation");
    }

    fn mutate_chromosome_genes<R: Rng>(
        &mut self,
        number_of_mutations: usize,
        allow_duplicates: bool,
        chromosome: &mut Self::Chromosome,
        scale_index: Option<usize>,
        rng: &mut R,
    );

    fn builder() -> GenotypeBuilder<Self> {
        GenotypeBuilder::<Self>::default()
    }
    fn set_seed_genes_list(&mut self, seed_genes_list: Vec<Self::Genes>);
    fn seed_genes_list(&self) -> &Vec<Self::Genes>;
    fn max_scale_index(&self) -> Option<usize>;

    fn expected_number_of_sampled_index_duplicates(&self, number_of_samples: usize) -> usize {
        if number_of_samples > 1 {
            number_of_samples * (number_of_samples - 1) / (2 * self.genes_size())
        } else {
            0
        }
    }
    fn expected_number_of_sampled_index_duplicates_report(&self) -> String {
        [
            self.genes_size() / 256,
            self.genes_size() / 128,
            self.genes_size() / 64,
            self.genes_size() / 32,
            self.genes_size() / 16,
            self.genes_size() / 8,
            self.genes_size() / 4,
            self.genes_size() / 2,
        ]
        .iter()
        .map(|number_of_samples| {
            (
                number_of_samples,
                self.expected_number_of_sampled_index_duplicates(*number_of_samples),
            )
        })
        .filter(|(_, c)| *c > 0)
        .map(|(n, e)| format!("{} => {}", n, e))
        .join(", ")
    }
}

/// Genotype suitable for [Evolve](crate::strategy::evolve::Evolve).
pub trait EvolveGenotype: Genotype {
    /// Crossover genes between a pair of chromosomes.
    /// Choose between allowing duplicates or not (~2x slower).
    /// panics if there are no valid crossover indexes
    fn crossover_chromosome_genes<R: Rng>(
        &mut self,
        number_of_crossovers: usize,
        allow_duplicates: bool,
        father: &mut Self::Chromosome,
        mother: &mut Self::Chromosome,
        rng: &mut R,
    );
    /// Crossover points between a pair of chromosomes.
    /// Choose between allowing duplicates or not (not much slower)
    /// panics if there are no valid crossover points
    fn crossover_chromosome_points<R: Rng>(
        &mut self,
        number_of_crossovers: usize,
        allow_duplicates: bool,
        father: &mut Self::Chromosome,
        mother: &mut Self::Chromosome,
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
}

/// Genotype suitable for [HillClimb](crate::strategy::hill_climb::HillClimb).
pub trait IncrementalGenotype: Genotype {
    /// all neighbouring mutations of the chromosome
    /// used in HillClimbVariant::SteepestAscent
    fn fill_neighbouring_population<R: Rng>(
        &mut self,
        _chromosome: &Self::Chromosome,
        _population: &mut Population<Self::Chromosome>,
        _scale_index: Option<usize>,
        _rng: &mut R,
    );

    /// chromosome neighbours size for the all possible neighbouring mutation combinations
    fn neighbouring_population_size(&self) -> BigUint;
}

/// Genotype suitable for [Permutate](crate::strategy::permutate::Permutate).
/// Not all genotypes are permutable, only countable ones (e.g. range genotypes cannot be permutated).
pub trait PermutableGenotype: Genotype {
    /// chromosome iterator for the all possible gene combinations for [Permutate](crate::strategy::permutate::Permutate)
    fn chromosome_permutations_into_iter(&self) -> impl Iterator<Item = Self::Chromosome> + Send;

    /// chromosome iterator size for the all possible gene combinations for [Permutate](crate::strategy::permutate::Permutate)
    fn chromosome_permutations_size(&self) -> BigUint;
}
