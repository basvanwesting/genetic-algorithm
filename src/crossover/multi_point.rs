use super::Crossover;
use crate::genotype::Genotype;
use crate::strategy::evolve::{EvolveConfig, EvolveReporter, EvolveState};
use crate::strategy::StrategyState;
use itertools::Itertools;
use rand::Rng;
use std::time::Instant;

/// Crossover multiple gene positions from which on the rest of the genes are taken from the other
/// parent. This goes back and forth. The gene positions are chosen with uniform probability.
/// Choose between allowing duplicate crossovers on the same point or not (not much slower, as
/// crossover itself is relatively expensive). Optionally keep parents around to compete with
/// children later on.
///
/// Not allowed for [UniqueGenotype](crate::genotype::UniqueGenotype) as it would not preserve the gene uniqueness in the children.
/// Allowed for [MultiUniqueGenotype](crate::genotype::MultiUniqueGenotype) as there are valid crossover points between each new set
#[derive(Clone, Debug)]
pub struct MultiPoint {
    pub number_of_crossovers: usize,
    pub allow_duplicates: bool,
    pub keep_parent: bool,
}
impl Crossover for MultiPoint {
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
            genotype.crossover_chromosome_pair_multi_point(
                self.number_of_crossovers,
                self.allow_duplicates,
                father,
                mother,
                rng,
            );
        }

        if self.keep_parent {
            state.population.chromosomes.append(&mut parent_chromosomes);
        }
        state.add_duration("crossover", now.elapsed());
    }
    fn require_crossover_points(&self) -> bool {
        true
    }
}

impl MultiPoint {
    pub fn new(number_of_crossovers: usize, allow_duplicates: bool, keep_parent: bool) -> Self {
        Self {
            number_of_crossovers,
            allow_duplicates,
            keep_parent,
        }
    }
}
