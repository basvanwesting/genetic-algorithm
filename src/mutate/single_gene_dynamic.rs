use super::{Mutate, MutateEvent};
use crate::chromosome::Chromosome;
use crate::genotype::EvolveGenotype;
use crate::strategy::evolve::{EvolveConfig, EvolveState};
use crate::strategy::StrategyReporter;
use crate::strategy::{StrategyAction, StrategyState};
use rand::distributions::{Bernoulli, Distribution};
use rand::Rng;
use std::cmp::Ordering;
use std::time::Instant;

/// Selects [Chromosomes](crate::chromosome::Chromosome) in the
/// [Population](crate::population::Population) with the dynamically updated mutation_probability.
/// Then mutates the selected chromosomes once, where the [Genotype](crate::genotype::Genotype)
/// determines whether this is random, relative or scaled. The mutation probability is dynamically
/// increased or decreased to achieve a target population cardinality
#[derive(Debug, Clone, Default)]
pub struct SingleGeneDynamic {
    pub mutation_probability: f32,
    pub mutation_probability_step: f32,
    pub target_cardinality: usize,
}

impl Mutate for SingleGeneDynamic {
    fn call<G: EvolveGenotype, R: Rng, SR: StrategyReporter<Genotype = G>>(
        &mut self,
        genotype: &mut G,
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

impl SingleGeneDynamic {
    pub fn new(mutation_probability_step: f32, target_cardinality: usize) -> Self {
        Self {
            mutation_probability_step,
            target_cardinality,
            ..Default::default()
        }
    }
}
