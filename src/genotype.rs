//! The search space for the algorithm.
mod binary;
mod builder;
mod continuous;
mod discrete;
mod multi_continuous;
mod multi_discrete;
mod unique_discrete;

pub use self::binary::Binary as BinaryGenotype;
pub use self::builder::{
    Builder as GenotypeBuilder, TryFromBuilderError as TryFromGenotypeBuilderError,
};
pub use self::continuous::Continuous as ContinuousGenotype;
pub use self::discrete::Discrete as DiscreteGenotype;
pub use self::multi_continuous::MultiContinuous as MultiContinuousGenotype;
pub use self::multi_discrete::MultiDiscrete as MultiDiscreteGenotype;
pub use self::unique_discrete::UniqueDiscrete as UniqueDiscreteGenotype;

use crate::chromosome::Chromosome;
use itertools::Itertools;
use num::BigUint;
use rand::Rng;
use std::fmt;

// trait alias, experimental
//pub trait Allele = Clone + std::fmt::Debug;

/// Each implemented genotype handles its own random genes initialization and mutation.
pub trait Genotype: Clone + fmt::Debug + fmt::Display + TryFrom<GenotypeBuilder<Self>> {
    type Allele: Clone + std::fmt::Debug;
    fn genes_size(&self) -> usize;
    /// a random chromosome factory to seed the initial population for [Evolve](crate::strategy::evolve::Evolve)
    fn chromosome_factory<R: Rng>(&self, rng: &mut R) -> Chromosome<Self>;
    fn mutate_chromosome<R: Rng>(&self, chromosome: &mut Chromosome<Self>, rng: &mut R);
    /// to guard against invalid crossover strategies which break the internal consistency
    /// of the genes, unique genotypes can't simply exchange genes without gene duplication issues
    fn crossover_points(&self) -> Vec<usize> {
        (0..self.genes_size()).collect()
    }
    /// to guard against invalid crossover strategies which break the internal consistency
    /// of the genes, unique genotypes can't simply exchange genes without gene duplication issues
    fn crossover_indexes(&self) -> Vec<usize> {
        (0..self.genes_size()).collect()
    }
    fn builder() -> GenotypeBuilder<Self> {
        GenotypeBuilder::<Self>::default()
    }
}

//Evolvable is implicit, until proven otherwise
//pub trait EvolvableGenotype: Genotype {}
/// Not all genotypes are permutable, only countable ones (e.g. continuous genotypes cannot be permutated).
pub trait PermutableGenotype: Genotype {
    fn allele_values(&self) -> Vec<Self::Allele>;

    /// chromosome iterator for the all possible gene combinations for [Permutate](crate::strategy::permutate::Permutate)
    fn chromosome_permutations_into_iter<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = Chromosome<Self>> + 'a> {
        Box::new(
            (0..self.genes_size())
                .map(|_| self.allele_values())
                .multi_cartesian_product()
                .map(|genes| Chromosome::new(genes)),
        )
    }

    /// chromosome iterator size for the all possible gene combinations for [Permutate](crate::strategy::permutate::Permutate)
    fn chromosome_permutations_size(&self) -> BigUint {
        BigUint::from(self.allele_values().len()).pow(self.genes_size() as u32)
    }
}
