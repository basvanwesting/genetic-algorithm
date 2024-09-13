use crate::extension::ExtensionEvent;
use crate::genotype::Genotype;
use crate::mutate::MutateEvent;
use crate::strategy::{StrategyConfig, StrategyReporter, StrategyState, STRATEGY_ACTIONS};
use std::marker::PhantomData;

/// The noop reporter, silences reporting
#[derive(Clone)]
pub struct Noop<G: Genotype>(pub PhantomData<G>);
impl<G: Genotype> Default for Noop<G> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
impl<G: Genotype> Noop<G> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<G: Genotype> StrategyReporter for Noop<G> {
    type Genotype = G;
}

/// A Duration reporter generic over Genotype.
#[derive(Clone)]
pub struct Duration<G: Genotype> {
    _phantom: PhantomData<G>,
}
impl<G: Genotype> Default for Duration<G> {
    fn default() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}
impl<G: Genotype> Duration<G> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<G: Genotype> StrategyReporter for Duration<G> {
    type Genotype = G;

    fn on_start<S: StrategyState<Self::Genotype>, C: StrategyConfig>(
        &mut self,
        _genotype: &Self::Genotype,
        state: &S,
        _config: &C,
    ) {
        println!("start - iteration: {}", state.current_iteration());
    }
    fn on_finish<S: StrategyState<Self::Genotype>, C: StrategyConfig>(
        &mut self,
        _genotype: &Self::Genotype,
        state: &S,
        _config: &C,
    ) {
        println!("finish - iteration: {}", state.current_iteration());
        STRATEGY_ACTIONS.iter().for_each(|action| {
            if let Some(duration) = state.durations().get(action) {
                println!("  {:?}: {:?}", action, duration,);
            }
        });
        println!("  Total: {:?}", &state.total_duration());
    }
}
