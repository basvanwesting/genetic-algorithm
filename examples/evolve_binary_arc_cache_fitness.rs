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
    pub cache_pointer: Arc<RwLock<LruCache<GenesHash, FitnessValue>>>,
    pub cache_counter_pointer: Arc<RwLock<(usize, usize)>>,
}
impl CachedExpensiveCount {
    pub fn new(
        micro_seconds: MicroSeconds,
        cache_pointer: Arc<RwLock<LruCache<GenesHash, FitnessValue>>>,
        cache_counter_pointer: Arc<RwLock<(usize, usize)>>,
    ) -> Self {
        Self {
            micro_seconds,
            cache_pointer,
            cache_counter_pointer,
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
            .cache_pointer
            .read()
            .map(|c| c.peek(&hash).cloned())
            .unwrap();

        if let Some(value) = maybe_value {
            self.cache_counter_pointer.write().unwrap().0 += 1;
            // println!("cache-hit");
            Some(value)
        } else {
            self.cache_counter_pointer.write().unwrap().1 += 1;
            // println!("cache-miss");
            thread::sleep(time::Duration::from_micros(self.micro_seconds));
            let value = chromosome.genes.iter().filter(|&value| *value).count() as FitnessValue;
            self.cache_pointer.write().unwrap().put(hash, value);
            Some(value)
        }
    }
}

fn main() {
    env_logger::init();

    let cache: LruCache<GenesHash, FitnessValue> =
        LruCache::new(NonZeroUsize::new(100 * 1000).unwrap());
    let cache_pointer = Arc::new(RwLock::new(cache));
    let cache_counter_pointer = Arc::new(RwLock::new((0, 0)));

    let genotype = BinaryGenotype::builder()
        .with_genes_size(100)
        .with_genes_hashing(true)
        .build()
        .unwrap();

    println!("{}", genotype);

    let evolve = Evolve::builder()
        // let (evolve, _others) = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(100)
        .with_max_stale_generations(1000)
        // .with_target_fitness_score(100)
        .with_mutate(MutateSingleGene::new(0.05))
        //.with_fitness(ExpensiveCount::new(1000))
        .with_fitness(CachedExpensiveCount::new(
            10,
            cache_pointer,
            cache_counter_pointer.clone(),
        ))
        .with_par_fitness(true)
        .with_crossover(CrossoverClone::new())
        .with_select(SelectTournament::new(4, 0.9))
        .with_reporter(EvolveReporterSimple::new(100))
        .call()
        // .call_repeatedly(10)
        .unwrap();

    println!("{}", evolve);

    let cache_hits = cache_counter_pointer.read().unwrap().0;
    let cache_misses = cache_counter_pointer.read().unwrap().1;

    println! {"cache_hits: {}, cache_misses: {}", cache_hits, cache_misses};
}

// Not very useful of you can find a target_score (hit: 243, miss: 1252)
// But useful of you end condition is stale (hit: 4684, miss: 2032)
