//! The crossover phase where every two parent chromosomes create two children chromosomes. The
//! [competition](crate::compete) phase determines the order of the parent pairing (overall with
//! fitter first). If you choose to keep the parents, the parents will compete with their own
//! children and the population is temporarily overbooked and half of it will be discarded in the
//! [competition](crate::compete) phase.
mod clone;
mod single_gene;
mod single_point;
mod uniform;

pub use self::clone::Clone as CrossoverClone;
pub use self::single_gene::SingleGene as CrossoverSingleGene;
pub use self::single_point::SinglePoint as CrossoverSinglePoint;
pub use self::uniform::Uniform as CrossoverUniform;

use crate::genotype::Genotype;
use crate::population::Population;
use rand::Rng;

pub trait Crossover: Clone + std::fmt::Debug {
    fn call<T: Genotype, R: Rng>(&self, genotype: &T, population: &mut Population<T>, rng: &mut R);

    /// a flag to guard against invalid Crossover strategies which break the internal consistency
    /// of the genes, unique genotypes can't simply exchange genes without gene duplication issues
    fn allow_unique_genotype(&self) -> bool {
        false
    }
}

#[derive(Clone, Debug)]
pub enum Crossovers {
    Clone,
    SingleGene,
    SinglePoint,
    Uniform,
}
pub type KeepParent = bool;

/// Wrapper for use in [meta analysis](crate::meta)
#[derive(Clone, Debug)]
pub struct CrossoverDispatch(pub Crossovers, pub KeepParent);
impl Crossover for CrossoverDispatch {
    fn call<T: Genotype, R: Rng>(&self, genotype: &T, population: &mut Population<T>, rng: &mut R) {
        let keep_parent = self.1;
        match self.0 {
            Crossovers::Clone => CrossoverClone(keep_parent).call(genotype, population, rng),
            Crossovers::SingleGene => {
                CrossoverSingleGene(keep_parent).call(genotype, population, rng)
            }
            Crossovers::SinglePoint => {
                CrossoverSinglePoint(keep_parent).call(genotype, population, rng)
            }
            Crossovers::Uniform => CrossoverUniform(keep_parent).call(genotype, population, rng),
        }
    }

    fn allow_unique_genotype(&self) -> bool {
        let keep_parent = self.1;
        match self.0 {
            Crossovers::Clone => CrossoverClone(keep_parent).allow_unique_genotype(),
            Crossovers::SingleGene => CrossoverSingleGene(keep_parent).allow_unique_genotype(),
            Crossovers::SinglePoint => CrossoverSinglePoint(keep_parent).allow_unique_genotype(),
            Crossovers::Uniform => CrossoverUniform(keep_parent).allow_unique_genotype(),
        }
    }
}
