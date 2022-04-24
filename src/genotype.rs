use crate::chromosome::Chromosome;
use crate::gene::Gene;
use rand::prelude::*;
use std::fmt;

pub trait Genotype<T: Gene>: fmt::Display {
    fn gene_size(&self) -> usize;
    fn gene_values(&self) -> Vec<T>;
    fn chromosome_factory<R: Rng>(&self, rng: &mut R) -> Chromosome<T>;
    fn mutate_chromosome<R: Rng>(&self, chromosome: &mut Chromosome<T>, rng: &mut R);
}

mod binary;
pub use self::binary::Binary as BinaryGenotype;

mod discrete;
pub use self::discrete::Discrete as DiscreteGenotype;

mod discrete_unique;
pub use self::discrete_unique::DiscreteUnique as DiscreteUniqueGenotype;

mod continuous;
pub use self::continuous::Continuous as ContinuousGenotype;

mod range;
pub use self::range::Range as RangeGenotype;

mod range_unique;
pub use self::range_unique::RangeUnique as RangeUniqueGenotype;
