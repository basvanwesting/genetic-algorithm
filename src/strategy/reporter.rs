use crate::genotype::Genotype;
use crate::strategy::{StrategyConfig, StrategyReporter, StrategyState, STRATEGY_ACTIONS};
use std::marker::PhantomData;
use std::io::Write;

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

/// A Simple Strategy reporter generic over Genotype.
/// A report is triggered every period generations
#[derive(Clone)]
pub struct Simple<G: Genotype> {
    pub period: usize,
    pub show_genes: bool,
    pub show_equal_fitness: bool,
    _phantom: PhantomData<G>,
}
impl<G: Genotype> Default for Simple<G> {
    fn default() -> Self {
        Self {
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
    pub fn new_with_flags(period: usize, show_genes: bool, show_equal_fitness: bool) -> Self {
        Self {
            period,
            show_genes,
            show_equal_fitness,
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
        _config: &C,
    ) {
        if state.current_generation() % self.period == 0 {
            println!(
                "periodic - current_generation: {}, stale_generations: {}, best_generation: {}, scale_index: {:?}",
                state.current_generation(),
                state.stale_generations(),
                state.best_generation(),
                state.current_scale_index(),
            );
        }
    }

    fn on_new_best_chromosome<S: StrategyState<Self::Genotype>, C: StrategyConfig>(
        &mut self,
        genotype: &Self::Genotype,
        state: &S,
        _config: &C,
    ) {
        println!(
            "new best - generation: {}, fitness_score: {:?}, scale_index: {:?}, genes: {:?}",
            state.current_generation(),
            state.best_fitness_score(),
            state.current_scale_index(),
            if self.show_genes {
                Some(genotype.best_genes())
            } else {
                None
            },
        );
    }

    fn on_new_best_chromosome_equal_fitness<S: StrategyState<Self::Genotype>, C: StrategyConfig>(
        &mut self,
        genotype: &Self::Genotype,
        state: &S,
        _config: &C,
    ) {
        if self.show_equal_fitness {
            println!(
                "equal best - generation: {}, fitness_score: {:?}, scale_index: {:?}, genes: {:?}",
                state.current_generation(),
                state.best_fitness_score(),
                state.current_scale_index(),
                if self.show_genes {
                    Some(genotype.best_genes())
                } else {
                    None
                },
            );
        }
    }
}

/// A buffered version of the Simple Strategy reporter generic over Genotype.
/// Write errors are ignored
#[derive(Clone)]
pub struct Buffer<G: Genotype> {
    pub buffer: Vec<u8>,
    pub period: usize,
    pub show_genes: bool,
    pub show_equal_fitness: bool,
    _phantom: PhantomData<G>,
}
impl<G: Genotype> Default for Buffer<G> {
    fn default() -> Self {
        Self {
            buffer: vec![],
            period: 1,
            show_genes: false,
            show_equal_fitness: false,
            _phantom: PhantomData,
        }
    }
}
impl<G: Genotype> Buffer<G> {
    pub fn new(period: usize) -> Self {
        Self {
            period,
            ..Default::default()
        }
    }
    pub fn new_with_flags(period: usize, show_genes: bool, show_equal_fitness: bool) -> Self {
        Self {
            period,
            show_genes,
            show_equal_fitness,
            ..Default::default()
        }
    }
}
impl<G: Genotype> StrategyReporter for Buffer<G> {
    type Genotype = G;

    fn flush(&mut self, output: &mut Vec<u8>) {
        output.append(&mut self.buffer);
    }
    fn on_init<S: StrategyState<Self::Genotype>, C: StrategyConfig>(
        &mut self,
        genotype: &Self::Genotype,
        state: &S,
        _config: &C,
    ) {
        writeln!(&mut self.buffer, "init - iteration: {}", state.current_iteration()).unwrap_or(());
        genotype
            .seed_genes_list()
            .iter()
            .for_each(|genes| writeln!(&mut self.buffer, "init - seed_genes: {:?}", genes).unwrap_or(()));
    }
    fn on_start<S: StrategyState<Self::Genotype>, C: StrategyConfig>(
        &mut self,
        _genotype: &Self::Genotype,
        state: &S,
        _config: &C,
    ) {
        writeln!(&mut self.buffer, "start - iteration: {}", state.current_iteration()).unwrap_or(());
    }

    fn on_finish<S: StrategyState<Self::Genotype>, C: StrategyConfig>(
        &mut self,
        _genotype: &Self::Genotype,
        state: &S,
        _config: &C,
    ) {
        writeln!(&mut self.buffer, "finish - iteration: {}", state.current_iteration()).unwrap_or(());
        STRATEGY_ACTIONS.iter().for_each(|action| {
            if let Some(duration) = state.durations().get(action) {
                writeln!(&mut self.buffer, "  {:?}: {:?}", action, duration,).unwrap_or(());
            }
        });
        writeln!(&mut self.buffer, "  Total: {:?}", &state.total_duration()).unwrap_or(());
    }

    fn on_new_generation<S: StrategyState<Self::Genotype>, C: StrategyConfig>(
        &mut self,
        _genotype: &Self::Genotype,
        state: &S,
        _config: &C,
    ) {
        if state.current_generation() % self.period == 0 {
            writeln!(&mut self.buffer, 
                "periodic - current_generation: {}, stale_generations: {}, best_generation: {}, scale_index: {:?}",
                state.current_generation(),
                state.stale_generations(),
                state.best_generation(),
                state.current_scale_index(),
            ).unwrap_or(());
        }
    }

    fn on_new_best_chromosome<S: StrategyState<Self::Genotype>, C: StrategyConfig>(
        &mut self,
        genotype: &Self::Genotype,
        state: &S,
        _config: &C,
    ) {
        writeln!(&mut self.buffer, 
            "new best - generation: {}, fitness_score: {:?}, scale_index: {:?}, genes: {:?}",
            state.current_generation(),
            state.best_fitness_score(),
            state.current_scale_index(),
            if self.show_genes {
                Some(genotype.best_genes())
            } else {
                None
            },
        ).unwrap_or(());
    }

    fn on_new_best_chromosome_equal_fitness<S: StrategyState<Self::Genotype>, C: StrategyConfig>(
        &mut self,
        genotype: &Self::Genotype,
        state: &S,
        _config: &C,
    ) {
        if self.show_equal_fitness {
            writeln!(&mut self.buffer, 
                "equal best - generation: {}, fitness_score: {:?}, scale_index: {:?}, genes: {:?}",
                state.current_generation(),
                state.best_fitness_score(),
                state.current_scale_index(),
                if self.show_genes {
                    Some(genotype.best_genes())
                } else {
                    None
                },
            ).unwrap_or(());
        }
    }
}
