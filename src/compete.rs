//! The competition phase, where chromosomes are lined up for pairing in the
//! [crossover](crate::crossover) phase. Excess chromosomes, beyond the target_population_size,
//! are dropped.
//mod dispatch;
mod elite;
mod tournament;

//pub use self::dispatch::Dispatch as CompeteDispatch;
pub use self::elite::Elite as CompeteElite;
pub use self::tournament::Tournament as CompeteTournament;

use crate::genotype::Genotype;
use crate::population::Population;
use crate::strategy::evolve::EvolveConfig;
use rand::prelude::*;

#[derive(Clone, Debug)]
pub enum Compete {
    Elite(CompeteElite),
    Tournament(CompeteTournament),
}

impl Compete {
    pub fn call<T: Genotype, R: Rng>(
        &mut self,
        population: &mut Population<T>,
        evolve_config: &EvolveConfig,
        rng: &mut R,
    ) {
        match self {
            Compete::Elite(compete) => compete.call(population, evolve_config, rng),
            Compete::Tournament(compete) => compete.call(population, evolve_config, rng),
        }
    }
}
impl From<CompeteElite> for Compete {
    fn from(compete: CompeteElite) -> Self {
        Compete::Elite(compete)
    }
}
impl From<CompeteTournament> for Compete {
    fn from(compete: CompeteTournament) -> Self {
        Compete::Tournament(compete)
    }
}
