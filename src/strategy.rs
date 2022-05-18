//! solution strategies for finding the best chromosomes.
pub mod evolve;
pub mod permutate;

use crate::chromosome::Chromosome;
use crate::genotype::Genotype;
use rand::Rng;

pub trait Strategy<G: Genotype> {
    fn call<R: Rng>(&mut self, rng: &mut R);
    fn best_chromosome(&self) -> Option<Chromosome<G>>;
}
