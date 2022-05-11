//! The crossover phase where every two parent chromosomes create two children chromosomes. The
//! [competition](crate::compete) phase determines the order of the parent pairing (overall with
//! fitter first). If you choose to keep the parents, the parents will compete with their own
//! children and the population is temporarily overbooked and half of it will be discarded in the
//! [competition](crate::compete) phase.
mod all;
mod clone;
mod range;
mod single;

pub use self::all::All as CrossoverAll;
pub use self::clone::Clone as CrossoverClone;
pub use self::range::Range as CrossoverRange;
pub use self::single::Single as CrossoverSingle;

use crate::genotype::Genotype;
use crate::population::Population;
use rand::Rng;

pub trait Crossover: Clone + std::fmt::Debug {
    fn call<T: Genotype, R: Rng>(
        &self,
        genotype: &T,
        population: Population<T>,
        rng: &mut R,
    ) -> Population<T>;

    /// a flag to guard against invalid Crossover strategies which break the internal consistency
    /// of the genes, unique genotypes can't simply exchange genes without gene duplication issues
    fn allow_unique_genotype(&self) -> bool {
        false
    }
}

#[derive(Clone, Debug)]
pub enum Crossovers {
    Single,
    All,
    Range,
    Clone,
}
pub type KeepParent = bool;

/// Wrapper for use in [meta analysis](crate::meta)
#[derive(Clone, Debug)]
pub struct CrossoverDispatch(pub Crossovers, pub KeepParent);
impl Crossover for CrossoverDispatch {
    fn call<T: Genotype, R: Rng>(
        &self,
        genotype: &T,
        population: Population<T>,
        rng: &mut R,
    ) -> Population<T> {
        let keep_parent = self.1;
        match self.0 {
            Crossovers::Single => CrossoverSingle(keep_parent).call(genotype, population, rng),
            Crossovers::All => CrossoverAll(keep_parent).call(genotype, population, rng),
            Crossovers::Range => CrossoverRange(keep_parent).call(genotype, population, rng),
            Crossovers::Clone => CrossoverClone(keep_parent).call(genotype, population, rng),
        }
    }

    fn allow_unique_genotype(&self) -> bool {
        let keep_parent = self.1;
        match self.0 {
            Crossovers::Single => CrossoverSingle(keep_parent).allow_unique_genotype(),
            Crossovers::All => CrossoverAll(keep_parent).allow_unique_genotype(),
            Crossovers::Range => CrossoverRange(keep_parent).allow_unique_genotype(),
            Crossovers::Clone => CrossoverClone(keep_parent).allow_unique_genotype(),
        }
    }
}
