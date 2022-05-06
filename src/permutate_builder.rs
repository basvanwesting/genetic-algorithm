use crate::fitness::{Fitness, FitnessOrdering};
use crate::genotype::PermutableGenotype;
use crate::permutate::Permutate;

#[derive(Debug, Clone)]
pub struct TryFromPermutateBuilderError;

#[derive(Clone, Debug)]
pub struct PermutateBuilder<G: PermutableGenotype, F: Fitness<Genotype = G>> {
    pub genotype: Option<G>,
    pub fitness: Option<F>,
    pub fitness_ordering: FitnessOrdering,
}

impl<G: PermutableGenotype, F: Fitness<Genotype = G>> PermutateBuilder<G, F> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Result<Permutate<G, F>, TryFromPermutateBuilderError> {
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

    pub fn is_valid(&self) -> bool {
        self.genotype.is_some() && self.fitness.is_some()
    }
}

impl<G: PermutableGenotype, F: Fitness<Genotype = G>> Default for PermutateBuilder<G, F> {
    fn default() -> Self {
        Self {
            genotype: None,
            fitness_ordering: FitnessOrdering::Maximize,
            fitness: None,
        }
    }
}
