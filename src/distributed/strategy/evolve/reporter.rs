use crate::distributed::extension::ExtensionEvent;
use crate::distributed::genotype::EvolveGenotype;
use crate::distributed::mutate::MutateEvent;
use crate::distributed::strategy::{
    StrategyConfig, StrategyReporter, StrategyState, STRATEGY_ACTIONS,
};
use std::fmt::Arguments;
use std::io::Write;
use std::marker::PhantomData;

/// A Simple Evolve Reporter generic over Genotype.
/// A report is triggered every period generations
///
/// Example output:
///
/// ```"not rust",ignore
/// enter - evolve, iteration: 0
/// new best - generation: 0,  fitness_score: Some(-3112), scale_index: None, genes: None
/// new best - generation: 6,  fitness_score: Some(-2012), scale_index: None, genes: None
/// new best - generation: 38, fitness_score: Some(-1440), scale_index: None, genes: None
/// new best - generation: 47, fitness_score: Some(-1439), scale_index: None, genes: None
/// periodic - current_generation: 50, stale_generations: 2, best_generation: 47, scale_index: None, population_cardinality: Some(6), current_population_size: 800, #extension_events: 0
/// new best - generation: 51, fitness_score: Some(-1437), scale_index: None, genes: None
/// new best - generation: 60, fitness_score: Some(-1435), scale_index: None, genes: None
/// new best - generation: 77, fitness_score: Some(-1120), scale_index: None, genes: None
/// new best - generation: 99, fitness_score: Some(-639),  scale_index: None, genes: None
/// periodic - current_generation: 100, stale_generations: 0, best_generation: 99, scale_index: None, population_cardinality: Some(11), current_population_size: 800, #extension_events: 1
/// new best - generation: 146, fitness_score: Some(-125), scale_index: None, genes: None
/// periodic - current_generation: 150, stale_generations: 3,   best_generation: 146, scale_index: None, population_cardinality: Some(59),  current_population_size: 800, #extension_events: 1
/// periodic - current_generation: 200, stale_generations: 53,  best_generation: 146, scale_index: None, population_cardinality: Some(592), current_population_size: 800, #extension_events: 3
/// periodic - current_generation: 250, stale_generations: 103, best_generation: 146, scale_index: None, population_cardinality: Some(4),   current_population_size: 800, #extension_events: 2
/// periodic - current_generation: 300, stale_generations: 153, best_generation: 146, scale_index: None, population_cardinality: Some(335), current_population_size: 800, #extension_events: 3
/// periodic - current_generation: 350, stale_generations: 203, best_generation: 146, scale_index: None, population_cardinality: Some(1),   current_population_size: 800, #extension_events: 2
/// new best - generation: 379, fitness_score: Some(66), scale_index: None, genes: None
/// periodic - current_generation: 400, stale_generations: 20,  best_generation: 379, scale_index: None, population_cardinality: Some(570), current_population_size: 800, #extension_events: 3
/// periodic - current_generation: 450, stale_generations: 70,  best_generation: 379, scale_index: None, population_cardinality: Some(5),   current_population_size: 800, #extension_events: 2
/// periodic - current_generation: 500, stale_generations: 120, best_generation: 379, scale_index: None, population_cardinality: Some(368), current_population_size: 800, #extension_events: 3
/// periodic - current_generation: 550, stale_generations: 170, best_generation: 379, scale_index: None, population_cardinality: Some(692), current_population_size: 800, #extension_events: 3
/// periodic - current_generation: 600, stale_generations: 220, best_generation: 379, scale_index: None, population_cardinality: Some(75),  current_population_size: 800, #extension_events: 2
/// exit - evolve, iteration: 0
///   SetupAndCleanup: 141.833µs
///   Extension: 2.139ms
///   Select: 11.807ms
///   Crossover: 16.921ms
///   Mutate: 4.337ms
///   Fitness: 231.512ms
///   UpdateBestChromosome: 1.497ms
///   Other: 6.050ms
///   Total: 274.404ms (84% fitness)
/// ```
///
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
    fn writeln(&mut self, args: Arguments<'_>) {
        if let Some(buffer) = self.buffer.as_mut() {
            buffer.write_fmt(args).unwrap_or(());
            writeln!(buffer).unwrap_or(())
        } else {
            std::io::stdout().write_fmt(args).unwrap_or(());
            println!()
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
    fn on_enter<S: StrategyState<Self::Genotype>, C: StrategyConfig>(
        &mut self,
        genotype: &Self::Genotype,
        state: &S,
        config: &C,
    ) {
        let number_of_seed_genes = genotype.seed_genes_list().len();
        if number_of_seed_genes > 0 {
            self.writeln(format_args!(
                "enter - {}, iteration: {}, number of seed genes: {}",
                config.variant(),
                state.current_iteration(),
                number_of_seed_genes
            ));
        } else {
            self.writeln(format_args!(
                "enter - {}, iteration: {}",
                config.variant(),
                state.current_iteration()
            ));
        }
    }
    fn on_exit<S: StrategyState<Self::Genotype>, C: StrategyConfig>(
        &mut self,
        _genotype: &Self::Genotype,
        state: &S,
        config: &C,
    ) {
        let fitness_report = if let Some((hits, misses, ratio)) =
            config.fitness_cache().map(|c| c.hit_miss_stats())
        {
            format!(
                "({:.0}% fitness, cache hits/misses/ratio: {}/{}/{:.2})",
                state.fitness_duration_rate() * 100.0,
                hits,
                misses,
                ratio
            )
        } else {
            format!("({:.0}% fitness)", state.fitness_duration_rate() * 100.0)
        };

        self.writeln(format_args!(
            "exit - {}, iteration: {}",
            config.variant(),
            state.current_iteration()
        ));
        STRATEGY_ACTIONS.iter().for_each(|action| {
            if let Some(duration) = state.durations().get(action) {
                self.writeln(format_args!("  {:?}: {:.3?}", action, duration));
            }
        });
        self.writeln(format_args!(
            "  Total: {:.3?} {}",
            &state.total_duration(),
            fitness_report
        ));
    }

    /// Is triggered after selection
    fn on_new_generation<S: StrategyState<Self::Genotype>, C: StrategyConfig>(
        &mut self,
        _genotype: &Self::Genotype,
        state: &S,
        config: &C,
    ) {
        if state.current_generation() % self.period == 0 {
            let number_of_extension_events = self.number_of_extension_events;
            let fitness_cache_hit_miss_ratio = config.fitness_cache().map(|c| c.hit_miss_stats().2);
            let (parents_size, offspring_size) =
                state.population_as_ref().parents_and_offspring_size();

            self.writeln(format_args!(
                "periodic - current_generation: {}, stale_generations: {}, best_generation: {}, scale_index: {:?}, population_cardinality: {:?}, current_population_size: {} ({}p/{}o), fitness_cache_hit_miss_ratio: {:.2?}, #extension_events: {}",
                state.current_generation(),
                state.stale_generations(),
                state.best_generation(),
                state.current_scale_index(),
                state.population_cardinality(),
                state.population_as_ref().size(),
                parents_size,
                offspring_size,
                fitness_cache_hit_miss_ratio,
                number_of_extension_events,
            ));
            self.number_of_mutate_events = 0;
            self.number_of_extension_events = 0;
        }
    }

    fn on_new_best_chromosome<S: StrategyState<Self::Genotype>, C: StrategyConfig>(
        &mut self,
        _genotype: &Self::Genotype,
        state: &S,
        _config: &C,
    ) {
        self.writeln(format_args!(
            "new best - generation: {}, fitness_score: {:?}, scale_index: {:?}, genes: {:?}",
            state.current_generation(),
            state.best_fitness_score(),
            state.current_scale_index(),
            if self.show_genes {
                state.best_genes()
            } else {
                None
            },
        ));
    }

    fn on_new_best_chromosome_equal_fitness<S: StrategyState<Self::Genotype>, C: StrategyConfig>(
        &mut self,
        _genotype: &Self::Genotype,
        state: &S,
        _config: &C,
    ) {
        if self.show_equal_fitness {
            self.writeln(format_args!(
                "equal best - generation: {}, fitness_score: {:?}, scale_index: {:?}, genes: {:?}",
                state.current_generation(),
                state.best_fitness_score(),
                state.current_scale_index(),
                if self.show_genes {
                    state.best_genes()
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
                ExtensionEvent::MassDeduplication(message) => self.writeln(format_args!(
                    "extension event - mass deduplication - generation {} - {}",
                    state.current_generation(),
                    message
                )),
                ExtensionEvent::MassDegeneration(message) => self.writeln(format_args!(
                    "extension event - mass degeneration - generation {} - {}",
                    state.current_generation(),
                    message
                )),
                ExtensionEvent::MassExtinction(message) => self.writeln(format_args!(
                    "extension event - mass extinction - generation {} - {}",
                    state.current_generation(),
                    message
                )),
                ExtensionEvent::MassGenesis(message) => self.writeln(format_args!(
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
                MutateEvent::ChangeMutationProbability(message) => self.writeln(format_args!(
                    "mutate event - change mutation probability - generation {} - {}",
                    state.current_generation(),
                    message
                )),
            }
        }
    }
}
