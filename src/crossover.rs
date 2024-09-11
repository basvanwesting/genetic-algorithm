//! The crossover phase where every two parent chromosomes create two children chromosomes. The
//! [selection](crate::select) phase determines the order and the amount of the parent pairing
//! (overall with fitter first).
//!
//! If the [selection](crate::select) dropped some chromosomes due to the selection_rate, the
//! crossover will restore the population towards the target_population_size by keeping the best
//! parents alive. Excess parents are dropped.
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

use crate::genotype::Genotype;
use crate::strategy::evolve::{EvolveConfig, EvolveReporter, EvolveState};
use crate::strategy::{StrategyAction, StrategyState};
use rand::Rng;
use std::cmp::Ordering;
use std::time::Instant;

pub trait Crossover: Clone + Send + Sync + std::fmt::Debug {
    fn call<G: Genotype, R: Rng, SR: EvolveReporter<Genotype = G>>(
        &mut self,
        genotype: &mut G,
        state: &mut EvolveState<G>,
        config: &EvolveConfig,
        reporter: &mut SR,
        rng: &mut R,
    );

    /// The population is restored towards the target_population_size by keeping the best parents
    /// alive. Excess parents are dropped. The number of crossovers to execute from the front of
    /// the population is returned
    fn prepare_population<G: Genotype>(
        &mut self,
        genotype: &mut G,
        state: &mut EvolveState<G>,
        config: &EvolveConfig,
    ) -> usize {
        let population_size = state.population.size();
        match config.target_population_size.cmp(&population_size) {
            Ordering::Greater => {
                let parent_survivors =
                    (config.target_population_size - population_size).min(population_size);
                genotype.chromosome_cloner_range(
                    &mut state.population.chromosomes,
                    0..parent_survivors,
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
        population_size.min(config.target_population_size)
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
