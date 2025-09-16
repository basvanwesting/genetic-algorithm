//! The search space for the algorithm.
mod binary;
mod builder;
mod list;
mod multi_list;
mod multi_range;
mod multi_unique;
mod range;
mod unique;

pub use self::binary::Binary as BinaryGenotype;
pub use self::builder::{
    Builder as GenotypeBuilder, TryFromBuilderError as TryFromGenotypeBuilderError,
};
pub use self::list::List as ListGenotype;
pub use self::multi_list::MultiList as MultiListGenotype;
pub use self::multi_range::MultiRange as MultiRangeGenotype;
pub use self::multi_unique::MultiUnique as MultiUniqueGenotype;
pub use self::range::Range as RangeGenotype;
pub use self::unique::Unique as UniqueGenotype;

pub use crate::allele::{Allele, RangeAllele};
use crate::chromosome::{Chromosome, Genes};
use crate::population::Population;
pub use crate::impl_allele;
use itertools::Itertools;
use num::BigUint;
use rand::Rng;
use std::fmt;

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
    Clone + Send + Sync + fmt::Debug + fmt::Display + TryFrom<GenotypeBuilder<Self>>
{
    type Allele: Allele;

    fn genes_size(&self) -> usize;
    fn genes_capacity(&self) -> usize;
    fn genes_slice<'a>(&'a self, chromosome: &'a Chromosome<Self::Allele>) -> &'a [Self::Allele];

    fn random_genes_factory<R: Rng>(&self, rng: &mut R) -> Genes<Self::Allele>;
    fn set_random_genes<R: Rng>(&self, chromosome: &mut Chromosome<Self::Allele>, rng: &mut R) {
        let genes = self.random_genes_factory(rng);
        chromosome.set_genes(genes);
    }

    fn mutation_type(&self) -> MutationType {
        MutationType::Random
    }
    fn mutate_chromosome_genes<R: Rng>(
        &self,
        number_of_mutations: usize,
        allow_duplicates: bool,
        chromosome: &mut Chromosome<Self::Allele>,
        scale_index: Option<usize>,
        rng: &mut R,
    );

    fn builder() -> GenotypeBuilder<Self> {
        GenotypeBuilder::<Self>::default()
    }
    fn with_seed_genes_list(&self, seed_genes_list: Vec<Genes<Self::Allele>>) -> Self;

    fn seed_genes_list(&self) -> &Vec<Genes<Self::Allele>>;
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
        &self,
        population_size: usize,
        rng: &mut R,
    ) -> Population<Self::Allele> {
        if self.seed_genes_list().is_empty() {
            Population::new(
                (0..population_size)
                    .map(|_| Chromosome::new(self.random_genes_factory(rng)))
                    .collect::<Vec<_>>(),
            )
        } else {
            Population::new(
                self.seed_genes_list()
                    .clone()
                    .iter()
                    .cycle()
                    .take(population_size)
                    .map(|genes| Chromosome::new(genes.clone()))
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
        &self,
        number_of_crossovers: usize,
        allow_duplicates: bool,
        father: &mut Chromosome<Self::Allele>,
        mother: &mut Chromosome<Self::Allele>,
        rng: &mut R,
    );
    /// Crossover points between a pair of chromosomes.
    /// Choose between allowing duplicates or not (not much slower)
    /// panics if there are no valid crossover points
    fn crossover_chromosome_points<R: Rng>(
        &self,
        number_of_crossovers: usize,
        allow_duplicates: bool,
        father: &mut Chromosome<Self::Allele>,
        mother: &mut Chromosome<Self::Allele>,
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
        &self,
        _chromosome: &Chromosome<Self::Allele>,
        _population: &mut Population<Self::Allele>,
        _scale_index: Option<usize>,
        _rng: &mut R,
    );

    /// chromosome neighbours size for the all possible neighbouring mutation combinations
    fn neighbouring_population_size(&self) -> BigUint;
}

/// Genotype suitable for [Permutate](crate::strategy::permutate::Permutate).
/// Not all genotypes are permutable, only countable ones (e.g. range genotypes cannot be permutated, unless scaled).
pub trait PermutateGenotype: Genotype {
    /// chromosome iterator for the all possible gene combinations for [Permutate](crate::strategy::permutate::Permutate)
    fn chromosome_permutations_into_iter<'a>(
        &'a self,
        _chromosome: Option<&Chromosome<Self::Allele>>,
        _scale_index: Option<usize>,
    ) -> Box<dyn Iterator<Item = Chromosome<Self::Allele>> + Send + 'a>;

    /// chromosome iterator size for the all possible gene combinations for [Permutate](crate::strategy::permutate::Permutate)
    fn chromosome_permutations_size(&self) -> BigUint;

    /// not all mutation_types implemented for certain genotypes
    fn mutation_type_allows_permutation(&self) -> bool {
        false
    }
}
