use genetic_algorithm::chromosome::{Chromosome, GenesKey};
use genetic_algorithm::compete::CompeteTournament;
use genetic_algorithm::crossover::CrossoverClone;
use genetic_algorithm::evolve::Evolve;
use genetic_algorithm::fitness::Fitness;
use genetic_algorithm::genotype::BinaryGenotype;
use genetic_algorithm::mutate::MutateOnce;
use lru::LruCache;
use rand::prelude::*;
use rand::rngs::SmallRng;
use std::{thread, time};

pub type MicroSeconds = u64;
pub type CacheSize = usize;

#[derive(Clone, Debug)]
pub struct ExpensiveCount {
    pub micro_seconds: MicroSeconds,
}
impl ExpensiveCount {
    pub fn new(micro_seconds: MicroSeconds) -> Self {
        Self { micro_seconds }
    }
}
impl Fitness for ExpensiveCount {
    type Genotype = BinaryGenotype;
    fn call_for_chromosome(&mut self, chromosome: &Chromosome<Self::Genotype>) -> isize {
        thread::sleep(time::Duration::from_micros(self.micro_seconds));
        chromosome.genes.iter().filter(|&value| *value).count() as isize
    }
}

#[derive(Debug)]
pub struct CachedExpensiveCount {
    pub micro_seconds: MicroSeconds,
    pub cache_size: CacheSize,
    pub cache: LruCache<GenesKey, isize>,
}
impl CachedExpensiveCount {
    pub fn new(micro_seconds: MicroSeconds, cache_size: CacheSize) -> Self {
        Self {
            micro_seconds,
            cache_size,
            cache: LruCache::new(cache_size),
        }
    }
}
impl Fitness for CachedExpensiveCount {
    type Genotype = BinaryGenotype;
    fn call_for_chromosome(&mut self, chromosome: &Chromosome<Self::Genotype>) -> isize {
        //print!("cache try ({}), ", self.cache.len());
        *self
            .cache
            .get_or_insert(chromosome.genes_key(), || {
                //println!("miss");
                thread::sleep(time::Duration::from_micros(self.micro_seconds));
                chromosome.genes.iter().filter(|&value| *value).count() as isize
            })
            .unwrap()
    }
}
impl Clone for CachedExpensiveCount {
    fn clone(&self) -> Self {
        Self::new(self.micro_seconds, self.cache_size)
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
        //.with_fitness(ExpensiveCount::new(1000))
        .with_fitness(CachedExpensiveCount::new(1000, 1500))
        .with_crossover(CrossoverClone(true))
        .with_compete(CompeteTournament(4))
        .call();

    println!("{}", evolve);
}
