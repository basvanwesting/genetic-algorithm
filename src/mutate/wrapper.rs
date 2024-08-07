pub use super::multi_gene_random::MultiGeneRandom as MutateMultiGeneRandom;
pub use super::multi_gene_random_dynamic::MultiGeneRandomDynamic as MutateMultiGeneRandomDynamic;
pub use super::single_gene_distance::SingleGeneDistance as MutateSingleGeneDistance;
pub use super::single_gene_random::SingleGeneRandom as MutateSingleGeneRandom;
pub use super::single_gene_random_dynamic::SingleGeneRandomDynamic as MutateSingleGeneRandomDynamic;
pub use super::Mutate;

use crate::genotype::Genotype;
use crate::population::Population;
use crate::strategy::evolve::EvolveReporter;
use rand::Rng;

#[derive(Clone, Debug)]
pub enum Wrapper {
    MultiGeneRandom(MutateMultiGeneRandom),
    MultiGeneRandomDynamic(MutateMultiGeneRandomDynamic),
    SingleGeneDistance(MutateSingleGeneDistance),
    SingleGeneRandom(MutateSingleGeneRandom),
    SingleGeneRandomDynamic(MutateSingleGeneRandomDynamic),
}

impl Mutate for Wrapper {
    fn call<G: Genotype, R: Rng, SR: EvolveReporter<Genotype = G>>(
        &mut self,
        genotype: &G,
        population: &mut Population<G>,
        reporter: &mut SR,
        rng: &mut R,
    ) {
        match self {
            Wrapper::MultiGeneRandom(mutate) => mutate.call(genotype, population, reporter, rng),
            Wrapper::MultiGeneRandomDynamic(mutate) => {
                mutate.call(genotype, population, reporter, rng)
            }
            Wrapper::SingleGeneDistance(mutate) => mutate.call(genotype, population, reporter, rng),
            Wrapper::SingleGeneRandom(mutate) => mutate.call(genotype, population, reporter, rng),
            Wrapper::SingleGeneRandomDynamic(mutate) => {
                mutate.call(genotype, population, reporter, rng)
            }
        }
    }
    fn report(&self) -> String {
        match self {
            Wrapper::MultiGeneRandom(mutate) => mutate.report(),
            Wrapper::MultiGeneRandomDynamic(mutate) => mutate.report(),
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
