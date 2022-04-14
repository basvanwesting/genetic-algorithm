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

    pub fn calculate_fitness(&mut self, context: &Context<T>) {
        self.chromosomes
            .iter_mut()
            .for_each(|o| o.fitness = Some((context.fitness_function)(o)));
    }
}
