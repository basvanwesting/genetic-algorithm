use super::{PermutateConfig, PermutateState};
use crate::genotype::{Allele, PermutableGenotype};
use crate::strategy::StrategyState;
use num::BigUint;
use std::marker::PhantomData;

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
///     type Allele = BinaryAllele;
///
///     fn on_new_generation(&mut self, state: &PermutateState<Self::Allele>, _config: &PermutateConfig) {
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
///     fn on_new_best_chromosome(&mut self, state: &PermutateState<Self::Allele>, _config: &PermutateConfig) {
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
    type Allele: Allele;

    fn on_init<G: PermutableGenotype>(
        &mut self,
        _genotype: &G,
        _state: &PermutateState<Self::Allele>,
        _config: &PermutateConfig,
    ) {
    }
    fn on_start<G: PermutableGenotype>(
        &mut self,
        _genotype: &G,
        _state: &PermutateState<Self::Allele>,
        _config: &PermutateConfig,
    ) {
    }
    fn on_finish(&mut self, _state: &PermutateState<Self::Allele>, _config: &PermutateConfig) {}
    fn on_new_generation(
        &mut self,
        _state: &PermutateState<Self::Allele>,
        _config: &PermutateConfig,
    ) {
    }
    fn on_new_best_chromosome(
        &mut self,
        _state: &PermutateState<Self::Allele>,
        _config: &PermutateConfig,
    ) {
    }
    fn on_new_best_chromosome_equal_fitness(
        &mut self,
        _state: &PermutateState<Self::Allele>,
        _config: &PermutateConfig,
    ) {
    }
}

/// The noop reporter, silences reporting
#[derive(Clone)]
pub struct Noop<A: Allele>(pub PhantomData<A>);
impl<A: Allele> Default for Noop<A> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
impl<A: Allele> Noop<A> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<A: Allele> Reporter for Noop<A> {
    type Allele = A;
}

/// A Simple reporter generic over Genotype.
/// A report is triggered every period generations
#[derive(Clone)]
pub struct Simple<A: Allele> {
    pub period: usize,
    pub show_genes: bool,
    _phantom: PhantomData<A>,
}
impl<A: Allele> Default for Simple<A> {
    fn default() -> Self {
        Self {
            period: 1,
            show_genes: false,
            _phantom: PhantomData,
        }
    }
}
impl<A: Allele> Simple<A> {
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
impl<A: Allele> Reporter for Simple<A> {
    type Allele = A;

    fn on_new_generation(
        &mut self,
        state: &PermutateState<Self::Allele>,
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

    fn on_new_best_chromosome(
        &mut self,
        state: &PermutateState<Self::Allele>,
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
    fn on_finish(&mut self, state: &PermutateState<Self::Allele>, _config: &PermutateConfig) {
        state.durations.iter().for_each(|(tag, duration)| {
            println!("  {}: {:?}", tag, duration);
        })
    }
}

/// A log-level based reporter for debug and trace, runs on each generation
#[derive(Clone)]
pub struct Log<A: Allele>(pub PhantomData<A>);
impl<A: Allele> Default for Log<A> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
impl<A: Allele> Log<A> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<A: Allele> Reporter for Log<A> {
    type Allele = A;

    fn on_new_generation(
        &mut self,
        state: &PermutateState<Self::Allele>,
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
