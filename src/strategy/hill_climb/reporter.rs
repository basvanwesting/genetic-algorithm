use super::{HillClimbConfig, HillClimbState};
use crate::genotype::{Genotype, IncrementalGenotype};
use crate::strategy::StrategyState;
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
///     fn on_new_generation(&mut self, state: &HillClimbState<Self::Genotype::Allele>, _config: &HillClimbConfig) {
///         if state.current_generation() % self.period == 0 {
///             println!(
///                 "periodic - current_generation: {}, best_generation: {}, current_scale: {:?}",
///                 state.current_generation(),
///                 state.best_generation(),
///                 state.current_scale.as_ref(),
///             );
///         }
///     }
///
///     fn on_new_best_chromosome(&mut self, state: &HillClimbState<Self::Genotype::Allele>, _config: &HillClimbConfig) {
///         println!(
///             "new best - generation: {}, fitness_score: {:?}, genes: {:?}, scale: {:?}",
///             state.current_generation(),
///             state.best_fitness_score(),
///             state.best_chromosome_as_ref().map(|c| &c.genes),
///             state.current_scale.as_ref(),
///         );
///     }
/// }
/// ```
pub trait Reporter: Clone + Send + Sync {
    type Genotype: IncrementalGenotype;

    fn on_start(
        &mut self,
        _genotype: &Self::Genotype,
        _state: &HillClimbState<<<Self as Reporter>::Genotype as Genotype>::Allele>,
        _config: &HillClimbConfig,
    ) {
    }
    fn on_finish(
        &mut self,
        _state: &HillClimbState<<<Self as Reporter>::Genotype as Genotype>::Allele>,
        _config: &HillClimbConfig,
    ) {
    }
    fn on_new_generation(
        &mut self,
        _state: &HillClimbState<<<Self as Reporter>::Genotype as Genotype>::Allele>,
        _config: &HillClimbConfig,
    ) {
    }
    /// used to report on true improvement (new best chromosome with improved fitness)
    fn on_new_best_chromosome(
        &mut self,
        _state: &HillClimbState<<<Self as Reporter>::Genotype as Genotype>::Allele>,
        _config: &HillClimbConfig,
    ) {
    }
    /// used to report on sideways move (new best chromosome with equal fitness)
    fn on_new_best_chromosome_equal_fitness(
        &mut self,
        _state: &HillClimbState<<<Self as Reporter>::Genotype as Genotype>::Allele>,
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
impl<G: IncrementalGenotype + Clone + Send + Sync> Reporter for Noop<G> {
    type Genotype = G;
}

/// A Simple reporter generic over Genotype.
/// A report is triggered every period generations
#[derive(Clone)]
pub struct Simple<G: IncrementalGenotype> {
    pub period: usize,
    pub show_genes: bool,
    _phantom: PhantomData<G>,
}
impl<G: IncrementalGenotype> Default for Simple<G> {
    fn default() -> Self {
        Self {
            period: 1,
            show_genes: false,
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
    pub fn new_with_flags(period: usize, show_genes: bool) -> Self {
        Self {
            period,
            show_genes,
            ..Default::default()
        }
    }
}
impl<G: IncrementalGenotype + Clone + Send + Sync> Reporter for Simple<G> {
    type Genotype = G;

    fn on_start(
        &mut self,
        genotype: &Self::Genotype,
        state: &HillClimbState<<<Self as Reporter>::Genotype as Genotype>::Allele>,
        _config: &HillClimbConfig,
    ) {
        println!("start - iteration: {}", state.current_iteration());
        genotype
            .seed_genes_list()
            .iter()
            .for_each(|genes| println!("start - seed_genes: {:?}", genes));
    }

    fn on_finish(
        &mut self,
        state: &HillClimbState<<<Self as Reporter>::Genotype as Genotype>::Allele>,
        _config: &HillClimbConfig,
    ) {
        println!("finish - iteration: {}", state.current_iteration());
    }

    fn on_new_generation(
        &mut self,
        state: &HillClimbState<<<Self as Reporter>::Genotype as Genotype>::Allele>,
        _config: &HillClimbConfig,
    ) {
        if state.current_generation() % self.period == 0 {
            println!(
                "periodic - current_generation: {}, best_generation: {}, current_scale: {:?}",
                state.current_generation(),
                state.best_generation(),
                state.current_scale.as_ref(),
            );
        }
    }

    fn on_new_best_chromosome(
        &mut self,
        state: &HillClimbState<<<Self as Reporter>::Genotype as Genotype>::Allele>,
        _config: &HillClimbConfig,
    ) {
        println!(
            "new best - generation: {}, fitness_score: {:?}, genes: {:?}, scale: {:?}",
            state.current_generation(),
            state.best_fitness_score(),
            if self.show_genes {
                state.best_chromosome_as_ref().map(|c| &c.genes)
            } else {
                None
            },
            state.current_scale.as_ref(),
        );
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
impl<G: IncrementalGenotype + Clone + Send + Sync> Reporter for Log<G> {
    type Genotype = G;

    fn on_new_generation(
        &mut self,
        state: &HillClimbState<<<Self as Reporter>::Genotype as Genotype>::Allele>,
        _config: &HillClimbConfig,
    ) {
        log::debug!(
            "generation (current/best): {}/{}, fitness score (best): {:?}, current scale: {:?}",
            state.current_generation(),
            state.best_generation(),
            state.best_fitness_score(),
            state.current_scale.as_ref(),
        );

        log::trace!(
            "best - fitness score: {:?}, genes: {:?}",
            state.best_fitness_score(),
            state.best_chromosome_as_ref().map(|c| &c.genes)
        );
        if log::log_enabled!(log::Level::Trace) {
            if let Some(chromosome) = state.contending_chromosome.as_ref() {
                log::trace!(
                    "contending - fitness score: {:?}, genes: {:?}",
                    chromosome.fitness_score,
                    chromosome.genes,
                );
            }
            if let Some(population) = state.neighbouring_population.as_ref() {
                population.chromosomes.iter().for_each(|chromosome| {
                    log::trace!(
                        "neighbour - fitness score: {:?}, genes: {:?}",
                        chromosome.fitness_score,
                        chromosome.genes,
                    );
                })
            }
        }
    }
}
