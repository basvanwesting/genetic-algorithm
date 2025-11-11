use genetic_algorithm::fitness::placeholders::CountdownNoisy;
use genetic_algorithm::strategy::evolve::prelude::*;
use std::ops::RangeInclusive;

const GENES_SIZE: usize = 40_695;
const ALLELE_RANGE: RangeInclusive<f32> = -150.0..=120.0;
const REPLACEMENT_RATE: f32 = 0.5;
const ELITISM_RATE: f32 = 0.02;
const SELECTION_RATE: f32 = 0.4;
const CROSSOVER_RATE: f32 = 0.8;
const POPULATION_SIZE: usize = 225;
const TARGET_GENERATION: usize = (500_f32 * SELECTION_RATE) as usize;
const TOURNAMENT_SIZE: usize = 20;
const MUTATIONS_PER_CHROMOSOME: usize = 50;

// Crossover is where the main work is taking place in the base loop
//
// | Genes Hashing | Chromosome Recycling | Genes Size | Select (ms) | Crossover (ms) | Mutate (ms) |
// |---------------|----------------------|------------|-------------|----------------|-------------|
// | false         | false                | 40695      | 101.216     | 491.237        | 15.988      |
// | true          | false                | 40695      | 194.109     | 1,008.000      | 117.058     |
// | false         | true                 | 40695      | 29.646      | 365.227        | 15.888      |
// | true          | true                 | 40695      | 30.539      | 788.115        | 116.259     |
// | false         | false                | 100        | 36.437      | 9.634          | 1.153       |
// | true          | false                | 100        | 31.486      | 8.977          | 1.180       |
// | false         | true                 | 100        | 38.661      | 9.749          | 1.318       |
// | true          | true                 | 100        | 35.119      | 9.579          | 1.360       |

fn main() {
    let genotype = RangeGenotype::builder()
        .with_genes_size(GENES_SIZE)
        .with_allele_range(ALLELE_RANGE)
        // .with_mutation_type(MutationType::Random) // not needed, is default
        // .with_mutation_type(MutationType::Range(0.1))
        .with_genes_hashing(true)
        .with_chromosome_recycling(true)
        .build()
        .unwrap();

    println!("{}", genotype);

    let evolve_builder = Evolve::builder()
        .with_genotype(genotype)
        // .with_select(SelectElite::new(REPLACEMENT_RATE, ELITISM_RATE))
        .with_select(SelectTournament::new(
            REPLACEMENT_RATE,
            ELITISM_RATE,
            TOURNAMENT_SIZE,
        ))
        // .with_crossover(CrossoverClone::new(SELECTION_RATE))
        // .with_crossover(CrossoverRejuvenate::new(1.0))
        .with_crossover(CrossoverMultiPoint::new(
            SELECTION_RATE,
            CROSSOVER_RATE,
            9,
            false,
        ))
        .with_mutate(MutateMultiGene::new(MUTATIONS_PER_CHROMOSOME, 0.2))
        .with_reporter(EvolveReporterSimple::new(100))
        .with_fitness(CountdownNoisy::new(
            POPULATION_SIZE * TARGET_GENERATION,
            POPULATION_SIZE * 10,
            1..(POPULATION_SIZE * 10),
        ))
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_max_stale_generations(100)
        .with_target_fitness_score(0)
        .with_target_population_size(POPULATION_SIZE);

    let now = std::time::Instant::now();
    let evolve = evolve_builder.call().unwrap();
    let duration = now.elapsed();

    println!(
        "best_generation: {:?}, best fitness score: {:?}, duration: {:?}",
        evolve.best_generation(),
        evolve.best_fitness_score(),
        duration
    );
}
