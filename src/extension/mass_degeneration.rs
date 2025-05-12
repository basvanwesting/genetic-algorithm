use super::{Extension, ExtensionEvent};
use crate::genotype::EvolveGenotype;
use crate::strategy::evolve::{EvolveConfig, EvolveState};
use crate::strategy::{StrategyAction, StrategyReporter, StrategyState};
use rand::Rng;
use std::time::Instant;

/// Simulates a cambrian explosion. The controlling metric is population cardinality in the
/// population after selection. When this cardinality drops to the threshold, the population is
/// mutated the provided number of times, where the [Genotype](crate::genotype::Genotype)
/// determines whether this is random, relative or scaled.
/// The elitism_rate ensures the passing of the best chromosomes before mutations are applied.
///
/// Duplicate mutations of the same gene are allowed. There is no change in population size.
#[derive(Debug, Clone)]
pub struct MassDegeneration {
    pub cardinality_threshold: usize,
    pub number_of_mutations: usize,
    pub elitism_rate: f32,
}

impl Extension for MassDegeneration {
    fn call<G: EvolveGenotype, R: Rng, SR: StrategyReporter<Genotype = G>>(
        &mut self,
        genotype: &mut G,
        state: &mut EvolveState<G>,
        config: &EvolveConfig,
        reporter: &mut SR,
        rng: &mut R,
    ) {
        if state.population.size() >= config.target_population_size {
            let now = Instant::now();
            if let Some(cardinality) = state.population_cardinality() {
                if cardinality <= self.cardinality_threshold {
                    reporter.on_extension_event(
                        ExtensionEvent::MassDegeneration("".to_string()),
                        genotype,
                        state,
                        config,
                    );
                    let population_size = state.population.size();

                    let elitism_size = ((population_size as f32 * self.elitism_rate).ceil()
                        as usize)
                        .min(population_size);
                    let mut elite_chromosomes = self.extract_unique_elite_chromosomes(
                        genotype,
                        state,
                        config,
                        elitism_size,
                    );
                    let elitism_size = elite_chromosomes.len();

                    for chromosome in state.population.chromosomes.iter_mut() {
                        genotype.mutate_chromosome_genes(
                            self.number_of_mutations,
                            true,
                            chromosome,
                            state.current_scale_index,
                            rng,
                        );
                    }

                    state.population.chromosomes.append(&mut elite_chromosomes);
                    // move back to front, elite_chromosomes internally unordered
                    for i in 0..elitism_size {
                        state
                            .population
                            .chromosomes
                            .swap(i, population_size - 1 - i);
                    }
                }
            }
            state.add_duration(StrategyAction::Extension, now.elapsed());
        }
    }
}

impl MassDegeneration {
    pub fn new(cardinality_threshold: usize, number_of_rounds: usize, elitism_rate: f32) -> Self {
        Self {
            cardinality_threshold,
            number_of_mutations: number_of_rounds,
            elitism_rate,
        }
    }
}
