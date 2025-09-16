use super::{Mutate, MutateEvent};
use crate::genotype::EvolveGenotype;
use crate::strategy::evolve::{EvolveConfig, EvolveState};
use std::marker::PhantomData;
use crate::strategy::{StrategyAction, StrategyReporter, StrategyState};
use rand::distributions::{Bernoulli, Distribution, Uniform};
use rand::Rng;
use std::cmp::Ordering;
use std::time::Instant;

/// Selects [Chromosomes](crate::chromosome::Chromosome) in the
/// [Population](crate::population::Population) with the dynamically updated mutation_probability.
/// Then mutates the selected chromosomes the provided number of times, where the
/// [Genotype](crate::genotype::Genotype) determines whether this is random, relative or scaled.
/// The mutation probability is dynamically increased or decreased to achieve a target population
/// cardinality.
///
/// Duplicate mutations of the same gene are avoided.
///
/// Useful when a single mutation would generally not lead to improvement, because the problem
/// space behaves more like a [UniqueGenotype](crate::genotype::UniqueGenotype) where genes must be
/// swapped (but the UniqueGenotype doesn't map to the problem space well). Set number_of_mutations
/// to two in that situation.
#[derive(Debug, Clone)]
pub struct MultiGeneDynamic<G: EvolveGenotype> {
    _phantom: PhantomData<G>,
    pub number_of_mutations: usize,
    pub mutation_probability: f32,
    pub mutation_probability_step: f32,
    pub target_cardinality: usize,
    pub number_of_mutations_sampler: Uniform<usize>,
}

impl<G: EvolveGenotype> Mutate for MultiGeneDynamic<G> {
    type Genotype = G;

    fn call<R: Rng, SR: StrategyReporter<Genotype = G>>(
        &mut self,
        genotype: &G,
        state: &mut EvolveState<G>,
        config: &EvolveConfig,
        reporter: &mut SR,
        rng: &mut R,
    ) {
        let now = Instant::now();

        if let Some(cardinality) = state.population_cardinality() {
            let changed = match cardinality.cmp(&self.target_cardinality) {
                Ordering::Greater => {
                    self.mutation_probability =
                        (self.mutation_probability - self.mutation_probability_step).max(0.0);
                    true
                }
                Ordering::Less => {
                    self.mutation_probability =
                        (self.mutation_probability + self.mutation_probability_step).min(1.0);
                    true
                }
                Ordering::Equal => false,
            };

            if changed {
                reporter.on_mutate_event(
                    MutateEvent::ChangeMutationProbability(format!(
                        "set to {:0.3}",
                        self.mutation_probability
                    )),
                    genotype,
                    state,
                    config,
                );
            }
        }

        let bool_sampler = Bernoulli::new(self.mutation_probability as f64).unwrap();
        for chromosome in state
            .population
            .chromosomes
            .iter_mut()
            .filter(|c| c.is_offspring())
        {
            if bool_sampler.sample(rng) {
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

impl<G: EvolveGenotype> MultiGeneDynamic<G> {
    pub fn new(
        number_of_mutations: usize,
        mutation_probability_step: f32,
        target_cardinality: usize,
    ) -> Self {
        let number_of_mutations_sampler = Uniform::from(1..=number_of_mutations);
        Self {
            _phantom: PhantomData,
            number_of_mutations,
            mutation_probability: 0.0,
            mutation_probability_step,
            target_cardinality,
            number_of_mutations_sampler,
        }
    }
}
