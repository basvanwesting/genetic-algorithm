use super::Extension;
use crate::genotype::{EvolveGenotype, Genotype};
use crate::strategy::evolve::{EvolveConfig, EvolveState};
use crate::strategy::StrategyReporter;
use rand::Rng;
use std::marker::PhantomData;

/// The placeholder for when no extension present
#[derive(Debug, Clone)]
pub struct Noop<G: Genotype>(PhantomData<G>);

impl<G: EvolveGenotype> Extension for Noop<G> {
    type Genotype = G;

    fn call<R: Rng, SR: StrategyReporter<Genotype = G>>(
        &mut self,
        _genotype: &G,
        _state: &mut EvolveState<G>,
        _config: &EvolveConfig,
        _reporter: &mut SR,
        _rng: &mut R,
    ) {
    }
}

impl<G: Genotype> Noop<G> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}
impl<G: Genotype> Default for Noop<G> {
    fn default() -> Self {
        Self::new()
    }
}
