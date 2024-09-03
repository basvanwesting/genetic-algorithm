use super::Mutate;
use crate::genotype::Genotype;
use crate::strategy::evolve::{EvolveConfig, EvolveReporter, EvolveState};
use crate::strategy::{StrategyAction, StrategyState};
use rand::distributions::{Bernoulli, Distribution};
use rand::Rng;
use std::time::Instant;

/// Selects [Chromosomes](crate::chromosome::Chromosome) in the
/// [Population](crate::population::Population) with the provided mutation_probability. Then
/// mutates the selected chromosomes the provided number of times, where the [Genotype] determines
/// whether this is random, relative or scaled.
///
/// Duplicate mutations of the same gene are allowed, as disallowing duplicates is relatively expensive
/// and mutations should be quite small, so there is little chance for conflict.
///
/// Useful when a single mutation
/// would generally not lead to improvement, because the problem space behaves more like a
/// [UniqueGenotype](crate::genotype::UniqueGenotype) where genes must be swapped (but the
/// UniqueGenotype doesn't map to the problem space well). Set number_of_mutations to two in that
/// situation.
#[derive(Debug, Clone)]
pub struct MultiGene {
    pub number_of_mutations: usize,
    pub mutation_probability: f32,
}

impl Mutate for MultiGene {
    fn call<G: Genotype, R: Rng, SR: EvolveReporter<Genotype = G>>(
        &mut self,
        genotype: &G,
        state: &mut EvolveState<G>,
        _config: &EvolveConfig,
        _reporter: &mut SR,
        rng: &mut R,
    ) {
        let now = Instant::now();
        let bool_sampler = Bernoulli::new(self.mutation_probability as f64).unwrap();
        for chromosome in state
            .population
            .chromosomes
            .iter_mut()
            .filter(|c| c.age == 0)
        {
            if bool_sampler.sample(rng) {
                genotype.mutate_chromosome_genes(
                    self.number_of_mutations,
                    true,
                    chromosome,
                    state.current_scale_index,
                    rng,
                );
            }
        }
        state.add_duration(StrategyAction::Mutate, now.elapsed());
    }
    fn report(&self) -> String {
        format!(
            "multi-gene-random: {}, {:2.2}",
            self.number_of_mutations, self.mutation_probability,
        )
    }
}

impl MultiGene {
    pub fn new(number_of_mutations: usize, mutation_probability: f32) -> Self {
        Self {
            number_of_mutations,
            mutation_probability,
        }
    }
}
