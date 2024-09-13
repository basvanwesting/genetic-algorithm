use super::Crossover;
use crate::genotype::Genotype;
use crate::strategy::evolve::{EvolveConfig, EvolveState};
use crate::strategy::{StrategyAction, StrategyReporter, StrategyState};
use itertools::Itertools;
use rand::Rng;
use std::time::Instant;

/// Crossover with 50% probability for each gene to come from one of the two parents.
/// Actually implemented as `CrossoverMultiGene::new(<genes_size> / 2, allow_duplicates=true)`
///
/// The population is restored towards the target_population_size by keeping the best parents
/// alive. Excess parents are dropped.
///
/// Not allowed for [UniqueGenotype](crate::genotype::UniqueGenotype) and
/// [MultiUniqueGenotype](crate::genotype::MultiUniqueGenotype) as it would not preserve the gene
/// uniqueness in the children.
#[derive(Clone, Debug, Default)]
pub struct Uniform;
impl Crossover for Uniform {
    fn call<G: Genotype, R: Rng, SR: StrategyReporter<Genotype = G>>(
        &mut self,
        genotype: &mut G,
        state: &mut EvolveState<G>,
        config: &EvolveConfig,
        _reporter: &mut SR,
        rng: &mut R,
    ) {
        let now = Instant::now();
        let crossover_size = self.prepare_population(genotype, state, config);
        let number_of_crossovers = genotype.genes_size() / 2;
        for (father, mother) in state
            .population
            .chromosomes
            .iter_mut()
            .take(crossover_size)
            .tuples()
        {
            genotype.crossover_chromosome_genes(number_of_crossovers, true, father, mother, rng);
        }

        state.add_duration(StrategyAction::Crossover, now.elapsed());
    }
    fn require_crossover_indexes(&self) -> bool {
        true
    }
}

impl Uniform {
    pub fn new() -> Self {
        Self
    }
}
