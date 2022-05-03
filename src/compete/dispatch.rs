use super::elite::Elite;
use super::tournament::{Tournament, TournamentSize};
use super::{Compete, Competes};
use crate::genotype::Genotype;
use crate::population::Population;
use rand::prelude::*;

#[derive(Clone, Debug)]
pub struct Dispatch(pub Competes, pub TournamentSize);
impl Compete for Dispatch {
    fn call<T: Genotype, R: Rng>(
        &self,
        population: Population<T>,
        target_population_size: usize,
        rng: &mut R,
    ) -> Population<T> {
        match self.0 {
            Competes::Elite => Elite.call(population, target_population_size, rng),
            Competes::Tournament => {
                Tournament(self.1).call(population, target_population_size, rng)
            }
        }
    }
}
