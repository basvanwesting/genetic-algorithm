use crate::chromosome::Chromosome;
use crate::context::Context;
use crate::fitness;

#[derive(Debug)]
pub struct Population {
    pub chromosomes: Vec<Chromosome>,
}

impl Population {
    pub fn new(chromosomes: Vec<Chromosome>) -> Self {
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

    pub fn calculate_fitness(&mut self) {
        self.chromosomes
            .iter_mut()
            .for_each(|o| o.fitness = Some(fitness::simple_sum(o)));
    }

    pub fn random_factory(context: &Context) -> Self {
        let chromosomes = (0..context.population_size)
            .map(|_| context.random_chromosome_factory())
            .collect();
        Self::new(chromosomes)
    }
}
