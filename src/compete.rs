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

#[derive(Clone, Debug, Default)]
pub enum Competes {
    #[default]
    Elite,
    Tournament,
}

/// Wrapper for use in [meta analysis](crate::meta)
#[derive(Clone, Debug, Default)]
pub struct CompeteDispatch {
    pub compete: Competes,
    pub tournament_size: usize,
}
impl Compete for CompeteDispatch {
    fn call<T: Genotype, R: Rng>(
        &self,
        population: &mut Population<T>,
        evolve_config: &EvolveConfig,
        rng: &mut R,
    ) {
        match self.compete {
            Competes::Elite => CompeteElite::new().call(population, evolve_config, rng),
            Competes::Tournament => {
                CompeteTournament::new(self.tournament_size).call(population, evolve_config, rng)
            }
        }
    }
}
