use genetic_algorithm::strategy::evolve::prelude::*;
use lru::LruCache;
use std::num::NonZeroUsize;
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
    fn calculate_for_chromosome(
        &mut self,
        chromosome: &FitnessChromosome<Self>,
        _genotype: &FitnessGenotype<Self>,
    ) -> Option<FitnessValue> {
        thread::sleep(time::Duration::from_micros(self.micro_seconds));
        Some(chromosome.genes.iter().filter(|&value| *value).count() as FitnessValue)
    }
}

#[derive(Debug)]
pub struct CachedExpensiveCount {
    pub micro_seconds: MicroSeconds,
    pub cache_size: CacheSize,
    pub cache: LruCache<GenesHash, FitnessValue>,
    pub cache_hits: usize,
    pub cache_misses: usize,
}
impl CachedExpensiveCount {
    pub fn new(micro_seconds: MicroSeconds, cache_size: CacheSize) -> Self {
        Self {
            micro_seconds,
            cache_size,
            cache: LruCache::new(NonZeroUsize::new(cache_size).unwrap()),
            cache_hits: 0,
            cache_misses: 0,
        }
    }
}
impl Fitness for CachedExpensiveCount {
    type Genotype = BinaryGenotype;
    fn calculate_for_chromosome(
        &mut self,
        chromosome: &FitnessChromosome<Self>,
        _genotype: &FitnessGenotype<Self>,
    ) -> Option<FitnessValue> {
        let hash = chromosome.genes_hash().unwrap();
        if let Some(value) = self.cache.get(&hash) {
            self.cache_hits += 1;
            Some(*value)
        } else {
            self.cache_misses += 1;
            thread::sleep(time::Duration::from_micros(self.micro_seconds));
            let value = chromosome.genes.iter().filter(|&value| *value).count() as FitnessValue;
            self.cache.put(hash, value);
            Some(value)
        }
    }
}
impl Clone for CachedExpensiveCount {
    fn clone(&self) -> Self {
        Self::new(self.micro_seconds, self.cache_size)
    }
}

fn main() {
    env_logger::init();

    let genotype = BinaryGenotype::builder()
        .with_genes_size(100)
        .with_genes_hashing(true)
        .build()
        .unwrap();

    println!("{}", genotype);

    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(100)
        .with_max_stale_generations(1000)
        // .with_target_fitness_score(100)
        .with_mutate(MutateSingleGene::new(0.05))
        //.with_fitness(ExpensiveCount::new(1000))
        .with_fitness(CachedExpensiveCount::new(10, 100 * 1000))
        .with_crossover(CrossoverClone::new())
        .with_select(SelectTournament::new(4, 0.9))
        .with_reporter(EvolveReporterSimple::new(100))
        .call()
        .unwrap();

    println!("{}", evolve);
    println! {"cache_hits: {}, cache_misses: {}", evolve.fitness.cache_hits, evolve.fitness.cache_misses};
}

// Not very useful of you can find a target_score (hit: 243, miss: 1252)
// But useful of you end condition is stale (hit: 4684, miss: 2032)
