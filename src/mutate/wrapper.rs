pub use super::multi_gene_random_dynamic::MultiGeneRandomDynamic as MutateMultiGeneRandomDynamic;
pub use super::multi_gene_random::MultiGeneRandom as MutateMultiGeneRandom;
pub use super::single_gene_distance::SingleGeneDistance as MutateSingleGeneDistance;
pub use super::single_gene_random::SingleGeneRandom as MutateSingleGeneRandom;
pub use super::single_gene_random_dynamic::SingleGeneRandomDynamic as MutateSingleGeneRandomDynamic;
pub use super::Mutate;

use crate::genotype::Genotype;
use crate::population::Population;
use rand::Rng;

#[derive(Clone, Debug)]
pub enum Wrapper {
    MultiGeneRandomDynamic(MutateMultiGeneRandomDynamic),
    MultiGeneRandom(MutateMultiGeneRandom),
    SingleGeneDistance(MutateSingleGeneDistance),
    SingleGeneRandom(MutateSingleGeneRandom),
    SingleGeneRandomDynamic(MutateSingleGeneRandomDynamic),
}

impl Mutate for Wrapper {
    fn call<T: Genotype, R: Rng>(
        &mut self,
        genotype: &T,
        population: &mut Population<T>,
        rng: &mut R,
    ) {
        match self {
            Wrapper::MultiGeneRandomDynamic(mutate) => mutate.call(genotype, population, rng),
            Wrapper::MultiGeneRandom(mutate) => mutate.call(genotype, population, rng),
            Wrapper::SingleGeneDistance(mutate) => mutate.call(genotype, population, rng),
            Wrapper::SingleGeneRandom(mutate) => mutate.call(genotype, population, rng),
            Wrapper::SingleGeneRandomDynamic(mutate) => mutate.call(genotype, population, rng),
        }
    }
    fn report(&self) -> String {
        match self {
            Wrapper::MultiGeneRandomDynamic(mutate) => mutate.report(),
            Wrapper::MultiGeneRandom(mutate) => mutate.report(),
            Wrapper::SingleGeneDistance(mutate) => mutate.report(),
            Wrapper::SingleGeneRandom(mutate) => mutate.report(),
            Wrapper::SingleGeneRandomDynamic(mutate) => mutate.report(),
        }
    }
}

impl From<MutateSingleGeneRandom> for Wrapper {
    fn from(mutate: MutateSingleGeneRandom) -> Self {
        Wrapper::SingleGeneRandom(mutate)
    }
}
impl From<MutateMultiGeneRandom> for Wrapper {
    fn from(mutate: MutateMultiGeneRandom) -> Self {
        Wrapper::MultiGeneRandom(mutate)
    }
}
impl From<MutateSingleGeneRandomDynamic> for Wrapper {
    fn from(mutate: MutateSingleGeneRandomDynamic) -> Self {
        Wrapper::SingleGeneRandomDynamic(mutate)
    }
}
impl From<MutateMultiGeneRandomDynamic> for Wrapper {
    fn from(mutate: MutateMultiGeneRandomDynamic) -> Self {
        Wrapper::MultiGeneRandomDynamic(mutate)
    }
}
impl From<MutateSingleGeneDistance> for Wrapper {
    fn from(mutate: MutateSingleGeneDistance) -> Self {
        Wrapper::SingleGeneDistance(mutate)
    }
}
