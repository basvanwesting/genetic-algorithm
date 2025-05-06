//! The selection phase, where chromosomes are lined up for pairing in the
//! [crossover](crate::crossover) phase, dropping the chromosomes outside of the
//! target_population_size.
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
}
