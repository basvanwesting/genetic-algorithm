use crate::fitness::{Fitness, FitnessOrdering};
use crate::genotype::PermutableGenotype;
use crate::permutate::Permutate;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TryFromBuilderError(pub &'static str);

/// The builder for an Permutate struct.
#[derive(Clone, Debug)]
pub struct Builder<G: PermutableGenotype, F: Fitness<Genotype = G>> {
    pub genotype: Option<G>,
    pub fitness: Option<F>,
    pub fitness_ordering: FitnessOrdering,
}

impl<G: PermutableGenotype, F: Fitness<Genotype = G>> Builder<G, F> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Result<Permutate<G, F>, TryFromBuilderError> {
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
    pub fn with_fitness(mut self, fitness: F) -> Self {
        self.fitness = Some(fitness);
        self
    }
}

impl<G: PermutableGenotype, F: Fitness<Genotype = G>> Default for Builder<G, F> {
    fn default() -> Self {
        Self {
            genotype: None,
            fitness_ordering: FitnessOrdering::Maximize,
            fitness: None,
        }
    }
}
