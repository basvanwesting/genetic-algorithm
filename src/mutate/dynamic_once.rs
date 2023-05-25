use super::Mutate;
use crate::genotype::Genotype;
use crate::population::Population;
use rand::distributions::{Bernoulli, Distribution};
use rand::Rng;

/// Selects [Chromosomes](crate::chromosome::Chromosome) in the [Population] with the provided
/// mutation_probability. Then mutates the selected chromosomes once using random mutation. The
/// mutation probability is dynamically increased or decreased to achieve a target population uniformity
#[derive(Debug, Clone, Default)]
pub struct DynamicOnce {
    pub mutation_probability: f32,
    pub mutation_probability_step: f32,
    pub target_uniformity: f32,
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
        log::trace!(
            "### dynamic_once mutation probability: {}",
            self.mutation_probability
        );

        let bool_sampler = Bernoulli::new(self.mutation_probability as f64).unwrap();
        for chromosome in population.chromosomes.iter_mut().filter(|c| c.age == 0) {
            if bool_sampler.sample(rng) {
                genotype.mutate_chromosome_random(chromosome, rng);
            }
        }
    }
    fn report(&self) -> String {
        format!("once: {:2.2}", self.mutation_probability)
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
}
