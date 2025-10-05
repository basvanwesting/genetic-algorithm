//! Generic strategy reporters:
//! * [Duration], only reports duration, non-strategy specific
//! * [Noop], silences reporting, non-strategy specific
//! * [Simple], prefer to use strategy specific implementations:
//!     * [EvolveReporterSimple](crate::strategy::evolve::EvolveReporterSimple)
//!     * [PermutateReporterSimple](crate::strategy::permutate::PermutateReporterSimple)
//!     * [HillClimbReporterSimple](crate::strategy::hill_climb::HillClimbReporterSimple)
//!
use crate::genotype::Genotype;
use crate::strategy::{
    StrategyConfig, StrategyReporter, StrategyState, STRATEGY_ACTIONS,
};
use std::fmt::Arguments;
use std::io::Write;
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
    pub buffer: Option<Vec<u8>>,
    _phantom: PhantomData<G>,
}
impl<G: Genotype> Default for Duration<G> {
    fn default() -> Self {
        Self {
            buffer: None,
            _phantom: PhantomData,
        }
    }
}
impl<G: Genotype> Duration<G> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn new_with_buffer() -> Self {
        Self {
            buffer: Some(Vec::new()),
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
impl<G: Genotype> StrategyReporter for Duration<G> {
    type Genotype = G;

    fn flush(&mut self, output: &mut Vec<u8>) {
        if let Some(buffer) = self.buffer.as_mut() {
            output.append(buffer);
        }
    }
    fn on_enter<S: StrategyState, C: StrategyConfig>(
        &mut self,
        _genotype: &Self::Genotype,
        state: &S,
        config: &C,
    ) {
        self.writeln(format_args!(
            "enter - {}, iteration: {}",
            config.variant(),
            state.current_iteration()
        ));
    }
    fn on_exit<S: StrategyState, C: StrategyConfig>(
        &mut self,
        _genotype: &Self::Genotype,
        state: &S,
        config: &C,
    ) {
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
            "  Total: {:.3?} ({:.0}% fitness)",
            &state.total_duration(),
            state.fitness_duration_rate() * 100.0
        ));
    }
}

/// A Simple Strategy reporter generic over Genotype.
/// A report is triggered every period generations
#[derive(Clone)]
pub struct Simple<G: Genotype> {
    pub buffer: Option<Vec<u8>>,
    pub period: usize,
    pub show_genes: bool,
    pub show_equal_fitness: bool,
    _phantom: PhantomData<G>,
}
impl<G: Genotype> Default for Simple<G> {
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
impl<G: Genotype> Simple<G> {
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
            period,
            buffer: if buffered { Some(Vec::new()) } else { None },
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
impl<G: Genotype> StrategyReporter for Simple<G> {
    type Genotype = G;

    fn flush(&mut self, output: &mut Vec<u8>) {
        if let Some(buffer) = self.buffer.as_mut() {
            output.append(buffer);
        }
    }
    fn on_enter<S: StrategyState, C: StrategyConfig>(
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
    fn on_exit<S: StrategyState, C: StrategyConfig>(
        &mut self,
        _genotype: &Self::Genotype,
        state: &S,
        config: &C,
    ) {
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
            "  Total: {:.3?} ({:.0}% fitness)",
            &state.total_duration(),
            state.fitness_duration_rate() * 100.0
        ));
    }

    fn on_new_generation<S: StrategyState, C: StrategyConfig>(
        &mut self,
        _genotype: &Self::Genotype,
        state: &S,
        _config: &C,
    ) {
        if state.current_generation() % self.period == 0 {
            self.writeln(format_args!(
                "periodic - current_generation: {}, stale_generations: {}, best_generation: {}, scale_index: {:?}",
                state.current_generation(),
                state.stale_generations(),
                state.best_generation(),
                state.current_scale_index(),
            ));
        }
    }

    fn on_new_best_chromosome<S: StrategyState, C: StrategyConfig>(
        &mut self,
        genotype: &Self::Genotype,
        state: &S,
        _config: &C,
    ) {
        self.writeln(format_args!(
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

    fn on_new_best_chromosome_equal_fitness<S: StrategyState, C: StrategyConfig>(
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
                state.current_scale_index(),
                if self.show_genes {
                    Some(genotype.best_genes())
                } else {
                    None
                },
            ));
        }
    }
}
