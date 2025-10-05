use super::HillClimbVariant;
use crate::genotype::HillClimbGenotype;
use crate::strategy::{
    StrategyConfig, StrategyReporter, StrategyState, StrategyVariant, STRATEGY_ACTIONS,
};
use std::fmt::Arguments;
use std::io::Write;
use std::marker::PhantomData;

/// A Simple HillClimb reporter generic over Genotype.
/// A report is triggered every period generations
#[derive(Clone)]
pub struct Simple<G: HillClimbGenotype> {
    pub buffer: Option<Vec<u8>>,
    pub period: usize,
    pub show_genes: bool,
    pub show_equal_fitness: bool,
    _phantom: PhantomData<G>,
}
impl<G: HillClimbGenotype> Default for Simple<G> {
    fn default() -> Self {
        Self {
            buffer: None,
            period: 1,
            show_genes: false,
            show_equal_fitness: false,
            _phantom: PhantomData,
        }
    }
}
impl<G: HillClimbGenotype> Simple<G> {
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
    ) -> Self {
        Self {
            buffer: if buffered { Some(Vec::new()) } else { None },
            period,
            show_genes,
            show_equal_fitness,
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
impl<G: HillClimbGenotype> StrategyReporter for Simple<G> {
    type Genotype = G;

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
        if let StrategyVariant::HillClimb(HillClimbVariant::SteepestAscent) = config.variant() {
            self.writeln(format_args!(
                "  neighbouring_population_size: {}",
                genotype.neighbouring_population_size(),
            ))
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

    fn on_new_generation<S: StrategyState<Self::Genotype>, C: StrategyConfig>(
        &mut self,
        genotype: &Self::Genotype,
        state: &S,
        _config: &C,
    ) {
        if state.current_generation() % self.period == 0 {
            self.writeln(format_args!(
                "periodic - current_generation: {}, stale_generations: {}, best_generation: {}, scale_index: {:?}, current_population_size: {} ({}r)",
                state.current_generation(),
                state.stale_generations(),
                state.best_generation(),
                genotype.current_scale_index(),
                state.population_as_ref().size(),
                state.population_as_ref().recycled_size(),
            ));
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
                Some(state.best_genes())
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
                    Some(state.best_genes())
                } else {
                    None
                },
            ));
        }
    }
}
