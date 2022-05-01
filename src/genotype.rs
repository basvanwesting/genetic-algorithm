use crate::chromosome::Chromosome;
use crate::gene::Gene;
use rand::prelude::*;
use std::fmt;

pub trait Genotype: Sized + fmt::Debug + fmt::Display {
    type Gene: Gene;
    fn gene_size(&self) -> usize;
    fn chromosome_factory<R: Rng>(&self, rng: &mut R) -> Chromosome<Self>;
    fn mutate_chromosome<R: Rng>(&self, chromosome: &mut Chromosome<Self>, rng: &mut R);
}

//Evolvable is implicit, until proven otherwise
//pub trait EvolvableGenotype: Genotype {}
pub trait PermutableGenotype: Genotype {
    fn gene_values(&self) -> Vec<Self::Gene>;
}

mod binary;
pub use self::binary::Binary as BinaryGenotype;

mod continuous;
pub use self::continuous::Continuous as ContinuousGenotype;

mod index;
pub use self::index::Index as IndexGenotype;

mod unique_index;
pub use self::unique_index::UniqueIndex as UniqueIndexGenotype;
