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

/// Stores the state of the strategy.
/// The expected general fields are:
/// * current_iteration: `usize`
/// * current_generation: `usize`
/// * best_generation: `usize`
/// * best_chromosome: `Option<Chromosome<G>>`
pub trait StrategyState<G: Genotype> {
    fn best_chromosome_as_ref(&self) -> Option<&Chromosome<G>>;
    fn best_chromosome(&self) -> Option<Chromosome<G>>;
    fn best_fitness_score(&self) -> Option<FitnessValue>;
    fn best_generation(&self) -> usize;
    fn current_generation(&self) -> usize;
    fn current_iteration(&self) -> usize;

    // return tuple (new_best_chomesome, improved_fitness). This way a sideways move in
    // best_chromosome (with equal fitness, which doesn't update the best_generation) can be
    // distinguished for reporting purposes
    fn set_best_chromosome(
        &mut self,
        best_chromosome: &Chromosome<G>,
        improved_fitness: bool,
    ) -> (bool, bool);

    // return tuple (new_best_chomesome, improved_fitness). This way a sideways move in
    // best_chromosome (with equal fitness, which doesn't update the best_generation) can be
    // distinguished for reporting purposes
    fn update_best_chromosome(
        &mut self,
        contending_chromosome: &Chromosome<G>,
        fitness_ordering: &FitnessOrdering,
        replace_on_equal_fitness: bool,
    ) -> (bool, bool) {
        match self.best_chromosome_as_ref() {
            None => self.set_best_chromosome(contending_chromosome, true),
            Some(current_best_chromosome) => {
                match (
                    current_best_chromosome.fitness_score,
                    contending_chromosome.fitness_score,
                ) {
                    (None, None) => (false, false),
                    (Some(_), None) => (false, false),
                    (None, Some(_)) => self.set_best_chromosome(contending_chromosome, true),
                    (Some(current_fitness_score), Some(contending_fitness_score)) => {
                        match fitness_ordering {
                            FitnessOrdering::Maximize => {
                                if contending_fitness_score > current_fitness_score {
                                    self.set_best_chromosome(contending_chromosome, true)
                                } else if replace_on_equal_fitness
                                    && contending_fitness_score == current_fitness_score
                                {
                                    self.set_best_chromosome(contending_chromosome, false)
                                } else {
                                    (false, false)
                                }
                            }
                            FitnessOrdering::Minimize => {
                                if contending_fitness_score < current_fitness_score {
                                    self.set_best_chromosome(contending_chromosome, true)
                                } else if replace_on_equal_fitness
                                    && contending_fitness_score == current_fitness_score
                                {
                                    self.set_best_chromosome(contending_chromosome, false)
                                } else {
                                    (false, false)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Reporter with event hooks in the Strategy process
///
/// Supertrait StrategyReporter is not used, because of error
/// [E0658: associated type defaults are unstable](https://github.com/rust-lang/rust/issues/29661)
/// So it is only shadowed, as if it existed as a supertrait for now.
///
/// ```ignore
/// pub trait PermutateReporter: StrategyReporter
/// where
///     <Self as StrategyReporter>::Genotype: PermutableGenotype,
/// {
///     type State = PermutateState<Self::Genotype>;
/// }
/// ```
pub trait StrategyReporter: Clone + Send + Sync {
    type Genotype: Genotype;
    type State: StrategyState<Self::Genotype>;

    fn on_start(&mut self, _state: &Self::State) {}
    fn on_finish(&mut self, _state: &Self::State) {}
    fn on_new_generation(&mut self, _state: &Self::State) {}
    fn on_new_best_chromosome(&mut self, _state: &Self::State) {}
}
