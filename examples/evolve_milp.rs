use genetic_algorithm::strategy::evolve::prelude::*;

const TARGET_SCORE: isize = (59.0 / PRECISION) as isize;
const PRECISION: f32 = 1e-5;
// const PENALTY: f32 = 1000.0;

// see https://www.mathworks.com/help/optim/ug/intlinprog.html#bts3gkc-2
#[derive(Clone, Debug)]
struct MILPFitness;
impl Fitness for MILPFitness {
    type Genotype = MultiRangeGenotype<f32>;
    fn calculate_for_chromosome(
        &mut self,
        chromosome: &FitnessChromosome<Self>,
        _genotype: &FitnessGenotype<Self>,
    ) -> Option<FitnessValue> {
        let x1 = chromosome.genes[0];
        let x2 = chromosome.genes[1].floor();

        // let mut score = 8.0 * x1 + x2;
        // if (x1 + 2.0 * x2) < -14.0 {
        //     score += PENALTY;
        // };
        // if (-4.0 * x1 - x2) > -33.0 {
        //     score += PENALTY;
        // };
        // if (2.0 * x1 + x2) > 20.0 {
        //     score += PENALTY;
        // };
        //
        // Some((score / PRECISION) as isize)

        if x1 + 2.0 * x2 >= -14.0 && -4.0 * x1 - x2 <= -33.0 && 2.0 * x1 + x2 <= 20.0 {
            let score = 8.0 * x1 + x2;
            Some((score / PRECISION) as isize)
        } else {
            None
        }
    }
}

fn main() {
    env_logger::init();

    let genotype = MultiRangeGenotype::builder()
        .with_allele_ranges(vec![(-10.0..=10.0), (0.0..=10.0)])
        // .with_allele_mutation_ranges(vec![(-1.0..=1.0), (-1.0..=1.0)])
        .with_allele_mutation_scaled_ranges(vec![
            vec![-0.1..=0.1, -0.1..=0.1],
            vec![-0.01..=0.01, -0.01..=0.01],
            vec![-0.001..=0.001, -0.001..=0.001],
            vec![-0.0001..=0.0001, -0.0001..=0.0001],
            vec![-0.00001..=0.00001, -0.00001..=0.00001],
            vec![-0.000001..=0.000001, -0.000001..=0.000001],
        ])
        .build()
        .unwrap();

    println!("genotype: {}", genotype);

    let evolve_builder = Evolve::builder()
        .with_genotype(genotype)
        // .with_reporter(EvolveReporterSimple::new(100))
        .with_target_population_size(1000)
        .with_max_stale_generations(100)
        .with_target_fitness_score(TARGET_SCORE)
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_mutate(MutateSingleGene::new(0.4))
        .with_fitness(MILPFitness)
        .with_crossover(CrossoverSingleGene::new(0.7, 0.8))
        .with_select(SelectTournament::new(0.5, 0.02, 4));

    for _ in 0..10 {
        let now = std::time::Instant::now();
        let evolve = evolve_builder.clone().call().unwrap();
        let duration = now.elapsed();

        if let Some((best_genes, fitness_score)) = evolve.best_genes_and_fitness_score() {
            let x1 = best_genes[0];
            let x2 = best_genes[1].floor();
            let result = 8.0 * x1 + x2;

            println!(
                "x1: {:.5}, x2: {} => {:.5} (fitness score: {:>3}, best_iteration: {:>3}, best_generation: {:>5}, duration: {:?}, scale_index: {:?})",
                x1, x2 as u8, result, fitness_score, evolve.state.current_iteration, evolve.best_generation(), duration, evolve.state.current_scale_index
            );
        } else {
            println!(
                "invalid solution with fitness score: none, duration {:?}",
                duration
            );
        }
    }
}
