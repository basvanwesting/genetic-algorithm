//! The competition phase, where chromosomes are lined up for pairing in the
//! [crossover](crate::crossover) phase. Excess chromosomes, beyond the target_population_size,
//! are dropped.
mod elite;
mod tournament;
mod wrapper;

pub use self::elite::Elite as CompeteElite;
pub use self::tournament::Tournament as CompeteTournament;
pub use self::wrapper::Wrapper as CompeteWrapper;

use crate::genotype::Allele;
use crate::strategy::evolve::{EvolveConfig, EvolveReporter, EvolveState};
use rand::prelude::*;

pub trait Compete: Clone + Send + Sync + std::fmt::Debug {
    fn call<A: Allele, R: Rng, SR: EvolveReporter<Allele = A>>(
        &mut self,
        state: &mut EvolveState<A>,
        config: &EvolveConfig,
        reporter: &mut SR,
        rng: &mut R,
        par: bool,
    );
}
