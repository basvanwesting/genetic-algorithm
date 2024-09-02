use genetic_algorithm::strategy::hill_climb::prelude::*;

//const TARGET_SCORE: isize = (59.0 / PRECISION) as isize;
//const PENALTY: f32 = 1000.0;
const PRECISION: f32 = 1e-5;

// see https://www.mathworks.com/help/optim/ug/intlinprog.html#bts3gkc-2
#[derive(Clone, Debug)]
struct MILPFitness;
impl Fitness for MILPFitness {
    type Genotype = RangeGenotype<f32>;
    fn calculate_for_chromosome(
        &mut self,
        chromosome: &Chromosome<Self::Genotype>,
    ) -> Option<FitnessValue> {
        let x1 = chromosome.genes[0];
        let x2 = chromosome.genes[1].floor();

        //let mut score = 8.0 * x1 + x2;
        //if (x1 + 2.0 * x2) < -14.0 {
        //score += PENALTY;
        //};
        //if (-4.0 * x1 - x2) > -33.0 {
        //score += PENALTY;
        //};
        //if (2.0 * x1 + x2) > 20.0 {
        //score += PENALTY;
        //};

        //Some((score / PRECISION) as isize)

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
        ])
        .build()
        .unwrap();

    println!("genotype: {}", genotype);

    let hill_climb_builder = HillClimb::builder()
        .with_genotype(genotype)
        // .with_variant(HillClimbVariant::Stochastic)
        // .with_max_stale_generations(1000)
        .with_variant(HillClimbVariant::SteepestAscent)
        .with_max_stale_generations(1)
        // .with_reporter(HillClimbReporterSimple::new(100))
        //.with_target_fitness_score(TARGET_SCORE)
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_fitness(MILPFitness);

    for _ in 0..10 {
        let now = std::time::Instant::now();
        let hill_climb = hill_climb_builder.clone().call_repeatedly(1000).unwrap();
        let duration = now.elapsed();

        if let Some(best_chromosome) = hill_climb.best_chromosome() {
            if let Some(fitness_score) = best_chromosome.fitness_score {
                let x1 = best_chromosome.genes[0];
                let x2 = best_chromosome.genes[1].floor();
                let result = 8.0 * x1 + x2;

                println!(
                    "x1: {:.5}, x2: {} => {:.5} (fitness score: {:>3}, best_iteration: {:>3}, best_generation: {:>5}, duration: {:?}, scale_index: {:?})",
                    x1, x2 as u8, result, fitness_score, hill_climb.state.current_iteration, hill_climb.best_generation(), duration, hill_climb.state.current_scale_index
                );
            } else {
                println!(
                    "invalid solution with fitness score: none, duration {:?}",
                    duration
                );
            }
        }
    }
}
