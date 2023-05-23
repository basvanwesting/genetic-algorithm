//! The mutation strategy, very important for avoiding local optimum lock-in. But don't overdo it,
//! as it degenerates the population too much if overused. Use a mutation probability generally between
//! 5% and 20%.
mod dynamic_once;
mod dynamic_rounds;
mod once;
mod twice;

pub use self::dynamic_once::DynamicOnce as MutateDynamicOnce;
pub use self::dynamic_rounds::DynamicRounds as MutateDynamicRounds;
pub use self::once::Once as MutateOnce;
pub use self::twice::Twice as MutateTwice;

use crate::genotype::Genotype;
use crate::population::Population;
use rand::Rng;

pub trait Mutate: Clone + std::fmt::Debug {
    fn call<T: Genotype, R: Rng>(
        &mut self,
        genotype: &T,
        population: &mut Population<T>,
        rng: &mut R,
    );
    fn report(&self) -> String;
}

#[derive(Clone, Debug, Default)]
pub enum Mutates {
    #[default]
    Once,
    Twice,
    DynamicOnce,
    DynamicRounds,
}

/// Wrapper for use in [meta analysis](crate::meta)
#[derive(Clone, Debug, Default)]
pub struct MutateDispatch {
    pub mutate: Mutates,
    pub mutation_probability: f32,
    pub mutation_probability_step: f32,
    pub target_uniformity: f32,
}
impl Mutate for MutateDispatch {
    fn call<T: Genotype, R: Rng>(
        &mut self,
        genotype: &T,
        population: &mut Population<T>,
        rng: &mut R,
    ) {
        match self.mutate {
            Mutates::Once => {
                MutateOnce::new(self.mutation_probability).call(genotype, population, rng)
            }
            Mutates::Twice => {
                MutateTwice::new(self.mutation_probability).call(genotype, population, rng)
            }
            Mutates::DynamicOnce => {
                MutateDynamicOnce::new(self.mutation_probability_step, self.target_uniformity)
                    .call(genotype, population, rng)
            }
            Mutates::DynamicRounds => {
                MutateDynamicRounds::new(self.mutation_probability, self.target_uniformity)
                    .call(genotype, population, rng)
            }
        }
    }
    fn report(&self) -> String {
        "dispatch".to_string()
    }
}
