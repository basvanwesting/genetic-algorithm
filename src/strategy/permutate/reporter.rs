use super::{PermutateReporter, PermutateState};
use crate::genotype::PermutableGenotype;
use crate::strategy::StrategyState;
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
                "current_generation: {}, best_generation: {}, best_fitness_score: {:?}",
                state.current_generation(),
                state.best_generation(),
                state.best_fitness_score(),
            );
        }
    }
}
