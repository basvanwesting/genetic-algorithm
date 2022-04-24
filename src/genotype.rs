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

mod discrete_random;
pub use self::discrete_random::DiscreteRandom as DiscreteRandomGenotype;

mod continuous_random;
pub use self::continuous_random::ContinuousRandom as ContinuousRandomGenotype;

mod discrete_unique;
pub use self::discrete_unique::DiscreteUnique as DiscreteUniqueGenotype;
