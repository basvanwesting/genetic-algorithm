use super::{Extension, ExtensionEvent};
use crate::chromosome::{Chromosome, GenesHash};
use crate::genotype::EvolveGenotype;
use crate::strategy::evolve::{EvolveConfig, EvolveState};
use crate::strategy::{StrategyAction, StrategyReporter, StrategyState};
use rand::Rng;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::time::Instant;

/// Simulates a cambrian explosion. The controlling metric is population cardinality in the
/// population after selection. When this cardinality drops to the threshold, the population is
/// reduced to only the unique individuals. Only works when genes_hash is stored on chromosome, as
/// this is the uniqueness key, otherwise the extension is ignored.
///
/// Population will recover in the following generations
#[derive(Debug, Clone)]
pub struct MassDeduplication {
    pub cardinality_threshold: usize,
}

impl Extension for MassDeduplication {
    fn call<G: EvolveGenotype, R: Rng, SR: StrategyReporter<Genotype = G>>(
        &mut self,
        genotype: &mut G,
        state: &mut EvolveState<G>,
        config: &EvolveConfig,
        reporter: &mut SR,
        _rng: &mut R,
    ) {
        if genotype.genes_hashing() && state.population.size() >= config.target_population_size {
            let now = Instant::now();
            // ensures min 1
            if let Some(cardinality) = state.population_cardinality() {
                if cardinality <= self.cardinality_threshold {
                    reporter.on_extension_event(
                        ExtensionEvent::MassDeduplication("".to_string()),
                        genotype,
                        state,
                        config,
                    );

                    let mut selected_chromosomes: HashMap<GenesHash, G::Chromosome> =
                        HashMap::new();
                    state
                        .population
                        .chromosomes
                        .drain(..)
                        .for_each(|chromosome| {
                            if let Some(genes_hash) = chromosome.genes_hash() {
                                match selected_chromosomes.entry(genes_hash) {
                                    Entry::Occupied(_) => {
                                        genotype.chromosome_destructor(chromosome);
                                    }
                                    Entry::Vacant(entry) => {
                                        entry.insert(chromosome);
                                    }
                                }
                            } else {
                                genotype.chromosome_destructor(chromosome);
                            }
                        });

                    state
                        .population
                        .chromosomes
                        .extend(selected_chromosomes.into_values());

                    // ensures min 2
                    if state.population.size() == 1 {
                        genotype.chromosome_cloner_expand(&mut state.population.chromosomes, 1);
                    }
                }
            }
            state.add_duration(StrategyAction::Extension, now.elapsed());
        }
    }
}

impl MassDeduplication {
    pub fn new(cardinality_threshold: usize) -> Self {
        Self {
            cardinality_threshold,
        }
    }
}
