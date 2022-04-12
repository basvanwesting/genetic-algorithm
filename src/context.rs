use crate::chromosome::Chromosome;
use rand::prelude::*;

pub struct Context {
    pub gene_size: usize,
    pub population_size: usize,
    pub tournament_size: usize,
}

impl Context {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_gene_size(mut self, gene_size: usize) -> Self {
        self.gene_size = gene_size;
        self
    }

    pub fn with_population_size(mut self, population_size: usize) -> Self {
        self.population_size = population_size;
        self
    }

    pub fn with_tournament_size(mut self, tournament_size: usize) -> Self {
        self.tournament_size = tournament_size;
        self
    }

    // defined here because needs to know gene type bool
    pub fn random_chromosome_factory(&self) -> Chromosome {
        //let mut genes: Vec<bool> = (0..self.gene_size).map(|_| rng.gen()).collect();
        let genes: Vec<bool> = rand::thread_rng()
            .sample_iter(rand::distributions::Standard)
            .take(self.gene_size)
            .collect();

        Chromosome::new(genes)
    }

    // defined here because needs to know gene type bool
    pub fn mutate_single_gene(&self, gene: &mut bool) {
        *gene = !*gene;
    }
}

impl Default for Context {
    fn default() -> Self {
        Context {
            gene_size: 10,
            population_size: 100,
            tournament_size: 4,
        }
    }
}
