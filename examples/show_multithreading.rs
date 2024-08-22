use genetic_algorithm::fitness::placeholders::CountTrueWithSleep;
use genetic_algorithm::strategy::evolve::prelude::*;
use genetic_algorithm::strategy::hill_climb::prelude::*;
use genetic_algorithm::strategy::permutate::prelude::*;
use rand::prelude::*;
use rand::rngs::SmallRng;

// const MULTITHREAD: bool = false;
const MULTITHREAD: bool = true;
// const PAR_CALL: bool = false;
const PAR_CALL: bool = true;

fn main() {
    env_logger::init();

    // call_evolve();
    //
    // call_hill_climb();
    //
    // call_permutate();
    //
    call_hill_climb_repeatedly();
}

#[allow(dead_code)]
fn call_evolve() {
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
}

#[allow(dead_code)]
fn call_hill_climb() {
    let mut rng = SmallRng::from_entropy();
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
}

#[allow(dead_code)]
fn call_permutate() {
    let mut rng = SmallRng::from_entropy();
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
    println!();
}

#[allow(dead_code)]
fn call_hill_climb_repeatedly() {
    let mut rng = SmallRng::from_entropy();
    let genotype = BinaryGenotype::builder()
        .with_genes_size(100)
        .build()
        .unwrap();

    println!("hill_climb_repeatedly: start");
    let hill_climb_builder = HillClimb::builder()
        .with_genotype(genotype.clone())
        // .with_variant(HillClimbVariant::SteepestAscent)
        // .with_max_stale_generations(1)
        .with_variant(HillClimbVariant::Stochastic)
        .with_max_stale_generations(1000)
        // .with_target_fitness_score(100)
        .with_fitness(CountTrueWithSleep::new(1000, true))
        .with_reporter(HillClimbIterationReporter)
        .with_multithreading(MULTITHREAD);

    let now = std::time::Instant::now();
    let hill_climb = if PAR_CALL {
        hill_climb_builder
            .call_par_repeatedly(20, &mut rng)
            .unwrap()
    } else {
        hill_climb_builder.call_repeatedly(3, &mut rng).unwrap()
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
pub struct HillClimbIterationReporter;
impl HillClimbReporter for HillClimbIterationReporter {
    type Allele = BinaryAllele;

    fn on_start<G: Genotype>(
        &mut self,
        _genotype: &G,
        state: &HillClimbState<Self::Allele>,
        _config: &HillClimbConfig,
    ) {
        println!("start - iteration: {}", state.current_iteration());
    }
    fn on_finish(&mut self, state: &HillClimbState<Self::Allele>, _config: &HillClimbConfig) {
        println!("finish - iteration: {}", state.current_iteration());
    }
}

#[derive(Clone)]
pub struct EvolveIterationReporter;
impl EvolveReporter for EvolveIterationReporter {
    type Allele = BinaryAllele;

    fn on_start<G: Genotype>(
        &mut self,
        _genotype: &G,
        state: &EvolveState<Self::Allele>,
        _config: &EvolveConfig,
    ) {
        println!("start - iteration: {}", state.current_iteration());
    }
    fn on_finish(&mut self, state: &EvolveState<Self::Allele>, _config: &EvolveConfig) {
        println!("finish - iteration: {}", state.current_iteration());
    }
}
