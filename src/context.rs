use crate::chromosome::Chromosome;
use rand::prelude::*;

pub struct Context {
    pub gene_size: usize,
    pub population_size: usize,
}

impl Context {
    pub fn new(gene_size: usize, population_size: usize) -> Self {
        Self {
            gene_size: gene_size,
            population_size: population_size,
        }
    }

    pub fn random_chromosome_factory(&self) -> Chromosome {
        //let mut genes: Vec<bool> = (0..self.gene_size).map(|_| rng.gen()).collect();
        let genes: Vec<bool> = rand::thread_rng()
            .sample_iter(rand::distributions::Standard)
            .take(self.gene_size)
            .collect();

        Chromosome::new(genes)
    }
}
