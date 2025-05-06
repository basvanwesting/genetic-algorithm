//! The crossover phase where every two parent chromosomes create two children chromosomes. The
//! [selection](crate::select) phase determines the order and the amount of the parent pairing
//! (overall with fitter first).
//!
//! For the crossover-rate, typically set between 0.7 and 0.9 (70%-90% of the population undergoes
//! crossover). Higher crossover rates promote exploration and recombination of genetic material.
//!
//! To further prevent the population from "dying out" or losing important genetic material:
//! Elitism is often used through the elitism_rate, where a small number of the best individuals
//! (e.g., top 1%-5%) are directly carried over to the next generation without modification.
//!
//! If the [selection](crate::select) dropped some chromosomes due to the selection_rate, the
//! crossover will restore the population towards the target_population_size by cycling through the
//! selected population as parents for the crossover. Excess parents are dropped.
mod clone;
mod multi_gene;
mod multi_point;
mod single_gene;
mod single_point;
mod uniform;
mod wrapper;

pub use self::clone::Clone as CrossoverClone;
pub use self::multi_gene::MultiGene as CrossoverMultiGene;
pub use self::multi_point::MultiPoint as CrossoverMultiPoint;
pub use self::single_gene::SingleGene as CrossoverSingleGene;
pub use self::single_point::SinglePoint as CrossoverSinglePoint;
pub use self::uniform::Uniform as CrossoverUniform;
pub use self::wrapper::Wrapper as CrossoverWrapper;

use crate::genotype::EvolveGenotype;
use crate::strategy::evolve::{EvolveConfig, EvolveState};
use crate::strategy::StrategyReporter;
use rand::Rng;
use std::cmp::Ordering;

pub trait Crossover: Clone + Send + Sync + std::fmt::Debug {
    fn call<G: EvolveGenotype, R: Rng, SR: StrategyReporter<Genotype = G>>(
        &mut self,
        genotype: &mut G,
        state: &mut EvolveState<G>,
        config: &EvolveConfig,
        reporter: &mut SR,
        rng: &mut R,
    );

    /// The population is restored towards the target_population_size by cycled cloning of the existing population.
    /// Excess parents are dropped.
    fn prepare_population<G: EvolveGenotype>(
        &mut self,
        genotype: &mut G,
        state: &mut EvolveState<G>,
        config: &EvolveConfig,
    ) {
        let population_size = state.population.size();
        match config.target_population_size.cmp(&population_size) {
            Ordering::Greater => {
                genotype.chromosome_cloner_restore(
                    &mut state.population.chromosomes,
                    config.target_population_size,
                );
            }
            Ordering::Less => {
                eprintln!(
                    "Crossover: population-size {} is more than target-population-size {}, this should never happen",
                    population_size,
                    config.target_population_size
                );
                genotype.chromosome_destructor_truncate(
                    &mut state.population.chromosomes,
                    config.target_population_size,
                );
            }
            Ordering::Equal => (),
        }
    }

    /// to guard against invalid Crossover strategies which break the internal consistency
    /// of the genes, unique genotypes can't simply exchange genes without gene duplication issues
    fn require_crossover_indexes(&self) -> bool {
        false
    }
    /// to guard against invalid Crossover strategies which break the internal consistency
    /// of the genes, unique genotypes can't simply exchange genes without gene duplication issues
    fn require_crossover_points(&self) -> bool {
        false
    }
}
