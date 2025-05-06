//! The selection phase, where chromosomes are lined up for pairing in the
//! [crossover](crate::crossover) phase, dropping the chromosomes outside of the selection_rate.
//!
//! For the selection-rate, typically set between 0.2 and 0.5 (20%-50% of the population is selected for reproduction).
//! A higher selection rate (closer to 50%) can accelerate convergence but risks premature convergence (getting stuck in local optima).
//! A lower selection rate (closer to 20%) maintains diversity but may slow down the algorithm.
//! Other sources suggest a higher selection-rate, somewhere in the 0.75-1.0 range. Apparantly
//! there is no broad consensus
mod elite;
mod tournament;
mod wrapper;

pub use self::elite::Elite as SelectElite;
pub use self::tournament::Tournament as SelectTournament;
pub use self::wrapper::Wrapper as SelectWrapper;

use crate::genotype::EvolveGenotype;
use crate::strategy::evolve::{EvolveConfig, EvolveState};
use crate::strategy::StrategyReporter;
use rand::prelude::*;

pub trait Select: Clone + Send + Sync + std::fmt::Debug {
    fn call<G: EvolveGenotype, R: Rng, SR: StrategyReporter<Genotype = G>>(
        &mut self,
        genotype: &mut G,
        state: &mut EvolveState<G>,
        config: &EvolveConfig,
        reporter: &mut SR,
        rng: &mut R,
    );
    fn selected_population_size(&self, working_population_size: usize) -> usize;
}
