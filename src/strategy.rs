//! solution strategies for finding the best chromosomes.
pub mod evolve;
pub mod hill_climb;
pub mod permutate;

use crate::chromosome::Chromosome;
use crate::fitness::{FitnessOrdering, FitnessValue};
use crate::genotype::Genotype;
use crate::population::Population;
use std::time::Duration;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum StrategyAction {
    Init,
    Extension,
    Compete,
    Crossover,
    Mutate,
    Fitness,
    UpdateBestChromosome,
    Other,
}
pub const STRATEGY_ACTIONS: [StrategyAction; 8] = [
    StrategyAction::Init,
    StrategyAction::Extension,
    StrategyAction::Compete,
    StrategyAction::Crossover,
    StrategyAction::Mutate,
    StrategyAction::Fitness,
    StrategyAction::UpdateBestChromosome,
    StrategyAction::Other,
];

pub trait Strategy<G: Genotype> {
    fn call(&mut self);
    fn best_chromosome(&self) -> Option<Chromosome<G>>;
    fn best_generation(&self) -> usize;
    fn best_fitness_score(&self) -> Option<FitnessValue>;
}

pub trait StrategyConfig {
    fn fitness_ordering(&self) -> FitnessOrdering;
    fn par_fitness(&self) -> bool;
    fn replace_on_equal_fitness(&self) -> bool;
}

/// Stores the state of the strategy.
/// The expected general fields are:
/// * current_iteration: `usize`
/// * current_generation: `usize`
/// * best_generation: `usize`
/// * best_chromosome: `Chromosome<G>`
/// * chromosome: `Chromosome<G>`
/// * populatoin: `Population<G>` // may be empty
pub trait StrategyState<G: Genotype> {
    fn chromosome_as_ref(&self) -> &Chromosome<G>;
    fn population_as_ref(&self) -> &Population<G>;
    fn chromosome_as_mut(&mut self) -> &mut Chromosome<G>;
    fn population_as_mut(&mut self) -> &mut Population<G>;
    fn best_chromosome_as_ref(&self) -> &Chromosome<G>;
    fn best_fitness_score(&self) -> Option<FitnessValue> {
        self.best_chromosome_as_ref().fitness_score
    }
    fn best_generation(&self) -> usize;
    fn current_generation(&self) -> usize;
    fn current_iteration(&self) -> usize;
    fn stale_generations(&self) -> usize;
    fn add_duration(&mut self, action: StrategyAction, duration: Duration);
    fn total_duration(&self) -> Duration;
    fn close_duration(&mut self, total_duration: Duration) {
        if let Some(other_duration) = total_duration.checked_sub(self.total_duration()) {
            self.add_duration(StrategyAction::Other, other_duration);
        }
    }

    fn increment_stale_generations(&mut self);
    fn reset_stale_generations(&mut self);

    // return tuple (new_best_chomesome, improved_fitness). This way a sideways move in
    // best_chromosome (with equal fitness, which doesn't update the best_generation) can be
    // distinguished for reporting purposes
    fn store_best_chromosome(&mut self, improved_fitness: bool) -> (bool, bool);

    // return tuple (new_best_chomesome, improved_fitness). This way a sideways move in
    // best_chromosome (with equal fitness, which doesn't update the best_generation) can be
    // distinguished for reporting purposes
    // TODO: because the StrategyReporter trait is not used, all StrategyState are implementing a
    // specialized version of this function for additional reporting
    fn update_best_chromosome(
        &mut self,
        fitness_ordering: &FitnessOrdering,
        replace_on_equal_fitness: bool,
    ) -> (bool, bool) {
        match (
            self.best_chromosome_as_ref().fitness_score,
            self.chromosome_as_ref().fitness_score,
        ) {
            (None, None) => (false, false),
            (Some(_), None) => (false, false),
            (None, Some(_)) => self.store_best_chromosome(true),
            (Some(current_fitness_score), Some(contending_fitness_score)) => match fitness_ordering
            {
                FitnessOrdering::Maximize => {
                    if contending_fitness_score > current_fitness_score {
                        self.store_best_chromosome(true)
                    } else if replace_on_equal_fitness
                        && contending_fitness_score == current_fitness_score
                    {
                        self.store_best_chromosome(false)
                    } else {
                        (false, false)
                    }
                }
                FitnessOrdering::Minimize => {
                    if contending_fitness_score < current_fitness_score {
                        self.store_best_chromosome(true)
                    } else if replace_on_equal_fitness
                        && contending_fitness_score == current_fitness_score
                    {
                        self.store_best_chromosome(false)
                    } else {
                        (false, false)
                    }
                }
            },
        }
    }
}

/// Reporter with event hooks in the Strategy process
///
/// As this is a primary API for clients, which are encouraged to implement their own reporters, we
/// want the API to resemble the Fitness API (which is also custom implemented by clients).
/// Therefore we only want to set a associated trait Allele in the API. The error [E0658:
/// associated type defaults are unstable](https://github.com/rust-lang/rust/issues/29661) blocks
/// this API design. Thus Supertrait StrategyReporter is not used. It is only shadowed, as if it
/// existed as a supertrait for now.
///
pub trait StrategyReporter: Clone + Send + Sync {
    type Genotype: Genotype;
    type State: StrategyState<Self::Genotype>;
    type Config: StrategyConfig;

    fn on_init(&mut self, _state: &Self::State, _config: &Self::Config) {}
    fn on_start(&mut self, _state: &Self::State, _config: &Self::Config) {}
    fn on_finish(&mut self, _state: &Self::State, _config: &Self::Config) {}
    fn on_new_generation(&mut self, _state: &Self::State, _config: &Self::Config) {}
    fn on_new_best_chromosome(&mut self, _state: &Self::State, _config: &Self::Config) {}
    fn on_new_best_chromosome_equal_fitness(
        &mut self,
        _state: &Self::State,
        _config: &Self::Config,
    ) {
    }
}
