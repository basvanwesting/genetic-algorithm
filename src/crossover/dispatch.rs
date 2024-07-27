pub use super::clone::Clone as CrossoverClone;
pub use super::single_gene::SingleGene as CrossoverSingleGene;
pub use super::single_point::SinglePoint as CrossoverSinglePoint;
pub use super::uniform::Uniform as CrossoverUniform;
pub use super::Crossover;

use crate::genotype::Genotype;
use crate::population::Population;
use rand::Rng;

#[derive(Clone, Debug, Default)]
pub enum Implementations {
    #[default]
    Clone,
    SingleGene,
    SinglePoint,
    Uniform,
}

/// Wrapper for use in benchmarks or [meta analysis](https://github.com/basvanwesting/genetic-algorithm-meta.git)
#[derive(Clone, Debug, Default)]
pub struct Dispatch {
    pub implementation: Implementations,
    pub keep_parent: bool,
}
impl Crossover for Dispatch {
    fn call<T: Genotype, R: Rng>(
        &mut self,
        genotype: &T,
        population: &mut Population<T>,
        rng: &mut R,
    ) {
        match self.implementation {
            Implementations::Clone => {
                CrossoverClone::new(self.keep_parent).call(genotype, population, rng)
            }
            Implementations::SingleGene => {
                CrossoverSingleGene::new(self.keep_parent).call(genotype, population, rng)
            }
            Implementations::SinglePoint => {
                CrossoverSinglePoint::new(self.keep_parent).call(genotype, population, rng)
            }
            Implementations::Uniform => {
                CrossoverUniform::new(self.keep_parent).call(genotype, population, rng)
            }
        }
    }
    fn require_crossover_indexes(&self) -> bool {
        match self.implementation {
            Implementations::Clone => {
                CrossoverClone::new(self.keep_parent).require_crossover_indexes()
            }
            Implementations::SingleGene => {
                CrossoverSingleGene::new(self.keep_parent).require_crossover_indexes()
            }
            Implementations::SinglePoint => {
                CrossoverSinglePoint::new(self.keep_parent).require_crossover_indexes()
            }
            Implementations::Uniform => {
                CrossoverUniform::new(self.keep_parent).require_crossover_indexes()
            }
        }
    }
    fn require_crossover_points(&self) -> bool {
        match self.implementation {
            Implementations::Clone => {
                CrossoverClone::new(self.keep_parent).require_crossover_points()
            }
            Implementations::SingleGene => {
                CrossoverSingleGene::new(self.keep_parent).require_crossover_points()
            }
            Implementations::SinglePoint => {
                CrossoverSinglePoint::new(self.keep_parent).require_crossover_points()
            }
            Implementations::Uniform => {
                CrossoverUniform::new(self.keep_parent).require_crossover_points()
            }
        }
    }
}

impl From<CrossoverClone> for Dispatch {
    fn from(implementation: CrossoverClone) -> Self {
        Dispatch {
            implementation: Implementations::Clone,
            keep_parent: implementation.keep_parent,
            ..Default::default()
        }
    }
}

impl From<CrossoverSingleGene> for Dispatch {
    fn from(implementation: CrossoverSingleGene) -> Self {
        Dispatch {
            implementation: Implementations::SingleGene,
            keep_parent: implementation.keep_parent,
            ..Default::default()
        }
    }
}

impl From<CrossoverSinglePoint> for Dispatch {
    fn from(implementation: CrossoverSinglePoint) -> Self {
        Dispatch {
            implementation: Implementations::SinglePoint,
            keep_parent: implementation.keep_parent,
            ..Default::default()
        }
    }
}

impl From<CrossoverUniform> for Dispatch {
    fn from(implementation: CrossoverUniform) -> Self {
        Dispatch {
            implementation: Implementations::Uniform,
            keep_parent: implementation.keep_parent,
            ..Default::default()
        }
    }
}
