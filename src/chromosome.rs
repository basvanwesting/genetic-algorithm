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
/// with each other in the [Evolve](crate::strategy::evolve::Evolve) strategy
pub trait Chromosome: Clone + Send {
    fn age(&self) -> usize;
    fn reset_age(&mut self);
    fn increment_age(&mut self);
    fn fitness_score(&self) -> Option<FitnessValue>;
    fn set_fitness_score(&mut self, fitness_score: Option<FitnessValue>);
    fn taint_fitness_score(&mut self);
}
pub trait OwnesGenes: Chromosome {
    type Genes: Genes;
    fn new(genes: Self::Genes) -> Self;
    fn genes(&self) -> &Self::Genes;
}
pub trait RefersGenes: Chromosome {
    fn new(row_id: usize) -> Self;
}

/// The GenesKey can be used for caching fitness scores, without lifetime concerns of the chromosome
pub type GenesKey = u64;

// /// The Chromosome is used as an individual in the [Population](crate::population::Population). It
// /// holds the genes and knows how to sort between itself with regard to it's fitness score.
// /// Chromosomes [select](crate::select), [crossover](crate::crossover) and [mutate](crate::mutate) with each other in the
// /// [Evolve](crate::strategy::evolve::Evolve) strategy
// #[derive(Clone, Debug)]
// pub struct LegacyChromosome<G: Genotype> {
//     pub genes: G::Genes,
//     pub fitness_score: Option<FitnessValue>,
//     pub age: usize,
//
//     /// User controlled alternative to `genes_key()`, set manually in
//     /// custom [Fitness](crate::fitness::Fitness) implementation. Defaults to 0
//     pub reference_id: usize,
// }
//
// /// Cannot Hash floats
// impl<G: Genotype> LegacyChromosome<G>
// where
//     G::Genes: Hash,
// {
//     pub fn genes_key(&self) -> GenesKey {
//         let mut s = DefaultHasher::new();
//         self.genes.hash(&mut s);
//         s.finish()
//     }
// }
// /// Impl Copy of Genes are Copy
// impl<G: Genotype> Copy for LegacyChromosome<G> where G::Genes: Copy {}
//
// impl<G: Genotype> LegacyChromosome<G> {
//     pub fn new(genes: G::Genes) -> Self {
//         Self {
//             genes,
//             fitness_score: None,
//             age: 0,
//             reference_id: usize::MAX,
//         }
//     }
//     /// Reset fitness_score for recalculation
//     pub fn taint_fitness_score(&mut self) {
//         self.age = 0;
//         self.fitness_score = None;
//     }
// }
//
// impl<G: Genotype> PartialEq for LegacyChromosome<G> {
//     fn eq(&self, other: &Self) -> bool {
//         self.fitness_score == other.fitness_score
//     }
// }
//
// impl<G: Genotype> Eq for LegacyChromosome<G> {}
//
// impl<G: Genotype> PartialOrd for LegacyChromosome<G> {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.fitness_score.cmp(&other.fitness_score))
//     }
// }
//
// impl<G: Genotype> Ord for LegacyChromosome<G> {
//     fn cmp(&self, other: &Self) -> Ordering {
//         self.partial_cmp(other).unwrap_or(Ordering::Equal)
//     }
// }
//
// impl<G: Genotype> fmt::Display for LegacyChromosome<G> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         if let Some(score) = self.fitness_score {
//             write!(f, "fitness score {}", score)
//         } else {
//             write!(f, "no fitness score")
//         }
//     }
// }

pub trait ChromosomeManager<G: Genotype> {
    /// mandatory, random genes unless seed genes are provided
    fn random_genes_factory<R: Rng>(&self, rng: &mut R) -> G::Genes;
    /// mandatory, a functionally invalid placeholder
    fn chromosome_constructor_empty(&self) -> G::Chromosome;
    /// mandatory, a test for functionally invalid placeholder
    fn chromosome_is_empty(&self, chromosome: &G::Chromosome) -> bool;

    /// provided, disable recycling by default, override when using recycling
    fn chromosome_recycling(&self) -> bool {
        false
    }
    /// provided, override if recycling bin needs initialization
    fn chromosomes_init(&mut self) {}
    /// optional, required if using recycling
    fn chromosome_bin_push(&mut self, _chromosome: G::Chromosome) {}
    /// optional, required if using recycling
    fn chromosome_bin_pop(&mut self) -> Option<G::Chromosome> {
        None
    }

    /// all provided below, fall back to cloning if recycling bin is empty
    /// make bin panic when empty if the fallback to cloning is unwanted

    fn chromosome_constructor<R: Rng>(&mut self, rng: &mut R) -> G::Chromosome;
    fn chromosome_destructor(&mut self, chromosome: G::Chromosome) {
        if self.chromosome_recycling() && !self.chromosome_is_empty(&chromosome) {
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
    fn chromosome_cloner(&mut self, chromosome: &G::Chromosome) -> G::Chromosome;
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
