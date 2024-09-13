//! solution strategies for finding the best chromosomes.
pub mod builder;
pub mod evolve;
pub mod hill_climb;
pub mod permutate;
pub mod reporter;

use crate::chromosome::Chromosome;
use crate::extension::ExtensionEvent;
use crate::fitness::{FitnessOrdering, FitnessValue};
use crate::genotype::Genotype;
use crate::mutate::MutateEvent;
use crate::population::Population;
use std::collections::HashMap;
use std::fmt::Display;
use std::time::Duration;

pub use self::builder::{
    Builder as StrategyBuilder, TryFromBuilderError as TryFromStrategyBuilderError,
};

pub use self::reporter::Duration as StrategyReporterDuration;
pub use self::reporter::Noop as StrategyReporterNoop;
pub use self::reporter::Simple as StrategyReporterSimple;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum StrategyAction {
    Init,
    Extension,
    Select,
    Crossover,
    Mutate,
    Fitness,
    UpdateBestChromosome,
    Other,
}
pub const STRATEGY_ACTIONS: [StrategyAction; 8] = [
    StrategyAction::Init,
    StrategyAction::Extension,
    StrategyAction::Select,
    StrategyAction::Crossover,
    StrategyAction::Mutate,
    StrategyAction::Fitness,
    StrategyAction::UpdateBestChromosome,
    StrategyAction::Other,
];

pub trait Strategy<G: Genotype> {
    fn call(&mut self);
    fn best_generation(&self) -> usize;
    fn best_fitness_score(&self) -> Option<FitnessValue>;
    fn best_genes(&self) -> Option<G::Genes>;
    fn best_genes_and_fitness_score(&self) -> Option<(G::Genes, FitnessValue)> {
        if let Some(fitness_value) = self.best_fitness_score() {
            self.best_genes().map(|genes| (genes, fitness_value))
        } else {
            None
        }
    }
}

pub trait StrategyConfig: Display {
    fn fitness_ordering(&self) -> FitnessOrdering;
    fn par_fitness(&self) -> bool;
    fn replace_on_equal_fitness(&self) -> bool;
    fn estimated_progress_perc(&self, _current_generation: usize) -> Option<u32> {
        None
    }
}

/// Stores the state of the strategy.
/// The expected general fields are:
/// * current_iteration: `usize`
/// * current_generation: `usize`
/// * best_generation: `usize`
/// * best_chromosome: `G::Chromosome`
/// * chromosome: `G::Chromosome`
/// * populatoin: `Population<G::Chromosome>` // may be empty
pub trait StrategyState<G: Genotype>: Display {
    fn chromosome_as_ref(&self) -> &Option<G::Chromosome>;
    fn chromosome_as_mut(&mut self) -> &mut Option<G::Chromosome>;
    fn population_as_ref(&self) -> &Population<G::Chromosome>;
    fn population_as_mut(&mut self) -> &mut Population<G::Chromosome>;
    fn best_fitness_score(&self) -> Option<FitnessValue>;
    fn best_generation(&self) -> usize;
    fn current_generation(&self) -> usize;
    fn current_iteration(&self) -> usize;
    fn stale_generations(&self) -> usize;
    fn current_scale_index(&self) -> Option<usize>;
    fn durations(&self) -> &HashMap<StrategyAction, Duration>;
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
    // TODO: because the StrategyReporter trait is not used, all StrategyState are implementing a
    // specialized version of this function for additional reporting
    fn is_better_chromosome(
        &self,
        contending_chromosome: &G::Chromosome,
        fitness_ordering: &FitnessOrdering,
        replace_on_equal_fitness: bool,
    ) -> (bool, bool) {
        match (
            self.best_fitness_score(),
            contending_chromosome.fitness_score(),
        ) {
            (None, None) => (false, false),
            (Some(_), None) => (false, false),
            (None, Some(_)) => (true, true),
            (Some(current_fitness_score), Some(contending_fitness_score)) => match fitness_ordering
            {
                FitnessOrdering::Maximize => {
                    if contending_fitness_score > current_fitness_score {
                        (true, true)
                    } else if replace_on_equal_fitness
                        && contending_fitness_score == current_fitness_score
                    {
                        (true, false)
                    } else {
                        (false, false)
                    }
                }
                FitnessOrdering::Minimize => {
                    if contending_fitness_score < current_fitness_score {
                        (true, true)
                    } else if replace_on_equal_fitness
                        && contending_fitness_score == current_fitness_score
                    {
                        (true, false)
                    } else {
                        (false, false)
                    }
                }
            },
        }
    }
}

/// Reporter with event hooks for all Strategies.
///
/// It has an associated type Genotype, just like Fitness, so you can implement reporting with
/// access to your domain's specific Genotype, Chromosome etc..
///
/// # Example:
/// You are encouraged to take a look at the [StrategyReporterSimple](self::reporter::Simple) implementation, and
/// then roll your own like below:
/// ```rust
/// use genetic_algorithm::strategy::evolve::prelude::*;
///
/// #[derive(Clone)]
/// pub struct CustomReporter { pub period: usize }
/// impl StrategyReporter for CustomReporter {
///     type Genotype = BinaryGenotype;
///
///     fn on_new_generation<S: StrategyState<Self::Genotype>, C: StrategyConfig>(
///         &mut self,
///         _genotype: &Self::Genotype,
///         state: &S,
///         _config: &C,
///     ) {
///         if state.current_generation() % self.period == 0 {
///             println!(
///                 "periodic - current_generation: {}, stale_generations: {}, best_generation: {}, current_scale_index: {:?}, fitness_score_cardinality: {}, current_population_size: {}",
///                 state.current_generation(),
///                 state.stale_generations(),
///                 state.best_generation(),
///                 state.current_scale_index(),
///                 state.population_as_ref().fitness_score_cardinality(),
///                 state.population_as_ref().size(),
///             );
///         }
///     }
///
///     fn on_new_best_chromosome<S: StrategyState<Self::Genotype>, C: StrategyConfig>(
///         &mut self,
///         _genotype: &Self::Genotype,
///         state: &S,
///         _config: &C,
///     ) {
///         println!(
///             "new best - generation: {}, fitness_score: {:?}, scale_index: {:?}, population_size: {}",
///             state.current_generation(),
///             state.best_fitness_score(),
///             state.current_scale_index(),
///             state.population_as_ref().size(),
///         );
///     }
///
///     fn on_finish<S: StrategyState<Self::Genotype>, C: StrategyConfig>(
///         &mut self,
///         _genotype: &Self::Genotype,
///         state: &S,
///         _config: &C,
///     ) {
///         println!("finish - iteration: {}", state.current_iteration());
///         STRATEGY_ACTIONS.iter().for_each(|action| {
///             if let Some(duration) = state.durations().get(action) {
///                 println!("  {:?}: {:?}", action, duration,);
///             }
///         });
///         println!("  Total: {:?}", &state.total_duration());
///     }
///
/// }
/// ```
pub trait StrategyReporter: Clone + Send + Sync {
    type Genotype: Genotype;
    fn on_init<S: StrategyState<Self::Genotype>, C: StrategyConfig>(
        &mut self,
        _genotype: &Self::Genotype,
        _state: &S,
        _config: &C,
    ) {
    }
    fn on_start<S: StrategyState<Self::Genotype>, C: StrategyConfig>(
        &mut self,
        _genotype: &Self::Genotype,
        _state: &S,
        _config: &C,
    ) {
    }
    fn on_finish<S: StrategyState<Self::Genotype>, C: StrategyConfig>(
        &mut self,
        _genotype: &Self::Genotype,
        _state: &S,
        _config: &C,
    ) {
    }
    fn on_new_generation<S: StrategyState<Self::Genotype>, C: StrategyConfig>(
        &mut self,
        _genotype: &Self::Genotype,
        _state: &S,
        _config: &C,
    ) {
    }
    fn on_new_best_chromosome<S: StrategyState<Self::Genotype>, C: StrategyConfig>(
        &mut self,
        _genotype: &Self::Genotype,
        _state: &S,
        _config: &C,
    ) {
    }
    fn on_new_best_chromosome_equal_fitness<S: StrategyState<Self::Genotype>, C: StrategyConfig>(
        &mut self,
        _genotype: &Self::Genotype,
        _state: &S,
        _config: &C,
    ) {
    }
    fn on_extension_event<S: StrategyState<Self::Genotype>, C: StrategyConfig>(
        &mut self,
        _event: ExtensionEvent,
        _genotype: &Self::Genotype,
        _state: &S,
        _config: &C,
    ) {
    }
    fn on_mutate_event<S: StrategyState<Self::Genotype>, C: StrategyConfig>(
        &mut self,
        _event: MutateEvent,
        _genotype: &Self::Genotype,
        _state: &S,
        _config: &C,
    ) {
    }
}
