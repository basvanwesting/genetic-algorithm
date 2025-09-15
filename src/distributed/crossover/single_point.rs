use super::Crossover;
use crate::distributed::genotype::EvolveGenotype;
use crate::distributed::strategy::evolve::{EvolveConfig, EvolveState};
use crate::distributed::strategy::{StrategyAction, StrategyReporter, StrategyState};
use itertools::Itertools;
use rand::distributions::{Bernoulli, Distribution};
use rand::Rng;
use std::time::Instant;

/// Crossover a single gene position from which on the rest of the genes are taken from the other
/// parent. The gene position is chosen with uniform probability.
///
/// Not allowed for [UniqueGenotype](crate::genotype::UniqueGenotype) as it would not preserve the gene uniqueness in the children.
/// Allowed for [MultiUniqueGenotype](crate::genotype::MultiUniqueGenotype) as there are valid crossover points between each new set
#[derive(Clone, Debug)]
pub struct SinglePoint {
    pub selection_rate: f32,
    pub crossover_rate: f32,
    pub crossover_sampler: Bernoulli,
}
impl Crossover for SinglePoint {
    fn call<G: EvolveGenotype, R: Rng, SR: StrategyReporter<Genotype = G>>(
        &mut self,
        genotype: &G,
        state: &mut EvolveState<G>,
        _config: &EvolveConfig,
        _reporter: &mut SR,
        rng: &mut R,
    ) {
        let now = Instant::now();
        let existing_population_size = state.population.chromosomes.len();
        let selected_population_size =
            (state.population.size() as f32 * self.selection_rate).ceil() as usize;
        self.expand_chromosome_population(
            &mut state.population.chromosomes,
            selected_population_size,
        );
        let iterator = state
            .population
            .chromosomes
            .iter_mut()
            .skip(existing_population_size);
        for (father, mother) in iterator.tuples() {
            if self.crossover_sampler.sample(rng) {
                genotype.crossover_chromosome_points(1, true, father, mother, rng);
            } else {
                father.reset_age();
                mother.reset_age();
            }
        }
        if selected_population_size % 2 == 1 {
            if let Some(chromosome) = state.population.chromosomes.last_mut() {
                chromosome.reset_age();
            }
        }

        state.add_duration(StrategyAction::Crossover, now.elapsed());
    }
    fn require_crossover_points(&self) -> bool {
        true
    }
}

impl SinglePoint {
    pub fn new(selection_rate: f32, crossover_rate: f32) -> Self {
        let crossover_sampler = Bernoulli::new(crossover_rate as f64).unwrap();
        Self {
            selection_rate,
            crossover_rate,
            crossover_sampler,
        }
    }
}
