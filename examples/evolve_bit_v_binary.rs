use genetic_algorithm::fitness::placeholders::{CountOnes, CountTrue};
use genetic_algorithm::strategy::evolve::prelude::*;

const GENES_SIZE: usize = 10_000;
const POPULATION_SIZE: usize = 100;

fn main() {
    env_logger::init();

    let genotype = BitGenotype::builder()
        .with_genes_size(GENES_SIZE)
        .build()
        .unwrap();
    // println!("{}", genotype);
    let now = std::time::Instant::now();
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(POPULATION_SIZE)
        .with_max_stale_generations(100)
        .with_target_fitness_score((POPULATION_SIZE * 2) as isize)
        .with_fitness(CountOnes)
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_mutate(MutateSingleGene::new(0.2))
        .with_crossover(CrossoverMultiPoint::new(10, true, 0.8))
        .with_compete(CompeteTournament::new(4))
        // .with_reporter(EvolveReporterSimple::new(usize::MAX))
        .call()
        .unwrap();
    let duration = now.elapsed();
    // println!("{}", evolve);
    // println!("genes: {:b}", evolve.best_chromosome().unwrap().genes);
    println!("BitGenotype, duration: {:?}", duration);
    println!(
        "BitGenotype, best_generation: {:?}",
        evolve.best_generation()
    );
    println!(
        "BitGenotype, best_fitness_score: {:?}",
        evolve.best_fitness_score()
    );

    let genotype = BinaryGenotype::builder()
        .with_genes_size(GENES_SIZE)
        .build()
        .unwrap();
    // println!("{}", genotype);
    let now = std::time::Instant::now();
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(POPULATION_SIZE)
        .with_max_stale_generations(100)
        .with_target_fitness_score((POPULATION_SIZE * 2) as isize)
        .with_fitness(CountTrue)
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_mutate(MutateSingleGene::new(0.2))
        .with_crossover(CrossoverMultiPoint::new(10, true, 0.8))
        .with_compete(CompeteTournament::new(4))
        // .with_reporter(EvolveReporterSimple::new(usize::MAX))
        .call()
        .unwrap();
    let duration = now.elapsed();
    // println!("{}", evolve);
    // println!("genes: {:b}", evolve.best_chromosome().unwrap().genes);
    println!(
        "BinaryGenotype, duration: {:?} - more cloning overhead, keeping parents",
        duration
    );
    println!(
        "BinaryGenotype, best_generation: {:?} - more crossover points seem to converge in less generations",
        evolve.best_generation()
    );
    println!(
        "BinaryGenotype, best_fitness_score: {:?}",
        evolve.best_fitness_score()
    );
}
