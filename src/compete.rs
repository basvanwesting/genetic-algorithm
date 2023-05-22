//! The competition phase, where chromosomes are lined up for pairing in the
//! [crossover](crate::crossover) phase. Excess chromosomes, beyond the target_population_size,
//! are dropped.
mod elite;
mod tournament;

pub use self::elite::Elite as CompeteElite;
pub use self::tournament::Tournament as CompeteTournament;

use crate::genotype::Genotype;
use crate::population::Population;
use crate::strategy::evolve::EvolveConfig;
use rand::prelude::*;

pub trait Compete: Clone + std::fmt::Debug {
    fn call<T: Genotype, R: Rng>(
        &self,
        population: &mut Population<T>,
        evolve_config: &EvolveConfig,
        rng: &mut R,
    );
}

#[derive(Clone, Debug)]
pub enum Competes {
    Elite,
    Tournament,
}
pub type TournamentSize = usize;

/// Wrapper for use in [meta analysis](crate::meta)
#[derive(Clone, Debug)]
pub struct CompeteDispatch(pub Competes, pub TournamentSize);
impl Compete for CompeteDispatch {
    fn call<T: Genotype, R: Rng>(
        &self,
        population: &mut Population<T>,
        evolve_config: &EvolveConfig,
        rng: &mut R,
    ) {
        match self.0 {
            Competes::Elite => CompeteElite.call(population, evolve_config, rng),
            Competes::Tournament => CompeteTournament(self.1).call(population, evolve_config, rng),
        }
    }
}
