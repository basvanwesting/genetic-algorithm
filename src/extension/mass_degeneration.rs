use super::{Extension, ExtensionEvent};
use crate::genotype::Genotype;
use crate::strategy::evolve::{EvolveConfig, EvolveReporter, EvolveState};
use crate::strategy::{StrategyAction, StrategyState};
use rand::Rng;
use std::time::Instant;

/// Simulates a cambrian explosion. The controlling metric is fitness score cardinality in the
/// population. When this cardinality drops to the threshold, the full population is mutated the
/// provided number of times, where the [Genotype] determines whether this is random, relative or
/// scaled.
/// Duplicate mutations of the same gene are allowed. There is no change in population size.
#[derive(Debug, Clone)]
pub struct MassDegeneration {
    pub cardinality_threshold: usize,
    pub number_of_mutations: usize,
}

impl Extension for MassDegeneration {
    fn call<G: Genotype, R: Rng, SR: EvolveReporter<Genotype = G>>(
        &mut self,
        genotype: &G,
        state: &mut EvolveState<G>,
        config: &EvolveConfig,
        reporter: &mut SR,
        rng: &mut R,
    ) {
        let now = Instant::now();
        if state.population.size() >= config.target_population_size
            && state.population.fitness_score_cardinality() <= self.cardinality_threshold
        {
            reporter.on_extension_event(
                ExtensionEvent::MassDegeneration("".to_string()),
                state,
                config,
            );
            for chromosome in state.population.chromosomes.iter_mut() {
                genotype.mutate_chromosome_genes(
                    self.number_of_mutations,
                    true,
                    chromosome,
                    state.current_scale_index,
                    rng,
                );
            }
        }
        state.add_duration(StrategyAction::Extension, now.elapsed());
    }
}

impl MassDegeneration {
    pub fn new(cardinality_threshold: usize, number_of_rounds: usize) -> Self {
        Self {
            cardinality_threshold,
            number_of_mutations: number_of_rounds,
        }
    }
}
