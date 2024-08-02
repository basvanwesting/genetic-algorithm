use super::{HillClimbReporter, HillClimbState};
use crate::genotype::IncrementalGenotype;
use crate::strategy::StrategyState;
use std::marker::PhantomData;

#[derive(Clone)]
pub struct Noop<G: IncrementalGenotype>(pub PhantomData<G>);
impl<G: IncrementalGenotype> Default for Noop<G> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
impl<G: IncrementalGenotype + Sync + Clone + Send> HillClimbReporter for Noop<G> {
    type Genotype = G;
}

#[derive(Clone)]
pub struct Simple<G: IncrementalGenotype> {
    pub frequency: usize,
    _phantom: PhantomData<G>,
}
impl<G: IncrementalGenotype> Simple<G> {
    pub fn new(frequency: usize) -> Self {
        Self {
            frequency,
            _phantom: PhantomData,
        }
    }
}
impl<G: IncrementalGenotype + Sync + Clone + Send> HillClimbReporter for Simple<G> {
    type Genotype = G;

    fn on_new_generation(&mut self, state: &HillClimbState<Self::Genotype>) {
        if state.current_generation() % self.frequency == 0 {
            println!(
                "current_generation: {}, best_generation: {}, best_fitness_score: {:?}, current scale: {:?}",
                state.current_generation(),
                state.best_generation(),
                state.best_fitness_score(),
                state.current_scale.as_ref(),
            );
        }
    }
}

#[derive(Clone)]
pub struct Log<G: IncrementalGenotype> {
    pub frequency: usize,
    _phantom: PhantomData<G>,
}
impl<G: IncrementalGenotype> Log<G> {
    pub fn new(frequency: usize) -> Self {
        Self {
            frequency,
            _phantom: PhantomData,
        }
    }
}
impl<G: IncrementalGenotype + Sync + Clone + Send> HillClimbReporter for Log<G> {
    type Genotype = G;

    fn on_new_generation(&mut self, state: &HillClimbState<Self::Genotype>) {
        log::debug!(
            "generation (current/best): {}/{}, fitness score (best): {:?}, current scale: {:?}",
            state.current_generation,
            state.best_generation,
            state.best_fitness_score(),
            state.current_scale.as_ref(),
        );
        log::trace!(
            "best - fitness score: {:?}, genes: {:?}",
            state.best_fitness_score(),
            state
                .best_chromosome
                .as_ref()
                .map_or(vec![], |c| c.genes.clone()),
        );
    }
}
