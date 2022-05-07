use crate::chromosome::Chromosome;
use crate::fitness::{Fitness, FitnessOrdering, FitnessValue};
use crate::genotype::PermutableGenotype;
use crate::permutate_builder::{PermutateBuilder, TryFromPermutateBuilderError};
use crate::population::Population;
use std::fmt;

pub struct Permutate<G: PermutableGenotype, F: Fitness<Genotype = G>> {
    pub genotype: G,
    pub fitness: F,

    pub fitness_ordering: FitnessOrdering,

    pub best_chromosome: Option<Chromosome<G>>,
    pub population: Population<G>,
}

impl<G: PermutableGenotype, F: Fitness<Genotype = G>> Permutate<G, F> {
    pub fn builder() -> PermutateBuilder<G, F> {
        PermutateBuilder::new()
    }

    pub fn call(mut self) -> Self {
        self.population = self.genotype.population_factory();
        self.population = self.fitness.call_for_population(self.population);
        self.update_best_chromosome();
        self
    }

    fn update_best_chromosome(&mut self) {
        self.best_chromosome = self
            .population
            .best_chromosome(self.fitness_ordering)
            .cloned();
    }

    fn best_fitness_score(&self) -> Option<FitnessValue> {
        self.best_chromosome.as_ref().and_then(|c| c.fitness_score)
    }
}

impl<G: PermutableGenotype, F: Fitness<Genotype = G>> TryFrom<PermutateBuilder<G, F>>
    for Permutate<G, F>
{
    type Error = TryFromPermutateBuilderError;

    fn try_from(builder: PermutateBuilder<G, F>) -> Result<Self, Self::Error> {
        if builder.genotype.is_none() {
            Err(TryFromPermutateBuilderError("Require a Genotype"))
        } else if builder.fitness.is_none() {
            Err(TryFromPermutateBuilderError("Require a Fitness"))
        } else {
            Ok(Self {
                genotype: builder.genotype.unwrap(),
                fitness: builder.fitness.unwrap(),

                fitness_ordering: builder.fitness_ordering,

                best_chromosome: None,
                population: Population::new_empty(),
            })
        }
    }
}

impl<G: PermutableGenotype, F: Fitness<Genotype = G>> fmt::Display for Permutate<G, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "permutate:")?;
        writeln!(f, "  genotype: {:?}", self.genotype)?;
        writeln!(f, "  fitness: {:?}", self.fitness)?;

        writeln!(f, "  population size: {:?}", self.population.size())?;
        writeln!(f, "  fitness_ordering: {:?}", self.fitness_ordering)?;

        writeln!(f, "  best fitness score: {:?}", self.best_fitness_score())?;
        writeln!(f, "  best_chromosome: {:?}", self.best_chromosome.as_ref())
    }
}
