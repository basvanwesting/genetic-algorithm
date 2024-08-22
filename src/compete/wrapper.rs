pub use super::elite::Elite as CompeteElite;
pub use super::tournament::Tournament as CompeteTournament;
pub use super::Compete;

use crate::genotype::Allele;
use crate::strategy::evolve::{EvolveConfig, EvolveReporter, EvolveState};
use rand::prelude::*;
use std::cell::RefCell;
use thread_local::ThreadLocal;

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
        thread_local: Option<&ThreadLocal<RefCell<R>>>,
    ) {
        match self {
            Wrapper::Elite(compete) => compete.call(state, config, reporter, rng, thread_local),
            Wrapper::Tournament(compete) => {
                compete.call(state, config, reporter, rng, thread_local)
            }
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
