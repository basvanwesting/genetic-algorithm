use super::{PermutateConfig, PermutateState};
use crate::genotype::{Allele, PermutableGenotype};
use crate::strategy::StrategyState;
use num::BigUint;

/// Reporter with event hooks in the Permutate process.
/// A new generation is simply handling a single new chromosome from the total population
///
/// # Example:
/// You are encouraged to take a look at the [PermutateReporterSimple](Simple) implementation, and
/// then roll your own like below:
/// ```rust
/// use genetic_algorithm::strategy::permutate::prelude::*;
/// use num::BigUint;
///
/// #[derive(Clone)]
/// pub struct CustomReporter { pub period: usize };
/// impl PermutateReporter for CustomReporter {
///     fn on_new_generation<A: Allele>(&mut self, state: &PermutateState<A>, _config: &PermutateConfig) {
///         if state.current_generation() % self.period == 0 {
///             println!(
///                 "progress: {:2.2}%, current_generation: {}, best_generation: {}",
///                 BigUint::from(state.current_generation() * 100) / &state.total_population_size,
///                 state.current_generation(),
///                 state.best_generation(),
///             );
///         }
///     }
///
///     fn on_new_best_chromosome<A: Allele>(&mut self, state: &PermutateState<A>, _config: &PermutateConfig) {
///         println!(
///             "new best - generation: {}, fitness_score: {:?}, genes: {:?}",
///             state.current_generation(),
///             state.best_fitness_score(),
///             state.best_chromosome_as_ref().map(|c| &c.genes),
///         );
///     }
/// }
/// ```
pub trait Reporter: Clone + Send + Sync {
    fn on_start<A: Allele, G: PermutableGenotype>(
        &mut self,
        _genotype: &G,
        _state: &PermutateState<A>,
        _config: &PermutateConfig,
    ) {
    }
    fn on_finish<A: Allele>(&mut self, _state: &PermutateState<A>, _config: &PermutateConfig) {}
    fn on_new_generation<A: Allele>(
        &mut self,
        _state: &PermutateState<A>,
        _config: &PermutateConfig,
    ) {
    }
    fn on_new_best_chromosome<A: Allele>(
        &mut self,
        _state: &PermutateState<A>,
        _config: &PermutateConfig,
    ) {
    }
}

/// The noop reporter, silences reporting
#[derive(Clone, Default)]
pub struct Noop;
impl Noop {
    pub fn new() -> Self {
        Self::default()
    }
}
impl Reporter for Noop {}

/// A Simple reporter generic over Genotype.
/// A report is triggered every period generations
#[derive(Clone)]
pub struct Simple {
    pub period: usize,
    pub show_genes: bool,
}
impl Default for Simple {
    fn default() -> Self {
        Self {
            period: 1,
            show_genes: false,
        }
    }
}
impl Simple {
    pub fn new(period: usize) -> Self {
        Self {
            period,
            ..Default::default()
        }
    }
    pub fn new_with_flags(period: usize, show_genes: bool) -> Self {
        Self {
            period,
            show_genes,
            ..Default::default()
        }
    }
}
impl Reporter for Simple {
    fn on_new_generation<A: Allele>(
        &mut self,
        state: &PermutateState<A>,
        _config: &PermutateConfig,
    ) {
        if state.current_generation() % self.period == 0 {
            let width = state.total_population_size.to_string().len();
            println!(
                "progress: {:3.3}%, current_generation: {:>width$}, best_generation: {:>width$}",
                BigUint::from(state.current_generation() * 100) / &state.total_population_size,
                state.current_generation(),
                state.best_generation(),
            );
        }
    }

    fn on_new_best_chromosome<A: Allele>(
        &mut self,
        state: &PermutateState<A>,
        _config: &PermutateConfig,
    ) {
        println!(
            "new best - generation: {}, fitness_score: {:?}, genes: {:?}",
            state.current_generation(),
            state.best_fitness_score(),
            if self.show_genes {
                state.best_chromosome_as_ref().map(|c| &c.genes)
            } else {
                None
            },
        );
    }
}

/// A log-level based reporter for debug and trace, runs on each generation
#[derive(Clone, Default)]
pub struct Log;
impl Log {
    pub fn new() -> Self {
        Self::default()
    }
}
impl Reporter for Log {
    fn on_new_generation<A: Allele>(
        &mut self,
        state: &PermutateState<A>,
        _config: &PermutateConfig,
    ) {
        log::debug!(
            "progress: {:2.2}%, current_generation: {}, best_generation: {}, best_fitness_score: {:?}",
            BigUint::from(state.current_generation() * 100) / &state.total_population_size,
            state.current_generation(),
            state.best_generation(),
            state.best_fitness_score(),
        );

        log::trace!(
            "best - fitness score: {:?}, genes: {:?}",
            state.best_fitness_score(),
            state.best_chromosome_as_ref().map(|c| &c.genes)
        );
    }
}
