pub use super::mass_degeneration::MassDegeneration as ExtensionMassDegeneration;
pub use super::mass_extinction::MassExtinction as ExtensionMassExtinction;
pub use super::mass_genesis::MassGenesis as ExtensionMassGenesis;
pub use super::mass_invasion::MassInvasion as ExtensionMassInvasion;
pub use super::noop::Noop as ExtensionNoop;
pub use super::Extension;

use crate::genotype::Genotype;
use crate::population::Population;
use crate::strategy::evolve::{EvolveConfig, EvolveState};
use rand::Rng;

#[derive(Clone, Debug, Default)]
pub enum Implementations {
    #[default]
    Noop,
    MassExtinction,
    MassGenesis,
    MassInvasion,
    MassDegeneration,
}

/// Wrapper for use in [meta analysis](crate::meta)
#[derive(Clone, Debug, Default)]
pub struct Dispatch {
    pub implementation: Implementations,
    pub uniformity_threshold: f32,
    pub survival_rate: f32,
    pub number_of_rounds: usize,
}

impl Extension for Dispatch {
    fn call<G: Genotype, R: Rng>(
        &mut self,
        genotype: &G,
        evolve_config: &EvolveConfig,
        evolve_state: &EvolveState<G>,
        population: &mut Population<G>,
        rng: &mut R,
    ) {
        match self.implementation {
            Implementations::MassExtinction => ExtensionMassExtinction {
                uniformity_threshold: self.uniformity_threshold,
                survival_rate: self.survival_rate,
            }
            .call(genotype, evolve_config, evolve_state, population, rng),
            Implementations::MassGenesis => ExtensionMassGenesis {
                uniformity_threshold: self.uniformity_threshold,
            }
            .call(genotype, evolve_config, evolve_state, population, rng),
            Implementations::MassInvasion => ExtensionMassInvasion {
                uniformity_threshold: self.uniformity_threshold,
                survival_rate: self.survival_rate,
            }
            .call(genotype, evolve_config, evolve_state, population, rng),
            Implementations::MassDegeneration => ExtensionMassDegeneration {
                uniformity_threshold: self.uniformity_threshold,
                number_of_rounds: self.number_of_rounds,
            }
            .call(genotype, evolve_config, evolve_state, population, rng),
            Implementations::Noop => {}
        }
    }
}

impl From<ExtensionNoop> for Dispatch {
    fn from(_implementation: ExtensionNoop) -> Self {
        Dispatch {
            implementation: Implementations::Noop,
            ..Default::default()
        }
    }
}

impl From<ExtensionMassExtinction> for Dispatch {
    fn from(implementation: ExtensionMassExtinction) -> Self {
        Dispatch {
            implementation: Implementations::MassExtinction,
            uniformity_threshold: implementation.uniformity_threshold,
            survival_rate: implementation.survival_rate,
            ..Default::default()
        }
    }
}

impl From<ExtensionMassGenesis> for Dispatch {
    fn from(implementation: ExtensionMassGenesis) -> Self {
        Dispatch {
            implementation: Implementations::MassGenesis,
            uniformity_threshold: implementation.uniformity_threshold,
            ..Default::default()
        }
    }
}

impl From<ExtensionMassInvasion> for Dispatch {
    fn from(implementation: ExtensionMassInvasion) -> Self {
        Dispatch {
            implementation: Implementations::MassInvasion,
            uniformity_threshold: implementation.uniformity_threshold,
            survival_rate: implementation.survival_rate,
            ..Default::default()
        }
    }
}

impl From<ExtensionMassDegeneration> for Dispatch {
    fn from(implementation: ExtensionMassDegeneration) -> Self {
        Dispatch {
            implementation: Implementations::MassDegeneration,
            uniformity_threshold: implementation.uniformity_threshold,
            number_of_rounds: implementation.number_of_rounds,
            ..Default::default()
        }
    }
}
