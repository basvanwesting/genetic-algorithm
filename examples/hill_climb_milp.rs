use genetic_algorithm::strategy::hill_climb::prelude::*;
use rand::prelude::*;
use rand::rngs::SmallRng;

const LP: f32 = 59.0;
const PRECISION: f32 = 1e-5;

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

        if x1 + 2.0 * x2 >= 14.0 && -4.0 * x1 - x2 <= 33.0 && 2.0 * x1 + x2 <= 20.0 {
            let score = (8.0 * x1 + x2 - LP).abs();
            Some((score / PRECISION) as isize)
        } else {
            None
        }
    }
}

fn main() {
    let mut rng = SmallRng::from_entropy();
    let genotype = MultiContinuousGenotype::builder()
        .with_allele_multi_range(vec![(-100.0..100.0), (0.0..100.0)])
        .with_allele_multi_neighbour_range(vec![(-1.0..1.0), (-1.0..1.0)])
        .build()
        .unwrap();

    println!("genotype: {}", genotype);

    let hill_climb_builder = HillClimb::builder()
        .with_genotype(genotype)
        .with_variant(HillClimbVariant::SteepestPermutation)
        .with_max_stale_generations(1000)
        .with_scaling((1.0, 0.5))
        .with_target_fitness_score(0)
        .with_random_chromosome_probability(0.2)
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_fitness(MILPFitness);

    for _ in 0..10 {
        let now = std::time::Instant::now();
        let hill_climb = hill_climb_builder
            .clone()
            .call_repeatedly(1000, &mut rng)
            .unwrap();
        let duration = now.elapsed();

        if let Some(best_chromosome) = hill_climb.best_chromosome() {
            if let Some(fitness_score) = best_chromosome.fitness_score {
                let x1 = best_chromosome.genes[0];
                let x2 = best_chromosome.genes[1].floor();
                let result = 8.0 * x1 + x2;

                println!(
                    "x1: {:.5}, x2: {} = {:.5} (fitness score: {:>3}, best_iteration: {:>3}, best_generation: {:>5}, duration: {:?})",
                    x1, x2 as u8, result, fitness_score, hill_climb.current_iteration, hill_climb.best_generation, duration
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
