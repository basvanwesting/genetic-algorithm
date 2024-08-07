use super::{Extension, ExtensionEvent};
use crate::genotype::Genotype;
use crate::strategy::evolve::{EvolveConfig, EvolveReporter, EvolveState};
use rand::distributions::{Bernoulli, Distribution};
use rand::Rng;

/// A version of [MassExtinction](crate::extension::ExtensionMassExtinction), where the extinct population is replaced by a random population
#[derive(Debug, Clone)]
pub struct MassInvasion {
    pub cardinality_threshold: usize,
    pub survival_rate: f32,
}

impl Extension for MassInvasion {
    fn call<G: Genotype, R: Rng, SR: EvolveReporter<Genotype = G>>(
        &mut self,
        genotype: &G,
        state: &mut EvolveState<G>,
        config: &EvolveConfig,
        reporter: &mut SR,
        rng: &mut R,
    ) {
        if state.population.size() >= config.target_population_size
            && state.population.fitness_score_cardinality() <= self.cardinality_threshold
        {
            reporter.on_extension_event(state, ExtensionEvent::MassInvasion("".to_string()));
            let bool_sampler = Bernoulli::new(self.survival_rate as f64).unwrap();
            for chromosome in &mut state.population.chromosomes {
                if !bool_sampler.sample(rng) {
                    chromosome.genes = genotype.random_genes_factory(rng);
                    chromosome.taint_fitness_score();
                }
            }
        }
    }
}

impl MassInvasion {
    pub fn new(cardinality_threshold: usize, survival_rate: f32) -> Self {
        Self {
            cardinality_threshold,
            survival_rate,
        }
    }
}
