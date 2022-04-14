use crate::chromosome::Chromosome;
use crate::context::Context;
use crate::gene::Gene;

#[derive(Debug)]
pub struct Population<T: Gene> {
    pub chromosomes: Vec<Chromosome<T>>,
}

impl<T: Gene> Population<T> {
    pub fn new(chromosomes: Vec<Chromosome<T>>) -> Self {
        Self {
            chromosomes: chromosomes,
        }
    }

    pub fn merge(&mut self, other: &mut Self) {
        self.chromosomes.append(&mut other.chromosomes);
    }

    pub fn sort(&mut self) {
        self.chromosomes.sort_unstable_by_key(|c| c.fitness);
    }

    pub fn size(&self) -> usize {
        self.chromosomes.len()
    }

    pub fn best_chromosome(&self) -> Option<&Chromosome<T>> {
        self.chromosomes.last()
    }

    pub fn uniformity(&self, context: &Context<T>, best_chromosome: &Chromosome<T>) -> f32 {
        if let Some(best_fitness) = best_chromosome.fitness {
            let number_of_best_chromosomes = self
                .chromosomes
                .iter()
                .filter(|c| c.fitness == Some(best_fitness))
                .count();

            number_of_best_chromosomes as f32 / context.population_size as f32
        } else {
            0.0
        }
    }

    pub fn mass_extinction(&mut self, keep_population_size: usize) {
        if self.size() > keep_population_size {
            self.chromosomes
                .drain(..(self.size() - keep_population_size));
        }
    }

    pub fn calculate_fitness(&mut self, context: &Context<T>) {
        self.chromosomes
            .iter_mut()
            .for_each(|o| o.fitness = Some((context.fitness_function)(o)));
    }
}
