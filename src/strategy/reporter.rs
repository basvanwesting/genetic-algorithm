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
impl<G: Genotype> StrategyReporter for Simple<G> {
    type Genotype = G;

    fn on_init<S: StrategyState<Self::Genotype>, C: StrategyConfig>(
        &mut self,
        genotype: &Self::Genotype,
        state: &S,
        _config: &C,
    ) {
        println!("init - iteration: {}", state.current_iteration());
        genotype
            .seed_genes_list()
            .iter()
            .for_each(|genes| println!("init - seed_genes: {:?}", genes));
    }
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

    fn on_new_generation<S: StrategyState<Self::Genotype>, C: StrategyConfig>(
        &mut self,
        _genotype: &Self::Genotype,
        state: &S,
        config: &C,
    ) {
        if state.current_generation() % self.period == 0 {
            let width = state.population_as_ref().size().to_string().len();
            println!(
                "periodic - progress: {}, current_generation: {}, stale_generations: {}, best_generation: {}, current_scale_index: {:?}, fitness_score_cardinality: {:>width$}, current_population_size: {:>width$}, #extension_events: {}",
                config
                    .estimated_progress_perc(state.current_generation())
                    .map_or("-".to_string(), |v| format!("{:3.3}%", v)),
                state.current_generation(),
                state.stale_generations(),
                state.best_generation(),
                state.current_scale_index(),
                state.population_as_ref().fitness_score_cardinality(),
                state.population_as_ref().size(),
                self.number_of_extension_events,
            );
            self.number_of_mutate_events = 0;
            self.number_of_extension_events = 0;
        }
    }

    fn on_new_best_chromosome<S: StrategyState<Self::Genotype>, C: StrategyConfig>(
        &mut self,
        genotype: &Self::Genotype,
        state: &S,
        _config: &C,
    ) {
        println!(
            "new best - generation: {}, fitness_score: {:?}, genes: {:?}, scale_index: {:?}, population_size: {}",
            state.current_generation(),
            state.best_fitness_score(),
            if self.show_genes {
                Some(genotype.best_genes())
            } else {
                None
            },
            state.current_scale_index(),
            state.population_as_ref().size(),
        );
    }

    fn on_extension_event<S: StrategyState<Self::Genotype>, C: StrategyConfig>(
        &mut self,
        event: ExtensionEvent,
        _genotype: &Self::Genotype,
        state: &S,
        _config: &C,
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
            }
        }
    }

    fn on_mutate_event<S: StrategyState<Self::Genotype>, C: StrategyConfig>(
        &mut self,
        event: MutateEvent,
        _genotype: &Self::Genotype,
        _state: &S,
        _config: &C,
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
