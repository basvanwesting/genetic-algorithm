use genetic_algorithm::fitness::placeholders::{CountOnes, CountTrue};
use genetic_algorithm::strategy::evolve::prelude::*;

const GENES_SIZE: usize = 10_000;
const POPULATION_SIZE: usize = 100;

#[derive(Clone)]
pub struct CustomBitReporter;
impl EvolveReporter for CustomBitReporter {
    type Genotype = BitGenotype;

    fn on_finish(&mut self, state: &EvolveState<Self::Genotype>, _config: &EvolveConfig) {
        println!("finish - iteration: {}", state.current_iteration());
        STRATEGY_ACTIONS.iter().for_each(|action| {
            if let Some(duration) = state.durations.get(action) {
                println!("  {:?}: {:?}", action, duration,);
            }
        });
        println!("  Total: {:?}", &state.total_duration());
    }
}
#[derive(Clone)]
pub struct CustomBinaryReporter;
impl EvolveReporter for CustomBinaryReporter {
    type Genotype = BinaryGenotype;

    fn on_finish(&mut self, state: &EvolveState<Self::Genotype>, _config: &EvolveConfig) {
        println!("finish - iteration: {}", state.current_iteration());
        STRATEGY_ACTIONS.iter().for_each(|action| {
            if let Some(duration) = state.durations.get(action) {
                println!("  {:?}: {:?}", action, duration,);
            }
        });
        println!("  Total: {:?}", &state.total_duration());
    }
}

fn main() {
    env_logger::init();

    let genotype = BitGenotype::builder()
        .with_genes_size(GENES_SIZE)
        .build()
        .unwrap();
    // println!("{}", genotype);
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(POPULATION_SIZE)
        .with_max_stale_generations(100)
        .with_target_fitness_score((POPULATION_SIZE * 2) as isize)
        .with_fitness(CountOnes)
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_mutate(MutateSingleGene::new(0.2))
        .with_crossover(CrossoverMultiPoint::new(10, true))
        .with_select(SelectTournament::new(4, 0.9))
        .with_reporter(CustomBitReporter)
        .call()
        .unwrap();
    // println!("{}", evolve);
    // println!("genes: {:b}", evolve.best_chromosome().unwrap().genes);
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
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(POPULATION_SIZE)
        .with_max_stale_generations(100)
        .with_target_fitness_score((POPULATION_SIZE * 2) as isize)
        .with_fitness(CountTrue)
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_mutate(MutateSingleGene::new(0.2))
        .with_crossover(CrossoverMultiPoint::new(10, true))
        .with_select(SelectTournament::new(4, 0.9))
        .with_reporter(CustomBinaryReporter)
        .call()
        .unwrap();
    // println!("{}", evolve);
    // println!("genes: {:b}", evolve.best_chromosome().unwrap().genes);
    println!(
        "BinaryGenotype, best_generation: {:?} - more crossover points seem to converge in less generations",
        evolve.best_generation()
    );
    println!(
        "BinaryGenotype, best_fitness_score: {:?}",
        evolve.best_fitness_score()
    );
}
