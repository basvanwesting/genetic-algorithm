pub use super::elite::Elite as CompeteElite;
pub use super::tournament::Tournament as CompeteTournament;
pub use super::Compete;

use crate::genotype::Genotype;
use crate::population::Population;
use crate::strategy::evolve::EvolveConfig;
use rand::prelude::*;

#[derive(Clone, Debug)]
pub enum Wrapper {
    Elite(CompeteElite),
    Tournament(CompeteTournament),
}

impl Compete for Wrapper {
    fn call<T: Genotype, R: Rng>(
        &mut self,
        population: &mut Population<T>,
        evolve_config: &EvolveConfig,
        rng: &mut R,
    ) {
        match self {
            Wrapper::Elite(compete) => compete.call(population, evolve_config, rng),
            Wrapper::Tournament(compete) => compete.call(population, evolve_config, rng),
        }
    }
}
impl From<CompeteElite> for Wrapper {
    fn from(compete: CompeteElite) -> Self {
        Wrapper::Elite(compete)
    }
}
impl From<CompeteTournament> for Wrapper {
    fn from(compete: CompeteTournament) -> Self {
        Wrapper::Tournament(compete)
    }
}
