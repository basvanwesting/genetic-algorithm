//! The chromosome is a container for the genes and stores some useful values

mod bit;
mod row;
mod vector;

pub use self::bit::Bit as BitChromosome;
pub use self::row::Row as DynamicMatrixChromosome;
pub use self::row::Row as StaticMatrixChromosome;
pub use self::vector::Vector as ListChromosome;
pub use self::vector::Vector as MultiListChromosome;
pub use self::vector::Vector as MultiRangeChromosome;
pub use self::vector::Vector as MultiUniqueChromosome;
pub use self::vector::Vector as RangeChromosome;
pub use self::vector::Vector as UniqueChromosome;
pub type BinaryChromosome = self::vector::Vector<bool>;

use crate::fitness::FitnessValue;
use crate::genotype::{Genes, Genotype};
use rand::prelude::*;
use std::ops::Range;

/// The GenesHash is used for determining cardinality in the population
/// It could also be used for caching fitness scores, without lifetime concerns of the chromosome
pub type GenesHash = u64;

/// The Chromosome is used as an individual in the [Population](crate::population::Population).
/// Chromosomes [select](crate::select), [crossover](crate::crossover) and [mutate](crate::mutate)
/// with each other in the [Evolve](crate::strategy::evolve::Evolve) strategy.
/// Each [Genotype] has its own associated [Chromosome] type.
///
/// Chromosomes who implement [GenesOwner] own their genes. Chromosomes who implement
/// [GenesPointer] don't and just point to genes owned by the Genotype. Therefore the chromosome
/// itself doesn't have a global interface regarding it's genes and use of
/// [Evolve::best_chromosome()](crate::strategy::evolve::Evolve::best_chromosome),
/// [HillCllimb::best_chromosome()](crate::strategy::hill_climb::HillClimb::best_chromosome) and
/// [Permutate::best_chromosome()](crate::strategy::permutate::Permutate::best_chromosome) on the
/// Strategy are discouraged (although they are available when using a Genotype with a [GenesOwner]
/// chromosome). Use [Strategy::best_genes()](crate::strategy::Strategy::best_genes) or
/// [Strategy::best_genes_and_fitness_score()](crate::strategy::Strategy::best_genes_and_fitness_score)
/// instead.
///
/// In the [Fitness](crate::fitness::Fitness) context an associated [Genotype] type is set. And in
/// that contest the ownership model of the genes is known, so we can use the [Chromosome] struct
/// without ambiguity.
pub trait Chromosome: Clone + Send {
    fn age(&self) -> usize;
    fn reset_age(&mut self);
    fn increment_age(&mut self);
    fn fitness_score(&self) -> Option<FitnessValue>;
    fn set_fitness_score(&mut self, fitness_score: Option<FitnessValue>);
    fn genes_hash(&self) -> Option<GenesHash>;
    fn set_genes_hash(&mut self, genes_hash: Option<GenesHash>);
    fn reset_state(&mut self, genes_hash: Option<GenesHash>);
    fn copy_fields_from(&mut self, other: &Self);
}
pub trait GenesOwner: Chromosome {
    type Genes: Genes;
    fn new(genes: Self::Genes) -> Self;
    fn genes(&self) -> &Self::Genes;
}
pub trait GenesPointer: Chromosome {
    fn new(row_id: usize) -> Self;
}

pub trait ChromosomeManager<G: Genotype> {
    /// Mandatory, random genes unless seed genes are provided
    fn random_genes_factory<R: Rng>(&self, rng: &mut R) -> G::Genes;
    /// Mandatory, also resolves taints
    fn copy_genes(&mut self, source: &G::Chromosome, target: &mut G::Chromosome);
    /// Mandatory, also taints
    fn set_genes(&mut self, chromosome: &mut G::Chromosome, genes: &G::Genes);
    /// Mandatory
    fn chromosome_bin_push(&mut self, _chromosome: G::Chromosome);
    /// Mandatory
    /// Take from the recycling bin or create new chromosome with capacities set.
    /// Raise on empty bin here if fixed number of chromosomes is used
    fn chromosome_bin_find_or_create(&mut self) -> G::Chromosome;

    /// Provided, override if recycling bin needs setup
    fn chromosomes_setup(&mut self) {}
    /// Provided, override if recycling bin needs cleanup
    fn chromosomes_cleanup(&mut self) {}

    fn set_random_genes<R: Rng>(&mut self, chromosome: &mut G::Chromosome, rng: &mut R) {
        let genes = self.random_genes_factory(rng);
        self.set_genes(chromosome, &genes);
    }
    fn chromosome_constructor_genes(&mut self, genes: &G::Genes) -> G::Chromosome {
        let mut chromosome = self.chromosome_bin_find_or_create();
        self.set_genes(&mut chromosome, genes);
        chromosome
    }
    fn chromosome_constructor_random<R: Rng>(&mut self, rng: &mut R) -> G::Chromosome {
        let genes = self.random_genes_factory(rng);
        self.chromosome_constructor_genes(&genes)
    }
    fn chromosome_cloner(&mut self, chromosome: &G::Chromosome) -> G::Chromosome {
        let mut new_chromosome = self.chromosome_bin_find_or_create();
        self.copy_genes(chromosome, &mut new_chromosome);
        new_chromosome
    }
    fn chromosome_destructor(&mut self, chromosome: G::Chromosome) {
        self.chromosome_bin_push(chromosome)
    }
    fn chromosome_destructor_truncate(
        &mut self,
        chromosomes: &mut Vec<G::Chromosome>,
        target_population_size: usize,
    ) {
        chromosomes
            .drain(target_population_size..)
            .for_each(|c| self.chromosome_destructor(c));
    }
    fn chromosome_cloner_range(
        &mut self,
        chromosomes: &mut Vec<G::Chromosome>,
        range: Range<usize>,
    ) {
        for i in range {
            let chromosome = &chromosomes[i];
            chromosomes.push(self.chromosome_cloner(chromosome));
        }
    }
}
