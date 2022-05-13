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
use rand::prelude::*;
use std::fmt;

// trait alias, experimental
//pub trait Gene = Clone + std::fmt::Debug;

/// Each implemented genotype handles its own random genes initialization and mutation.
pub trait Genotype: Clone + fmt::Debug + fmt::Display + TryFrom<GenotypeBuilder<Self>> {
    type Gene: Clone + std::fmt::Debug;
    fn gene_size(&self) -> usize;
    /// a random chromosome factory to seed the initial population for [Evolve](crate::evolve::Evolve)
    fn chromosome_factory<R: Rng>(&self, rng: &mut R) -> Chromosome<Self>;
    fn mutate_chromosome<R: Rng>(&self, chromosome: &mut Chromosome<Self>, rng: &mut R);
    /// a flag to guard against invalid crossover strategies which break the internal consistency
    /// of the genes, unique genotypes can't simply exchange genes without gene duplication issues
    fn is_unique(&self) -> bool {
        false
    }
    fn builder() -> GenotypeBuilder<Self> {
        GenotypeBuilder::<Self>::default()
    }
}

//Evolvable is implicit, until proven otherwise
//pub trait EvolvableGenotype: Genotype {}
/// Not all genotypes are permutable, only countable ones (e.g. continuous genotypes cannot be permutated).
pub trait PermutableGenotype: Genotype {
    fn gene_values(&self) -> Vec<Self::Gene>;

    /// chromosome iterator for the all possible gene combinations for [Permutate](crate::permutate::Permutate)
    fn chromosome_permutations_into_iter<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = Chromosome<Self>> + 'a> {
        Box::new(
            (0..self.gene_size())
                .map(|_| self.gene_values())
                .multi_cartesian_product()
                .map(|genes| Chromosome::new(genes)),
        )
    }

    /// chromosome iterator size for the all possible gene combinations for [Permutate](crate::permutate::Permutate)
    fn chromosome_permutations_size(&self) -> BigUint {
        BigUint::from(self.gene_values().len()).pow(self.gene_size() as u32)
    }
}
