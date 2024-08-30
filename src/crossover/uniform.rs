use super::Crossover;
use crate::genotype::Genotype;
use crate::strategy::evolve::{EvolveConfig, EvolveReporter, EvolveState};
use itertools::Itertools;
use rand::Rng;
use std::time::Instant;

/// Crossover with 50% probability for each gene to come from one of the two parents.
/// Optionally keep parents around to compete with children later on.
///
/// Actually implemented as `CrossoverMultiGene::new(<genes_size> / 2, allow_duplicates=true, keep_parent)`
///
/// Not allowed for [UniqueGenotype](crate::genotype::UniqueGenotype) and
/// [MultiUniqueGenotype](crate::genotype::MultiUniqueGenotype) as it would not preserve the gene
/// uniqueness in the children.
#[derive(Clone, Debug)]
pub struct Uniform {
    pub keep_parent: bool,
}
impl Crossover for Uniform {
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

        let number_of_crossovers = genotype.genes_size() / 2;
        for (father, mother) in state.population.chromosomes.iter_mut().tuples() {
            genotype.crossover_chromosome_pair_multi_gene(
                number_of_crossovers,
                true,
                father,
                mother,
                rng,
            );
        }

        if self.keep_parent {
            state.population.chromosomes.append(&mut parent_chromosomes);
        }
        *state.durations.entry("crossover").or_default() += now.elapsed();
    }
    fn require_crossover_indexes(&self) -> bool {
        true
    }
}

impl Uniform {
    pub fn new(keep_parent: bool) -> Self {
        Self { keep_parent }
    }
}
