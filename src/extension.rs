//! The extension strategy, useful for avoiding local optimum lock-in, but generic in nature
mod mass_extinction;
mod noop;

pub use self::mass_extinction::MassExtinction as ExtensionMassExtinction;
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
            Extensions::Noop => {}
        }
    }
}
