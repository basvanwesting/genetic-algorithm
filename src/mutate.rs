//! The mutation strategy, very important for avoiding local optimum lock-in. But don't overdo it,
//! as it degenerates the population too much if overused. Use a mutation probability generally between
//! 5% and 20%.
mod once;

pub use self::once::Once as MutateOnce;

use crate::genotype::Genotype;
use crate::population::Population;
use rand::Rng;

pub trait Mutate: Clone + std::fmt::Debug {
    fn call<T: Genotype, R: Rng>(&self, genotype: &T, population: &mut Population<T>, rng: &mut R);
}

#[derive(Clone, Debug)]
pub enum Mutates {
    Once,
}
pub type MutationProbability = f32;

/// Wrapper for use in [meta analysis](crate::meta)
#[derive(Clone, Debug)]
pub struct MutateDispatch(pub Mutates, pub MutationProbability);
impl Mutate for MutateDispatch {
    fn call<T: Genotype, R: Rng>(&self, genotype: &T, population: &mut Population<T>, rng: &mut R) {
        match self.0 {
            Mutates::Once => MutateOnce(self.1).call(genotype, population, rng),
        }
    }
}
