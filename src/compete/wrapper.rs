pub use super::elite::Elite as CompeteElite;
pub use super::tournament::Tournament as CompeteTournament;
pub use super::Compete;

use crate::genotype::Allele;
use crate::strategy::evolve::{EvolveConfig, EvolveReporter, EvolveState};
use rand::prelude::*;

#[derive(Clone, Debug)]
pub enum Wrapper {
    Elite(CompeteElite),
    Tournament(CompeteTournament),
}

impl Compete for Wrapper {
    fn call<A: Allele, R: Rng + Clone + Send + Sync, SR: EvolveReporter<Allele = A>>(
        &mut self,
        state: &mut EvolveState<A>,
        config: &EvolveConfig,
        reporter: &mut SR,
        rng: &mut R,
    ) {
        match self {
            Wrapper::Elite(compete) => compete.call(state, config, reporter, rng),
            Wrapper::Tournament(compete) => compete.call(state, config, reporter, rng),
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
