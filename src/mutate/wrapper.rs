pub use super::dynamic_rounds::DynamicRounds as MutateDynamicRounds;
pub use super::single_gene_distance::SingleGeneDistance as MutateSingleGeneDistance;
pub use super::single_gene_random::SingleGeneRandom as MutateSingleGeneRandom;
pub use super::single_gene_random_dynamic::SingleGeneRandomDynamic as MutateSingleGeneRandomDynamic;
pub use super::twice::Twice as MutateTwice;
pub use super::Mutate;

use crate::genotype::Genotype;
use crate::population::Population;
use rand::Rng;

#[derive(Clone, Debug)]
pub enum Wrapper {
    DynamicRounds(MutateDynamicRounds),
    SingleGeneDistance(MutateSingleGeneDistance),
    SingleGeneRandom(MutateSingleGeneRandom),
    SingleGeneRandomDynamic(MutateSingleGeneRandomDynamic),
    Twice(MutateTwice),
}

impl Mutate for Wrapper {
    fn call<T: Genotype, R: Rng>(
        &mut self,
        genotype: &T,
        population: &mut Population<T>,
        rng: &mut R,
    ) {
        match self {
            Wrapper::DynamicRounds(mutate) => mutate.call(genotype, population, rng),
            Wrapper::SingleGeneDistance(mutate) => mutate.call(genotype, population, rng),
            Wrapper::SingleGeneRandom(mutate) => mutate.call(genotype, population, rng),
            Wrapper::SingleGeneRandomDynamic(mutate) => mutate.call(genotype, population, rng),
            Wrapper::Twice(mutate) => mutate.call(genotype, population, rng),
        }
    }
    fn report(&self) -> String {
        match self {
            Wrapper::DynamicRounds(mutate) => mutate.report(),
            Wrapper::SingleGeneDistance(mutate) => mutate.report(),
            Wrapper::SingleGeneRandom(mutate) => mutate.report(),
            Wrapper::SingleGeneRandomDynamic(mutate) => mutate.report(),
            Wrapper::Twice(mutate) => mutate.report(),
        }
    }
}

impl From<MutateSingleGeneRandom> for Wrapper {
    fn from(mutate: MutateSingleGeneRandom) -> Self {
        Wrapper::SingleGeneRandom(mutate)
    }
}
impl From<MutateTwice> for Wrapper {
    fn from(mutate: MutateTwice) -> Self {
        Wrapper::Twice(mutate)
    }
}
impl From<MutateSingleGeneRandomDynamic> for Wrapper {
    fn from(mutate: MutateSingleGeneRandomDynamic) -> Self {
        Wrapper::SingleGeneRandomDynamic(mutate)
    }
}
impl From<MutateDynamicRounds> for Wrapper {
    fn from(mutate: MutateDynamicRounds) -> Self {
        Wrapper::DynamicRounds(mutate)
    }
}
impl From<MutateSingleGeneDistance> for Wrapper {
    fn from(mutate: MutateSingleGeneDistance) -> Self {
        Wrapper::SingleGeneDistance(mutate)
    }
}
