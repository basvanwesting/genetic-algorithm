//! The mutation strategy, very important for avoiding local optimum lock-in. But don't overdo it,
//! as it degenerates the population too much if overused. Use a mutation probability generally between
//! 5% and 20%.
mod multi_gene;
mod multi_gene_dynamic;
mod single_gene;
mod single_gene_dynamic;
mod wrapper;

pub use self::multi_gene::MultiGene as MutateMultiGene;
pub use self::multi_gene_dynamic::MultiGeneDynamic as MutateMultiGeneDynamic;
pub use self::single_gene::SingleGene as MutateSingleGene;
pub use self::single_gene_dynamic::SingleGeneDynamic as MutateSingleGeneDynamic;
pub use self::wrapper::Wrapper as MutateWrapper;

use crate::genotype::Genotype;
use crate::strategy::evolve::{EvolveConfig, EvolveReporter, EvolveState};
use rand::Rng;

pub trait Mutate: Clone + Send + Sync + std::fmt::Debug {
    fn call<G: Genotype, R: Rng, SR: EvolveReporter<Allele = G::Allele>>(
        &mut self,
        genotype: &G,
        state: &mut EvolveState<G::Allele>,
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
