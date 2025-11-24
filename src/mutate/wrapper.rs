pub use super::multi_gene::MultiGene as MutateMultiGene;
pub use super::multi_gene_dynamic::MultiGeneDynamic as MutateMultiGeneDynamic;
pub use super::multi_gene_range::MultiGeneRange as MutateMultiGeneRange;
pub use super::single_gene::SingleGene as MutateSingleGene;
pub use super::single_gene_dynamic::SingleGeneDynamic as MutateSingleGeneDynamic;
pub use super::Mutate;

use crate::genotype::EvolveGenotype;
use crate::strategy::evolve::{EvolveConfig, EvolveState};
use crate::strategy::StrategyReporter;
use rand::Rng;

#[derive(Clone, Debug)]
pub enum Wrapper<G: EvolveGenotype> {
    MultiGene(MutateMultiGene<G>),
    MultiGeneDynamic(MutateMultiGeneDynamic<G>),
    MultiGeneRange(MutateMultiGeneRange<G>),
    SingleGene(MutateSingleGene<G>),
    SingleGeneDynamic(MutateSingleGeneDynamic<G>),
}

impl<G: EvolveGenotype> Mutate for Wrapper<G> {
    type Genotype = G;

    fn before(&mut self, genotype: &G, state: &mut EvolveState<G>, config: &EvolveConfig) {
        match self {
            Wrapper::MultiGene(mutate) => mutate.before(genotype, state, config),
            Wrapper::MultiGeneDynamic(mutate) => mutate.before(genotype, state, config),
            Wrapper::MultiGeneRange(mutate) => mutate.before(genotype, state, config),
            Wrapper::SingleGene(mutate) => mutate.before(genotype, state, config),
            Wrapper::SingleGeneDynamic(mutate) => mutate.before(genotype, state, config),
        }
    }

    fn call<R: Rng, SR: StrategyReporter<Genotype = G>>(
        &mut self,
        genotype: &G,
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

    fn after(&mut self, genotype: &G, state: &mut EvolveState<G>, config: &EvolveConfig) {
        match self {
            Wrapper::MultiGene(mutate) => mutate.after(genotype, state, config),
            Wrapper::MultiGeneDynamic(mutate) => mutate.after(genotype, state, config),
            Wrapper::MultiGeneRange(mutate) => mutate.after(genotype, state, config),
            Wrapper::SingleGene(mutate) => mutate.after(genotype, state, config),
            Wrapper::SingleGeneDynamic(mutate) => mutate.after(genotype, state, config),
        }
    }
}

impl<G: EvolveGenotype> From<MutateSingleGene<G>> for Wrapper<G> {
    fn from(mutate: MutateSingleGene<G>) -> Self {
        Wrapper::SingleGene(mutate)
    }
}
impl<G: EvolveGenotype> From<MutateMultiGene<G>> for Wrapper<G> {
    fn from(mutate: MutateMultiGene<G>) -> Self {
        Wrapper::MultiGene(mutate)
    }
}
impl<G: EvolveGenotype> From<MutateSingleGeneDynamic<G>> for Wrapper<G> {
    fn from(mutate: MutateSingleGeneDynamic<G>) -> Self {
        Wrapper::SingleGeneDynamic(mutate)
    }
}
impl<G: EvolveGenotype> From<MutateMultiGeneDynamic<G>> for Wrapper<G> {
    fn from(mutate: MutateMultiGeneDynamic<G>) -> Self {
        Wrapper::MultiGeneDynamic(mutate)
    }
}
impl<G: EvolveGenotype> From<MutateMultiGeneRange<G>> for Wrapper<G> {
    fn from(mutate: MutateMultiGeneRange<G>) -> Self {
        Wrapper::MultiGeneRange(mutate)
    }
}
