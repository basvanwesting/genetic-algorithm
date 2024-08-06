use super::EvolveState;
use crate::genotype::Genotype;
use crate::strategy::StrategyState;
use std::marker::PhantomData;

/// Reporter with event hooks in the Evolve process.
///
/// # Example:
/// You are encouraged to roll your own.
/// ```rust
/// use genetic_algorithm::strategy::evolve::prelude::*;
///
/// #[derive(Clone)]
/// pub struct CustomReporter(usize);
/// impl EvolveReporter for CustomReporter {
///     type Genotype = BinaryGenotype;
///
///     fn on_new_best_chromosome(&mut self, state: &EvolveState<Self::Genotype>) {
///         println!(
///             "current_generation: {}, best_fitness_score: {:?}, genes: {:?}",
///             state.current_generation(),
///             state.best_fitness_score(),
///             state
///               .best_chromosome_as_ref()
///               .map_or(vec![], |c| c.genes.clone()),
///         );
///     }
/// }
/// ```
pub trait Reporter: Clone + Send {
    type Genotype: Genotype;

    fn on_start(&mut self, _state: &EvolveState<Self::Genotype>) {}
    fn on_finish(&mut self, _state: &EvolveState<Self::Genotype>) {}
    fn on_new_generation(&mut self, _state: &EvolveState<Self::Genotype>) {}
    fn on_new_best_chromosome(&mut self, _state: &EvolveState<Self::Genotype>) {}
}

/// The noop reporter, silences reporting
#[derive(Clone)]
pub struct Noop<G: Genotype>(pub PhantomData<G>);
impl<G: Genotype> Default for Noop<G> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
impl<G: Genotype + Sync + Clone + Send> Reporter for Noop<G> {
    type Genotype = G;
}

/// A Simple reporter generic over Genotype.
/// A report is triggered every period generations
#[derive(Clone)]
pub struct Simple<G: Genotype> {
    pub period: usize,
    _phantom: PhantomData<G>,
}
impl<G: Genotype> Simple<G> {
    pub fn new(period: usize) -> Self {
        Self {
            period,
            _phantom: PhantomData,
        }
    }
}
impl<G: Genotype + Sync + Clone + Send> Reporter for Simple<G> {
    type Genotype = G;

    fn on_new_generation(&mut self, state: &EvolveState<Self::Genotype>) {
        if state.current_generation() % self.period == 0 {
            println!(
                "current_generation: {}, best_generation: {}, best_fitness_score: {:?}, current_population_size: {}",
                state.current_generation(),
                state.best_generation(),
                state.best_fitness_score(),
                state.population.size(),
            );
        }
    }

    fn on_new_best_chromosome(&mut self, state: &EvolveState<Self::Genotype>) {
        println!(
            "current_generation: {}, best_generation: now, best_fitness_score: {:?}, current_population_size: {}",
            state.current_generation(),
            state.best_fitness_score(),
            state.population.size(),
        );
    }
}

/// A log-level based reporter for debug and trace, runs on each generation
#[derive(Clone)]
pub struct Log<G: Genotype>(pub PhantomData<G>);
impl<G: Genotype> Default for Log<G> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
impl<G: Genotype + Sync + Clone + Send> Reporter for Log<G> {
    type Genotype = G;

    fn on_new_generation(&mut self, state: &EvolveState<Self::Genotype>) {
        log::debug!(
            "generation (current/best/mean-age): {}/{}/{:2.2}, fitness score (best/count/median/mean/stddev/uniformity/best-prevalence): {:?} / {} / {:?} / {:.0} / {:.0} / {:4.4} / {}",
            state.current_generation(),
            state.best_generation(),
            state.population.age_mean(),
            state.best_fitness_score(),
            state.population.fitness_score_count(),
            state.population.fitness_score_median(),
            state.population.fitness_score_mean(),
            state.population.fitness_score_stddev(),
            state.population.fitness_score_uniformity(),
            state.population.fitness_score_prevalence(state.best_fitness_score()),
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
