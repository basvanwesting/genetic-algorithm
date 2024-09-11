pub use super::multi_gene::MultiGene as MutateMultiGene;
pub use super::multi_gene_dynamic::MultiGeneDynamic as MutateMultiGeneDynamic;
pub use super::single_gene::SingleGene as MutateSingleGene;
pub use super::single_gene_dynamic::SingleGeneDynamic as MutateSingleGeneDynamic;
pub use super::Mutate;

use crate::genotype::Genotype;
use crate::strategy::evolve::{EvolveConfig, EvolveReporter, EvolveState};
use rand::Rng;

#[derive(Clone, Debug)]
pub enum Wrapper {
    MultiGene(MutateMultiGene),
    MultiGeneDynamic(MutateMultiGeneDynamic),
    SingleGene(MutateSingleGene),
    SingleGeneDynamic(MutateSingleGeneDynamic),
}

impl Mutate for Wrapper {
    fn call<G: Genotype, R: Rng, SR: EvolveReporter<Genotype = G>>(
        &mut self,
        genotype: &mut G,
        state: &mut EvolveState<G>,
        config: &EvolveConfig,
        reporter: &mut SR,
        rng: &mut R,
    ) {
        match self {
            Wrapper::MultiGene(mutate) => mutate.call(genotype, state, config, reporter, rng),
            Wrapper::MultiGeneDynamic(mutate) => {
                mutate.call(genotype, state, config, reporter, rng)
            }
            Wrapper::SingleGene(mutate) => mutate.call(genotype, state, config, reporter, rng),
            Wrapper::SingleGeneDynamic(mutate) => {
                mutate.call(genotype, state, config, reporter, rng)
            }
        }
    }
}

impl From<MutateSingleGene> for Wrapper {
    fn from(mutate: MutateSingleGene) -> Self {
        Wrapper::SingleGene(mutate)
    }
}
impl From<MutateMultiGene> for Wrapper {
    fn from(mutate: MutateMultiGene) -> Self {
        Wrapper::MultiGene(mutate)
    }
}
impl From<MutateSingleGeneDynamic> for Wrapper {
    fn from(mutate: MutateSingleGeneDynamic) -> Self {
        Wrapper::SingleGeneDynamic(mutate)
    }
}
impl From<MutateMultiGeneDynamic> for Wrapper {
    fn from(mutate: MutateMultiGeneDynamic) -> Self {
        Wrapper::MultiGeneDynamic(mutate)
    }
}
