use crate::genotype::Genotype;
use crate::population::Population;
use rand::prelude::*;

pub trait Compete: Clone + std::fmt::Debug {
    fn call<T: Genotype, R: Rng>(
        &self,
        population: Population<T>,
        target_population_size: usize,
        rng: &mut R,
    ) -> Population<T>;
}

#[derive(Clone, Debug)]
pub enum Competes {
    Elite,
    Tournament,
}
pub type TournamentSize = usize;

#[derive(Clone, Debug)]
pub struct CompeteDispatch(pub Competes, pub TournamentSize);
impl Compete for CompeteDispatch {
    fn call<T: Genotype, R: Rng>(
        &self,
        population: Population<T>,
        target_population_size: usize,
        rng: &mut R,
    ) -> Population<T> {
        match self.0 {
            Competes::Elite => CompeteElite.call(population, target_population_size, rng),
            Competes::Tournament => {
                CompeteTournament(self.1).call(population, target_population_size, rng)
            }
        }
    }
}

mod elite;
pub use self::elite::Elite as CompeteElite;

mod tournament;
pub use self::tournament::Tournament as CompeteTournament;
