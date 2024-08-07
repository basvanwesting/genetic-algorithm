use super::{EvolveConfig, EvolveState};
use crate::extension::ExtensionEvent;
use crate::genotype::Genotype;
use crate::mutate::MutateEvent;
use crate::strategy::StrategyState;
use std::marker::PhantomData;

/// Reporter with event hooks in the Evolve process.
///
/// # Example:
/// You are encouraged to take a look at the [EvolveReporterSimple](Simple) implementation, and
/// then roll your own like below:
/// ```rust
/// use genetic_algorithm::strategy::evolve::prelude::*;
///
/// #[derive(Clone)]
/// pub struct CustomReporter { pub period: usize }
/// impl EvolveReporter for CustomReporter {
///     type Genotype = BinaryGenotype;
///
///     fn on_new_generation(&mut self, state: &EvolveState<Self::Genotype>, _config: &EvolveConfig) {
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
///     fn on_new_best_chromosome(&mut self, state: &EvolveState<Self::Genotype>, _config: &EvolveConfig) {
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
pub trait Reporter: Clone + Send + Sync {
    type Genotype: Genotype;

    fn on_start(
        &mut self,
        _genotype: &Self::Genotype,
        _state: &EvolveState<Self::Genotype>,
        _config: &EvolveConfig,
    ) {
    }
    fn on_finish(&mut self, _state: &EvolveState<Self::Genotype>, _config: &EvolveConfig) {}
    fn on_new_generation(&mut self, _state: &EvolveState<Self::Genotype>, _config: &EvolveConfig) {}
    fn on_new_best_chromosome(
        &mut self,
        _state: &EvolveState<Self::Genotype>,
        _config: &EvolveConfig,
    ) {
    }
    fn on_extension_event(
        &mut self,
        _event: ExtensionEvent,
        _state: &EvolveState<Self::Genotype>,
        _config: &EvolveConfig,
    ) {
    }
    fn on_mutate_event(
        &mut self,
        _event: MutateEvent,
        _state: &EvolveState<Self::Genotype>,
        _config: &EvolveConfig,
    ) {
    }
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
impl<G: Genotype + Clone + Send + Sync> Reporter for Noop<G> {
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
    number_of_mutate_events: usize,
    number_of_extension_events: usize,
    _phantom: PhantomData<G>,
}
impl<G: Genotype> Default for Simple<G> {
    fn default() -> Self {
        Self {
            period: 1,
            show_genes: false,
            show_mutate_event: false,
            show_extension_event: false,
            number_of_mutate_events: 0,
            number_of_extension_events: 0,
            _phantom: PhantomData,
        }
    }
}
impl<G: Genotype> Simple<G> {
    pub fn new(period: usize) -> Self {
        Self {
            period,
            ..Default::default()
        }
    }
    pub fn new_with_flags(
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
}
impl<G: Genotype + Clone + Send + Sync> Reporter for Simple<G> {
    type Genotype = G;

    fn on_start(
        &mut self,
        genotype: &Self::Genotype,
        state: &EvolveState<Self::Genotype>,
        _config: &EvolveConfig,
    ) {
        println!("start - iteration: {}", state.current_iteration());
        genotype
            .seed_genes_list()
            .iter()
            .for_each(|genes| println!("start - seed_genes: {:?}", genes));
    }

    fn on_finish(&mut self, state: &EvolveState<Self::Genotype>, _config: &EvolveConfig) {
        println!("finish - iteration: {}", state.current_iteration());
    }

    fn on_new_generation(&mut self, state: &EvolveState<Self::Genotype>, config: &EvolveConfig) {
        if state.current_generation() % self.period == 0 {
            let width = config.target_population_size.to_string().len();
            println!(
                "periodic - current_generation: {}, best_generation: {}, fitness_score_cardinality: {:>width$}, current_population_size: {:>width$}, #extension_events: {}",
                state.current_generation(),
                state.best_generation(),
                state.population.fitness_score_cardinality(),
                state.population.size(),
                self.number_of_extension_events,
            );
            self.number_of_mutate_events = 0;
            self.number_of_extension_events = 0;
        }
    }

    fn on_new_best_chromosome(
        &mut self,
        state: &EvolveState<Self::Genotype>,
        _config: &EvolveConfig,
    ) {
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

    fn on_extension_event(
        &mut self,
        event: ExtensionEvent,
        state: &EvolveState<Self::Genotype>,
        _config: &EvolveConfig,
    ) {
        self.number_of_extension_events += 1;
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

    fn on_mutate_event(
        &mut self,
        event: MutateEvent,
        _state: &EvolveState<Self::Genotype>,
        _config: &EvolveConfig,
    ) {
        self.number_of_mutate_events += 1;
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
impl<G: Genotype + Clone + Send + Sync> Reporter for Log<G> {
    type Genotype = G;

    fn on_new_generation(&mut self, state: &EvolveState<Self::Genotype>, _config: &EvolveConfig) {
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
