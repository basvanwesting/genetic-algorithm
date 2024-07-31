//! solution strategies for finding the best chromosomes.
pub mod evolve;
pub mod hill_climb;
pub mod permutate;

use crate::chromosome::Chromosome;
use crate::fitness::{FitnessOrdering, FitnessValue};
use crate::genotype::Genotype;
use rand::Rng;

pub trait Strategy<G: Genotype> {
    fn call<R: Rng>(&mut self, rng: &mut R);
    fn best_chromosome(&self) -> Option<Chromosome<G>>;
    fn best_generation(&self) -> usize;
    fn best_fitness_score(&self) -> Option<FitnessValue>;
}

pub trait StrategyConfig {
    fn fitness_ordering(&self) -> FitnessOrdering;
    fn multithreading(&self) -> bool;
}

pub trait StrategyState<G: Genotype, C: StrategyConfig> {
    fn best_chromosome(&self) -> Option<Chromosome<G>>;
    fn best_fitness_score(&self) -> Option<FitnessValue>;
    fn best_generation(&self) -> usize;
    fn update_best_chromosome(&mut self, contending_best_chromosome: &Chromosome<G>, config: &C);
}
