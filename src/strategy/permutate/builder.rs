use super::{Permutate, PermutateReporter};
use crate::fitness::{Fitness, FitnessOrdering};
use crate::genotype::PermutableGenotype;
use crate::strategy::Strategy;
use rand::Rng;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TryFromBuilderError(pub &'static str);

/// The builder for an Permutate struct.
#[derive(Clone, Debug)]
pub struct Builder<
    G: PermutableGenotype,
    F: Fitness<Genotype = G>,
    SR: PermutateReporter<Genotype = G>,
> {
    pub genotype: Option<G>,
    pub fitness: Option<F>,
    pub fitness_ordering: FitnessOrdering,
    pub multithreading: bool,
    pub reporter: Option<SR>,
}

impl<G: PermutableGenotype, F: Fitness<Genotype = G>, SR: PermutateReporter<Genotype = G>>
    Builder<G, F, SR>
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Result<Permutate<G, F, SR>, TryFromBuilderError> {
        self.try_into()
    }
    pub fn call<R: Rng>(self, rng: &mut R) -> Result<Permutate<G, F, SR>, TryFromBuilderError> {
        let mut permutate: Permutate<G, F, SR> = self.try_into()?;
        permutate.call(rng);
        Ok(permutate)
    }

    pub fn with_genotype(mut self, genotype: G) -> Self {
        self.genotype = Some(genotype);
        self
    }
    pub fn with_fitness_ordering(mut self, fitness_ordering: FitnessOrdering) -> Self {
        self.fitness_ordering = fitness_ordering;
        self
    }
    pub fn with_multithreading(mut self, multithreading: bool) -> Self {
        self.multithreading = multithreading;
        self
    }
    pub fn with_fitness(mut self, fitness: F) -> Self {
        self.fitness = Some(fitness);
        self
    }
    pub fn with_reporter(mut self, reporter: SR) -> Self {
        self.reporter = Some(reporter);
        self
    }
}

impl<G: PermutableGenotype, F: Fitness<Genotype = G>, SR: PermutateReporter<Genotype = G>> Default
    for Builder<G, F, SR>
{
    fn default() -> Self {
        Self {
            genotype: None,
            fitness_ordering: FitnessOrdering::Maximize,
            multithreading: false,
            fitness: None,
            reporter: None,
            // reporter: Some(SR::default()),
        }
    }
}
