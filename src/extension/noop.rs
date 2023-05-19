use super::{Extension, ExtensionDispatch, Extensions};
use crate::genotype::Genotype;
use crate::population::Population;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Noop;

impl Extension for Noop {
    fn call<T: Genotype, R: Rng>(
        &self,
        _genotype: &T,
        _population: &mut Population<T>,
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
