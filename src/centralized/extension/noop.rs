use super::Extension;
use crate::centralized::genotype::EvolveGenotype;
use crate::centralized::strategy::evolve::{EvolveConfig, EvolveState};
use crate::centralized::strategy::StrategyReporter;
use rand::Rng;

/// The placeholder for when no extension present
#[derive(Debug, Clone)]
pub struct Noop;

impl Extension for Noop {
    fn call<G: EvolveGenotype, R: Rng, SR: StrategyReporter<Genotype = G>>(
        &mut self,
        _genotype: &mut G,
        _state: &mut EvolveState,
        _config: &EvolveConfig,
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
impl Default for Noop {
    fn default() -> Self {
        Self::new()
    }
}
