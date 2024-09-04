pub use super::mass_degeneration::MassDegeneration as ExtensionMassDegeneration;
pub use super::mass_extinction::MassExtinction as ExtensionMassExtinction;
pub use super::mass_genesis::MassGenesis as ExtensionMassGenesis;
pub use super::noop::Noop as ExtensionNoop;
pub use super::Extension;

use crate::genotype::Genotype;
use crate::strategy::evolve::{EvolveConfig, EvolveReporter, EvolveState};
use rand::Rng;

#[derive(Clone, Debug)]
pub enum Wrapper {
    Noop(ExtensionNoop),
    MassExtinction(ExtensionMassExtinction),
    MassGenesis(ExtensionMassGenesis),
    MassDegeneration(ExtensionMassDegeneration),
}

impl Extension for Wrapper {
    fn call<G: Genotype, R: Rng, SR: EvolveReporter<Genotype = G>>(
        &mut self,
        genotype: &mut G,
        state: &mut EvolveState<G>,
        config: &EvolveConfig,
        reporter: &mut SR,
        rng: &mut R,
    ) {
        match self {
            Wrapper::Noop(extension) => extension.call(genotype, state, config, reporter, rng),
            Wrapper::MassExtinction(extension) => {
                extension.call(genotype, state, config, reporter, rng)
            }
            Wrapper::MassGenesis(extension) => {
                extension.call(genotype, state, config, reporter, rng)
            }
            Wrapper::MassDegeneration(extension) => {
                extension.call(genotype, state, config, reporter, rng)
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
impl From<ExtensionMassDegeneration> for Wrapper {
    fn from(extension: ExtensionMassDegeneration) -> Self {
        Wrapper::MassDegeneration(extension)
    }
}
