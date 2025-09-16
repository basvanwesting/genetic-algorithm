use super::Mutate;
use crate::genotype::EvolveGenotype;
use crate::strategy::evolve::{EvolveConfig, EvolveState};
use crate::strategy::{StrategyAction, StrategyReporter, StrategyState};
use rand::distributions::{Bernoulli, Distribution, Uniform};
use rand::Rng;
use std::marker::PhantomData;
use std::time::Instant;

/// Selects [Chromosomes](crate::chromosome::Chromosome) in the
/// [Population](crate::population::Population) with the provided mutation_probability. Then
/// mutates the selected chromosomes the provided number of times, where the
/// [Genotype](crate::genotype::Genotype) determines whether this is random, relative or scaled.
///
/// Duplicate mutations of the same gene are avoided.
///
/// Useful when a single mutation would generally not lead to improvement, because the problem
/// space behaves more like a [UniqueGenotype](crate::genotype::UniqueGenotype) where genes must be
/// swapped (but the UniqueGenotype doesn't map to the problem space well). Set number_of_mutations
/// to two in that situation.
#[derive(Debug, Clone)]
pub struct MultiGene<G: EvolveGenotype> {
    _phantom: PhantomData<G>,
    pub number_of_mutations: usize,
    pub mutation_probability: f32,
    pub number_of_mutations_sampler: Uniform<usize>,
    pub mutation_probability_sampler: Bernoulli,
}

impl<G: EvolveGenotype> Mutate for MultiGene<G> {
    type Genotype = G;

    fn call<R: Rng, SR: StrategyReporter<Genotype = G>>(
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
                    self.number_of_mutations,
                    false,
                    chromosome,
                    state.current_scale_index,
                    rng,
                );
            }
        }
        state.add_duration(StrategyAction::Mutate, now.elapsed());
    }
}

impl<G: EvolveGenotype> MultiGene<G> {
    pub fn new(number_of_mutations: usize, mutation_probability: f32) -> Self {
        let number_of_mutations_sampler = Uniform::from(1..=number_of_mutations);
        let mutation_probability_sampler = Bernoulli::new(mutation_probability as f64).unwrap();
        Self {
            _phantom: PhantomData,
            number_of_mutations,
            mutation_probability,
            number_of_mutations_sampler,
            mutation_probability_sampler,
        }
    }
}
