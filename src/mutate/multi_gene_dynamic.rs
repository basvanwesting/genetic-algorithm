use super::{Mutate, MutateEvent};
use crate::chromosome::Chromosome;
use crate::genotype::Genotype;
use crate::strategy::evolve::{EvolveConfig, EvolveState};
use crate::strategy::{StrategyAction, StrategyReporter, StrategyState};
use rand::distributions::{Bernoulli, Distribution};
use rand::Rng;
use std::time::Instant;

/// Selects [Chromosomes](crate::chromosome::Chromosome) in the
/// [Population](crate::population::Population) with the dynamically updated mutation_probability.
/// Then mutates the selected chromosomes the provided number of times, where the [Genotype]
/// determines whether this is random, relative or scaled. The mutation probability is dynamically
/// increased or decreased to achieve a target population cardinality.
///
/// Duplicate mutations of the same gene are allowed, as disallowing duplicates is relatively expensive
/// and mutations should be quite small, so there is little chance for conflict.
///
/// Useful when a single
/// mutation would generally not lead to improvement, because the problem space behaves more like a
/// [UniqueGenotype](crate::genotype::UniqueGenotype) where genes must be swapped (but the
/// UniqueGenotype doesn't map to the problem space well). Set number_of_mutations to two in that
/// situation.
#[derive(Debug, Clone, Default)]
pub struct MultiGeneDynamic {
    pub number_of_mutations: usize,
    pub mutation_probability: f32,
    pub mutation_probability_step: f32,
    pub target_cardinality: usize,
}

impl Mutate for MultiGeneDynamic {
    fn call<G: Genotype, R: Rng, SR: StrategyReporter<Genotype = G>>(
        &mut self,
        genotype: &mut G,
        state: &mut EvolveState<G>,
        config: &EvolveConfig,
        reporter: &mut SR,
        rng: &mut R,
    ) {
        let now = Instant::now();
        if state.population.fitness_score_cardinality() < self.target_cardinality {
            self.mutation_probability =
                (self.mutation_probability + self.mutation_probability_step).min(1.0);
        } else {
            self.mutation_probability =
                (self.mutation_probability - self.mutation_probability_step).max(0.0);
        }
        reporter.on_mutate_event(
            MutateEvent::ChangeMutationProbability(format!(
                "set to {:0.3}",
                self.mutation_probability
            )),
            genotype,
            state,
            config,
        );

        let bool_sampler = Bernoulli::new(self.mutation_probability as f64).unwrap();
        for chromosome in state
            .population
            .chromosomes
            .iter_mut()
            .filter(|c| c.age() == 0)
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
}

impl MultiGeneDynamic {
    pub fn new(
        number_of_mutations: usize,
        mutation_probability_step: f32,
        target_cardinality: usize,
    ) -> Self {
        Self {
            number_of_mutations,
            mutation_probability_step,
            target_cardinality,
            ..Default::default()
        }
    }
}
