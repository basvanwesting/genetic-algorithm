use super::{EvolveReporter, EvolveState};
use crate::genotype::Genotype;
use crate::strategy::StrategyState;
use std::marker::PhantomData;

#[derive(Clone)]
pub struct Noop<G: Genotype>(pub PhantomData<G>);
impl<G: Genotype> Default for Noop<G> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
impl<G: Genotype + Sync + Clone + Send> EvolveReporter for Noop<G> {
    type Genotype = G;
}

#[derive(Clone)]
pub struct Simple<G: Genotype> {
    pub frequency: usize,
    _phantom: PhantomData<G>,
}
impl<G: Genotype> Simple<G> {
    pub fn new(frequency: usize) -> Self {
        Self {
            frequency,
            _phantom: PhantomData,
        }
    }
}
impl<G: Genotype + Sync + Clone + Send> EvolveReporter for Simple<G> {
    type Genotype = G;

    fn on_new_generation(&mut self, state: &EvolveState<Self::Genotype>) {
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

#[derive(Clone)]
pub struct Log<G: Genotype> {
    pub frequency: usize,
    _phantom: PhantomData<G>,
}
impl<G: Genotype> Log<G> {
    pub fn new(frequency: usize) -> Self {
        Self {
            frequency,
            _phantom: PhantomData,
        }
    }
}
impl<G: Genotype + Sync + Clone + Send> EvolveReporter for Log<G> {
    type Genotype = G;

    fn on_new_generation(&mut self, state: &EvolveState<Self::Genotype>) {
        log::debug!(
            "current_generation: {}, best_generation: {}, best_fitness_score: {:?}",
            state.current_generation(),
            state.best_generation(),
            state.best_fitness_score(),
            // "generation (current/best/mean-age): {}/{}/{:2.2}, fitness score (best/count/median/mean/stddev/uniformity/best-prevalence): {:?} / {} / {:?} / {:.0} / {:.0} / {:4.4} / {}, mutation: {:?}",
            // state.current_generation(),
            // state.best_generation(),
            // population.age_mean(),
            // state.best_fitness_score(),
            // population.fitness_score_count(),
            // population.fitness_score_median(),
            // population.fitness_score_mean(),
            // population.fitness_score_stddev(),
            // population.fitness_score_uniformity(),
            // population.fitness_score_prevalence(self.best_fitness_score()),
        );
        log::trace!(
            "best - fitness score: {:?}, genes: {:?}",
            state.best_fitness_score(),
            state
                .best_chromosome_as_ref()
                .map_or(vec![], |c| c.genes.clone()),
        );
    }
}
