use genetic_algorithm::strategy::evolve::prelude::*;
use rand::prelude::*;
use rand::rngs::SmallRng;

const TARGET_SCORE: isize = (59.0 / PRECISION) as isize;
const PRECISION: f32 = 1e-5;
const PENALTY: f32 = 1000.0;

// see https://www.mathworks.com/help/optim/ug/intlinprog.html#bts3gkc-2
#[derive(Clone, Debug)]
struct MILPFitness;
impl Fitness for MILPFitness {
    type Genotype = MultiContinuousGenotype;
    fn calculate_for_chromosome(
        &mut self,
        chromosome: &Chromosome<Self::Genotype>,
    ) -> Option<FitnessValue> {
        let x1 = chromosome.genes[0];
        let x2 = chromosome.genes[1].floor();

        let mut score = 8.0 * x1 + x2;
        if (x1 + 2.0 * x2) < -14.0 {
            score += PENALTY;
        };
        if (-4.0 * x1 - x2) > -33.0 {
            score += PENALTY;
        };
        if (2.0 * x1 + x2) > 20.0 {
            score += PENALTY;
        };

        Some((score / PRECISION) as isize)

        //if x1 + 2.0 * x2 >= -14.0 && -4.0 * x1 - x2 <= -33.0 && 2.0 * x1 + x2 <= 20.0 {
        //let score = 8.0 * x1 + x2;
        //Some((score / PRECISION) as isize)
        //} else {
        //None
        //}
    }
}

fn main() {
    env_logger::init();

    let mut rng = SmallRng::from_entropy();
    let genotype = MultiContinuousGenotype::builder()
        .with_allele_ranges(vec![(-10.0..10.0), (0.0..10.0)])
        .build()
        .unwrap();

    let evolve_builder = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(1000)
        .with_max_stale_generations(100)
        .with_target_fitness_score(TARGET_SCORE)
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_mutate(MutateOnce::new(0.4))
        .with_fitness(MILPFitness)
        .with_crossover(CrossoverSingleGene(true))
        .with_compete(CompeteElite)
        .with_extension(ExtensionNoop);

    for _ in 0..10 {
        let now = std::time::Instant::now();
        let evolve = evolve_builder.clone().call(&mut rng).unwrap();
        let duration = now.elapsed();

        if let Some(best_chromosome) = evolve.best_chromosome() {
            if let Some(fitness_score) = best_chromosome.fitness_score {
                let x1 = best_chromosome.genes[0];
                let x2 = best_chromosome.genes[1].floor();
                let result = 8.0 * x1 + x2;

                println!(
                    "x1: {:.5}, x2: {} = {:.5} (fitness score: {: >5}, duration {:?})",
                    x1, x2 as u8, result, fitness_score, duration
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
