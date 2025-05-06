//! The crossover phase where two parent chromosomes create two children chromosomes. The
//! [selection](crate::select) phase determines the order the parent pairing (overall with fitter
//! first).
//!
//! For the selection-rate, typically set between 0.2 and 0.5 (20%-50% of the population is
//! selected for reproduction). A higher selection rate (closer to 50%) can accelerate convergence
//! but risks premature convergence (getting stuck in local optima). A lower selection rate (closer
//! to 20%) maintains diversity but may slow down the algorithm. Other sources suggest a higher
//! selection-rate, somewhere in the 0.75-1.0 range. Apparantly there is no broad consensus.
//!
//! For the crossover-rate, typically set between 0.7 and 0.9 (70%-90% of the population undergoes
//! crossover). Higher crossover rates promote exploration and recombination of genetic material.
//!
//! The crossover adds children, thus potentially increasing the population_size above the
//! target_population_size
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

pub trait Crossover: Clone + Send + Sync + std::fmt::Debug {
    fn call<G: EvolveGenotype, R: Rng, SR: StrategyReporter<Genotype = G>>(
        &mut self,
        genotype: &mut G,
        state: &mut EvolveState<G>,
        config: &EvolveConfig,
        reporter: &mut SR,
        rng: &mut R,
    );

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
