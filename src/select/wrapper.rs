pub use super::elite::Elite as SelectElite;
pub use super::tournament::Tournament as SelectTournament;
pub use super::Select;

use crate::genotype::Genotype;
use crate::strategy::evolve::{EvolveConfig, EvolveState};
use crate::strategy::StrategyReporter;
use rand::prelude::*;

#[derive(Clone, Debug)]
pub enum Wrapper {
    Elite(SelectElite),
    Tournament(SelectTournament),
}

impl Select for Wrapper {
    fn call<G: Genotype, R: Rng, SR: StrategyReporter<Genotype = G>>(
        &mut self,
        genotype: &mut G,
        state: &mut EvolveState<G>,
        config: &EvolveConfig,
        reporter: &mut SR,
        rng: &mut R,
    ) {
        match self {
            Wrapper::Elite(select) => select.call(genotype, state, config, reporter, rng),
            Wrapper::Tournament(select) => select.call(genotype, state, config, reporter, rng),
        }
    }
}
impl From<SelectElite> for Wrapper {
    fn from(select: SelectElite) -> Self {
        Wrapper::Elite(select)
    }
}
impl From<SelectTournament> for Wrapper {
    fn from(select: SelectTournament) -> Self {
        Wrapper::Tournament(select)
    }
}
