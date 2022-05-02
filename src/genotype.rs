use crate::chromosome::Chromosome;
use crate::gene::Gene;
use crate::population::Population;
use itertools::Itertools;
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
    fn population_factory(&self) -> Population<Self> {
        let chromosomes = (0..self.gene_size())
            .map(|_| self.gene_values())
            .multi_cartesian_product()
            .map(|genes| Chromosome::new(genes))
            .collect();

        Population::new(chromosomes)
    }
}

mod binary;
pub use self::binary::Binary as BinaryGenotype;

mod continuous;
pub use self::continuous::Continuous as ContinuousGenotype;

mod index;
pub use self::index::Index as IndexGenotype;

mod unique_index;
pub use self::unique_index::UniqueIndex as UniqueIndexGenotype;

mod multi_index;
pub use self::multi_index::MultiIndex as MultiIndexGenotype;
