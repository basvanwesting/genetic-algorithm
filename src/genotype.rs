mod binary;
mod builder;
mod continuous;
mod discrete;
mod index;
mod multi_index;
mod unique_discrete;
mod unique_index;

pub use self::binary::Binary as BinaryGenotype;
pub use self::builder::{
    Builder as GenotypeBuilder, TryFromBuilderError as TryFromGenotypeBuilderError,
};
pub use self::continuous::Continuous as ContinuousGenotype;
pub use self::discrete::Discrete as DiscreteGenotype;
pub use self::index::Index as IndexGenotype;
pub use self::multi_index::MultiIndex as MultiIndexGenotype;
pub use self::unique_discrete::UniqueDiscrete as UniqueDiscreteGenotype;
pub use self::unique_index::UniqueIndex as UniqueIndexGenotype;

use crate::chromosome::Chromosome;
use crate::population::Population;
use itertools::Itertools;
use rand::prelude::*;
use std::fmt;

// trait alias, experimental
//pub trait Gene = Clone + std::fmt::Debug;

pub trait Genotype: Clone + fmt::Debug + fmt::Display + TryFrom<GenotypeBuilder<Self>> {
    type Gene: Clone + std::fmt::Debug;
    fn gene_size(&self) -> usize;
    fn chromosome_factory<R: Rng>(&self, rng: &mut R) -> Chromosome<Self>;
    fn mutate_chromosome<R: Rng>(&self, chromosome: &mut Chromosome<Self>, rng: &mut R);
    fn is_unique(&self) -> bool {
        false
    }
    fn builder() -> GenotypeBuilder<Self> {
        GenotypeBuilder::<Self>::default()
    }
}

//Evolvable is implicit, until proven otherwise
//pub trait EvolvableGenotype: Genotype {}
pub trait PermutableGenotype: Genotype {
    fn gene_values(&self) -> Vec<Self::Gene>;
    fn population_factory(&self) -> Population<Self> {
        let chromosomes = (0..self.gene_size())
            .map(|_| self.gene_values())
            .multi_cartesian_product()
            .map(|genes| Chromosome::new(genes))
            .collect();

        Population::new(chromosomes)
    }
    fn population_factory_size(&self) -> usize {
        self.gene_values().len().pow(self.gene_size() as u32)
    }
}
