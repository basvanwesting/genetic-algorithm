pub use super::clone::Clone as CrossoverClone;
pub use super::multi_gene::MultiGene as CrossoverMultiGene;
pub use super::multi_point::MultiPoint as CrossoverMultiPoint;
pub use super::rejuvenate::Rejuvenate as CrossoverRejuvenate;
pub use super::single_gene::SingleGene as CrossoverSingleGene;
pub use super::single_point::SinglePoint as CrossoverSinglePoint;
pub use super::uniform::Uniform as CrossoverUniform;
pub use super::Crossover;

use crate::genotype::EvolveGenotype;
use crate::strategy::evolve::{EvolveConfig, EvolveState};
use crate::strategy::StrategyReporter;
use rand::Rng;

#[derive(Clone, Debug)]
pub enum Wrapper<G: EvolveGenotype> {
    Clone(CrossoverClone<G>),
    MultiGene(CrossoverMultiGene<G>),
    MultiPoint(CrossoverMultiPoint<G>),
    Rejuvenate(CrossoverRejuvenate<G>),
    SingleGene(CrossoverSingleGene<G>),
    SinglePoint(CrossoverSinglePoint<G>),
    Uniform(CrossoverUniform<G>),
}

impl<G: EvolveGenotype> Crossover for Wrapper<G> {
    type Genotype = G;

    fn call<R: Rng, SR: StrategyReporter<Genotype = G>>(
        &mut self,
        genotype: &G,
        state: &mut EvolveState<G>,
        config: &EvolveConfig,
        reporter: &mut SR,
        rng: &mut R,
    ) {
        match self {
            Wrapper::Clone(crossover) => crossover.call(genotype, state, config, reporter, rng),
            Wrapper::MultiGene(crossover) => crossover.call(genotype, state, config, reporter, rng),
            Wrapper::MultiPoint(crossover) => {
                crossover.call(genotype, state, config, reporter, rng)
            }
            Wrapper::Rejuvenate(crossover) => {
                crossover.call(genotype, state, config, reporter, rng)
            }
            Wrapper::SingleGene(crossover) => {
                crossover.call(genotype, state, config, reporter, rng)
            }
            Wrapper::SinglePoint(crossover) => {
                crossover.call(genotype, state, config, reporter, rng)
            }
            Wrapper::Uniform(crossover) => crossover.call(genotype, state, config, reporter, rng),
        }
    }

    /// to guard against invalid Crossover strategies which break the internal consistency
    /// of the genes, unique genotypes can't simply exchange genes without gene duplication issues
    fn require_crossover_indexes(&self) -> bool {
        match self {
            Wrapper::Clone(crossover) => crossover.require_crossover_indexes(),
            Wrapper::MultiGene(crossover) => crossover.require_crossover_indexes(),
            Wrapper::MultiPoint(crossover) => crossover.require_crossover_indexes(),
            Wrapper::Rejuvenate(crossover) => crossover.require_crossover_indexes(),
            Wrapper::SingleGene(crossover) => crossover.require_crossover_indexes(),
            Wrapper::SinglePoint(crossover) => crossover.require_crossover_indexes(),
            Wrapper::Uniform(crossover) => crossover.require_crossover_indexes(),
        }
    }
    /// to guard against invalid Crossover strategies which break the internal consistency
    /// of the genes, unique genotypes can't simply exchange genes without gene duplication issues
    fn require_crossover_points(&self) -> bool {
        match self {
            Wrapper::Clone(crossover) => crossover.require_crossover_points(),
            Wrapper::MultiGene(crossover) => crossover.require_crossover_points(),
            Wrapper::MultiPoint(crossover) => crossover.require_crossover_points(),
            Wrapper::Rejuvenate(crossover) => crossover.require_crossover_points(),
            Wrapper::SingleGene(crossover) => crossover.require_crossover_points(),
            Wrapper::SinglePoint(crossover) => crossover.require_crossover_points(),
            Wrapper::Uniform(crossover) => crossover.require_crossover_points(),
        }
    }
}

impl<G: EvolveGenotype> From<CrossoverClone<G>> for Wrapper<G> {
    fn from(crossover: CrossoverClone<G>) -> Self {
        Wrapper::Clone(crossover)
    }
}
impl<G: EvolveGenotype> From<CrossoverMultiGene<G>> for Wrapper<G> {
    fn from(crossover: CrossoverMultiGene<G>) -> Self {
        Wrapper::MultiGene(crossover)
    }
}
impl<G: EvolveGenotype> From<CrossoverMultiPoint<G>> for Wrapper<G> {
    fn from(crossover: CrossoverMultiPoint<G>) -> Self {
        Wrapper::MultiPoint(crossover)
    }
}
impl<G: EvolveGenotype> From<CrossoverRejuvenate<G>> for Wrapper<G> {
    fn from(crossover: CrossoverRejuvenate<G>) -> Self {
        Wrapper::Rejuvenate(crossover)
    }
}
impl<G: EvolveGenotype> From<CrossoverSingleGene<G>> for Wrapper<G> {
    fn from(crossover: CrossoverSingleGene<G>) -> Self {
        Wrapper::SingleGene(crossover)
    }
}
impl<G: EvolveGenotype> From<CrossoverSinglePoint<G>> for Wrapper<G> {
    fn from(crossover: CrossoverSinglePoint<G>) -> Self {
        Wrapper::SinglePoint(crossover)
    }
}
impl<G: EvolveGenotype> From<CrossoverUniform<G>> for Wrapper<G> {
    fn from(crossover: CrossoverUniform<G>) -> Self {
        Wrapper::Uniform(crossover)
    }
}
