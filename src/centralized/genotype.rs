//! The search space for the algorithm.
mod builder;
mod dynamic_range;
mod static_binary;
mod static_range;

pub use self::builder::{
    Builder as GenotypeBuilder, TryFromBuilderError as TryFromGenotypeBuilderError,
};
pub use self::dynamic_range::DynamicRange as DynamicRangeGenotype;
pub use self::static_binary::StaticBinary as StaticBinaryGenotype;
pub use self::static_range::StaticRange as StaticRangeGenotype;

pub use crate::centralized::allele::{Allele, RangeAllele};
use crate::centralized::chromosome::{Chromosome, ChromosomeManager, GenesHash};
use crate::centralized::fitness::FitnessValue;
use crate::centralized::population::Population;
use itertools::Itertools;
use num::BigUint;
use rand::Rng;
use std::fmt;

/// Standard Genes for centralized [Genotype] storage (seed_genes_list and best_genes).
/// Note: Centralized chromosomes use GenesPointer (row_id) rather than owning genes directly.
pub trait Genes: Clone + Send + Sync + std::fmt::Debug {}
impl<T: Allele> Genes for Vec<T> {}
impl<T: Allele, const N: usize> Genes for Box<[T; N]> {}
impl Genes for () {}

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
    fn genes_hashing(&self) -> bool;
    fn calculate_genes_hash(&self, chromosome: &Self::Chromosome) -> Option<GenesHash>;
    // lives on Genotype because only genotype can calculate_genes_hash
    fn reset_chromosome_state(&self, chromosome: &mut Self::Chromosome) {
        chromosome.reset_state(self.calculate_genes_hash(chromosome));
    }
    // lives on Genotype for symmetry reasons with reset_chromosome_state
    fn copy_chromosome_state(&self, source: &Self::Chromosome, target: &mut Self::Chromosome) {
        target.copy_state(source)
    }
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

    fn mutation_type(&self) -> MutationType {
        MutationType::Random
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

    fn population_constructor<R: Rng>(
        &mut self,
        population_size: usize,
        rng: &mut R,
    ) -> Population<Self::Chromosome> {
        if self.seed_genes_list().is_empty() {
            Population::new(
                (0..population_size)
                    .map(|_| self.chromosome_constructor_random(rng))
                    .collect::<Vec<_>>(),
            )
        } else {
            Population::new(
                self.seed_genes_list()
                    .clone()
                    .iter()
                    .cycle()
                    .take(population_size)
                    .map(|genes| self.chromosome_constructor_genes(genes))
                    .collect::<Vec<_>>(),
            )
        }
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
pub trait HillClimbGenotype: Genotype {
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
/// Not all genotypes are permutable, only countable ones (e.g. range genotypes cannot be permutated, unless scaled).
pub trait PermutateGenotype: Genotype {
    /// chromosome iterator size for the all possible gene combinations for [Permutate](crate::strategy::permutate::Permutate)
    fn chromosome_permutations_size(&self) -> BigUint;

    /// Population-windowed permutation iterator for centralized genotypes.
    /// Returns an iterator of populations, each containing a window of permutations.
    /// This allows matrix-based genotypes to process permutations in batches.
    fn population_permutations_into_iter<'a>(
        &'a self,
        _window_size: usize,
        _scale_index: Option<usize>,
    ) -> Box<dyn Iterator<Item = Population<Self::Chromosome>> + Send + 'a> {
        todo!("Implement population_permutations_into_iter for centralized genotypes use best_genes as center for scale")
    }

    /// not all mutation_types implemented for certain genotypes
    fn mutation_type_allows_permutation(&self) -> bool {
        false
    }
}
