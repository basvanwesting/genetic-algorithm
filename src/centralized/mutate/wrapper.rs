pub use super::multi_gene::MultiGene as MutateMultiGene;
pub use super::multi_gene_dynamic::MultiGeneDynamic as MutateMultiGeneDynamic;
pub use super::multi_gene_range::MultiGeneRange as MutateMultiGeneRange;
pub use super::single_gene::SingleGene as MutateSingleGene;
pub use super::single_gene_dynamic::SingleGeneDynamic as MutateSingleGeneDynamic;
pub use super::Mutate;

use crate::centralized::genotype::EvolveGenotype;
use crate::centralized::strategy::evolve::{EvolveConfig, EvolveState};
use crate::centralized::strategy::StrategyReporter;
use rand::Rng;

#[derive(Clone, Debug)]
pub enum Wrapper {
    MultiGene(MutateMultiGene),
    MultiGeneDynamic(MutateMultiGeneDynamic),
    MultiGeneRange(MutateMultiGeneRange),
    SingleGene(MutateSingleGene),
    SingleGeneDynamic(MutateSingleGeneDynamic),
}

impl Mutate for Wrapper {
    fn call<G: EvolveGenotype, R: Rng, SR: StrategyReporter<Genotype = G>>(
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
            Wrapper::MultiGeneRange(mutate) => mutate.call(genotype, state, config, reporter, rng),
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
impl From<MutateMultiGeneRange> for Wrapper {
    fn from(mutate: MutateMultiGeneRange) -> Self {
        Wrapper::MultiGeneRange(mutate)
    }
}
