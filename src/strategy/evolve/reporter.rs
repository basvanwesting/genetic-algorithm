use crate::crossover::CrossoverEvent;
use crate::extension::ExtensionEvent;
use crate::genotype::EvolveGenotype;
use crate::mutate::MutateEvent;
use crate::select::SelectEvent;
use crate::strategy::{StrategyConfig, StrategyReporter, StrategyState, STRATEGY_ACTIONS};
use std::fmt::Arguments;
use std::io::Write;
use std::marker::PhantomData;

/// A Simple Evolve Reporter generic over Genotype.
/// A report is triggered every period generations
///
/// Example output:
///
/// ```"not rust",ignore
/// enter - evolve, iteration: 8
/// new best - generation: 0, fitness_score: Some(-2403), scale_index: None, genes: None
/// new best - generation: 2, fitness_score: Some(-2204), scale_index: None, genes: None
/// new best - generation: 6, fitness_score: Some(-2007), scale_index: None, genes: None
/// new best - generation: 9, fitness_score: Some(-1607), scale_index: None, genes: None
/// new best - generation: 11, fitness_score: Some(-1589), scale_index: None, genes: None
/// new best - generation: 14, fitness_score: Some(-1400), scale_index: None, genes: None
/// new best - generation: 17, fitness_score: Some(-994), scale_index: None, genes: None
/// new best - generation: 25, fitness_score: Some(-576), scale_index: None, genes: None
/// new best - generation: 27, fitness_score: Some(-561), scale_index: None, genes: None
/// new best - generation: 37, fitness_score: Some(-559), scale_index: None, genes: None
/// new best - generation: 40, fitness_score: Some(-553), scale_index: None, genes: None
/// periodic - current_generation: 50, stale_generations: 9, best_generation: 40, scale_index: None, population_cardinality: Some(13), current_population_size: 1000 (517p/483o,700r), fitness_cache_hit_miss_ratio: None, #events(S/E/C/M): 0/0/0/0
/// new best - generation: 53, fitness_score: Some(-549), scale_index: None, genes: None
/// new best - generation: 91, fitness_score: Some(-548), scale_index: None, genes: None
/// new best - generation: 92, fitness_score: Some(-141), scale_index: None, genes: None
/// periodic - current_generation: 100, stale_generations: 7, best_generation: 92, scale_index: None, population_cardinality: Some(3), current_population_size: 1000 (517p/483o,700r), fitness_cache_hit_miss_ratio: None, #events(S/E/C/M): 0/4/0/0
/// new best - generation: 142, fitness_score: Some(-130), scale_index: None, genes: None
/// periodic - current_generation: 150, stale_generations: 7, best_generation: 142, scale_index: None, population_cardinality: Some(3), current_population_size: 1000 (517p/483o,700r), fitness_cache_hit_miss_ratio: None, #events(S/E/C/M): 0/5/0/0
/// periodic - current_generation: 200, stale_generations: 57, best_generation: 142, scale_index: None, population_cardinality: Some(702), current_population_size: 1000 (516p/484o,700r), fitness_cache_hit_miss_ratio: None, #events(S/E/C/M): 0/7/0/0
/// periodic - current_generation: 250, stale_generations: 107, best_generation: 142, scale_index: None, population_cardinality: Some(549), current_population_size: 1000 (515p/485o,700r), fitness_cache_hit_miss_ratio: None, #events(S/E/C/M): 0/7/0/0
/// periodic - current_generation: 300, stale_generations: 157, best_generation: 142, scale_index: None, population_cardinality: Some(347), current_population_size: 1000 (517p/483o,700r), fitness_cache_hit_miss_ratio: None, #events(S/E/C/M): 0/7/0/0
/// periodic - current_generation: 350, stale_generations: 207, best_generation: 142, scale_index: None, population_cardinality: Some(147), current_population_size: 1000 (516p/484o,700r), fitness_cache_hit_miss_ratio: None, #events(S/E/C/M): 0/7/0/0
/// exit - evolve, iteration: 8
///   SetupAndCleanup: 145.999µs
///   Extension: 4.771ms
///   Select: 17.371ms
///   Crossover: 11.509ms
///   Mutate: 3.090ms
///   Fitness: 138.344ms
///   UpdateBestChromosome: 1.416ms
///   Other: 3.359ms
///   Total: 180.007ms (77% fitness)
/// ```
///
#[derive(Clone)]
pub struct Simple<G: EvolveGenotype> {
    pub buffer: Option<Vec<u8>>,
    pub period: usize,
    pub show_genes: bool,
    pub show_equal_fitness: bool,
    pub show_select_event: bool,
    pub show_extension_event: bool,
    pub show_crossover_event: bool,
    pub show_mutate_event: bool,
    number_of_select_events: usize,
    number_of_extension_events: usize,
    number_of_crossover_events: usize,
    number_of_mutate_events: usize,
    _phantom: PhantomData<G>,
}
impl<G: EvolveGenotype> Default for Simple<G> {
    fn default() -> Self {
        Self {
            buffer: None,
            period: 1,
            show_genes: false,
            show_equal_fitness: false,
            show_select_event: false,
            show_extension_event: false,
            show_crossover_event: false,
            show_mutate_event: false,
            number_of_select_events: 0,
            number_of_extension_events: 0,
            number_of_crossover_events: 0,
            number_of_mutate_events: 0,
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
    #[allow(clippy::too_many_arguments)]
    pub fn new_with_flags(
        period: usize,
        buffered: bool,
        show_genes: bool,
        show_equal_fitness: bool,
        show_select_event: bool,
        show_extension_event: bool,
        show_crossover_event: bool,
        show_mutate_event: bool,
    ) -> Self {
        Self {
            buffer: if buffered { Some(Vec::new()) } else { None },
            period,
            show_genes,
            show_equal_fitness,
            show_select_event,
            show_extension_event,
            show_crossover_event,
            show_mutate_event,
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

    fn on_selection_complete<S: StrategyState<Self::Genotype>, C: StrategyConfig>(
        &mut self,
        genotype: &Self::Genotype,
        state: &S,
        config: &C,
    ) {
        if state.current_generation() % self.period == 0 {
            let number_of_select_events = self.number_of_select_events;
            let number_of_extension_events = self.number_of_extension_events;
            let number_of_crossover_events = self.number_of_crossover_events;
            let number_of_mutate_events = self.number_of_mutate_events;

            let fitness_cache_hit_miss_ratio = config.fitness_cache().map(|c| c.hit_miss_stats().2);
            let (parents_size, offspring_size) =
                state.population_as_ref().parents_and_offspring_size();

            self.writeln(format_args!(
                "periodic - current_generation: {}, stale_generations: {}, best_generation: {}, scale_index: {:?}, population_cardinality: {:?}, current_population_size: {} ({}p/{}o,{}r), fitness_cache_hit_miss_ratio: {:.2?}, #events(S/E/C/M): {}/{}/{}/{}",
                state.current_generation(),
                state.stale_generations(),
                state.best_generation(),
                genotype.current_scale_index(),
                state.population_cardinality(),
                state.population_as_ref().size(),
                parents_size,
                offspring_size,
                state.population_as_ref().recycled_size(),
                fitness_cache_hit_miss_ratio,
                number_of_select_events,
                number_of_extension_events,
                number_of_crossover_events,
                number_of_mutate_events,
            ));

            // reset event counters
            self.number_of_select_events = 0;
            self.number_of_extension_events = 0;
            self.number_of_crossover_events = 0;
            self.number_of_mutate_events = 0;
        }
    }

    fn on_new_best_chromosome<S: StrategyState<Self::Genotype>, C: StrategyConfig>(
        &mut self,
        genotype: &Self::Genotype,
        state: &S,
        _config: &C,
    ) {
        self.writeln(format_args!(
            "new best - generation: {}, fitness_score: {:?}, scale_index: {:?}, genes: {:?}",
            state.current_generation(),
            state.best_fitness_score(),
            genotype.current_scale_index(),
            if self.show_genes {
                state.best_genes()
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
            self.writeln(format_args!(
                "equal best - generation: {}, fitness_score: {:?}, scale_index: {:?}, genes: {:?}",
                state.current_generation(),
                state.best_fitness_score(),
                genotype.current_scale_index(),
                if self.show_genes {
                    state.best_genes()
                } else {
                    None
                },
            ));
        }
    }

    fn on_select_event<S: StrategyState<Self::Genotype>, C: StrategyConfig>(
        &mut self,
        event: SelectEvent,
        _genotype: &Self::Genotype,
        state: &S,
        _config: &C,
    ) {
        self.number_of_select_events += 1;
        if self.show_select_event {
            self.writeln(format_args!(
                "select event - generation {} - {}",
                state.current_generation(),
                event.0,
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
            self.writeln(format_args!(
                "extension event - generation {} - {}",
                state.current_generation(),
                event.0,
            ));
        }
    }

    fn on_crossover_event<S: StrategyState<Self::Genotype>, C: StrategyConfig>(
        &mut self,
        event: CrossoverEvent,
        _genotype: &Self::Genotype,
        state: &S,
        _config: &C,
    ) {
        self.number_of_crossover_events += 1;
        if self.show_crossover_event {
            self.writeln(format_args!(
                "crossover event - generation {} - {}",
                state.current_generation(),
                event.0,
            ));
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
            self.writeln(format_args!(
                "mutate event - generation {} - {}",
                state.current_generation(),
                event.0,
            ));
        }
    }
}
