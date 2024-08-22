use super::Extension;
use crate::genotype::Genotype;
use crate::strategy::evolve::{EvolveConfig, EvolveReporter, EvolveState};
use rand::Rng;

/// The placeholder for when no extension present
#[derive(Debug, Clone)]
pub struct Noop;

impl Extension for Noop {
    fn call<G: Genotype, R: Rng + Clone + Send + Sync, SR: EvolveReporter<Allele = G::Allele>>(
        &mut self,
        _genotype: &G,
        _state: &mut EvolveState<G::Allele>,
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
