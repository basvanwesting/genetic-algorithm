//! The extension strategy, useful for avoiding local optimum lock-in, but generic in nature
mod mass_extinction;
mod noop;

pub use self::mass_extinction::MassExtinction as ExtensionMassExtinction;
pub use self::noop::Noop as ExtensionNoop;

use crate::genotype::Genotype;
use crate::population::Population;
use rand::Rng;

pub trait Extension: Clone + std::fmt::Debug {
    fn call<T: Genotype, R: Rng>(&self, genotype: &T, population: &mut Population<T>, rng: &mut R);
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
    pub minimal_population_size: usize,
    pub number_of_rounds: usize,
}

impl Extension for ExtensionDispatch {
    fn call<T: Genotype, R: Rng>(&self, genotype: &T, population: &mut Population<T>, rng: &mut R) {
        match self.extension {
            Extensions::MassExtinction => ExtensionMassExtinction {
                uniformity_threshold: self.uniformity_threshold,
                survival_rate: self.survival_rate,
                minimal_population_size: self.minimal_population_size,
            }
            .call(genotype, population, rng),
            Extensions::Noop => {}
        }
    }
}
