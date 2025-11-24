pub use super::elite::Elite as SelectElite;
pub use super::tournament::Tournament as SelectTournament;
pub use super::Select;

use crate::genotype::EvolveGenotype;
use crate::strategy::evolve::{EvolveConfig, EvolveState};
use crate::strategy::StrategyReporter;
use rand::prelude::*;

#[derive(Clone, Debug)]
pub enum Wrapper<G: EvolveGenotype> {
    Elite(SelectElite<G>),
    Tournament(SelectTournament<G>),
}

impl<G: EvolveGenotype> Select for Wrapper<G> {
    type Genotype = G;

    fn before(&mut self, genotype: &G, state: &mut EvolveState<G>, config: &EvolveConfig) {
        match self {
            Wrapper::Elite(select) => select.before(genotype, state, config),
            Wrapper::Tournament(select) => select.before(genotype, state, config),
        }
    }

    fn call<R: Rng, SR: StrategyReporter<Genotype = G>>(
        &mut self,
        genotype: &G,
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

    fn after(&mut self, genotype: &G, state: &mut EvolveState<G>, config: &EvolveConfig) {
        match self {
            Wrapper::Elite(select) => select.after(genotype, state, config),
            Wrapper::Tournament(select) => select.after(genotype, state, config),
        }
    }
}

impl<G: EvolveGenotype> From<SelectElite<G>> for Wrapper<G> {
    fn from(select: SelectElite<G>) -> Self {
        Wrapper::Elite(select)
    }
}
impl<G: EvolveGenotype> From<SelectTournament<G>> for Wrapper<G> {
    fn from(select: SelectTournament<G>) -> Self {
        Wrapper::Tournament(select)
    }
}
