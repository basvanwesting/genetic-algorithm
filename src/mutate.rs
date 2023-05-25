//! The mutation strategy, very important for avoiding local optimum lock-in. But don't overdo it,
//! as it degenerates the population too much if overused. Use a mutation probability generally between
//! 5% and 20%.
mod dispatch;
mod dynamic_once;
mod dynamic_rounds;
mod once;
mod twice;

pub use self::dispatch::Dispatch as MutateDispatch;
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
