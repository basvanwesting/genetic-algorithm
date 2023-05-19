//! The mutation strategy, very important for avoiding local optimum lock-in. But don't overdo it,
//! as it degenerates the population too much if overused. Use a mutation probability generally between
//! 5% and 20%.
mod dynamic_once;
mod once;

pub use self::dynamic_once::DynamicOnce as MutateDynamicOnce;
pub use self::once::Once as MutateOnce;

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
}

#[derive(Clone, Debug, Default)]
pub enum Mutates {
    #[default]
    Once,
    DynamicOnce,
}
pub type MutationProbability = f32;
pub type TargetUniformity = f32;

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
            Mutates::Once => MutateOnce(self.mutation_probability).call(genotype, population, rng),
            Mutates::DynamicOnce => {
                MutateDynamicOnce::new(self.mutation_probability_step, self.target_uniformity)
                    .call(genotype, population, rng)
            }
        }
    }
}
