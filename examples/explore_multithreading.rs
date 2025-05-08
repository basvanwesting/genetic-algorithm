use genetic_algorithm::fitness::placeholders::CountTrueWithSleep;
use genetic_algorithm::strategy::evolve::prelude::*;
use genetic_algorithm::strategy::hill_climb::prelude::*;
use genetic_algorithm::strategy::permutate::prelude::*;

// const INTERNAL_MULTITHREAD: bool = false;
const INTERNAL_MULTITHREAD: bool = true;
// const EXTERNAL_MULTITHREAD: bool = false;
const EXTERNAL_MULTITHREAD: bool = true;

fn main() {
    env_logger::init();

    call_evolve();
    call_hill_climb();
    call_permutate();
    call_evolve_repeatedly();
    call_hill_climb_repeatedly();
    call_evolve_speciated();
}

#[allow(dead_code)]
fn call_evolve() {
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
        .with_par_fitness(INTERNAL_MULTITHREAD)
        .with_mutate(MutateSingleGene::new(0.2))
        .with_crossover(CrossoverClone::new(0.5))
        .with_select(SelectTournament::new(0.5, 0.02, 4))
        // .with_reporter(EvolveReporterSimple::new(1000))
        .build()
        .unwrap();

    let now = std::time::Instant::now();
    evolve.call();
    let duration = now.elapsed();

    if let Some(fitness_score) = evolve.best_fitness_score() {
        println!("evolve fitness score: {}", fitness_score);
    } else {
        println!("evolve invalid solution with fitness score: None");
    }
    println!("evolve: {:?}", duration);
    println!();
}

#[allow(dead_code)]
fn call_hill_climb() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(100)
        .build()
        .unwrap();

    println!("hill_climb: start");
    let mut hill_climb = HillClimb::builder()
        .with_genotype(genotype.clone())
        .with_variant(HillClimbVariant::SteepestAscent)
        .with_max_stale_generations(1)
        .with_target_fitness_score(100)
        .with_fitness(CountTrueWithSleep::new(1000, true))
        .with_par_fitness(INTERNAL_MULTITHREAD)
        // .with_reporter(HillClimbReporterSimple::new(1000))
        .build()
        .unwrap();

    let now = std::time::Instant::now();
    hill_climb.call();
    let duration = now.elapsed();

    if let Some(fitness_score) = hill_climb.best_fitness_score() {
        println!("hill_climb fitness score: {}", fitness_score);
    } else {
        println!("hill_climb invalid solution with fitness score: None");
    }
    println!("hill_climb: {:?}", duration);
    println!();
}

#[allow(dead_code)]
fn call_permutate() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(12)
        .build()
        .unwrap();

    println!("permutate: start");
    let mut permutate = Permutate::builder()
        .with_genotype(genotype.clone())
        .with_fitness(CountTrueWithSleep::new(1000, true))
        .with_par_fitness(INTERNAL_MULTITHREAD)
        // .with_reporter(PermutateReporterSimple::new(1000))
        .build()
        .unwrap();

    let now = std::time::Instant::now();
    permutate.call();
    let duration = now.elapsed();

    if let Some(fitness_score) = permutate.best_fitness_score() {
        println!("permutate fitness score: {}", fitness_score);
    } else {
        println!("permutate invalid solution with fitness score: None");
    }
    println!("permutate: {:?}", duration);
    println!();
}

#[allow(dead_code)]
fn call_evolve_repeatedly() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(100)
        .build()
        .unwrap();

    println!("evolve_repeatedly: start");
    let evolve_builder = Evolve::builder()
        .with_genotype(genotype.clone())
        .with_target_population_size(100)
        .with_max_stale_generations(100)
        // .with_target_fitness_score(100) // short-circuit
        .with_fitness(CountTrueWithSleep::new(1000, false))
        .with_mutate(MutateSingleGene::new(0.2))
        .with_crossover(CrossoverClone::new(0.5))
        .with_select(SelectTournament::new(0.5, 0.02, 4))
        .with_reporter(IterationReporter)
        .with_par_fitness(INTERNAL_MULTITHREAD);

    let now = std::time::Instant::now();
    let (evolve, _) = if EXTERNAL_MULTITHREAD {
        evolve_builder.call_par_repeatedly(20).unwrap()
    } else {
        evolve_builder.call_repeatedly(3).unwrap()
    };
    let duration = now.elapsed();

    if let Some(fitness_score) = evolve.best_fitness_score() {
        println!("evolve_repeatedly fitness score: {}", fitness_score);
    } else {
        println!("evolve_repeatedly invalid solution with fitness score: None");
    }
    println!("evolve_repeatedly: {:?}", duration);
    println!();
}

#[allow(dead_code)]
fn call_evolve_speciated() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(100)
        .build()
        .unwrap();

    println!("evolve_speciated: start");
    let evolve_builder = Evolve::builder()
        .with_genotype(genotype.clone())
        .with_target_population_size(100)
        .with_max_stale_generations(100)
        // .with_target_fitness_score(100) // short-circuit
        .with_fitness(CountTrueWithSleep::new(1000, false))
        .with_mutate(MutateSingleGene::new(0.2))
        .with_crossover(CrossoverClone::new(0.5))
        .with_select(SelectTournament::new(0.5, 0.02, 4))
        .with_reporter(IterationReporter)
        .with_par_fitness(INTERNAL_MULTITHREAD);

    let now = std::time::Instant::now();
    let (evolve, _) = if EXTERNAL_MULTITHREAD {
        evolve_builder.call_par_speciated(20).unwrap()
    } else {
        evolve_builder.call_speciated(3).unwrap()
    };
    let duration = now.elapsed();

    if let Some(fitness_score) = evolve.best_fitness_score() {
        println!("evolve_speciated fitness score: {}", fitness_score);
    } else {
        println!("evolve_speciated invalid solution with fitness score: None");
    }
    println!("evolve_speciated: {:?}", duration);
    println!();
}

#[allow(dead_code)]
fn call_hill_climb_repeatedly() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(100)
        .build()
        .unwrap();

    println!("hill_climb_repeatedly: start");
    let hill_climb_builder = HillClimb::builder()
        .with_genotype(genotype.clone())
        // .with_variant(HillClimbVariant::SteepestAscent) // internal multi-threading
        // .with_max_stale_generations(1)
        .with_variant(HillClimbVariant::Stochastic) // no internal multi-threading due to sequential nature
        .with_max_stale_generations(1000)
        // .with_target_fitness_score(100) // short-circuit
        .with_fitness(CountTrueWithSleep::new(1000, false))
        .with_reporter(IterationReporter)
        .with_par_fitness(INTERNAL_MULTITHREAD);

    let now = std::time::Instant::now();
    let (hill_climb, _) = if EXTERNAL_MULTITHREAD {
        hill_climb_builder.call_par_repeatedly(20).unwrap()
    } else {
        hill_climb_builder.call_repeatedly(3).unwrap()
    };
    let duration = now.elapsed();

    if let Some(fitness_score) = hill_climb.best_fitness_score() {
        println!("hill_climb_repeatedly fitness score: {}", fitness_score);
    } else {
        println!("hill_climb_repeatedly invalid solution with fitness score: None");
    }
    println!("hill_climb_repeatedly: {:?}", duration);
    println!();
}

#[derive(Clone)]
pub struct IterationReporter;
impl StrategyReporter for IterationReporter {
    type Genotype = BinaryGenotype;

    fn on_start<S: StrategyState<Self::Genotype>, C: StrategyConfig>(
        &mut self,
        genotype: &Self::Genotype,
        state: &S,
        _config: &C,
    ) {
        let number_of_seed_genes = genotype.seed_genes_list().len();
        if number_of_seed_genes > 0 {
            println!(
                "  start - iteration: {}, number of seed genes: {:?}",
                state.current_iteration(),
                number_of_seed_genes
            );
        } else {
            println!("  start - iteration: {}", state.current_iteration());
        }
    }
    fn on_finish<S: StrategyState<Self::Genotype>, C: StrategyConfig>(
        &mut self,
        _genotype: &Self::Genotype,
        state: &S,
        _config: &C,
    ) {
        println!("  finish - iteration: {}", state.current_iteration());
    }
}
