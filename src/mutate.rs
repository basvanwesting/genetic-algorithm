//! The mutation strategy, very important for avoiding local optimum lock-in. But don't overdo it,
//! as it degenerates the population too much if overused. Use a mutation probability generally between
//! 5% and 20%.
mod neighbour;
mod random;
mod random_or_neighbour;

pub use self::neighbour::Neighbour as MutateNeighbour;
pub use self::random::Random as MutateRandom;
pub use self::random::Random as MutateOnce; // backwards compatibility
pub use self::random_or_neighbour::RandomOrNeighbour as MutateRandomOrNeighbour;

use crate::genotype::Genotype;
use crate::population::Population;
use rand::Rng;

pub trait Mutate: Clone + std::fmt::Debug {
    fn call<T: Genotype, R: Rng>(&self, genotype: &T, population: &mut Population<T>, rng: &mut R);
}

#[derive(Clone, Debug)]
pub enum Mutates {
    Random,
    Neighbour,
    RandomOrNeighbour,
}
pub type RandomMutationProbability = f32;
pub type NeighbourMutationProbability = f32;

/// Wrapper for use in [meta analysis](crate::meta)
#[derive(Clone, Debug)]
pub struct MutateDispatch(
    pub Mutates,
    pub RandomMutationProbability,
    pub NeighbourMutationProbability,
);
impl Mutate for MutateDispatch {
    fn call<T: Genotype, R: Rng>(&self, genotype: &T, population: &mut Population<T>, rng: &mut R) {
        match self.0 {
            Mutates::Random => MutateRandom(self.1).call(genotype, population, rng),
            Mutates::Neighbour => MutateNeighbour(self.2).call(genotype, population, rng),
            Mutates::RandomOrNeighbour => {
                MutateRandomOrNeighbour(self.1, self.2).call(genotype, population, rng)
            }
        }
    }
}
