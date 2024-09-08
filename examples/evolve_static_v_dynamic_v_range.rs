use genetic_algorithm::strategy::evolve::prelude::*;

const GENES_SIZE: usize = 1000;
const POPULATION_SIZE: usize = 100;

#[derive(Clone, Debug)]
pub struct StaticDistanceTo(pub f32, pub f32); // target, precision
impl Fitness for StaticDistanceTo {
    type Genotype = StaticMatrixGenotype<f32, GENES_SIZE, { POPULATION_SIZE + 2 }>;
    fn call_for_population(
        &mut self,
        population: &mut Population<Self::Genotype>,
        genotype: &mut Self::Genotype,
        _thread_local: Option<&ThreadLocal<RefCell<Self>>>,
    ) {
        // pure matrix data calculation on [[T; N] M]
        let results: Vec<FitnessValue> = genotype
            .data
            .iter()
            .map(|genes| {
                genes
                    .iter()
                    .map(|v| (v - self.0).abs() / self.1)
                    .sum::<f32>() as FitnessValue
            })
            .collect();

        for chromosome in population.chromosomes.iter_mut() {
            chromosome.fitness_score = Some(results[chromosome.reference_id]);
        }
    }
}

#[derive(Clone, Debug)]
pub struct DynamicDistanceTo(pub f32, pub f32); // target, precision
impl Fitness for DynamicDistanceTo {
    type Genotype = DynamicMatrixGenotype;
    fn call_for_population(
        &mut self,
        population: &mut Population<Self::Genotype>,
        genotype: &mut Self::Genotype,
        _thread_local: Option<&ThreadLocal<RefCell<Self>>>,
    ) {
        // pure matrix data calculation on vec![T; N*M]
        let results: Vec<FitnessValue> = genotype
            .data
            .chunks(GENES_SIZE)
            .map(|genes| {
                genes
                    .iter()
                    .map(|v| (v - self.0).abs() / self.1)
                    .sum::<f32>() as FitnessValue
            })
            .collect();

        for chromosome in population.chromosomes.iter_mut() {
            chromosome.fitness_score = Some(results[chromosome.reference_id]);
        }
    }
}

#[derive(Clone, Debug)]
pub struct RangeDistanceTo(pub f32, pub f32); // target, precision
impl Fitness for RangeDistanceTo {
    type Genotype = RangeGenotype<f32>;
    fn calculate_for_chromosome(
        &mut self,
        chromosome: &Chromosome<Self::Genotype>,
        _genotype: &Self::Genotype,
    ) -> Option<FitnessValue> {
        Some(
            chromosome
                .genes
                .iter()
                .map(|v| (v - self.0).abs() / self.1)
                .sum::<f32>() as FitnessValue,
        )
    }
}

#[derive(Clone)]
pub struct CustomStaticMatrixReporter;
impl EvolveReporter for CustomStaticMatrixReporter {
    type Genotype = StaticMatrixGenotype<f32, GENES_SIZE, { POPULATION_SIZE + 2 }>;

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
pub struct CustomDynamicMatrixReporter;
impl EvolveReporter for CustomDynamicMatrixReporter {
    type Genotype = DynamicMatrixGenotype;

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
pub struct CustomRangeReporter;
impl EvolveReporter for CustomRangeReporter {
    type Genotype = RangeGenotype;

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

    let genotype = StaticMatrixGenotype::<f32, GENES_SIZE, { POPULATION_SIZE + 2 }>::builder()
        .with_genes_size(GENES_SIZE)
        .with_allele_range(0.0..=1.0) // won't converge, with low max_stale_generations, converges just fine with higher max_stale_generations
        // .with_allele_mutation_range(-0.1..=0.1) // won't converge, with low max_stale_generations, converges just fine with higher max_stale_generations
        // .with_allele_mutation_range(-0.001..=0.001) // slow converge
        .with_allele_mutation_scaled_range(vec![
            -0.1..=0.1,
            -0.05..=0.05,
            -0.025..=0.025,
            -0.01..=0.01,
            -0.005..=0.005,
            -0.0025..=0.0025,
            -0.001..=0.001,
        ])
        .build()
        .unwrap();
    // println!("{}", genotype);
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(POPULATION_SIZE)
        .with_max_stale_generations(100)
        .with_target_fitness_score((GENES_SIZE as f32 * 0.001 / 1e-5) as isize)
        .with_fitness(StaticDistanceTo(0.5, 1e-5))
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_mutate(MutateSingleGene::new(0.2))
        // .with_crossover(CrossoverMultiPoint::new(10, true))
        .with_crossover(CrossoverUniform::new())
        .with_select(SelectTournament::new(4, 0.9))
        .with_reporter(CustomStaticMatrixReporter)
        .call()
        .unwrap();
    // println!("{}", evolve);
    // println!("genes: {:b}", evolve.best_chromosome().unwrap().genes);
    println!(
        "StaticMatrixGenotype, best_generation: {:?}",
        evolve.best_generation()
    );
    println!(
        "StaticMatrixGenotype, best_fitness_score: {:?}",
        evolve.best_fitness_score()
    );

    let genotype = DynamicMatrixGenotype::builder()
        .with_genes_size(GENES_SIZE)
        .with_allele_range(0.0..=1.0) // won't converge, with low max_stale_generations, converges just fine with higher max_stale_generations
        // .with_allele_mutation_range(-0.1..=0.1) // won't converge, with low max_stale_generations, converges just fine with higher max_stale_generations
        // .with_allele_mutation_range(-0.001..=0.001) // slow converge
        .with_allele_mutation_scaled_range(vec![
            -0.1..=0.1,
            -0.05..=0.05,
            -0.025..=0.025,
            -0.01..=0.01,
            -0.005..=0.005,
            -0.0025..=0.0025,
            -0.001..=0.001,
        ])
        .build()
        .unwrap();
    // println!("{}", genotype);
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(POPULATION_SIZE)
        .with_max_stale_generations(100)
        .with_target_fitness_score((GENES_SIZE as f32 * 0.001 / 1e-5) as isize)
        .with_fitness(DynamicDistanceTo(0.5, 1e-5))
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_mutate(MutateSingleGene::new(0.2))
        .with_crossover(CrossoverUniform::new())
        .with_select(SelectTournament::new(4, 0.9))
        .with_reporter(CustomDynamicMatrixReporter)
        .call()
        .unwrap();
    // println!("{}", evolve);
    // println!("genes: {:b}", evolve.best_chromosome().unwrap().genes);
    println!(
        "DynamicMatrixGenotype, best_generation: {:?}",
        evolve.best_generation()
    );
    println!(
        "DynamicMatrixGenotype, best_fitness_score: {:?}",
        evolve.best_fitness_score()
    );

    let genotype = RangeGenotype::builder()
        .with_genes_size(GENES_SIZE)
        .with_allele_range(0.0..=1.0) // won't converge, with low max_stale_generations, converges just fine with higher max_stale_generations
        // .with_allele_mutation_range(-0.1..=0.1) // won't converge, with low max_stale_generations, converges just fine with higher max_stale_generations
        // .with_allele_mutation_range(-0.001..=0.001) // slow converge
        .with_allele_mutation_scaled_range(vec![
            -0.1..=0.1,
            -0.05..=0.05,
            -0.025..=0.025,
            -0.01..=0.01,
            -0.005..=0.005,
            -0.0025..=0.0025,
            -0.001..=0.001,
        ])
        // .with_chromosome_recycling(true)
        .build()
        .unwrap();
    // println!("{}", genotype);
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(POPULATION_SIZE)
        .with_max_stale_generations(100)
        .with_target_fitness_score((GENES_SIZE as f32 * 0.001 / 1e-5) as isize)
        .with_fitness(RangeDistanceTo(0.5, 1e-5))
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_mutate(MutateSingleGene::new(0.2))
        .with_crossover(CrossoverUniform::new())
        .with_select(SelectTournament::new(4, 0.9))
        .with_reporter(CustomRangeReporter)
        .call()
        .unwrap();
    // println!("{}", evolve);
    // println!("genes: {:b}", evolve.best_chromosome().unwrap().genes);
    println!(
        "RangeGenotype, best_generation: {:?}",
        evolve.best_generation()
    );
    println!(
        "RangeGenotype, best_fitness_score: {:?}",
        evolve.best_fitness_score()
    );
}
