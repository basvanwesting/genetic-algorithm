use super::Permutate;
pub use crate::errors::TryFromStrategyBuilderError as TryFromBuilderError;
use crate::fitness::{Fitness, FitnessOrdering, FitnessValue};
use crate::genotype::PermutateGenotype;
use crate::strategy::{Strategy, StrategyReporter, StrategyReporterNoop};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

/// The builder for an Permutate struct.
#[derive(Clone, Debug)]
pub struct Builder<
    G: PermutateGenotype,
    F: Fitness<Genotype = G>,
    SR: StrategyReporter<Genotype = G>,
> {
    pub genotype: Option<G>,
    pub fitness: Option<F>,
    pub fitness_ordering: FitnessOrdering,
    pub par_fitness: bool,
    pub replace_on_equal_fitness: bool,
    pub target_fitness_score: Option<FitnessValue>,
    pub abort_flag: Option<Arc<AtomicBool>>,
    pub reporter: SR,
}

impl<G: PermutateGenotype, F: Fitness<Genotype = G>> Default
    for Builder<G, F, StrategyReporterNoop<G>>
{
    fn default() -> Self {
        Self {
            genotype: None,
            fitness_ordering: FitnessOrdering::Maximize,
            par_fitness: false,
            replace_on_equal_fitness: true,
            target_fitness_score: None,
            abort_flag: None,
            fitness: None,
            reporter: StrategyReporterNoop::new(),
        }
    }
}
impl<G: PermutateGenotype, F: Fitness<Genotype = G>> Builder<G, F, StrategyReporterNoop<G>> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<G: PermutateGenotype, F: Fitness<Genotype = G>, SR: StrategyReporter<Genotype = G>>
    Builder<G, F, SR>
{
    pub fn build(self) -> Result<Permutate<G, F, SR>, TryFromBuilderError> {
        self.try_into()
    }
    pub fn with_genotype(mut self, genotype: G) -> Self {
        self.genotype = Some(genotype);
        self
    }
    pub fn with_fitness_ordering(mut self, fitness_ordering: FitnessOrdering) -> Self {
        self.fitness_ordering = fitness_ordering;
        self
    }
    pub fn with_par_fitness(mut self, par_fitness: bool) -> Self {
        self.par_fitness = par_fitness;
        self
    }
    pub fn with_replace_on_equal_fitness(mut self, replace_on_equal_fitness: bool) -> Self {
        self.replace_on_equal_fitness = replace_on_equal_fitness;
        self
    }
    /// Optional ending condition: stop permutating as soon as the best chromosome reaches this
    /// fitness score, instead of exhausting the whole permutation space.
    pub fn with_target_fitness_score(mut self, target_fitness_score: FitnessValue) -> Self {
        self.target_fitness_score = Some(target_fitness_score);
        self
    }
    pub fn with_target_fitness_score_option(
        mut self,
        target_fitness_score_option: Option<FitnessValue>,
    ) -> Self {
        self.target_fitness_score = target_fitness_score_option;
        self
    }
    /// Provide a cooperative abort signal, checked once per chromosome. Set the flag to `true`
    /// (e.g. from another thread) to stop the run early, returning the best chromosome found so
    /// far.
    pub fn with_abort_flag(mut self, abort_flag: Arc<AtomicBool>) -> Self {
        self.abort_flag = Some(abort_flag);
        self
    }
    pub fn with_abort_flag_option(mut self, abort_flag_option: Option<Arc<AtomicBool>>) -> Self {
        self.abort_flag = abort_flag_option;
        self
    }
    pub fn with_fitness(mut self, fitness: F) -> Self {
        self.fitness = Some(fitness);
        self
    }
    pub fn with_reporter<SR2: StrategyReporter<Genotype = G>>(
        self,
        reporter: SR2,
    ) -> Builder<G, F, SR2> {
        Builder {
            genotype: self.genotype,
            fitness_ordering: self.fitness_ordering,
            par_fitness: self.par_fitness,
            replace_on_equal_fitness: self.replace_on_equal_fitness,
            target_fitness_score: self.target_fitness_score,
            abort_flag: self.abort_flag,
            fitness: self.fitness,
            reporter,
        }
    }
}
impl<G: PermutateGenotype, F: Fitness<Genotype = G>, SR: StrategyReporter<Genotype = G>>
    Builder<G, F, SR>
{
    pub fn call(self) -> Result<Permutate<G, F, SR>, TryFromBuilderError> {
        let mut permutate: Permutate<G, F, SR> = self.try_into()?;
        permutate.call();
        Ok(permutate)
    }
}
