//! The competition phase, where chromosomes are lined up for pairing in the
//! [crossover](crate::crossover) phase. Excess chromosomes, beyond the target_population_size,
//! are dropped.
mod elite;
mod tournament;

pub use self::elite::Elite as CompeteElite;
pub use self::tournament::Tournament as CompeteTournament;

use crate::fitness::FitnessOrdering;
use crate::genotype::Genotype;
use crate::population::Population;
use rand::prelude::*;

pub trait Compete: Clone + std::fmt::Debug {
    fn call<T: Genotype, R: Rng>(
        &self,
        population: &mut Population<T>,
        fitness_ordering: FitnessOrdering,
        target_population_size: usize,
        rng: &mut R,
    );
}

#[derive(Clone, Debug)]
pub enum Competes {
    Elite,
    Tournament,
}
pub type TournamentSize = usize;

/// Wrapper for use in [meta analysis](crate::meta)
#[derive(Clone, Debug)]
pub struct CompeteDispatch(pub Competes, pub TournamentSize);
impl Compete for CompeteDispatch {
    fn call<T: Genotype, R: Rng>(
        &self,
        population: &mut Population<T>,
        fitness_ordering: FitnessOrdering,
        target_population_size: usize,
        rng: &mut R,
    ) {
        match self.0 {
            Competes::Elite => {
                CompeteElite.call(population, fitness_ordering, target_population_size, rng)
            }
            Competes::Tournament => CompeteTournament(self.1).call(
                population,
                fitness_ordering,
                target_population_size,
                rng,
            ),
        }
    }
}
