use genetic_algorithm::fitness::placeholders::CountTrueWithSleep;
use genetic_algorithm::strategy::evolve::prelude::*;
use genetic_algorithm::strategy::hill_climb::prelude::*;
use genetic_algorithm::strategy::permutate::prelude::*;
use rand::prelude::*;
use rand::rngs::SmallRng;

// const MULTITHREAD: bool = false;
const MULTITHREAD: bool = true;

fn main() {
    env_logger::init();

    let mut rng = SmallRng::from_entropy();
    let genotype = BinaryGenotype::builder()
        .with_genes_size(100)
        .build()
        .unwrap();

    println!("evolve: start");
    let mut evolve = Evolve::builder()
        .with_genotype(genotype.clone())
        .with_target_population_size(100)
        .with_max_stale_generations(100)
        .with_target_fitness_score(100)
        .with_fitness(CountTrueWithSleep::new(1000, true))
        .with_multithreading(MULTITHREAD)
        .with_mutate(MutateSingleGene::new(0.2))
        .with_crossover(CrossoverClone::new(true))
        .with_compete(CompeteTournament::new(4))
        // .with_reporter(EvolveReporterSimple::new(1000))
        .build()
        .unwrap();

    let now = std::time::Instant::now();
    evolve.call(&mut rng);
    let duration = now.elapsed();

    if let Some(fitness_score) = evolve.best_fitness_score() {
        println!("evolve fitness score: {}", fitness_score);
    } else {
        println!("evolve invalid solution with fitness score: None");
    }
    println!("evolve: {:?}", duration);
    println!();

    println!("hill_climb: start");
    let mut hill_climb = HillClimb::builder()
        .with_genotype(genotype.clone())
        .with_variant(HillClimbVariant::SteepestAscent)
        .with_max_stale_generations(1)
        .with_target_fitness_score(100)
        .with_fitness(CountTrueWithSleep::new(1000, true))
        .with_multithreading(MULTITHREAD)
        // .with_reporter(HillClimbReporterSimple::new(1000))
        .build()
        .unwrap();

    let now = std::time::Instant::now();
    hill_climb.call(&mut rng);
    let duration = now.elapsed();

    if let Some(fitness_score) = hill_climb.best_fitness_score() {
        println!("hill_climb fitness score: {}", fitness_score);
    } else {
        println!("hill_climb invalid solution with fitness score: None");
    }
    println!("hill_climb: {:?}", duration);
    println!();

    let genotype = BinaryGenotype::builder()
        .with_genes_size(12)
        .build()
        .unwrap();

    println!("permutate: start");
    let mut permutate = Permutate::builder()
        .with_genotype(genotype.clone())
        .with_fitness(CountTrueWithSleep::new(1000, true))
        .with_multithreading(MULTITHREAD)
        .with_reporter(PermutateReporterSimple::new(1000))
        .build()
        .unwrap();

    let now = std::time::Instant::now();
    permutate.call(&mut rng);
    let duration = now.elapsed();

    if let Some(fitness_score) = permutate.best_fitness_score() {
        println!("permutate fitness score: {}", fitness_score);
    } else {
        println!("permutate invalid solution with fitness score: None");
    }
    println!("permutate: {:?}", duration);
}
