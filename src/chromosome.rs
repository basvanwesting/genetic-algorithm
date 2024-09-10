//! The chromosome is a container for the genes and caches a fitness score

mod binary;
mod bit;
mod dynamic_matrix;
mod list;
mod multi_list;
mod multi_range;
mod multi_unique;
mod range;
mod static_matrix;
mod unique;

pub use self::binary::Binary as BinaryChromosome;
pub use self::bit::Bit as BitChromosome;
pub use self::dynamic_matrix::DynamicMatrix as DynamicMatrixChromosome;
pub use self::list::List as ListChromosome;
pub use self::multi_list::MultiList as MultiListChromosome;
pub use self::multi_range::MultiRange as MultiRangeChromosome;
pub use self::multi_unique::MultiUnique as MultiUniqueChromosome;
pub use self::range::Range as RangeChromosome;
pub use self::static_matrix::StaticMatrix as StaticMatrixChromosome;
pub use self::unique::Unique as UniqueChromosome;

use crate::fitness::FitnessValue;
use crate::genotype::{Genes, Genotype};
use rand::prelude::*;
use std::ops::Range;

/// The Chromosome is used as an individual in the [Population](crate::population::Population).
/// Chromosomes [select](crate::select), [crossover](crate::crossover) and [mutate](crate::mutate)
/// with each other in the [Evolve](crate::strategy::evolve::Evolve) strategy.
///
/// Some chromosomes store their own genes and some reference external data. Therefore the use of
/// [Evolve::best_chromosome()](crate::strategy::evolve::Evolve::best_chromosome),
/// [HillCllimb::best_chromosome()](crate::strategy::hill_climb::HillClimb::best_chromosome) and
/// [Permutate::best_chromosome()](crate::strategy::permutate::Permutate::best_chromosome) on the
/// Strategy are discouraged.
/// Use [Strategy::best_genes()](crate::strategy::Strategy::best_genes) or
/// [Strategy::best_genes_and_fitness_score()](crate::strategy::Strategy::best_genes_and_fitness_score)
/// instead.
pub trait Chromosome: Clone + Send {
    fn age(&self) -> usize;
    fn reset_age(&mut self);
    fn increment_age(&mut self);
    fn fitness_score(&self) -> Option<FitnessValue>;
    fn set_fitness_score(&mut self, fitness_score: Option<FitnessValue>);
    fn taint_fitness_score(&mut self);
}
pub trait OwnsGenes: Chromosome {
    type Genes: Genes;
    fn new(genes: Self::Genes) -> Self;
    fn genes(&self) -> &Self::Genes;
}
pub trait RefersGenes: Chromosome {
    fn new(row_id: usize) -> Self;
}

/// The GenesKey can be used for caching fitness scores, without lifetime concerns of the chromosome
pub type GenesKey = u64;

pub trait ChromosomeManager<G: Genotype> {
    /// mandatory, random genes unless seed genes are provided
    fn random_genes_factory<R: Rng>(&self, rng: &mut R) -> G::Genes;
    /// mandatory
    fn chromosome_constructor<R: Rng>(&mut self, rng: &mut R) -> G::Chromosome;
    /// mandatory
    fn chromosome_cloner(&mut self, chromosome: &G::Chromosome) -> G::Chromosome;
    /// provided, disable recycling by default, override when using recycling
    fn chromosome_recycling(&self) -> bool {
        false
    }
    /// provided, override if recycling bin needs initialization
    fn chromosomes_init(&mut self) {}
    /// optional, override if using recycling
    fn chromosome_bin_push(&mut self, _chromosome: G::Chromosome) {}
    /// optional, override if using recycling
    fn chromosome_bin_pop(&mut self) -> Option<G::Chromosome> {
        None
    }

    fn chromosome_destructor(&mut self, chromosome: G::Chromosome) {
        if self.chromosome_recycling() {
            self.chromosome_bin_push(chromosome)
        }
    }
    fn chromosome_destructor_truncate(
        &mut self,
        chromosomes: &mut Vec<G::Chromosome>,
        target_population_size: usize,
    ) {
        if self.chromosome_recycling() {
            chromosomes
                .drain(target_population_size..)
                .for_each(|c| self.chromosome_destructor(c));
        } else {
            chromosomes.truncate(target_population_size);
        }
    }
    fn chromosome_cloner_range(
        &mut self,
        chromosomes: &mut Vec<G::Chromosome>,
        range: Range<usize>,
    ) {
        if self.chromosome_recycling() {
            for i in range {
                let chromosome = &chromosomes[i];
                chromosomes.push(self.chromosome_cloner(chromosome));
            }
        } else {
            chromosomes.extend_from_within(range);
        }
    }
}
