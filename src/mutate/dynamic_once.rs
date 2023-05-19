use super::{Mutate, MutateDispatch, Mutates, MutationProbability, TargetUniformity};
use crate::genotype::Genotype;
use crate::population::Population;
use rand::distributions::{Bernoulli, Distribution};
use rand::Rng;

/// Selects [Chromosomes](crate::chromosome::Chromosome) in the [Population] with the provided
/// [MutationProbability]. Then mutates the selected chromosomes once using random mutation. The
/// mutation probability is dynamically increased or decreased to achieve a target population uniformity
#[derive(Debug, Clone, Default)]
pub struct DynamicOnce {
    pub mutation_probability: f32,
    mutation_probability_step: f32,
    target_uniformity: f32,
}
impl Mutate for DynamicOnce {
    fn call<T: Genotype, R: Rng>(
        &mut self,
        genotype: &T,
        population: &mut Population<T>,
        rng: &mut R,
    ) {
        if population.fitness_score_uniformity() > self.target_uniformity {
            self.mutation_probability += self.mutation_probability_step;
        } else if self.mutation_probability > self.mutation_probability_step {
            self.mutation_probability -= self.mutation_probability_step;
        }
        log::debug!(
            "### dynamic_once mutation probability: {}",
            self.mutation_probability
        );

        let bool_sampler = Bernoulli::new(self.mutation_probability as f64).unwrap();
        for chromosome in &mut population.chromosomes {
            if bool_sampler.sample(rng) {
                genotype.mutate_chromosome_random(chromosome, rng);
            }
        }
    }
}

impl DynamicOnce {
    pub fn new(mutation_probability_step: f32, target_uniformity: f32) -> Self {
        Self {
            mutation_probability_step,
            target_uniformity,
            ..Default::default()
        }
    }

    pub fn new_dispatch(mutation_probability_step: f32, target_uniformity: f32) -> MutateDispatch {
        MutateDispatch {
            mutate: Mutates::DynamicOnce,
            mutation_probability_step,
            target_uniformity,
            ..Default::default()
        }
    }
}
