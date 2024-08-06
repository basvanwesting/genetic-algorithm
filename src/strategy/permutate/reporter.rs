use super::{PermutateReporter, PermutateState};
use crate::genotype::PermutableGenotype;
use crate::strategy::StrategyState;
use num::BigUint;
use std::marker::PhantomData;

#[derive(Clone)]
pub struct Noop<G: PermutableGenotype>(pub PhantomData<G>);
impl<G: PermutableGenotype> Default for Noop<G> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
impl<G: PermutableGenotype + Sync + Clone + Send> PermutateReporter for Noop<G> {
    type Genotype = G;
}

#[derive(Clone)]
pub struct Simple<G: PermutableGenotype> {
    pub frequency: usize,
    _phantom: PhantomData<G>,
}
impl<G: PermutableGenotype> Simple<G> {
    pub fn new(frequency: usize) -> Self {
        Self {
            frequency,
            _phantom: PhantomData,
        }
    }
}
impl<G: PermutableGenotype + Sync + Clone + Send> PermutateReporter for Simple<G> {
    type Genotype = G;

    fn on_new_generation(&mut self, state: &PermutateState<Self::Genotype>) {
        if state.current_generation() % self.frequency == 0 {
            println!(
                "progress: {:2.2}%, current_generation: {}, best_generation: {}, best_fitness_score: {:?}",
                BigUint::from(state.current_generation() * 100) / &state.total_population_size,
                state.current_generation(),
                state.best_generation(),
                state.best_fitness_score(),
            );
        }
    }

    fn on_new_best_chromosome(&mut self, state: &PermutateState<Self::Genotype>) {
        println!(
            "progress: {:2.2}%, current_generation: {}, best_generation: now, best_fitness_score: {:?}",
            BigUint::from(state.current_generation() * 100) / &state.total_population_size,
            state.current_generation(),
            state.best_fitness_score(),
        );
    }
}

/// A log-level based reporter for debug and trace, runs on each generation
#[derive(Clone)]
pub struct Log<G: PermutableGenotype>(pub PhantomData<G>);
impl<G: PermutableGenotype> Default for Log<G> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
impl<G: PermutableGenotype + Sync + Clone + Send> PermutateReporter for Log<G> {
    type Genotype = G;

    fn on_new_generation(&mut self, state: &PermutateState<Self::Genotype>) {
        log::debug!(
            "progress: {:2.2}%, current_generation: {}, best_generation: {}, best_fitness_score: {:?}",
            BigUint::from(state.current_generation() * 100) / &state.total_population_size,
            state.current_generation(),
            state.best_generation(),
            state.best_fitness_score(),
        );

        if log::log_enabled!(log::Level::Trace) {
            log::trace!(
                "best - fitness score: {:?}, genes: {:?}",
                state.best_fitness_score(),
                state
                    .best_chromosome_as_ref()
                    .map_or(vec![], |c| c.genes.clone()),
            );
        }
    }
}
