use super::EvolveState;
use crate::extension::ExtensionEvent;
use crate::genotype::Genotype;
use crate::mutate::MutateEvent;
use crate::strategy::StrategyState;
use std::marker::PhantomData;

/// Reporter with event hooks in the Evolve process.
///
/// # Example:
/// You are encouraged to roll your own, like the [EvolveReporterSimple](Simple) implementation below
/// ```rust
/// use genetic_algorithm::strategy::evolve::prelude::*;
///
/// #[derive(Clone)]
/// pub struct CustomReporter { pub period: usize }
/// impl EvolveReporter for CustomReporter {
///     type Genotype = BinaryGenotype;
///
///     fn on_new_generation(&mut self, state: &EvolveState<Self::Genotype>) {
///         if state.current_generation() % self.period == 0 {
///             println!(
///                 "periodic - current_generation: {}, best_generation: {}, fitness_score_cardinality: {}, current_population_size: {}",
///                 state.current_generation(),
///                 state.best_generation(),
///                 state.population.fitness_score_cardinality(),
///                 state.population.size(),
///             );
///         }
///     }
///
///     fn on_new_best_chromosome(&mut self, state: &EvolveState<Self::Genotype>) {
///         println!(
///             "new best - generation: {}, fitness_score: {:?}, genes: {:?}, population_size: {}",
///             state.current_generation(),
///             state.best_fitness_score(),
///             state.best_chromosome_as_ref().map(|c| &c.genes),
///             state.population.size(),
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
    fn on_extension_event(&mut self, _state: &EvolveState<Self::Genotype>, _event: ExtensionEvent) {
    }
    fn on_mutate_event(&mut self, _event: MutateEvent) {}
}

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
impl<G: Genotype + Sync + Clone + Send> Reporter for Noop<G> {
    type Genotype = G;
}

/// A Simple reporter generic over Genotype.
/// A report is triggered every period generations
#[derive(Clone)]
pub struct Simple<G: Genotype> {
    pub period: usize,
    pub show_genes: bool,
    pub show_mutate_event: bool,
    pub show_extension_event: bool,
    _phantom: PhantomData<G>,
}
impl<G: Genotype> Default for Simple<G> {
    fn default() -> Self {
        Self {
            period: 1,
            show_genes: false,
            show_mutate_event: false,
            show_extension_event: false,
            _phantom: PhantomData,
        }
    }
}
impl<G: Genotype> Simple<G> {
    pub fn new(
        period: usize,
        show_genes: bool,
        show_mutate_event: bool,
        show_extension_event: bool,
    ) -> Self {
        Self {
            period,
            show_genes,
            show_mutate_event,
            show_extension_event,
            ..Default::default()
        }
    }
    // TODO: what is a good name for this?
    pub fn new_short(period: usize) -> Self {
        Self {
            period,
            ..Default::default()
        }
    }
}
impl<G: Genotype + Sync + Clone + Send> Reporter for Simple<G> {
    type Genotype = G;

    fn on_new_generation(&mut self, state: &EvolveState<Self::Genotype>) {
        if state.current_generation() % self.period == 0 {
            println!(
                "periodic - current_generation: {}, best_generation: {}, fitness_score_cardinality: {}, current_population_size: {}",
                state.current_generation(),
                state.best_generation(),
                state.population.fitness_score_cardinality(),
                state.population.size(),
            );
        }
    }

    fn on_new_best_chromosome(&mut self, state: &EvolveState<Self::Genotype>) {
        println!(
            "new best - generation: {}, fitness_score: {:?}, genes: {:?}, population_size: {}",
            state.current_generation(),
            state.best_fitness_score(),
            if self.show_genes {
                state.best_chromosome_as_ref().map(|c| &c.genes)
            } else {
                None
            },
            state.population.size(),
        );
    }

    fn on_extension_event(&mut self, state: &EvolveState<Self::Genotype>, event: ExtensionEvent) {
        if self.show_extension_event {
            match event {
                ExtensionEvent::MassDegeneration(message) => {
                    println!(
                        "extension event - mass degeneration - current generation {} - {}",
                        state.current_generation(),
                        message
                    )
                }
                ExtensionEvent::MassExtinction(message) => {
                    println!(
                        "extension event - mass extinction - current generation {} - {}",
                        state.current_generation(),
                        message
                    )
                }
                ExtensionEvent::MassGenesis(message) => {
                    println!(
                        "extension event - mass genesis - current generation {} - {}",
                        state.current_generation(),
                        message
                    )
                }
                ExtensionEvent::MassInvasion(message) => {
                    println!(
                        "extension event - mass invasion - current generation {} - {}",
                        state.current_generation(),
                        message
                    )
                }
            }
        }
    }

    fn on_mutate_event(&mut self, event: MutateEvent) {
        if self.show_mutate_event {
            match event {
                MutateEvent::ChangeMutationProbability(message) => {
                    println!("mutate event - change mutation probability - {}", message)
                }
            }
        }
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
impl<G: Genotype> Log<G> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<G: Genotype + Sync + Clone + Send> Reporter for Log<G> {
    type Genotype = G;

    fn on_new_generation(&mut self, state: &EvolveState<Self::Genotype>) {
        log::debug!(
            "generation (current/best/mean-age): {}/{}/{:2.2}, fitness score (best/count/median/mean/stddev/cardinality): {:?} / {} / {:?} / {:.0} / {:.0} / {}",
            state.current_generation(),
            state.best_generation(),
            state.population.age_mean(),
            state.best_fitness_score(),
            state.population.fitness_score_count(),
            state.population.fitness_score_median(),
            state.population.fitness_score_mean(),
            state.population.fitness_score_stddev(),
            state.population.fitness_score_cardinality(),
        );

        log::trace!(
            "best - fitness score: {:?}, genes: {:?}",
            state.best_fitness_score(),
            state.best_chromosome_as_ref().map(|c| &c.genes)
        );
    }
}
