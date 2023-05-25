//! The crossover phase where every two parent chromosomes create two children chromosomes. The
//! [competition](crate::compete) phase determines the order of the parent pairing (overall with
//! fitter first). If you choose to keep the parents, the parents will compete with their own
//! children and the population is temporarily overbooked and half of it will be discarded in the
//! [competition](crate::compete) phase.
//mod dispatch;
mod clone;
mod single_gene;
mod single_point;
mod uniform;

//pub use self::dispatch::Dispatch as CrossoverDispatch;
pub use self::clone::Clone as CrossoverClone;
pub use self::single_gene::SingleGene as CrossoverSingleGene;
pub use self::single_point::SinglePoint as CrossoverSinglePoint;
pub use self::uniform::Uniform as CrossoverUniform;

use crate::genotype::Genotype;
use crate::population::Population;
use rand::Rng;

#[derive(Clone, Debug)]
pub enum Crossover {
    Clone(CrossoverClone),
    SingleGene(CrossoverSingleGene),
    SinglePoint(CrossoverSinglePoint),
    Uniform(CrossoverUniform),
}

impl Crossover {
    pub fn call<T: Genotype, R: Rng>(
        &mut self,
        genotype: &T,
        population: &mut Population<T>,
        rng: &mut R,
    ) {
        match self {
            Crossover::Clone(crossover) => crossover.call(genotype, population, rng),
            Crossover::SingleGene(crossover) => crossover.call(genotype, population, rng),
            Crossover::SinglePoint(crossover) => crossover.call(genotype, population, rng),
            Crossover::Uniform(crossover) => crossover.call(genotype, population, rng),
        }
    }

    /// to guard against invalid Crossover strategies which break the internal consistency
    /// of the genes, unique genotypes can't simply exchange genes without gene duplication issues
    pub fn require_crossover_indexes(&self) -> bool {
        match self {
            Crossover::Clone(crossover) => crossover.require_crossover_indexes(),
            Crossover::SingleGene(crossover) => crossover.require_crossover_indexes(),
            Crossover::SinglePoint(crossover) => crossover.require_crossover_indexes(),
            Crossover::Uniform(crossover) => crossover.require_crossover_indexes(),
        }
    }
    /// to guard against invalid Crossover strategies which break the internal consistency
    /// of the genes, unique genotypes can't simply exchange genes without gene duplication issues
    pub fn require_crossover_points(&self) -> bool {
        match self {
            Crossover::Clone(crossover) => crossover.require_crossover_points(),
            Crossover::SingleGene(crossover) => crossover.require_crossover_points(),
            Crossover::SinglePoint(crossover) => crossover.require_crossover_points(),
            Crossover::Uniform(crossover) => crossover.require_crossover_points(),
        }
    }
}

impl From<CrossoverClone> for Crossover {
    fn from(crossover: CrossoverClone) -> Self {
        Crossover::Clone(crossover)
    }
}
impl From<CrossoverSingleGene> for Crossover {
    fn from(crossover: CrossoverSingleGene) -> Self {
        Crossover::SingleGene(crossover)
    }
}
impl From<CrossoverSinglePoint> for Crossover {
    fn from(crossover: CrossoverSinglePoint) -> Self {
        Crossover::SinglePoint(crossover)
    }
}
impl From<CrossoverUniform> for Crossover {
    fn from(crossover: CrossoverUniform) -> Self {
        Crossover::Uniform(crossover)
    }
}
