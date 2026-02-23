//! The search space for the algorithm.
mod binary;
mod builder;
mod list;
mod multi_list;
mod multi_range;
mod multi_unique;
mod mutation_type;
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
pub use self::mutation_type::MutationType;
pub use self::range::Range as RangeGenotype;
pub use self::unique::Unique as UniqueGenotype;

pub use crate::allele::{Allele, RangeAllele};
use crate::chromosome::{Chromosome, Genes};
pub use crate::impl_allele;
use crate::population::Population;
use itertools::Itertools;
use num::BigUint;
use rand::Rng;
use std::fmt;

/// Standard genotype, suitable for [Evolve](crate::strategy::evolve::Evolve).
/// Each implemented genotype handles its own random genes initialization and mutation.
pub trait Genotype:
    Clone + Send + Sync + fmt::Debug + fmt::Display + TryFrom<GenotypeBuilder<Self>>
{
    type Allele: Allele;

    fn builder() -> GenotypeBuilder<Self> {
        GenotypeBuilder::<Self>::default()
    }
    fn genes_size(&self) -> usize;
    fn genes_capacity(&self) -> usize;
    fn genes_hashing(&self) -> bool;
    fn chromosome_recycling(&self) -> bool;
    fn random_genes_factory<R: Rng>(&self, rng: &mut R) -> Genes<Self::Allele>;
    fn sample_gene_index<R: Rng>(&self, rng: &mut R) -> usize;
    fn sample_gene_indices<R: Rng>(
        &self,
        count: usize,
        allow_duplicates: bool,
        rng: &mut R,
    ) -> Vec<usize>;
    fn mutate_chromosome_genes<R: Rng>(
        &self,
        number_of_mutations: usize,
        allow_duplicates: bool,
        chromosome: &mut Chromosome<Self::Allele>,
        rng: &mut R,
    );

    fn set_seed_genes_list(&mut self, seed_genes_list: Vec<Genes<Self::Allele>>);
    fn seed_genes_list(&self) -> &Vec<Genes<Self::Allele>>;
    fn set_genes_hashing(&mut self, genes_hashing: bool);
    fn max_scale_index(&self) -> Option<usize> {
        None
    }
    fn current_scale_index(&self) -> Option<usize> {
        None
    }
    fn reset_scale_index(&mut self) {}
    fn increment_scale_index(&mut self) -> bool {
        false
    }
    fn reset(&mut self) {
        self.reset_scale_index();
    }

    fn chromosome_constructor_random<R: Rng>(&self, rng: &mut R) -> Chromosome<Self::Allele> {
        let mut chromosome = Chromosome::new(self.random_genes_factory(rng));
        chromosome.reset_metadata(self.genes_hashing());
        chromosome
    }
    fn chromosome_constructor_genes(
        &self,
        genes: &Genes<Self::Allele>,
    ) -> Chromosome<Self::Allele> {
        let mut chromosome = Chromosome::new(genes.clone());
        chromosome.reset_metadata(self.genes_hashing());
        chromosome
    }
    fn population_constructor<R: Rng>(
        &self,
        population_size: usize,
        rng: &mut R,
    ) -> Population<Self::Allele> {
        if self.seed_genes_list().is_empty() {
            Population::new(
                (0..population_size)
                    .map(|_| self.chromosome_constructor_random(rng))
                    .collect::<Vec<_>>(),
                self.chromosome_recycling(),
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
                self.chromosome_recycling(),
            )
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
                if *number_of_samples > 1 {
                    number_of_samples * (number_of_samples - 1) / (2 * self.genes_size())
                } else {
                    0
                },
            )
        })
        .filter(|(_, c)| *c > 0)
        .map(|(n, e)| format!("{} => {}", n, e))
        .join(", ")
    }
    fn format_biguint_scientific(&self, n: &BigUint) -> String {
        let s = n.to_string();
        let len = s.len();

        if len <= 6 {
            s
        } else {
            let mantissa = format!("{}.{}", &s[0..1], &s[1..7]);
            format!("{}e{}", mantissa, len - 1)
        }
    }
}

/// Genotype suitable for [Evolve](crate::strategy::evolve::Evolve).
pub trait EvolveGenotype: Genotype {}

/// Genotype that supports gene-index-based crossover (swap individual genes).
/// Not implemented by [UniqueGenotype] or [MultiUniqueGenotype] (would break uniqueness).
pub trait SupportsGeneCrossover: Genotype {
    fn crossover_chromosome_genes<R: Rng>(
        &self,
        number_of_crossovers: usize,
        allow_duplicates: bool,
        father: &mut Chromosome<Self::Allele>,
        mother: &mut Chromosome<Self::Allele>,
        rng: &mut R,
    );
}

/// Genotype that supports point-based crossover (swap sections at crossover points).
/// Not implemented by [UniqueGenotype]. Implemented by [MultiUniqueGenotype].
pub trait SupportsPointCrossover: Genotype {
    fn crossover_chromosome_points<R: Rng>(
        &self,
        number_of_crossovers: usize,
        allow_duplicates: bool,
        father: &mut Chromosome<Self::Allele>,
        mother: &mut Chromosome<Self::Allele>,
        rng: &mut R,
    );
}

/// Genotype suitable for [HillClimb](crate::strategy::hill_climb::HillClimb).
pub trait HillClimbGenotype: Genotype {
    /// all neighbouring mutations of the chromosome
    /// used in HillClimbVariant::SteepestAscent
    fn fill_neighbouring_population<R: Rng>(
        &self,
        _chromosome: &Chromosome<Self::Allele>,
        _population: &mut Population<Self::Allele>,
        _rng: &mut R,
    );

    /// chromosome neighbours size for the all possible neighbouring mutation combinations
    fn neighbouring_population_size(&self) -> BigUint;

    fn neighbouring_population_size_report(&self) -> String {
        self.format_biguint_scientific(&self.neighbouring_population_size())
    }
}

/// Genotype suitable for [Permutate](crate::strategy::permutate::Permutate).
/// Not all genotypes are permutable, only countable ones (e.g. range genotypes cannot be permutated, unless scaled).
pub trait PermutateGenotype: Genotype {
    /// chromosome iterator for the all possible gene combinations for [Permutate](crate::strategy::permutate::Permutate)
    fn chromosome_permutations_into_iter<'a>(
        &'a self,
        _chromosome: Option<&Chromosome<Self::Allele>>,
    ) -> Box<dyn Iterator<Item = Chromosome<Self::Allele>> + Send + 'a>;

    /// chromosome iterator size for the all possible gene combinations for [Permutate](crate::strategy::permutate::Permutate)
    fn chromosome_permutations_size(&self) -> BigUint;

    fn chromosome_permutations_size_report(&self) -> String {
        self.format_biguint_scientific(&self.chromosome_permutations_size())
    }

    /// not all mutation_types implemented for certain genotypes
    fn allows_permutation(&self) -> bool {
        false
    }
}
