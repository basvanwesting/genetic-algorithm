pub use super::mass_degeneration::MassDegeneration as ExtensionMassDegeneration;
pub use super::mass_extinction::MassExtinction as ExtensionMassExtinction;
pub use super::mass_genesis::MassGenesis as ExtensionMassGenesis;
pub use super::mass_invasion::MassInvasion as ExtensionMassInvasion;
pub use super::noop::Noop as ExtensionNoop;
pub use super::Extension;

use crate::genotype::Genotype;
use crate::population::Population;
use crate::strategy::evolve::EvolveConfig;
use rand::Rng;

#[derive(Clone, Debug)]
pub enum Wrapper {
    Noop(ExtensionNoop),
    MassExtinction(ExtensionMassExtinction),
    MassGenesis(ExtensionMassGenesis),
    MassInvasion(ExtensionMassInvasion),
    MassDegeneration(ExtensionMassDegeneration),
}

impl Extension for Wrapper {
    fn call<G: Genotype, R: Rng>(
        &mut self,
        genotype: &G,
        evolve_config: &EvolveConfig,
        population: &mut Population<G>,
        rng: &mut R,
    ) {
        match self {
            Wrapper::Noop(extension) => extension.call(genotype, evolve_config, population, rng),
            Wrapper::MassExtinction(extension) => {
                extension.call(genotype, evolve_config, population, rng)
            }
            Wrapper::MassGenesis(extension) => {
                extension.call(genotype, evolve_config, population, rng)
            }
            Wrapper::MassInvasion(extension) => {
                extension.call(genotype, evolve_config, population, rng)
            }
            Wrapper::MassDegeneration(extension) => {
                extension.call(genotype, evolve_config, population, rng)
            }
        }
    }
}

impl From<ExtensionNoop> for Wrapper {
    fn from(extension: ExtensionNoop) -> Self {
        Wrapper::Noop(extension)
    }
}
impl From<ExtensionMassExtinction> for Wrapper {
    fn from(extension: ExtensionMassExtinction) -> Self {
        Wrapper::MassExtinction(extension)
    }
}
impl From<ExtensionMassGenesis> for Wrapper {
    fn from(extension: ExtensionMassGenesis) -> Self {
        Wrapper::MassGenesis(extension)
    }
}
impl From<ExtensionMassInvasion> for Wrapper {
    fn from(extension: ExtensionMassInvasion) -> Self {
        Wrapper::MassInvasion(extension)
    }
}
impl From<ExtensionMassDegeneration> for Wrapper {
    fn from(extension: ExtensionMassDegeneration) -> Self {
        Wrapper::MassDegeneration(extension)
    }
}
