//! The extension strategy, useful for avoiding local optimum lock-in, but generic in nature
mod mass_degeneration;
mod mass_extinction;
mod mass_genesis;
mod mass_invasion;
mod noop;

pub use self::mass_degeneration::MassDegeneration as ExtensionMassDegeneration;
pub use self::mass_extinction::MassExtinction as ExtensionMassExtinction;
pub use self::mass_genesis::MassGenesis as ExtensionMassGenesis;
pub use self::mass_invasion::MassInvasion as ExtensionMassInvasion;
pub use self::noop::Noop as ExtensionNoop;

use crate::compete::Compete;
use crate::crossover::Crossover;
use crate::fitness::Fitness;
use crate::genotype::Genotype;
use crate::mutate::Mutate;
use crate::population::Population;
use crate::strategy::evolve::Evolve;
use rand::Rng;

pub trait Extension: Clone + std::fmt::Debug {
    fn call<
        G: Genotype,
        M: Mutate,
        F: Fitness<Genotype = G>,
        S: Crossover,
        C: Compete,
        E: Extension,
        R: Rng,
    >(
        &self,
        evolve: &Evolve<G, M, F, S, C, E>,
        population: &mut Population<G>,
        rng: &mut R,
    );
}

#[derive(Clone, Debug, Default)]
pub enum Extensions {
    #[default]
    Noop,
    MassExtinction,
    MassGenesis,
    MassInvasion,
    MassDegeneration,
}

/// Wrapper for use in [meta analysis](crate::meta)
#[derive(Clone, Debug, Default)]
pub struct ExtensionDispatch {
    pub extension: Extensions,
    pub uniformity_threshold: f32,
    pub survival_rate: f32,
    pub number_of_rounds: usize,
}

impl Extension for ExtensionDispatch {
    fn call<
        G: Genotype,
        M: Mutate,
        F: Fitness<Genotype = G>,
        S: Crossover,
        C: Compete,
        E: Extension,
        R: Rng,
    >(
        &self,
        evolve: &Evolve<G, M, F, S, C, E>,
        population: &mut Population<G>,
        rng: &mut R,
    ) {
        match self.extension {
            Extensions::MassExtinction => ExtensionMassExtinction {
                uniformity_threshold: self.uniformity_threshold,
                survival_rate: self.survival_rate,
            }
            .call(evolve, population, rng),
            Extensions::MassGenesis => ExtensionMassGenesis {
                uniformity_threshold: self.uniformity_threshold,
            }
            .call(evolve, population, rng),
            Extensions::MassInvasion => ExtensionMassInvasion {
                uniformity_threshold: self.uniformity_threshold,
                survival_rate: self.survival_rate,
            }
            .call(evolve, population, rng),
            Extensions::MassDegeneration => ExtensionMassDegeneration {
                uniformity_threshold: self.uniformity_threshold,
                number_of_rounds: self.number_of_rounds,
            }
            .call(evolve, population, rng),
            Extensions::Noop => {}
        }
    }
}
