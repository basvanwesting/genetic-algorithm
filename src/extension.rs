//! When approacking a (local) optimum in the fitness score, the variation in the population goes
//! down dramatically. This reduces the efficiency, but also has the risk of local optimum lock-in.
//! To increase the variation in the population, an [extension](crate::extension) mechanisms can optionally be used
//mod dispatch;
mod mass_degeneration;
mod mass_extinction;
mod mass_genesis;
mod mass_invasion;
mod noop;

//pub use self::dispatch::Dispatch as ExtensionDispatch;
pub use self::mass_degeneration::MassDegeneration as ExtensionMassDegeneration;
pub use self::mass_extinction::MassExtinction as ExtensionMassExtinction;
pub use self::mass_genesis::MassGenesis as ExtensionMassGenesis;
pub use self::mass_invasion::MassInvasion as ExtensionMassInvasion;
pub use self::noop::Noop as ExtensionNoop;

use crate::genotype::Genotype;
use crate::population::Population;
use crate::strategy::evolve::{EvolveConfig, EvolveState};
use rand::Rng;

#[derive(Clone, Debug)]
pub enum Extension {
    Noop(ExtensionNoop),
    MassExtinction(ExtensionMassExtinction),
    MassGenesis(ExtensionMassGenesis),
    MassInvasion(ExtensionMassInvasion),
    MassDegeneration(ExtensionMassDegeneration),
}

impl Extension {
    pub fn call<G: Genotype, R: Rng>(
        &mut self,
        genotype: &G,
        evolve_config: &EvolveConfig,
        evolve_state: &EvolveState<G>,
        population: &mut Population<G>,
        rng: &mut R,
    ) {
        match self {
            Extension::Noop(extension) => {
                extension.call(genotype, evolve_config, evolve_state, population, rng)
            }
            Extension::MassExtinction(extension) => {
                extension.call(genotype, evolve_config, evolve_state, population, rng)
            }
            Extension::MassGenesis(extension) => {
                extension.call(genotype, evolve_config, evolve_state, population, rng)
            }
            Extension::MassInvasion(extension) => {
                extension.call(genotype, evolve_config, evolve_state, population, rng)
            }
            Extension::MassDegeneration(extension) => {
                extension.call(genotype, evolve_config, evolve_state, population, rng)
            }
        }
    }
}

impl From<ExtensionNoop> for Extension {
    fn from(extension: ExtensionNoop) -> Self {
        Extension::Noop(extension)
    }
}
impl From<ExtensionMassExtinction> for Extension {
    fn from(extension: ExtensionMassExtinction) -> Self {
        Extension::MassExtinction(extension)
    }
}
impl From<ExtensionMassGenesis> for Extension {
    fn from(extension: ExtensionMassGenesis) -> Self {
        Extension::MassGenesis(extension)
    }
}
impl From<ExtensionMassInvasion> for Extension {
    fn from(extension: ExtensionMassInvasion) -> Self {
        Extension::MassInvasion(extension)
    }
}
impl From<ExtensionMassDegeneration> for Extension {
    fn from(extension: ExtensionMassDegeneration) -> Self {
        Extension::MassDegeneration(extension)
    }
}
