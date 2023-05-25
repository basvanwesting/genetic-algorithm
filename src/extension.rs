//! When approacking a (local) optimum in the fitness score, the variation in the population goes
//! down dramatically. This reduces the efficiency, but also has the risk of local optimum lock-in.
//! To increase the variation in the population, an [extension](crate::extension) mechanisms can optionally be used
mod dispatch;
mod mass_degeneration;
mod mass_extinction;
mod mass_genesis;
mod mass_invasion;
mod noop;

pub use self::dispatch::Dispatch as ExtensionDispatch;
pub use self::mass_degeneration::MassDegeneration as ExtensionMassDegeneration;
pub use self::mass_extinction::MassExtinction as ExtensionMassExtinction;
pub use self::mass_genesis::MassGenesis as ExtensionMassGenesis;
pub use self::mass_invasion::MassInvasion as ExtensionMassInvasion;
pub use self::noop::Noop as ExtensionNoop;

use crate::genotype::Genotype;
use crate::population::Population;
use crate::strategy::evolve::{EvolveConfig, EvolveState};
use rand::Rng;

pub trait Extension: Clone + std::fmt::Debug {
    fn call<G: Genotype, R: Rng>(
        &mut self,
        genotype: &G,
        evolve_config: &EvolveConfig,
        evolve_state: &EvolveState<G>,
        population: &mut Population<G>,
        rng: &mut R,
    );
}
