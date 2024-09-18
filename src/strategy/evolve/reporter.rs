use crate::extension::ExtensionEvent;
use crate::genotype::EvolveGenotype;
use crate::mutate::MutateEvent;
use crate::strategy::{StrategyConfig, StrategyReporter, StrategyState, STRATEGY_ACTIONS};
use std::io::Write;
use std::marker::PhantomData;

/// A Simple Evolve reporter generic over Genotype.
/// A report is triggered every period generations
#[derive(Clone)]
pub struct Simple<G: EvolveGenotype> {
    pub buffer: Option<Vec<u8>>,
    pub period: usize,
    pub show_genes: bool,
    pub show_equal_fitness: bool,
    pub show_mutate_event: bool,
    pub show_extension_event: bool,
    number_of_mutate_events: usize,
    number_of_extension_events: usize,
    _phantom: PhantomData<G>,
}
impl<G: EvolveGenotype> Default for Simple<G> {
    fn default() -> Self {
        Self {
            buffer: None,
            period: 1,
            show_genes: false,
            show_equal_fitness: false,
            show_mutate_event: false,
            show_extension_event: false,
            number_of_mutate_events: 0,
            number_of_extension_events: 0,
            _phantom: PhantomData,
        }
    }
}
impl<G: EvolveGenotype> Simple<G> {
    pub fn new(period: usize) -> Self {
        Self {
            period,
            ..Default::default()
        }
    }
    pub fn new_with_buffer(period: usize) -> Self {
        Self {
            buffer: Some(Vec::new()),
            period,
            ..Default::default()
        }
    }
    pub fn new_with_flags(
        period: usize,
        buffered: bool,
        show_genes: bool,
        show_equal_fitness: bool,
        show_mutate_event: bool,
        show_extension_event: bool,
    ) -> Self {
        Self {
            buffer: if buffered { Some(Vec::new()) } else { None },
            period,
            show_genes,
            show_equal_fitness,
            show_mutate_event,
            show_extension_event,
            ..Default::default()
        }
    }
    fn write(&mut self, message: String) {
        if let Some(buffer) = self.buffer.as_mut() {
            buffer.write_all(message.as_bytes()).unwrap_or(());
            writeln!(buffer).unwrap_or(());
        } else {
            println!("{}", message);
        }
    }
}
impl<G: EvolveGenotype> StrategyReporter for Simple<G> {
    type Genotype = G;

    fn flush(&mut self, output: &mut Vec<u8>) {
        if let Some(buffer) = self.buffer.as_mut() {
            output.append(buffer);
        }
    }
    fn on_init<S: StrategyState<Self::Genotype>, C: StrategyConfig>(
        &mut self,
        genotype: &Self::Genotype,
        state: &S,
        _config: &C,
    ) {
        let number_of_seed_genes = genotype.seed_genes_list().len();
        if number_of_seed_genes > 0 {
            self.write(format!(
                "init - iteration: {}, number of seed genes: {}",
                state.current_iteration(),
                number_of_seed_genes
            ));
        } else {
            self.write(format!("init - iteration: {}", state.current_iteration()));
        }
    }
    fn on_start<S: StrategyState<Self::Genotype>, C: StrategyConfig>(
        &mut self,
        _genotype: &Self::Genotype,
        state: &S,
        _config: &C,
    ) {
        self.write(format!("start - iteration: {}", state.current_iteration()));
    }

    fn on_finish<S: StrategyState<Self::Genotype>, C: StrategyConfig>(
        &mut self,
        _genotype: &Self::Genotype,
        state: &S,
        _config: &C,
    ) {
        self.write(format!("finish - iteration: {}", state.current_iteration()));
        STRATEGY_ACTIONS.iter().for_each(|action| {
            if let Some(duration) = state.durations().get(action) {
                self.write(format!("  {:?}: {:?}", action, duration,));
            }
        });
        self.write(format!("  Total: {:?}", &state.total_duration()));
    }

    /// Is triggered after selection
    fn on_new_generation<S: StrategyState<Self::Genotype>, C: StrategyConfig>(
        &mut self,
        _genotype: &Self::Genotype,
        state: &S,
        _config: &C,
    ) {
        if state.current_generation() % self.period == 0 {
            self.write(format!(
                "periodic - current_generation: {}, stale_generations: {}, best_generation: {}, scale_index: {:?}, fitness_score_cardinality: {}, selected_population_size: {}, #extension_events: {}",
                state.current_generation(),
                state.stale_generations(),
                state.best_generation(),
                state.current_scale_index(),
                state.population_as_ref().fitness_score_cardinality(),
                state.population_as_ref().size(),
                self.number_of_extension_events,
            ));
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
        self.write(format!(
            "new best - generation: {}, fitness_score: {:?}, scale_index: {:?}, genes: {:?}",
            state.current_generation(),
            state.best_fitness_score(),
            state.current_scale_index(),
            if self.show_genes {
                Some(genotype.best_genes())
            } else {
                None
            },
        ));
    }

    fn on_new_best_chromosome_equal_fitness<S: StrategyState<Self::Genotype>, C: StrategyConfig>(
        &mut self,
        genotype: &Self::Genotype,
        state: &S,
        _config: &C,
    ) {
        if self.show_equal_fitness {
            self.write(format!(
                "equal best - generation: {}, fitness_score: {:?}, scale_index: {:?}, genes: {:?}",
                state.current_generation(),
                state.best_fitness_score(),
                state.current_scale_index(),
                if self.show_genes {
                    Some(genotype.best_genes())
                } else {
                    None
                },
            ));
        }
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
                ExtensionEvent::MassDegeneration(message) => self.write(format!(
                    "extension event - mass degeneration - generation {} - {}",
                    state.current_generation(),
                    message
                )),
                ExtensionEvent::MassExtinction(message) => self.write(format!(
                    "extension event - mass extinction - generation {} - {}",
                    state.current_generation(),
                    message
                )),
                ExtensionEvent::MassGenesis(message) => self.write(format!(
                    "extension event - mass genesis - generation {} - {}",
                    state.current_generation(),
                    message
                )),
            }
        }
    }

    fn on_mutate_event<S: StrategyState<Self::Genotype>, C: StrategyConfig>(
        &mut self,
        event: MutateEvent,
        _genotype: &Self::Genotype,
        state: &S,
        _config: &C,
    ) {
        self.number_of_mutate_events += 1;
        if self.show_mutate_event {
            match event {
                MutateEvent::ChangeMutationProbability(message) => self.write(format!(
                    "mutate event - change mutation probability - generation {} - {}",
                    state.current_generation(),
                    message
                )),
            }
        }
    }
}
