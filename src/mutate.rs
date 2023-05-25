//! The mutation strategy, very important for avoiding local optimum lock-in. But don't overdo it,
//! as it degenerates the population too much if overused. Use a mutation probability generally between
//! 5% and 20%.
//mod dispatch;
mod dynamic_once;
mod dynamic_rounds;
mod once;
mod twice;

//pub use self::dispatch::Dispatch as MutateDispatch;
pub use self::dynamic_once::DynamicOnce as MutateDynamicOnce;
pub use self::dynamic_rounds::DynamicRounds as MutateDynamicRounds;
pub use self::once::Once as MutateOnce;
pub use self::twice::Twice as MutateTwice;

use crate::genotype::Genotype;
use crate::population::Population;
use rand::Rng;

#[derive(Clone, Debug)]
pub enum Mutate {
    Once(MutateOnce),
    Twice(MutateTwice),
    DynamicOnce(MutateDynamicOnce),
    DynamicRounds(MutateDynamicRounds),
}

impl Mutate {
    pub fn call<T: Genotype, R: Rng>(
        &mut self,
        genotype: &T,
        population: &mut Population<T>,
        rng: &mut R,
    ) {
        match self {
            Mutate::Once(mutate) => mutate.call(genotype, population, rng),
            Mutate::Twice(mutate) => mutate.call(genotype, population, rng),
            Mutate::DynamicOnce(mutate) => mutate.call(genotype, population, rng),
            Mutate::DynamicRounds(mutate) => mutate.call(genotype, population, rng),
        }
    }
    pub fn report(&self) -> String {
        match self {
            Mutate::Once(mutate) => mutate.report(),
            Mutate::Twice(mutate) => mutate.report(),
            Mutate::DynamicOnce(mutate) => mutate.report(),
            Mutate::DynamicRounds(mutate) => mutate.report(),
        }
    }
}

impl From<MutateOnce> for Mutate {
    fn from(mutate: MutateOnce) -> Self {
        Mutate::Once(mutate)
    }
}
impl From<MutateTwice> for Mutate {
    fn from(mutate: MutateTwice) -> Self {
        Mutate::Twice(mutate)
    }
}
impl From<MutateDynamicOnce> for Mutate {
    fn from(mutate: MutateDynamicOnce) -> Self {
        Mutate::DynamicOnce(mutate)
    }
}
impl From<MutateDynamicRounds> for Mutate {
    fn from(mutate: MutateDynamicRounds) -> Self {
        Mutate::DynamicRounds(mutate)
    }
}
