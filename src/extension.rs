//! When approacking a (local) optimum in the fitness score, the variation in the population goes
//! down dramatically. This reduces the efficiency, but also has the risk of local optimum lock-in.
//! To increase the variation in the population, an [extension](crate::extension) mechanisms can optionally be used
mod mass_degeneration;
mod mass_extinction;
mod mass_genesis;
mod mass_invasion;
mod noop;
mod wrapper;

pub use self::mass_degeneration::MassDegeneration as ExtensionMassDegeneration;
pub use self::mass_extinction::MassExtinction as ExtensionMassExtinction;
pub use self::mass_genesis::MassGenesis as ExtensionMassGenesis;
pub use self::mass_invasion::MassInvasion as ExtensionMassInvasion;
pub use self::noop::Noop as ExtensionNoop;
pub use self::wrapper::Wrapper as ExtensionWrapper;

use crate::genotype::Genotype;
use crate::strategy::evolve::{EvolveConfig, EvolveReporter, EvolveState};
use rand::Rng;

pub trait Extension: Clone + std::fmt::Debug {
    fn call<G: Genotype, R: Rng, SR: EvolveReporter>(
        &mut self,
        genotype: &G,
        state: &mut EvolveState<G::Allele>,
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
    MassInvasion(String),
}
