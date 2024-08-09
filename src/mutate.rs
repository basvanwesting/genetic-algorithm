//! The mutation strategy, very important for avoiding local optimum lock-in. But don't overdo it,
//! as it degenerates the population too much if overused. Use a mutation probability generally between
//! 5% and 20%.
mod multi_gene_random;
mod multi_gene_random_dynamic;
mod single_gene_neighbour;
mod single_gene_random;
mod single_gene_random_dynamic;
mod wrapper;

pub use self::multi_gene_random::MultiGeneRandom as MutateMultiGeneRandom;
pub use self::multi_gene_random_dynamic::MultiGeneRandomDynamic as MutateMultiGeneRandomDynamic;
pub use self::single_gene_neighbour::SingleGeneNeighbour as MutateSingleGeneNeighbour;
pub use self::single_gene_random::SingleGeneRandom as MutateSingleGeneRandom;
pub use self::single_gene_random_dynamic::SingleGeneRandomDynamic as MutateSingleGeneRandomDynamic;
pub use self::wrapper::Wrapper as MutateWrapper;

use crate::genotype::Genotype;
use crate::strategy::evolve::{EvolveConfig, EvolveReporter, EvolveState};
use rand::Rng;

pub trait Mutate: Clone + std::fmt::Debug {
    fn call<G: Genotype, R: Rng, SR: EvolveReporter<Genotype = G>>(
        &mut self,
        genotype: &G,
        state: &mut EvolveState<G>,
        config: &EvolveConfig,
        reporter: &mut SR,
        rng: &mut R,
    );
    fn report(&self) -> String;
}

#[derive(Clone, Debug)]
pub enum MutateEvent {
    ChangeMutationProbability(String),
}
