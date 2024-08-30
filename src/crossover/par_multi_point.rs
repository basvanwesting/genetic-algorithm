use super::Crossover;
use crate::genotype::Genotype;
use crate::strategy::evolve::{EvolveConfig, EvolveReporter, EvolveState};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use rayon::prelude::*;
use std::time::Instant;

/// Multithreaded version of [CrossoverMultiPoint](super::CrossoverMultiPoint) as it is the worst
/// performing crossover. Only more efficient for large genes_sizes and number_of_crossovers, so
/// don't just default to this version. It is more of an implementation example.
///
/// Not allowed for [UniqueGenotype](crate::genotype::UniqueGenotype) as it would not preserve the gene uniqueness in the children.
/// Allowed for [MultiUniqueGenotype](crate::genotype::MultiUniqueGenotype) as there are valid crossover points between each new set
#[derive(Clone, Debug)]
pub struct ParMultiPoint {
    pub number_of_crossovers: usize,
    pub allow_duplicates: bool,
    pub keep_parent: bool,
}
impl Crossover for ParMultiPoint {
    fn call<G: Genotype, R: Rng, SR: EvolveReporter<Allele = G::Allele>>(
        &mut self,
        genotype: &G,
        state: &mut EvolveState<G::Allele>,
        _config: &EvolveConfig,
        _reporter: &mut SR,
        _rng: &mut R,
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

        state
            .population
            .chromosomes
            .par_chunks_mut(2)
            .for_each_init(
                || SmallRng::from_rng(rand::thread_rng()).unwrap(),
                |rng, chunk| {
                    if let [father, mother] = chunk {
                        genotype.crossover_chromosome_pair_multi_point(
                            self.number_of_crossovers,
                            self.allow_duplicates,
                            father,
                            mother,
                            rng,
                        );
                    }
                },
            );

        if self.keep_parent {
            state.population.chromosomes.append(&mut parent_chromosomes);
        }
        *state.durations.entry("crossover").or_default() += now.elapsed();
    }
    fn require_crossover_points(&self) -> bool {
        true
    }
}

impl ParMultiPoint {
    pub fn new(number_of_crossovers: usize, allow_duplicates: bool, keep_parent: bool) -> Self {
        Self {
            number_of_crossovers,
            allow_duplicates,
            keep_parent,
        }
    }
}
