pub use super::dynamic_once::DynamicOnce as MutateDynamicOnce;
pub use super::dynamic_rounds::DynamicRounds as MutateDynamicRounds;
pub use super::once::Once as MutateOnce;
pub use super::twice::Twice as MutateTwice;
pub use super::Mutate;

use crate::genotype::Genotype;
use crate::population::Population;
use rand::Rng;

#[derive(Clone, Debug)]
pub enum Wrapper {
    Once(MutateOnce),
    Twice(MutateTwice),
    DynamicOnce(MutateDynamicOnce),
    DynamicRounds(MutateDynamicRounds),
}

impl Mutate for Wrapper {
    fn call<T: Genotype, R: Rng>(
        &mut self,
        genotype: &T,
        population: &mut Population<T>,
        rng: &mut R,
    ) {
        match self {
            Wrapper::Once(mutate) => mutate.call(genotype, population, rng),
            Wrapper::Twice(mutate) => mutate.call(genotype, population, rng),
            Wrapper::DynamicOnce(mutate) => mutate.call(genotype, population, rng),
            Wrapper::DynamicRounds(mutate) => mutate.call(genotype, population, rng),
        }
    }
    fn report(&self) -> String {
        match self {
            Wrapper::Once(mutate) => mutate.report(),
            Wrapper::Twice(mutate) => mutate.report(),
            Wrapper::DynamicOnce(mutate) => mutate.report(),
            Wrapper::DynamicRounds(mutate) => mutate.report(),
        }
    }
}

impl From<MutateOnce> for Wrapper {
    fn from(mutate: MutateOnce) -> Self {
        Wrapper::Once(mutate)
    }
}
impl From<MutateTwice> for Wrapper {
    fn from(mutate: MutateTwice) -> Self {
        Wrapper::Twice(mutate)
    }
}
impl From<MutateDynamicOnce> for Wrapper {
    fn from(mutate: MutateDynamicOnce) -> Self {
        Wrapper::DynamicOnce(mutate)
    }
}
impl From<MutateDynamicRounds> for Wrapper {
    fn from(mutate: MutateDynamicRounds) -> Self {
        Wrapper::DynamicRounds(mutate)
    }
}
