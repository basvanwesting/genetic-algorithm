use super::Extension;
use crate::genotype::Genotype;
use crate::population::Population;
use crate::strategy::evolve::{EvolveConfig, EvolveState};
use rand::Rng;

/// The placeholder for when no extension present
#[derive(Debug, Clone)]
pub struct Noop;

impl Extension for Noop {
    fn call<G: Genotype, R: Rng>(
        &mut self,
        _genotype: &G,
        _evolve_config: &EvolveConfig,
        _evolve_state: &EvolveState<G>,
        _population: &mut Population<G>,
        _rng: &mut R,
    ) {
    }
}

impl Noop {
    pub fn new() -> Self {
        Self
    }
}
