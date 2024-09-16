//! When approacking a (local) optimum in the fitness score, the variation in the population goes
//! down dramatically. This reduces the efficiency, but also has the risk of local optimum lock-in.
//! To increase the variation in the population, an [extension](crate::extension) mechanisms can optionally be used
mod mass_degeneration;
mod mass_extinction;
mod mass_genesis;
mod noop;
mod wrapper;

pub use self::mass_degeneration::MassDegeneration as ExtensionMassDegeneration;
pub use self::mass_extinction::MassExtinction as ExtensionMassExtinction;
pub use self::mass_genesis::MassGenesis as ExtensionMassGenesis;
pub use self::noop::Noop as ExtensionNoop;
pub use self::wrapper::Wrapper as ExtensionWrapper;

use crate::genotype::EvolveGenotype;
use crate::strategy::evolve::{EvolveConfig, EvolveState};
use crate::strategy::StrategyReporter;
use rand::Rng;

pub trait Extension: Clone + Send + Sync + std::fmt::Debug {
    fn call<G: EvolveGenotype, R: Rng, SR: StrategyReporter<Genotype = G>>(
        &mut self,
        genotype: &mut G,
        state: &mut EvolveState<G>,
        config: &EvolveConfig,
        reporter: &mut SR,
        rng: &mut R,
    );
}

#[derive(Clone, Debug)]
pub enum ExtensionEvent {
    MassDegeneration(String),
    MassExtinction(String),
    MassGenesis(String),
}
