use super::{Extension, ExtensionDispatch, Extensions};
use crate::compete::Compete;
use crate::crossover::Crossover;
use crate::fitness::Fitness;
use crate::genotype::Genotype;
use crate::mutate::Mutate;
use crate::population::Population;
use crate::strategy::evolve::Evolve;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Noop;

impl Extension for Noop {
    fn call<
        G: Genotype,
        M: Mutate,
        F: Fitness<Genotype = G>,
        S: Crossover,
        C: Compete,
        E: Extension,
        R: Rng,
    >(
        &self,
        _evolve: &Evolve<G, M, F, S, C, E>,
        _population: &mut Population<G>,
        _rng: &mut R,
    ) {
    }
}

impl Noop {
    pub fn new_dispatch() -> ExtensionDispatch {
        ExtensionDispatch {
            extension: Extensions::Noop,
            ..Default::default()
        }
    }
}
