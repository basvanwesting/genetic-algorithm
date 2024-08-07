use super::Extension;
use crate::genotype::Genotype;
use crate::population::Population;
use crate::strategy::evolve::{EvolveConfig, EvolveReporter};
use rand::Rng;

/// The placeholder for when no extension present
#[derive(Debug, Clone)]
pub struct Noop;

impl Extension for Noop {
    fn call<G: Genotype, R: Rng, SR: EvolveReporter<Genotype = G>>(
        &mut self,
        _genotype: &G,
        _evolve_config: &EvolveConfig,
        _population: &mut Population<G>,
        _reporter: &mut SR,
        _rng: &mut R,
    ) {
    }
}

impl Noop {
    pub fn new() -> Self {
        Self
    }
}
