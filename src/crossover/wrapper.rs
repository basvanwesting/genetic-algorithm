pub use super::clone::Clone as CrossoverClone;
pub use super::multi_gene::MultiGene as CrossoverMultiGene;
pub use super::multi_point::MultiPoint as CrossoverMultiPoint;
pub use super::par_multi_point::ParMultiPoint as CrossoverParMultiPoint;
pub use super::single_gene::SingleGene as CrossoverSingleGene;
pub use super::single_point::SinglePoint as CrossoverSinglePoint;
pub use super::uniform::Uniform as CrossoverUniform;
pub use super::Crossover;

use crate::genotype::Genotype;
use crate::strategy::evolve::{EvolveConfig, EvolveReporter, EvolveState};
use rand::Rng;

#[derive(Clone, Debug)]
pub enum Wrapper {
    Clone(CrossoverClone),
    SingleGene(CrossoverSingleGene),
    SinglePoint(CrossoverSinglePoint),
    MultiGene(CrossoverMultiGene),
    MultiPoint(CrossoverMultiPoint),
    ParMultiPoint(CrossoverParMultiPoint),
    Uniform(CrossoverUniform),
}

impl Crossover for Wrapper {
    fn call<G: Genotype, R: Rng, SR: EvolveReporter<Allele = G::Allele>>(
        &mut self,
        genotype: &G,
        state: &mut EvolveState<G::Allele>,
        config: &EvolveConfig,
        reporter: &mut SR,
        rng: &mut R,
    ) {
        match self {
            Wrapper::Clone(crossover) => crossover.call(genotype, state, config, reporter, rng),
            Wrapper::SingleGene(crossover) => {
                crossover.call(genotype, state, config, reporter, rng)
            }
            Wrapper::SinglePoint(crossover) => {
                crossover.call(genotype, state, config, reporter, rng)
            }
            Wrapper::MultiGene(crossover) => crossover.call(genotype, state, config, reporter, rng),
            Wrapper::MultiPoint(crossover) => {
                crossover.call(genotype, state, config, reporter, rng)
            }
            Wrapper::ParMultiPoint(crossover) => {
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
            Wrapper::SingleGene(crossover) => crossover.require_crossover_indexes(),
            Wrapper::SinglePoint(crossover) => crossover.require_crossover_indexes(),
            Wrapper::MultiGene(crossover) => crossover.require_crossover_indexes(),
            Wrapper::MultiPoint(crossover) => crossover.require_crossover_indexes(),
            Wrapper::ParMultiPoint(crossover) => crossover.require_crossover_indexes(),
            Wrapper::Uniform(crossover) => crossover.require_crossover_indexes(),
        }
    }
    /// to guard against invalid Crossover strategies which break the internal consistency
    /// of the genes, unique genotypes can't simply exchange genes without gene duplication issues
    fn require_crossover_points(&self) -> bool {
        match self {
            Wrapper::Clone(crossover) => crossover.require_crossover_points(),
            Wrapper::SingleGene(crossover) => crossover.require_crossover_points(),
            Wrapper::SinglePoint(crossover) => crossover.require_crossover_points(),
            Wrapper::MultiGene(crossover) => crossover.require_crossover_points(),
            Wrapper::MultiPoint(crossover) => crossover.require_crossover_points(),
            Wrapper::ParMultiPoint(crossover) => crossover.require_crossover_points(),
            Wrapper::Uniform(crossover) => crossover.require_crossover_points(),
        }
    }
}

impl From<CrossoverClone> for Wrapper {
    fn from(crossover: CrossoverClone) -> Self {
        Wrapper::Clone(crossover)
    }
}
impl From<CrossoverSingleGene> for Wrapper {
    fn from(crossover: CrossoverSingleGene) -> Self {
        Wrapper::SingleGene(crossover)
    }
}
impl From<CrossoverSinglePoint> for Wrapper {
    fn from(crossover: CrossoverSinglePoint) -> Self {
        Wrapper::SinglePoint(crossover)
    }
}
impl From<CrossoverMultiGene> for Wrapper {
    fn from(crossover: CrossoverMultiGene) -> Self {
        Wrapper::MultiGene(crossover)
    }
}
impl From<CrossoverMultiPoint> for Wrapper {
    fn from(crossover: CrossoverMultiPoint) -> Self {
        Wrapper::MultiPoint(crossover)
    }
}
impl From<CrossoverParMultiPoint> for Wrapper {
    fn from(crossover: CrossoverParMultiPoint) -> Self {
        Wrapper::ParMultiPoint(crossover)
    }
}
impl From<CrossoverUniform> for Wrapper {
    fn from(crossover: CrossoverUniform) -> Self {
        Wrapper::Uniform(crossover)
    }
}
