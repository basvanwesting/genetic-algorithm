use super::Crossover;
use crate::genotype::Genotype;
use crate::strategy::evolve::{EvolveConfig, EvolveReporter, EvolveState};
use crate::strategy::{StrategyAction, StrategyState};
use itertools::Itertools;
use rand::Rng;
use std::time::Instant;

/// Crossover a single gene position from which on the rest of the genes are taken from the other
/// parent. The gene position is chosen with uniform probability. Optionally keep a percentage of
/// the parents around to compete with children later on.
///
/// Not allowed for [UniqueGenotype](crate::genotype::UniqueGenotype) as it would not preserve the gene uniqueness in the children.
/// Allowed for [MultiUniqueGenotype](crate::genotype::MultiUniqueGenotype) as there are valid crossover points between each new set
#[derive(Clone, Debug)]
pub struct SinglePoint {
    pub parent_survival_rate: f32,
}
impl Crossover for SinglePoint {
    fn call<G: Genotype, R: Rng, SR: EvolveReporter<Genotype = G>>(
        &mut self,
        genotype: &G,
        state: &mut EvolveState<G>,
        _config: &EvolveConfig,
        _reporter: &mut SR,
        rng: &mut R,
    ) {
        let now = Instant::now();
        let population_size = state.population.size();
        let parent_survivors = std::cmp::min(
            (population_size as f32 * self.parent_survival_rate) as usize,
            population_size,
        );
        state
            .population
            .chromosomes
            .extend_from_within(..parent_survivors);

        for (father, mother) in state
            .population
            .chromosomes
            .iter_mut()
            .take(population_size)
            .tuples()
        {
            genotype.crossover_chromosome_pair_single_point(father, mother, rng);
        }

        state.add_duration(StrategyAction::Crossover, now.elapsed());
    }
    fn require_crossover_points(&self) -> bool {
        true
    }
}

impl SinglePoint {
    pub fn new(parent_survival_rate: f32) -> Self {
        Self {
            parent_survival_rate,
        }
    }
}
