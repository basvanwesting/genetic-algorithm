use crate::compete::CompeteDispatch;
use crate::crossover::CrossoverDispatch;
use crate::fitness::FitnessValue;
use crate::mutate::MutateDispatch;
use std::ops::Range;

#[derive(Clone, Debug)]
pub struct EvolveConfig {
    pub population_size: usize,
    pub max_stale_generations_option: Option<usize>,
    pub target_fitness_score_option: Option<FitnessValue>,
    pub degeneration_range_option: Option<Range<f32>>,
    pub mutate: MutateDispatch,
    pub crossover: CrossoverDispatch,
    pub compete: CompeteDispatch,
}
