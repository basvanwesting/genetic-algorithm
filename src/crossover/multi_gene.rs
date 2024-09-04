use super::Crossover;
use crate::genotype::Genotype;
use crate::strategy::evolve::{EvolveConfig, EvolveReporter, EvolveState};
use crate::strategy::{StrategyAction, StrategyState};
use itertools::Itertools;
use rand::Rng;
use std::time::Instant;

/// Crossover multiple genes between the parents. The gene positions are chosen with uniform
/// probability.
/// Choose between allowing duplicate crossovers of the same gene or not (~2x slower).
/// The population is restored towards the target_population_size by keeping the best parents
/// alive. Excess parents are dropped.
///
/// Not allowed for [UniqueGenotype](crate::genotype::UniqueGenotype) and
/// [MultiUniqueGenotype](crate::genotype::MultiUniqueGenotype) as it would not preserve the gene
/// uniqueness in the children.
#[derive(Clone, Debug)]
pub struct MultiGene {
    pub number_of_crossovers: usize,
    pub allow_duplicates: bool,
}
impl Crossover for MultiGene {
    fn call<G: Genotype, R: Rng, SR: EvolveReporter<Genotype = G>>(
        &mut self,
        genotype: &G,
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
            genotype.crossover_chromosome_genes(
                self.number_of_crossovers,
                self.allow_duplicates,
                father,
                mother,
                rng,
            );
        }
        state.add_duration(StrategyAction::Crossover, now.elapsed());
    }
    fn require_crossover_indexes(&self) -> bool {
        true
    }
}

impl MultiGene {
    pub fn new(number_of_crossovers: usize, allow_duplicates: bool) -> Self {
        Self {
            number_of_crossovers,
            allow_duplicates,
        }
    }
}
