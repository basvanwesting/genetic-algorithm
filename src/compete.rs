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

mod elite;
pub use self::elite::Elite as CompeteElite;

mod tournament;
pub use self::tournament::Tournament as CompeteTournament;

mod dispatch;
pub use self::dispatch::Dispatch as CompeteDispatch;
