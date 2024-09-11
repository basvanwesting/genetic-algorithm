use super::{HillClimbConfig, HillClimbState};
use crate::chromosome::Chromosome;
use crate::genotype::IncrementalGenotype;
use crate::strategy::{StrategyState, STRATEGY_ACTIONS};
use std::marker::PhantomData;

/// Reporter with event hooks in the HillClimb process.
/// Since the HillClimb process sets a new best chromosome, even if the fitness is equal, there is
/// an extra event hook for this situation.
///
/// # Example:
/// You are encouraged to take a look at the [HillClimbReporterSimple](Simple) implementation, and
/// then roll your own like below:
/// ```rust
/// use genetic_algorithm::strategy::hill_climb::prelude::*;
///
/// #[derive(Clone)]
/// pub struct CustomReporter { pub period: usize }
/// impl HillClimbReporter for CustomReporter {
///     type Genotype = BinaryGenotype;
///
///     fn on_new_generation(&mut self, _genotype: &Self::Genotype, state: &HillClimbState<Self::Genotype>, _config: &HillClimbConfig) {
///         if state.current_generation() % self.period == 0 {
///             println!(
///                 "periodic - current_generation: {}, stale_generations: {}, best_generation: {}, current_scale_index: {:?}",
///                 state.current_generation(),
///                 state.stale_generations(),
///                 state.best_generation(),
///                 state.current_scale_index.as_ref(),
///             );
///         }
///     }
///
///     fn on_new_best_chromosome(&mut self, genotype: &Self::Genotype, state: &HillClimbState<Self::Genotype>, _config: &HillClimbConfig) {
///         println!(
///             "new best - generation: {}, fitness_score: {:?}, genes: {:?}, scale_index: {:?}",
///             state.current_generation(),
///             state.best_fitness_score(),
///             genotype.best_genes(),
///             state.current_scale_index.as_ref(),
///         );
///     }
///
///     fn on_finish(&mut self, _genotype: &Self::Genotype, state: &HillClimbState<Self::Genotype>, _config: &HillClimbConfig) {
///         println!("finish - iteration: {}", state.current_iteration());
///         STRATEGY_ACTIONS.iter().for_each(|action| {
///             if let Some(duration) = state.durations.get(action) {
///                 println!("  {:?}: {:?}", action, duration,);
///             }
///         });
///         println!("  Total: {:?}", &state.total_duration());
///     }
///
///
/// }
/// ```
pub trait Reporter: Clone + Send + Sync {
    type Genotype: IncrementalGenotype;

    fn on_init(
        &mut self,
        _genotype: &Self::Genotype,
        _state: &HillClimbState<Self::Genotype>,
        _config: &HillClimbConfig,
    ) {
    }
    fn on_start(
        &mut self,
        _genotype: &Self::Genotype,
        _state: &HillClimbState<Self::Genotype>,
        _config: &HillClimbConfig,
    ) {
    }
    fn on_finish(
        &mut self,
        _genotype: &Self::Genotype,
        _state: &HillClimbState<Self::Genotype>,
        _config: &HillClimbConfig,
    ) {
    }
    fn on_new_generation(
        &mut self,
        _genotype: &Self::Genotype,
        _state: &HillClimbState<Self::Genotype>,
        _config: &HillClimbConfig,
    ) {
    }
    /// used to report on true improvement (new best chromosome with improved fitness)
    fn on_new_best_chromosome(
        &mut self,
        _genotype: &Self::Genotype,
        _state: &HillClimbState<Self::Genotype>,
        _config: &HillClimbConfig,
    ) {
    }
    /// used to report on sideways move (new best chromosome with equal fitness)
    fn on_new_best_chromosome_equal_fitness(
        &mut self,
        _genotype: &Self::Genotype,
        _state: &HillClimbState<Self::Genotype>,
        _config: &HillClimbConfig,
    ) {
    }
}

/// The noop reporter, silences reporting
#[derive(Clone)]
pub struct Noop<G: IncrementalGenotype>(pub PhantomData<G>);
impl<G: IncrementalGenotype> Default for Noop<G> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
impl<G: IncrementalGenotype> Noop<G> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<G: IncrementalGenotype> Reporter for Noop<G> {
    type Genotype = G;
}

/// A Duration reporter generic over Genotype.
#[derive(Clone)]
pub struct Duration<G: IncrementalGenotype> {
    _phantom: PhantomData<G>,
}
impl<G: IncrementalGenotype> Default for Duration<G> {
    fn default() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}
impl<G: IncrementalGenotype> Duration<G> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<G: IncrementalGenotype> Reporter for Duration<G> {
    type Genotype = G;

    fn on_start(
        &mut self,
        _genotype: &Self::Genotype,
        state: &HillClimbState<Self::Genotype>,
        _config: &HillClimbConfig,
    ) {
        println!("start - iteration: {}", state.current_iteration());
    }
    fn on_finish(
        &mut self,
        _genotype: &Self::Genotype,
        state: &HillClimbState<Self::Genotype>,
        _config: &HillClimbConfig,
    ) {
        println!("finish - iteration: {}", state.current_iteration());
        STRATEGY_ACTIONS.iter().for_each(|action| {
            if let Some(duration) = state.durations.get(action) {
                println!("  {:?}: {:?}", action, duration,);
            }
        });
        println!("  Total: {:?}", &state.total_duration());
    }
}

/// A Simple reporter generic over Genotype.
/// A report is triggered every period generations
#[derive(Clone)]
pub struct Simple<G: IncrementalGenotype> {
    pub period: usize,
    pub show_genes: bool,
    pub show_equal_fitness: bool,
    _phantom: PhantomData<G>,
}
impl<G: IncrementalGenotype> Default for Simple<G> {
    fn default() -> Self {
        Self {
            period: 1,
            show_genes: false,
            show_equal_fitness: false,
            _phantom: PhantomData,
        }
    }
}
impl<G: IncrementalGenotype> Simple<G> {
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
impl<G: IncrementalGenotype> Reporter for Simple<G> {
    type Genotype = G;

    fn on_init(
        &mut self,
        genotype: &G,
        state: &HillClimbState<Self::Genotype>,
        _config: &HillClimbConfig,
    ) {
        println!("init - iteration: {}", state.current_iteration());
        genotype
            .seed_genes_list()
            .iter()
            .for_each(|genes| println!("init - seed_genes: {:?}", genes));
    }
    fn on_start(
        &mut self,
        _genotype: &Self::Genotype,
        state: &HillClimbState<Self::Genotype>,
        _config: &HillClimbConfig,
    ) {
        println!("start - iteration: {}", state.current_iteration());
    }

    fn on_finish(
        &mut self,
        _genotype: &Self::Genotype,
        state: &HillClimbState<Self::Genotype>,
        _config: &HillClimbConfig,
    ) {
        println!("finish - iteration: {}", state.current_iteration());
        STRATEGY_ACTIONS.iter().for_each(|action| {
            if let Some(duration) = state.durations.get(action) {
                println!("  {:?}: {:?}", action, duration,);
            }
        });
        println!("  Total: {:?}", &state.total_duration());
    }

    fn on_new_generation(
        &mut self,
        _genotype: &Self::Genotype,
        state: &HillClimbState<Self::Genotype>,
        _config: &HillClimbConfig,
    ) {
        if state.current_generation() % self.period == 0 {
            println!(
                "periodic - current_generation: {}, stale_generations: {}, best_generation: {}, current_scale_index: {:?}",
                state.current_generation(),
                state.stale_generations(),
                state.best_generation(),
                state.current_scale_index.as_ref(),
            );
        }
    }

    fn on_new_best_chromosome(
        &mut self,
        genotype: &Self::Genotype,
        state: &HillClimbState<Self::Genotype>,
        _config: &HillClimbConfig,
    ) {
        println!(
            "new best - generation: {}, fitness_score: {:?}, genes: {:?}, scale_index: {:?}",
            state.current_generation(),
            state.best_fitness_score(),
            if self.show_genes {
                Some(genotype.best_genes())
            } else {
                None
            },
            state.current_scale_index.as_ref(),
        );
    }

    fn on_new_best_chromosome_equal_fitness(
        &mut self,
        genotype: &Self::Genotype,
        state: &HillClimbState<Self::Genotype>,
        _config: &HillClimbConfig,
    ) {
        if self.show_equal_fitness {
            println!(
                "equal best - generation: {}, fitness_score: {:?}, genes: {:?}, scale_index: {:?}",
                state.current_generation(),
                state.best_fitness_score(),
                if self.show_genes {
                    Some(genotype.best_genes())
                } else {
                    None
                },
                state.current_scale_index.as_ref(),
            );
        }
    }
}

/// A log-level based reporter for debug and trace, runs on each generation
#[derive(Clone)]
pub struct Log<G: IncrementalGenotype>(pub PhantomData<G>);
impl<G: IncrementalGenotype> Default for Log<G> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
impl<G: IncrementalGenotype> Log<G> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<G: IncrementalGenotype> Reporter for Log<G> {
    type Genotype = G;

    fn on_new_generation(
        &mut self,
        genotype: &Self::Genotype,
        state: &HillClimbState<Self::Genotype>,
        _config: &HillClimbConfig,
    ) {
        log::debug!(
            "generation (current/best): {}/{}, fitness score (best): {:?}, current scale index: {:?}",
            state.current_generation(),
            state.best_generation(),
            state.best_fitness_score(),
            state.current_scale_index.as_ref(),
        );

        log::trace!(
            "best - fitness score: {:?}, genes: {:?}",
            state.best_fitness_score(),
            genotype.best_genes(),
        );
        if log::log_enabled!(log::Level::Trace) {
            log::trace!(
                "contending - fitness score: {:?}, genes: {:?}",
                state.chromosome.as_ref().and_then(|c| c.fitness_score()),
                state.chromosome.as_ref().map(|c| genotype.genes_slice(c)),
            );
            state.population.chromosomes.iter().for_each(|chromosome| {
                log::trace!(
                    "neighbour - fitness score: {:?}, genes: {:?}",
                    chromosome.fitness_score(),
                    genotype.genes_slice(chromosome),
                );
            })
        }
    }
}
