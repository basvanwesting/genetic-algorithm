//! solution strategies for finding the best chromosomes.
//!
//! There are 4 strategies:
//! * [Evolve, Standard](self::evolve::Evolve)
//! * [Permutate, Standard](self::permutate::Permutate)
//! * [HillClimb, Stochastic](self::hill_climb::HillClimb)
//! * [HillClimb, SteepestAscent](self::hill_climb::HillClimb)
//!
//! See strategies for details. Normally, you build a specific strategy and call directly from the
//! specific builder. But there is an option for building the superset [StrategyBuilder] and calling
//! from there. You call from the builder, as repeatedly options clone the builder first. The
//! execution is switched based on the provided `with_variant()`. The call options are:
//! * `call()` simply run once
//! * `call_repeatedly(usize)`, call repeatedly and take the best (or short-circuit on target fitness score)
//!   * fallback to `call()` once for Permutate
//! * `call_par_repeatedly(usize)`, as above, but high level parallel execution
//!   * fallback to `call()` once for Permutate, but force `with_par_fitness(true)`
//! * `call_speciated(usize)`, call repeatedly and then run one final round with the best chromosomes from the previous rounds as seeds
//!   * fallback to `call()` once for Permutate
//!   * fallback to `call_repeatedly(usize)` for HillClimb
//! * `call_par_speciated(usize)`, as above, but high level parallel execution
//!   * fallback to `call()` once for Permutate, but force `with_par_fitness(true)`
//!   * fallback to `call_par_repeatedly(usize)` for HillClimb
//!
//! *Note: Only Genotypes which implement all strategies are eligable for the superset builder.*
//! *RangeGenotype and other floating point range based genotypes currently do not support Permutation*
//!
//! Example:
//! ```
//! use genetic_algorithm::strategy::prelude::*;
//! use genetic_algorithm::fitness::placeholders::CountTrue;
//!
//! // the search space
//! let genotype = BinaryGenotype::builder()
//!     .with_genes_size(10)
//!     .with_genes_hashing(true) // store genes_hash on chromosome (required for fitness_cache, optional for better population cardinality estimation)
//!     .build()
//!     .unwrap();
//!
//! // the search strategy (superset), steps marked (E)volve, (H)illClimb and (P)ermutate
//! let builder = StrategyBuilder::new()
//!     .with_genotype(genotype)                                // (E,H,P) the genotype
//!     .with_select(SelectElite::new(0.5, 0.02))                        // (E) sort the chromosomes by fitness to determine crossover order and drop excess population above target_population_size
//!     .with_extension(ExtensionMassExtinction::new(10, 0.1))  // (E) optional builder step, simulate cambrian explosion by mass extinction, when fitness score cardinality drops to 10 after the selection, trim to 10% of population
//!     .with_crossover(CrossoverUniform::new(0.7, 0.8))        // (E) crossover all individual genes between 2 chromosomes for offspring with 40% parent selection (60% do not produce offspring) and 80% chance of crossover (20% of parents just clone)
//!     .with_mutate(MutateSingleGene::new(0.2))                // (E) mutate offspring for a single gene with a 20% probability per chromosome
//!     .with_fitness(CountTrue)                                // (E,H,P) count the number of true values in the chromosomes
//!     .with_fitness_ordering(FitnessOrdering::Minimize)       // (E,H,P) aim for the least true values
//!     .with_fitness_cache(1000)                               // (E) enable caching of fitness values, only works when genes_hash is stored in chromosome.
//!     .with_par_fitness(true)                                 // (E,H,P) optional, defaults to false, use parallel fitness calculation
//!     .with_target_population_size(100)                       // (E) evolve with 100 chromosomes
//!     .with_target_fitness_score(0)                           // (E,H) ending condition if 0 times true in the best chromosome
//!     .with_valid_fitness_score(1)                            // (E,H) block ending conditions until at most a 1 times true in the best chromosome
//!     .with_max_stale_generations(100)                        // (E,H) stop searching if there is no improvement in fitness score for 100 generations
//!     .with_max_chromosome_age(10)                            // (E) kill chromosomes after 10 generations
//!     .with_reporter(StrategyReporterSimple::new(usize::MAX)) // (E,H,P) optional builder step, report on new best chromsomes only
//!     .with_replace_on_equal_fitness(true)                    // (E,H,P) optional, defaults to false, maybe useful to avoid repeatedly seeding with the same best chromosomes after mass extinction events
//!     .with_rng_seed_from_u64(0);                             // (E,H) for testing with deterministic results
//!
//! // the search strategy (specified)
//! let (strategy, _) = builder
//!     .with_variant(StrategyVariant::Permutate(PermutateVariant::Standard))
//!     // .with_variant(StrategyVariant::Evolve(EvolveVariant::Standard))build str
//!     // .with_variant(StrategyVariant::HillClimb(HillClimbVariant::Stochastic))
//!     // .with_variant(StrategyVariant::HillClimb(HillClimbVariant::SteepAscent))
//!     .call_speciated(3)
//!     .unwrap();
//!
//! // it's all about the best genes after all
//! let (best_genes, best_fitness_score) = strategy.best_genes_and_fitness_score().unwrap();
//! assert_eq!(best_genes, vec![false; 10]);
//! assert_eq!(best_fitness_score, 0);
//! ````
pub mod builder;
pub mod evolve;
pub mod hill_climb;
pub mod permutate;
pub mod prelude;
pub mod reporter;

use self::evolve::EvolveVariant;
use self::hill_climb::HillClimbVariant;
use self::permutate::PermutateVariant;
use crate::chromosome::Chromosome;
use crate::extension::ExtensionEvent;
use crate::fitness::{FitnessCache, FitnessOrdering, FitnessValue};
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
    SetupAndCleanup,
    Extension,
    Select,
    Crossover,
    Mutate,
    Fitness,
    UpdateBestChromosome,
    Other,
}
pub const STRATEGY_ACTIONS: [StrategyAction; 8] = [
    StrategyAction::SetupAndCleanup,
    StrategyAction::Extension,
    StrategyAction::Select,
    StrategyAction::Crossover,
    StrategyAction::Mutate,
    StrategyAction::Fitness,
    StrategyAction::UpdateBestChromosome,
    StrategyAction::Other,
];

#[derive(Copy, Clone, Debug)]
pub enum StrategyVariant {
    Evolve(EvolveVariant),
    HillClimb(HillClimbVariant),
    Permutate(PermutateVariant),
}
impl Display for StrategyVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StrategyVariant::Evolve(EvolveVariant::Standard) => write!(f, "evolve"),
            StrategyVariant::HillClimb(HillClimbVariant::Stochastic) => {
                write!(f, "hill_climb/stochastic")
            }
            StrategyVariant::HillClimb(HillClimbVariant::SteepestAscent) => {
                write!(f, "hill_climb/steepest_ascent")
            }
            StrategyVariant::Permutate(PermutateVariant::Standard) => write!(f, "permutate"),
        }
    }
}

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
    /// strategy can be boxed, need a way to get to the reporter
    fn flush_reporter(&mut self, _output: &mut Vec<u8>);
}

pub trait StrategyConfig: Display {
    fn variant(&self) -> StrategyVariant;
    fn fitness_ordering(&self) -> FitnessOrdering;
    // stored on config instead of state as it is a cache external to the strategy
    fn fitness_cache(&self) -> Option<&FitnessCache> {
        None
    }
    fn par_fitness(&self) -> bool;
    fn replace_on_equal_fitness(&self) -> bool;
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
    fn population_cardinality(&self) -> Option<usize>;
    fn durations(&self) -> &HashMap<StrategyAction, Duration>;
    fn add_duration(&mut self, action: StrategyAction, duration: Duration);
    fn total_duration(&self) -> Duration;
    fn close_duration(&mut self, total_duration: Duration) {
        if let Some(other_duration) = total_duration.checked_sub(self.total_duration()) {
            self.add_duration(StrategyAction::Other, other_duration);
        }
    }
    fn fitness_duration_rate(&self) -> f32 {
        let fitness_duration = self
            .durations()
            .get(&StrategyAction::Fitness)
            .copied()
            .unwrap_or_else(Duration::default);
        fitness_duration.as_secs_f32() / self.total_duration().as_secs_f32()
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
/// You are encouraged to roll your own implementation, depending on your needs.
///
/// Event hooks in lifecycle:
/// * `on_enter` (before setup)
/// * `on_start` (of run loop)
/// * *in run loop:*
///     * `on_new_generation`
///     * `on_new_best_chromosome`
///     * `on_new_best_chromosome_equal_fitness`
///     * `on_extension_event`
///     * `on_mutate_event`
/// * `on_finish` (of run loop)
/// * `on_exit` (after cleanup)
///
/// It has an associated type Genotype, just like Fitness, so you can implement reporting with
/// access to your domain's specific Genotype and Chromosome etc..
///
/// For reference, take a look at the provided strategy independent
/// [StrategyReporterSimple](self::reporter::Simple) implementation, or strategy specific
/// [EvolveReporterSimple](self::evolve::EvolveReporterSimple),
/// [HillClimbReporterSimple](self::hill_climb::HillClimbReporterSimple) and
/// [PermutateReporterSimple](self::permutate::PermutateReporterSimple) implementations.
///
/// # Example:
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
///                 "periodic - current_generation: {}, stale_generations: {}, best_generation: {}, scale_index: {:?}",
///                 state.current_generation(),
///                 state.stale_generations(),
///                 state.best_generation(),
///                 state.current_scale_index(),
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
///             "new best - generation: {}, fitness_score: {:?}, scale_index: {:?}",
///             state.current_generation(),
///             state.best_fitness_score(),
///             state.current_scale_index(),
///         );
///     }
///
///     fn on_exit<S: StrategyState<Self::Genotype>, C: StrategyConfig>(
///         &mut self,
///         _genotype: &Self::Genotype,
///         state: &S,
///         _config: &C,
///     ) {
///         println!("exit - iteration: {}", state.current_iteration());
///         STRATEGY_ACTIONS.iter().for_each(|action| {
///             if let Some(duration) = state.durations().get(action) {
///                 println!("  {:?}: {:.3?}", action, duration);
///             }
///         });
///         println!(
///             "  Total: {:.3?} ({:.0}% fitness)",
///             &state.total_duration(),
///             state.fitness_duration_rate() * 100.0
///         );
///     }
/// }
/// ```
pub trait StrategyReporter: Clone + Send + Sync {
    type Genotype: Genotype;

    fn flush(&mut self, _output: &mut Vec<u8>) {}
    fn on_enter<S: StrategyState<Self::Genotype>, C: StrategyConfig>(
        &mut self,
        _genotype: &Self::Genotype,
        _state: &S,
        _config: &C,
    ) {
    }
    fn on_exit<S: StrategyState<Self::Genotype>, C: StrategyConfig>(
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
