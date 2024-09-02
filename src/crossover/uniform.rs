use super::Crossover;
use crate::genotype::Genotype;
use crate::strategy::evolve::{EvolveConfig, EvolveReporter, EvolveState};
use crate::strategy::{StrategyAction, StrategyState};
use itertools::Itertools;
use rand::Rng;
use std::time::Instant;

/// Crossover with 50% probability for each gene to come from one of the two parents.
/// Optionally keep a percentage of the parents around to compete with children later on.
///
/// Actually implemented as `CrossoverMultiGene::new(<genes_size> / 2, allow_duplicates=true, parent_survival_rate)`
///
/// Not allowed for [UniqueGenotype](crate::genotype::UniqueGenotype) and
/// [MultiUniqueGenotype](crate::genotype::MultiUniqueGenotype) as it would not preserve the gene
/// uniqueness in the children.
#[derive(Clone, Debug)]
pub struct Uniform {
    pub parent_survival_rate: f32,
}
impl Crossover for Uniform {
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

        let number_of_crossovers = genotype.genes_size() / 2;
        for (father, mother) in state
            .population
            .chromosomes
            .iter_mut()
            .take(population_size)
            .tuples()
        {
            genotype.crossover_chromosome_pair_multi_gene(
                number_of_crossovers,
                true,
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

impl Uniform {
    pub fn new(parent_survival_rate: f32) -> Self {
        Self {
            parent_survival_rate,
        }
    }
}
