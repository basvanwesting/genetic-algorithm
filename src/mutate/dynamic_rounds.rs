use crate::genotype::Genotype;
use crate::population::Population;
use rand::distributions::{Bernoulli, Distribution};
use rand::Rng;

/// Selects [Chromosomes](crate::chromosome::Chromosome) in the [Population] with the provided
/// mutation_probability. Repeatedly (number of rounds) mutates the selected chromosomes once using
/// random mutation. The number of rounds is dynamically increased or decreased to achieve a target
/// population uniformity
#[derive(Debug, Clone, Default)]
pub struct DynamicRounds {
    pub mutation_probability: f32,
    pub number_of_rounds: usize,
    pub target_uniformity: f32,
}

impl DynamicRounds {
    pub fn call<T: Genotype, R: Rng>(
        &mut self,
        genotype: &T,
        population: &mut Population<T>,
        rng: &mut R,
    ) {
        if population.fitness_score_uniformity() > self.target_uniformity {
            self.number_of_rounds += 1
        } else if self.number_of_rounds > 0 {
            self.number_of_rounds -= 1
        }
        log::trace!(
            "### dynamic_rounds mutation probability: {}, rounds: {}",
            self.mutation_probability,
            self.number_of_rounds
        );

        let bool_sampler = Bernoulli::new(self.mutation_probability as f64).unwrap();
        for chromosome in population.chromosomes.iter_mut().filter(|c| c.age == 0) {
            for _ in 0..self.number_of_rounds {
                if bool_sampler.sample(rng) {
                    genotype.mutate_chromosome_random(chromosome, rng);
                }
            }
        }
    }
    pub fn report(&self) -> String {
        format!("rounds: {}", self.number_of_rounds)
    }
    pub fn new(mutation_probability: f32, target_uniformity: f32) -> Self {
        Self {
            mutation_probability,
            target_uniformity,
            ..Default::default()
        }
    }
}
