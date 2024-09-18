use super::HillClimbVariant;
use crate::genotype::HillClimbGenotype;
use crate::strategy::{
    StrategyConfig, StrategyReporter, StrategyState, StrategyVariant, STRATEGY_ACTIONS,
};
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
    fn write(&mut self, message: String) {
        if let Some(buffer) = self.buffer.as_mut() {
            buffer.write_all(message.as_bytes()).unwrap_or(());
            writeln!(buffer).unwrap_or(());
        } else {
            println!("{}", message);
        }
    }
}
impl<G: HillClimbGenotype> StrategyReporter for Simple<G> {
    type Genotype = G;

    fn on_init<S: StrategyState<Self::Genotype>, C: StrategyConfig>(
        &mut self,
        genotype: &Self::Genotype,
        state: &S,
        config: &C,
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
        if let StrategyVariant::HillClimb(HillClimbVariant::SteepestAscent) = config.variant() {
            self.write(format!(
                "init - neighbouring_population_size: {}",
                genotype.neighbouring_population_size(),
            ))
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

    fn on_new_generation<S: StrategyState<Self::Genotype>, C: StrategyConfig>(
        &mut self,
        _genotype: &Self::Genotype,
        state: &S,
        _config: &C,
    ) {
        if state.current_generation() % self.period == 0 {
            self.write(format!(
                "periodic - current_generation: {}, stale_generations: {}, best_generation: {}, scale_index: {:?}",
                state.current_generation(),
                state.stale_generations(),
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
}
