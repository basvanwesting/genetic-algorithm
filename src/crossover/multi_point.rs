use super::Crossover;
use crate::genotype::Genotype;
use crate::strategy::evolve::{EvolveConfig, EvolveReporter, EvolveState};
use crate::strategy::{StrategyAction, StrategyState};
use itertools::Itertools;
use rand::Rng;
use std::time::Instant;

/// Crossover multiple gene positions from which on the rest of the genes are taken from the other
/// parent. This goes back and forth. The gene positions are chosen with uniform probability.
/// Choose between allowing duplicate crossovers on the same point or not (not much slower, as
/// crossover itself is relatively expensive).
/// The population is restored towards the target_population_size by keeping the best parents
/// alive. Excess parents are dropped.
///
/// Not allowed for [UniqueGenotype](crate::genotype::UniqueGenotype) as it would not preserve the gene uniqueness in the children.
/// Allowed for [MultiUniqueGenotype](crate::genotype::MultiUniqueGenotype) as there are valid crossover points between each new set
#[derive(Clone, Debug)]
pub struct MultiPoint {
    pub number_of_crossovers: usize,
    pub allow_duplicates: bool,
}
impl Crossover for MultiPoint {
    fn call<G: Genotype, R: Rng, SR: EvolveReporter<Genotype = G>>(
        &mut self,
        genotype: &mut G,
        state: &mut EvolveState<G>,
        config: &EvolveConfig,
        _reporter: &mut SR,
        rng: &mut R,
    ) {
        let now = Instant::now();
        let crossover_size = self.prepare_population(genotype, state, config);
        for (father, mother) in state
            .population
            .chromosomes
            .iter_mut()
            .take(crossover_size)
            .tuples()
        {
            genotype.crossover_chromosome_points(
                self.number_of_crossovers,
                self.allow_duplicates,
                father,
                mother,
                rng,
            );
        }
        state.add_duration(StrategyAction::Crossover, now.elapsed());
    }
    fn require_crossover_points(&self) -> bool {
        true
    }
}

impl MultiPoint {
    pub fn new(number_of_crossovers: usize, allow_duplicates: bool) -> Self {
        Self {
            number_of_crossovers,
            allow_duplicates,
        }
    }
}
