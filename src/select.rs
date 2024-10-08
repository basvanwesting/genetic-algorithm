//! The selection phase, where chromosomes are lined up for pairing in the
//! [crossover](crate::crossover) phase, dropping the chromosomes outside of the selection_rate.
//! Ensure the selection_rate >= 0.5 otherwise the population will decline and can't restore.
//!
//! For some problem domains, where there is little incremental improvement in fitness, it is
//! better to keep all parents around as a jumping off point for the new generations. Achieve this
//! by setting the selection_rate to 0.5. This way the top 50% will reproduce and also be cloned to
//! restore the population size. All lesser offspring is eliminated by selection, keeping the top
//! 50% around each generation, until actually improved upon.
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
