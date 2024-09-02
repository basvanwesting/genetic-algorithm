use super::Crossover;
use crate::genotype::Genotype;
use crate::strategy::evolve::{EvolveConfig, EvolveReporter, EvolveState};
use crate::strategy::{StrategyAction, StrategyState};
use itertools::Itertools;
use rand::Rng;
use std::time::Instant;

/// Crossover a single gene between the parents. The gene position is chosen with uniform
/// probability. Optionally keep parents around to compete with children later on.
///
/// Not allowed for [UniqueGenotype](crate::genotype::UniqueGenotype) and
/// [MultiUniqueGenotype](crate::genotype::MultiUniqueGenotype) as it would not preserve the gene
/// uniqueness in the children.
#[derive(Clone, Debug)]
pub struct SingleGene {
    pub keep_parent: bool,
}
impl Crossover for SingleGene {
    fn call<G: Genotype, R: Rng, SR: EvolveReporter<Allele = G::Allele>>(
        &mut self,
        genotype: &G,
        state: &mut EvolveState<G::Allele>,
        _config: &EvolveConfig,
        _reporter: &mut SR,
        rng: &mut R,
    ) {
        let now = Instant::now();
        let population_size = state.population.size();
        if population_size < 2 {
            return;
        }
        if self.keep_parent {
            state
                .population
                .chromosomes
                .extend_from_within(..population_size);
        };

        for (father, mother) in state
            .population
            .chromosomes
            .iter_mut()
            .take(population_size)
            .tuples()
        {
            genotype.crossover_chromosome_pair_single_gene(father, mother, rng);
        }

        state.add_duration(StrategyAction::Crossover, now.elapsed());
    }
    fn require_crossover_indexes(&self) -> bool {
        true
    }
}

impl SingleGene {
    pub fn new(keep_parent: bool) -> Self {
        Self { keep_parent }
    }
}
