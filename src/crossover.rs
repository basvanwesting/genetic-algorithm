//! The crossover phase where every two parent chromosomes create two children chromosomes. The
//! [competition](crate::compete) phase determines the order of the parent pairing (overall with
//! fitter first). If you choose to keep the parents, the parents will compete with their own
//! children and the population is temporarily overbooked and half of it will be discarded in the
//! [competition](crate::compete) phase.
mod clone;
mod single_gene;
mod single_point;
mod uniform;
mod wrapper;

pub use self::clone::Clone as CrossoverClone;
pub use self::single_gene::SingleGene as CrossoverSingleGene;
pub use self::single_point::SinglePoint as CrossoverSinglePoint;
pub use self::uniform::Uniform as CrossoverUniform;
pub use self::wrapper::Wrapper as CrossoverWrapper;

use crate::genotype::Genotype;
use crate::population::Population;
use rand::Rng;

pub trait Crossover: Clone + std::fmt::Debug {
    fn call<T: Genotype, R: Rng>(
        &mut self,
        genotype: &T,
        population: &mut Population<T>,
        rng: &mut R,
    );

    /// to guard against invalid Crossover strategies which break the internal consistency
    /// of the genes, unique genotypes can't simply exchange genes without gene duplication issues
    fn require_crossover_indexes(&self) -> bool;
    /// to guard against invalid Crossover strategies which break the internal consistency
    /// of the genes, unique genotypes can't simply exchange genes without gene duplication issues
    fn require_crossover_points(&self) -> bool;
}
