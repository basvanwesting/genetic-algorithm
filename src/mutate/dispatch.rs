pub use super::dynamic_once::DynamicOnce as MutateDynamicOnce;
pub use super::dynamic_rounds::DynamicRounds as MutateDynamicRounds;
pub use super::once::Once as MutateOnce;
pub use super::twice::Twice as MutateTwice;
pub use super::Mutate;

use crate::genotype::Genotype;
use crate::population::Population;
use rand::Rng;

#[derive(Clone, Debug, Default)]
pub enum Implementations {
    #[default]
    Once,
    Twice,
    DynamicOnce,
    DynamicRounds,
}

/// Wrapper for use in benchmarks or [meta analysis](https://github.com/basvanwesting/genetic-algorithm-meta.git)
#[derive(Clone, Debug, Default)]
pub struct Dispatch {
    pub implementation: Implementations,
    pub mutation_probability: f32,
    pub mutation_probability_step: f32,
    pub target_uniformity: f32,
}
impl Mutate for Dispatch {
    fn call<T: Genotype, R: Rng>(
        &mut self,
        genotype: &T,
        population: &mut Population<T>,
        rng: &mut R,
    ) {
        match self.implementation {
            Implementations::Once => {
                MutateOnce::new(self.mutation_probability).call(genotype, population, rng)
            }
            Implementations::Twice => {
                MutateTwice::new(self.mutation_probability).call(genotype, population, rng)
            }
            Implementations::DynamicOnce => {
                MutateDynamicOnce::new(self.mutation_probability_step, self.target_uniformity)
                    .call(genotype, population, rng)
            }
            Implementations::DynamicRounds => {
                MutateDynamicRounds::new(self.mutation_probability, self.target_uniformity)
                    .call(genotype, population, rng)
            }
        }
    }
    fn report(&self) -> String {
        "dispatch".to_string()
    }
}

impl From<MutateOnce> for Dispatch {
    fn from(implementation: MutateOnce) -> Self {
        Dispatch {
            implementation: Implementations::Once,
            mutation_probability: implementation.mutation_probability,
            ..Default::default()
        }
    }
}

impl From<MutateTwice> for Dispatch {
    fn from(implementation: MutateTwice) -> Self {
        Dispatch {
            implementation: Implementations::Twice,
            mutation_probability: implementation.mutation_probability,
            ..Default::default()
        }
    }
}

impl From<MutateDynamicOnce> for Dispatch {
    fn from(implementation: MutateDynamicOnce) -> Self {
        Dispatch {
            implementation: Implementations::DynamicOnce,
            mutation_probability_step: implementation.mutation_probability_step,
            target_uniformity: implementation.target_uniformity,
            ..Default::default()
        }
    }
}

impl From<MutateDynamicRounds> for Dispatch {
    fn from(implementation: MutateDynamicRounds) -> Self {
        Dispatch {
            implementation: Implementations::DynamicRounds,
            mutation_probability: implementation.mutation_probability,
            target_uniformity: implementation.target_uniformity,
            ..Default::default()
        }
    }
}
