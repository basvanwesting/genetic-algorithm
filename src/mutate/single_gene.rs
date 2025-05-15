use super::Mutate;
use crate::chromosome::Chromosome;
use crate::genotype::EvolveGenotype;
use crate::strategy::evolve::{EvolveConfig, EvolveState};
use crate::strategy::{StrategyAction, StrategyReporter, StrategyState};
use rand::distributions::{Bernoulli, Distribution};
use rand::Rng;
use std::time::Instant;

/// Selects [Chromosomes](crate::chromosome::Chromosome) in the
/// [Population](crate::population::Population) with the provided mutation_probability. Then
/// mutates the selected chromosomes once, where the [Genotype](crate::genotype::Genotype)
/// determines whether this is random, relative or scaled.
#[derive(Debug, Clone)]
pub struct SingleGene {
    pub mutation_probability: f32,
    pub mutation_probability_sampler: Bernoulli,
}

impl Mutate for SingleGene {
    fn call<G: EvolveGenotype, R: Rng, SR: StrategyReporter<Genotype = G>>(
        &mut self,
        genotype: &mut G,
        state: &mut EvolveState<G>,
        _config: &EvolveConfig,
        _reporter: &mut SR,
        rng: &mut R,
    ) {
        let now = Instant::now();
        for chromosome in state
            .population
            .chromosomes
            .iter_mut()
            .filter(|c| c.is_offspring())
        {
            if self.mutation_probability_sampler.sample(rng) {
                genotype.mutate_chromosome_genes(
                    1,
                    true,
                    chromosome,
                    state.current_scale_index,
                    rng,
                );
            }
        }
        state.add_duration(StrategyAction::Mutate, now.elapsed());
    }
}

impl SingleGene {
    pub fn new(mutation_probability: f32) -> Self {
        let mutation_probability_sampler = Bernoulli::new(mutation_probability as f64).unwrap();
        Self {
            mutation_probability,
            mutation_probability_sampler,
        }
    }
}
