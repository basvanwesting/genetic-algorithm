pub use super::elite::Elite as CompeteElite;
pub use super::tournament::Tournament as CompeteTournament;
pub use super::Compete;

use crate::genotype::Genotype;
use crate::population::Population;
use crate::strategy::evolve::EvolveConfig;
use rand::prelude::*;

#[derive(Clone, Debug, Default)]
pub enum Implementations {
    #[default]
    Elite,
    Tournament,
}

/// Wrapper for use in [meta analysis](crate::meta)
#[derive(Clone, Debug, Default)]
pub struct Dispatch {
    pub implementation: Implementations,
    pub tournament_size: usize,
}

impl Compete for Dispatch {
    fn call<T: Genotype, R: Rng>(
        &mut self,
        population: &mut Population<T>,
        evolve_config: &EvolveConfig,
        rng: &mut R,
    ) {
        match self.implementation {
            Implementations::Elite => CompeteElite::new().call(population, evolve_config, rng),
            Implementations::Tournament => {
                CompeteTournament::new(self.tournament_size).call(population, evolve_config, rng)
            }
        }
    }
}

impl From<CompeteElite> for Dispatch {
    fn from(_compete: CompeteElite) -> Self {
        Dispatch {
            implementation: Implementations::Elite,
            ..Default::default()
        }
    }
}

impl From<CompeteTournament> for Dispatch {
    fn from(compete: CompeteTournament) -> Self {
        Dispatch {
            implementation: Implementations::Tournament,
            tournament_size: compete.tournament_size,
            ..Default::default()
        }
    }
}
