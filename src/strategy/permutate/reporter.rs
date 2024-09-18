//! Reporters directed at Permutate process specific data
use crate::genotype::PermutateGenotype;
use crate::strategy::{StrategyConfig, StrategyReporter, StrategyState, STRATEGY_ACTIONS};
use num::{BigUint, ToPrimitive};
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
    fn write(&mut self, message: String) {
        if let Some(buffer) = self.buffer.as_mut() {
            buffer.write_all(message.as_bytes()).unwrap_or(());
            writeln!(buffer).unwrap_or(());
        } else {
            println!("{}", message);
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
            self.write(format!(
                "progress: {}, current_generation: {}, best_generation: {}",
                progress.map_or("-".to_string(), |v| format!("{:3.3}%", v)),
                state.current_generation(),
                state.best_generation(),
            ));
        }
    }

    fn on_new_best_chromosome<S: StrategyState<Self::Genotype>, C: StrategyConfig>(
        &mut self,
        genotype: &Self::Genotype,
        state: &S,
        _config: &C,
    ) {
        self.write(format!(
            "new best - generation: {}, fitness_score: {:?}, genes: {:?}",
            state.current_generation(),
            state.best_fitness_score(),
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
                "equal best - generation: {}, fitness_score: {:?}, genes: {:?}",
                state.current_generation(),
                state.best_fitness_score(),
                if self.show_genes {
                    Some(genotype.best_genes())
                } else {
                    None
                },
            ));
        }
    }

    fn on_finish<S: StrategyState<Self::Genotype>, C: StrategyConfig>(
        &mut self,
        _genotype: &Self::Genotype,
        state: &S,
        _config: &C,
    ) {
        self.write(format!(
            "finish - generation: {}",
            state.current_generation()
        ));
        STRATEGY_ACTIONS.iter().for_each(|action| {
            if let Some(duration) = state.durations().get(action) {
                self.write(format!("  {:?}: {:?}", action, duration,));
            }
        });
        self.write(format!("  Total: {:?}", &state.total_duration()));
    }
}
