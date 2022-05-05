use genetic_algorithm::chromosome::Chromosome;
use genetic_algorithm::compete::CompeteTournament;
use genetic_algorithm::crossover::CrossoverClone;
use genetic_algorithm::evolve::Evolve;
use genetic_algorithm::fitness::Fitness;
use genetic_algorithm::genotype::{BinaryGenotype, Genotype};
use genetic_algorithm::mutate::MutateOnce;
use rand::prelude::*;
use rand::rngs::SmallRng;
use std::collections::HashMap;
use std::{thread, time};

pub type MicroSeconds = u64;

#[derive(Clone, Debug)]
pub struct CachedExpensiveCount {
    pub micro_seconds: MicroSeconds,
    pub cache: HashMap<Vec<<BinaryGenotype as Genotype>::Gene>, isize>,
}
impl CachedExpensiveCount {
    pub fn new(micro_seconds: MicroSeconds) -> Self {
        Self {
            micro_seconds,
            cache: HashMap::with_capacity(5000),
        }
    }
}
impl Fitness for CachedExpensiveCount {
    type Genotype = BinaryGenotype;
    fn call_for_chromosome(&mut self, chromosome: &Chromosome<Self::Genotype>) -> isize {
        print!("cache try ({}), ", self.cache.len());
        *self
            .cache
            .entry(chromosome.genes.clone())
            .or_insert_with(|| {
                println!("miss");
                thread::sleep(time::Duration::from_micros(self.micro_seconds));
                chromosome.genes.iter().filter(|&value| *value).count() as isize
            })
    }
}

fn main() {
    let rng = SmallRng::from_entropy();
    let genotype = BinaryGenotype::new().with_gene_size(100).build();

    println!("{}", genotype);

    let evolve = Evolve::new(genotype, rng)
        .with_population_size(100)
        .with_max_stale_generations(1000)
        .with_target_fitness_score(100)
        .with_mutate(MutateOnce(0.05))
        .with_fitness(CachedExpensiveCount::new(1000))
        .with_crossover(CrossoverClone(true))
        .with_compete(CompeteTournament(4))
        .call();

    println!("{}", evolve);
}
