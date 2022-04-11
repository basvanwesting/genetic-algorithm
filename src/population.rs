use crate::chromosome::Chromosome;
use crate::context::Context;

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

    pub fn random_factory(context: &Context) -> Self {
        let chromosomes = (0..context.population_size)
            .map(|_| context.random_chromosome_factory())
            .collect();
        Self::new(chromosomes)
    }
}
