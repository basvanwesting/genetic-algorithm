use crate::compete::CompeteDispatch;
use crate::crossover::CrossoverDispatch;
use crate::fitness::Fitness;
use crate::genotype::Genotype;
use crate::mutate::MutateDispatch;
use std::ops::Range;

#[derive(Clone, Debug)]
pub struct Config<G: Genotype, F: Fitness<Genotype = G>> {
    pub rounds: usize,
    pub evolve_genotype: G,
    pub evolve_fitness: F,
    pub population_sizes: Vec<usize>,
    pub max_stale_generations_options: Vec<Option<usize>>,
    pub target_fitness_score_options: Vec<Option<isize>>,
    pub degeneration_range_options: Vec<Option<Range<f32>>>,
    pub mutates: Vec<MutateDispatch>,
    pub crossovers: Vec<CrossoverDispatch>,
    pub competes: Vec<CompeteDispatch>,
}
