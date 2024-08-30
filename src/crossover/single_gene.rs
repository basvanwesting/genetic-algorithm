use super::Crossover;
use crate::genotype::Genotype;
use crate::strategy::evolve::{EvolveConfig, EvolveReporter, EvolveState};
use crate::strategy::StrategyState;
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
        if state.population.size() < 2 {
            return;
        }
        let mut parent_chromosomes = if self.keep_parent {
            state.population.chromosomes.clone()
        } else {
            vec![] // throwaway to keep compiler happy
        };

        for (father, mother) in state.population.chromosomes.iter_mut().tuples() {
            genotype.crossover_chromosome_pair_single_gene(father, mother, rng);
        }

        if self.keep_parent {
            state.population.chromosomes.append(&mut parent_chromosomes);
        }
        state.add_duration("crossover", now.elapsed());
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
