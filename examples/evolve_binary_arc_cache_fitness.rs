use genetic_algorithm::strategy::evolve::prelude::*;
use lru::LruCache;
use std::num::NonZeroUsize;
use std::sync::{Arc, RwLock};
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

#[derive(Debug, Clone)]
pub struct CachedExpensiveCount {
    pub micro_seconds: MicroSeconds,
    pub cache_state: Arc<RwLock<LruCache<GenesHash, FitnessValue>>>,
    pub cache_counter: Arc<RwLock<(usize, usize)>>,
    pub cache_hit_fitness_score: Arc<RwLock<isize>>,
}
impl CachedExpensiveCount {
    pub fn new(
        micro_seconds: MicroSeconds,
        cache_state: Arc<RwLock<LruCache<GenesHash, FitnessValue>>>,
        cache_counter: Arc<RwLock<(usize, usize)>>,
        cache_hit_fitness_score: Arc<RwLock<isize>>,
    ) -> Self {
        Self {
            micro_seconds,
            cache_state,
            cache_counter,
            cache_hit_fitness_score,
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

        let maybe_value = self
            .cache_state
            .read()
            .map(|c| c.peek(&hash).cloned())
            .unwrap();

        if let Some(value) = maybe_value {
            self.cache_counter.write().unwrap().0 += 1;
            *self.cache_hit_fitness_score.write().unwrap() += value;
            // println!("cache-hit");
            Some(value)
        } else {
            self.cache_counter.write().unwrap().1 += 1;
            // println!("cache-miss");
            thread::sleep(time::Duration::from_micros(self.micro_seconds));
            let value = chromosome.genes.iter().filter(|&value| *value).count() as FitnessValue;
            self.cache_state.write().unwrap().put(hash, value);
            Some(value)
        }
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

    let evolve_builder = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(100)
        .with_max_stale_generations(1000)
        // .with_target_fitness_score(100)
        .with_mutate(MutateSingleGene::new(0.05))
        .with_crossover(CrossoverClone::new())
        .with_select(SelectTournament::new(4, 0.9));

    // println!("{}", evolve);

    for repeats in [1, 2, 4, 8, 16, 32, 64, 128] {
        for cache_size in [10, 100, 1000, 10_000, 100_000, 1_000_000] {
            let cache: LruCache<GenesHash, FitnessValue> =
                LruCache::new(NonZeroUsize::new(cache_size).unwrap());
            let cache_state = Arc::new(RwLock::new(cache));
            let cache_counter = Arc::new(RwLock::new((0, 0)));
            let cache_hit_fitness_score = Arc::new(RwLock::new(0));

            let _ = evolve_builder
                .clone()
                .with_fitness(CachedExpensiveCount::new(
                    0,
                    cache_state,
                    cache_counter.clone(),
                    cache_hit_fitness_score.clone(),
                ))
                // .with_par_fitness(true)
                // .with_reporter(EvolveReporterSimple::new(100))
                .call_par_repeatedly(repeats);

            let cache_hits = cache_counter.read().unwrap().0;
            let cache_misses = cache_counter.read().unwrap().1;
            let ratio = cache_hits as f32 / cache_misses as f32;

            let hit_fitness_score = *cache_hit_fitness_score.read().unwrap();
            let avg_hit_fitness_score = hit_fitness_score as f32 / cache_hits as f32;

            println! {"repeats: {}, cache_size: {}, cache_hits: {}, cache_misses: {}, ratio: {}, avg_hit_fitness_score: {}", repeats, cache_size, cache_hits, cache_misses, ratio, avg_hit_fitness_score};
        }
    }
}

// Not very useful of you can find a target_score (hit: 243, miss: 1252)
// But useful of you end condition is stale (hit: 4684, miss: 2032)
