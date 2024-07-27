//! The competition phase, where chromosomes are lined up for pairing in the
//! [crossover](crate::crossover) phase. Excess chromosomes, beyond the target_population_size,
//! are dropped.
mod elite;
mod tournament;
mod wrapper;

pub use self::elite::Elite as CompeteElite;
pub use self::tournament::Tournament as CompeteTournament;
pub use self::wrapper::Wrapper as CompeteWrapper;

use crate::genotype::Genotype;
use crate::population::Population;
use crate::strategy::evolve::EvolveConfig;
use rand::prelude::*;

pub trait Compete: Clone + std::fmt::Debug {
    fn call<T: Genotype, R: Rng>(
        &mut self,
        population: &mut Population<T>,
        evolve_config: &EvolveConfig,
        rng: &mut R,
    );
}
