//! The mutation strategy, very important for avoiding local optimum lock-in. But don't overdo it,
//! as it degenerates the population too much if overused. Use a mutation probability generally between
//! 5% and 20%.
mod dynamic_rounds;
mod single_gene_distance;
mod single_gene_random;
mod single_gene_random_dynamic;
mod twice;
mod wrapper;

pub use self::dynamic_rounds::DynamicRounds as MutateDynamicRounds;
pub use self::single_gene_distance::SingleGeneDistance as MutateSingleGeneDistance;
pub use self::single_gene_random::SingleGeneRandom as MutateSingleGeneRandom;
pub use self::single_gene_random_dynamic::SingleGeneRandomDynamic as MutateSingleGeneRandomDynamic;
pub use self::twice::Twice as MutateTwice;
pub use self::wrapper::Wrapper as MutateWrapper;

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
