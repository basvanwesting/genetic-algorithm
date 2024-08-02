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

pub trait StrategyState<G: Genotype> {
    fn best_chromosome_as_ref(&self) -> Option<&Chromosome<G>>;
    fn best_chromosome(&self) -> Option<Chromosome<G>>;
    fn best_fitness_score(&self) -> Option<FitnessValue>;
    fn best_generation(&self) -> usize;
    fn current_generation(&self) -> usize;
    fn set_best_chromosome(
        &mut self,
        best_chromosome: &Chromosome<G>,
        set_best_generation: bool,
    ) -> bool;
    fn update_best_chromosome(
        &mut self,
        contending_best_chromosome: &Chromosome<G>,
        fitness_ordering: &FitnessOrdering,
        replace_on_equal_fitness: bool,
    ) -> bool {
        match self.best_chromosome_as_ref() {
            None => self.set_best_chromosome(contending_best_chromosome, true),
            Some(current_best_chromosome) => {
                match (
                    current_best_chromosome.fitness_score,
                    contending_best_chromosome.fitness_score,
                ) {
                    (None, None) => false,
                    (Some(_), None) => false,
                    (None, Some(_)) => self.set_best_chromosome(contending_best_chromosome, true),
                    (Some(current_fitness_score), Some(contending_fitness_score)) => {
                        match fitness_ordering {
                            FitnessOrdering::Maximize => {
                                if contending_fitness_score > current_fitness_score {
                                    self.set_best_chromosome(contending_best_chromosome, true)
                                } else if replace_on_equal_fitness
                                    && contending_fitness_score == current_fitness_score
                                {
                                    self.set_best_chromosome(contending_best_chromosome, false)
                                } else {
                                    false
                                }
                            }
                            FitnessOrdering::Minimize => {
                                if contending_fitness_score < current_fitness_score {
                                    self.set_best_chromosome(contending_best_chromosome, true)
                                } else if replace_on_equal_fitness
                                    && contending_fitness_score == current_fitness_score
                                {
                                    self.set_best_chromosome(contending_best_chromosome, false)
                                } else {
                                    false
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// pub trait StrategyReporter: Clone + Send {
//     type Genotype: Genotype;
//     type State: StrategyState<Self::Genotype>;
//
//     fn on_start(&mut self, _state: &Self::State) {}
//     fn on_finish(&mut self, _state: &Self::State) {}
//     fn on_new_generation(&mut self, _state: &Self::State) {}
//     fn on_new_best_chromosome(&mut self, _state: &Self::State) {}
// }
//
// use std::marker::PhantomData;
// #[derive(Clone)]
// pub struct NoopReporter<T: Genotype, S: StrategyState<T>>(pub PhantomData<T>, pub PhantomData<S>);
// impl<T: Genotype, S: StrategyState<T>> Default for NoopReporter<T, S> {
//     fn default() -> Self {
//         Self(PhantomData, PhantomData)
//     }
// }
// impl<T: Genotype + Sync, S: StrategyState<T> + Clone + Send> StrategyReporter
//     for NoopReporter<T, S>
// {
//     type Genotype = T;
//     type State = S;
// }
