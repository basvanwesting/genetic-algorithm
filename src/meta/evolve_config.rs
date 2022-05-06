use crate::compete::CompeteDispatch;
use crate::crossover::CrossoverDispatch;
use crate::fitness::{Fitness, FitnessOrdering, FitnessValue};
use crate::genotype::Genotype;
use crate::mutate::MutateDispatch;
use std::ops::Range;

#[derive(Clone, Debug)]
pub struct EvolveConfig<G: Genotype, F: Fitness<Genotype = G>> {
    pub genotype: G,
    pub fitness: F,
    pub population_size: usize,
    pub max_stale_generations: Option<usize>,
    pub target_fitness_score: Option<FitnessValue>,
    pub fitness_ordering: FitnessOrdering,
    pub degeneration_range: Option<Range<f32>>,
    pub mutate: Option<MutateDispatch>,
    pub crossover: Option<CrossoverDispatch>,
    pub compete: Option<CompeteDispatch>,
}

impl<G: Genotype, F: Fitness<Genotype = G>> EvolveConfig<G, F> {
    pub fn new(genotype: G, fitness: F) -> Self {
        Self {
            genotype,
            fitness,
            population_size: 0,
            max_stale_generations: None,
            target_fitness_score: None,
            fitness_ordering: FitnessOrdering::Maximize,
            degeneration_range: None,
            mutate: None,
            crossover: None,
            compete: None,
        }
    }
}
