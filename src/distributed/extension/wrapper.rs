pub use super::mass_deduplication::MassDeduplication as ExtensionMassDeduplication;
pub use super::mass_degeneration::MassDegeneration as ExtensionMassDegeneration;
pub use super::mass_extinction::MassExtinction as ExtensionMassExtinction;
pub use super::mass_genesis::MassGenesis as ExtensionMassGenesis;
pub use super::noop::Noop as ExtensionNoop;
pub use super::Extension;

use crate::distributed::genotype::EvolveGenotype;
use crate::distributed::strategy::evolve::{EvolveConfig, EvolveState};
use crate::distributed::strategy::StrategyReporter;
use rand::Rng;

#[derive(Clone, Debug)]
pub enum Wrapper {
    MassDeduplication(ExtensionMassDeduplication),
    MassDegeneration(ExtensionMassDegeneration),
    MassExtinction(ExtensionMassExtinction),
    MassGenesis(ExtensionMassGenesis),
    Noop(ExtensionNoop),
}

impl Extension for Wrapper {
    fn call<G: EvolveGenotype, R: Rng, SR: StrategyReporter<Genotype = G>>(
        &mut self,
        genotype: &G,
        state: &mut EvolveState<G>,
        config: &EvolveConfig,
        reporter: &mut SR,
        rng: &mut R,
    ) {
        match self {
            Wrapper::MassDeduplication(extension) => {
                extension.call(genotype, state, config, reporter, rng)
            }
            Wrapper::MassDegeneration(extension) => {
                extension.call(genotype, state, config, reporter, rng)
            }
            Wrapper::MassExtinction(extension) => {
                extension.call(genotype, state, config, reporter, rng)
            }
            Wrapper::MassGenesis(extension) => {
                extension.call(genotype, state, config, reporter, rng)
            }
            Wrapper::Noop(extension) => extension.call(genotype, state, config, reporter, rng),
        }
    }
}

impl From<ExtensionMassDeduplication> for Wrapper {
    fn from(extension: ExtensionMassDeduplication) -> Self {
        Wrapper::MassDeduplication(extension)
    }
}
impl From<ExtensionMassDegeneration> for Wrapper {
    fn from(extension: ExtensionMassDegeneration) -> Self {
        Wrapper::MassDegeneration(extension)
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
impl From<ExtensionNoop> for Wrapper {
    fn from(extension: ExtensionNoop) -> Self {
        Wrapper::Noop(extension)
    }
}
