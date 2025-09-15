use super::Mutate;
use crate::distributed::genotype::EvolveGenotype;
use crate::distributed::strategy::evolve::{EvolveConfig, EvolveState};
use crate::distributed::strategy::{StrategyAction, StrategyReporter, StrategyState};
use rand::distributions::{Bernoulli, Distribution, Uniform};
use rand::Rng;
use std::ops::RangeInclusive;
use std::time::Instant;

/// Selects [Chromosomes](crate::chromosome::Chromosome) in the
/// [Population](crate::population::Population) with the provided mutation_probability. Then
/// mutates the selected chromosomes a number of times (sampled uniform from the provided
/// number_of_mutations_range), where the [Genotype](crate::genotype::Genotype) determines whether
/// this is random, relative or scaled.
///
/// Duplicate mutations of the same gene are allowed, as disallowing duplicates is relatively expensive
/// and mutations should be quite small, so there is little chance for conflict.
#[derive(Debug, Clone)]
pub struct MultiGeneRange {
    pub number_of_mutations_range: RangeInclusive<usize>,
    pub mutation_probability: f32,
    pub number_of_mutations_sampler: Uniform<usize>,
    pub mutation_probability_sampler: Bernoulli,
}

impl Mutate for MultiGeneRange {
    fn call<G: EvolveGenotype, R: Rng, SR: StrategyReporter<Genotype = G>>(
        &mut self,
        genotype: &G,
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
                    self.number_of_mutations_sampler.sample(rng),
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

impl MultiGeneRange {
    pub fn new(
        number_of_mutations_range: RangeInclusive<usize>,
        mutation_probability: f32,
    ) -> Self {
        let number_of_mutations_sampler = Uniform::from(number_of_mutations_range.clone());
        let mutation_probability_sampler = Bernoulli::new(mutation_probability as f64).unwrap();
        Self {
            number_of_mutations_range,
            mutation_probability,
            number_of_mutations_sampler,
            mutation_probability_sampler,
        }
    }
}
