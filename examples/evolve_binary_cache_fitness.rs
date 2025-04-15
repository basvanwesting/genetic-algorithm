use genetic_algorithm::strategy::evolve::prelude::*;
use std::{thread, time};

pub type MicroSeconds = u64;

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
        .with_fitness(ExpensiveCount::new(0))
        .with_mutate(MutateSingleGene::new(0.05))
        .with_crossover(CrossoverClone::new())
        .with_select(SelectTournament::new(4, 0.9));

    // println!("{}", evolve);

    for repeats in [1, 2, 4, 8, 16, 32, 64, 128] {
        for cache_size in [100, 1000, 10_000] {
            let (evolve, _) = evolve_builder
                .clone()
                .with_fitness_cache(cache_size)
                // .with_par_fitness(true)
                // .with_reporter(EvolveReporterSimple::new(100))
                .call_par_repeatedly(repeats)
                .unwrap();

            let (cache_hits, cache_misses, cache_ratio) = evolve
                .config
                .fitness_cache_reference()
                .map(|c| c.hit_miss_stats())
                .unwrap();

            println! {"repeats: {}, cache_size: {}, cache_hits: {}, cache_misses: {}, cache_ratio: {:.2}", repeats, cache_size, cache_hits, cache_misses, cache_ratio};
        }
    }
}

// Not very useful of you can find a target_score (hit: 243, miss: 1252)
// But useful of you end condition is stale (hit: 4684, miss: 2032)
