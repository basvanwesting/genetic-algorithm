//! Reporters directed at Permutate process specific data
use crate::centralized::genotype::PermutateGenotype;
use crate::centralized::strategy::{
    StrategyConfig, StrategyReporter, StrategyState, STRATEGY_ACTIONS,
};
use num::{BigUint, ToPrimitive};
use std::fmt::Arguments;
use std::io::Write;
use std::marker::PhantomData;

/// A Simple Permutate reporter generic over Genotype.
/// A report is triggered every period generations
#[derive(Clone)]
pub struct Simple<G: PermutateGenotype> {
    pub buffer: Option<Vec<u8>>,
    pub period: usize,
    pub show_genes: bool,
    pub show_equal_fitness: bool,
    _phantom: PhantomData<G>,
}
impl<G: PermutateGenotype> Default for Simple<G> {
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
impl<G: PermutateGenotype> Simple<G> {
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
impl<G: PermutateGenotype> StrategyReporter for Simple<G> {
    type Genotype = G;

    fn flush(&mut self, output: &mut Vec<u8>) {
        if let Some(buffer) = self.buffer.as_mut() {
            output.append(buffer);
        }
    }
    fn on_enter<S: StrategyState<Self::Genotype>, C: StrategyConfig>(
        &mut self,
        genotype: &Self::Genotype,
        _state: &S,
        config: &C,
    ) {
        let number_of_seed_genes = genotype.seed_genes_list().len();
        if number_of_seed_genes > 0 {
            self.writeln(format_args!(
                "enter - {}, total generations: {}, number of seed genes: {}",
                config.variant(),
                genotype.chromosome_permutations_size(),
                number_of_seed_genes
            ));
        } else {
            self.writeln(format_args!(
                "enter - {}, total generations: {}",
                config.variant(),
                genotype.chromosome_permutations_size(),
            ));
        }
    }

    fn on_exit<S: StrategyState<Self::Genotype>, C: StrategyConfig>(
        &mut self,
        _genotype: &Self::Genotype,
        state: &S,
        config: &C,
    ) {
        self.writeln(format_args!(
            "exit - {}, generation: {}",
            config.variant(),
            state.current_generation()
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

    fn on_new_generation<S: StrategyState<Self::Genotype>, C: StrategyConfig>(
        &mut self,
        genotype: &Self::Genotype,
        state: &S,
        _config: &C,
    ) {
        if state.current_generation() % self.period == 0 {
            let progress = (BigUint::from(state.current_generation() * 100)
                / &genotype.chromosome_permutations_size())
                .to_u8();
            self.writeln(format_args!(
                "progress: {}, current_generation: {}, best_generation: {}, scale_index: {:?}",
                progress.map_or("-".to_string(), |v| format!("{:3.3}%", v)),
                state.current_generation(),
                state.best_generation(),
                state.current_scale_index(),
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
