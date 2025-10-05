use super::Permutate;
pub use crate::errors::TryFromStrategyBuilderError as TryFromBuilderError;
use crate::fitness::{Fitness, FitnessOrdering};
use crate::genotype::PermutateGenotype;
use crate::strategy::{Strategy, StrategyReporter, StrategyReporterNoop};

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
    pub replace_on_equal_fitness: bool,
    pub population_window_size: usize,
    pub reporter: SR,
}

impl<G: PermutateGenotype, F: Fitness<Genotype = G>> Default
    for Builder<G, F, StrategyReporterNoop<G>>
{
    fn default() -> Self {
        Self {
            genotype: None,
            fitness_ordering: FitnessOrdering::Maximize,
            replace_on_equal_fitness: false,
            population_window_size: 0,
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
    pub fn with_replace_on_equal_fitness(mut self, replace_on_equal_fitness: bool) -> Self {
        self.replace_on_equal_fitness = replace_on_equal_fitness;
        self
    }
    pub fn with_population_window_size(mut self, population_window_size: usize) -> Self {
        self.population_window_size = population_window_size;
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
            replace_on_equal_fitness: self.replace_on_equal_fitness,
            population_window_size: self.population_window_size,
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
