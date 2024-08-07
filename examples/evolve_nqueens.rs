use genetic_algorithm::strategy::evolve::prelude::*;
use rand::prelude::*;
use rand::rngs::SmallRng;

// see https://en.wikipedia.org/wiki/Eight_queens_puzzle
#[derive(Clone, Debug)]
struct NQueensFitness;
impl Fitness for NQueensFitness {
    type Genotype = UniqueGenotype<u8>;
    fn calculate_for_chromosome(
        &mut self,
        chromosome: &Chromosome<Self::Genotype>,
    ) -> Option<FitnessValue> {
        let mut score = 0;
        let max_index = chromosome.genes.len() - 1;
        for i in 0..max_index {
            for j in 0..max_index {
                if i != j {
                    let dx = i.abs_diff(j);
                    let dy = chromosome.genes[i].abs_diff(chromosome.genes[j]) as usize;
                    if dx == dy {
                        //diagonal clash
                        score += 1;
                    }
                }
            }
        }
        Some(score)
    }
}

fn main() {
    env_logger::init();

    const BOARD_SIZE: u8 = 64;

    let mut rng = SmallRng::from_entropy();
    let genotype = UniqueGenotype::builder()
        .with_allele_list((0..BOARD_SIZE).collect())
        .build()
        .unwrap();

    println!("{}", genotype);

    let mut evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(20)
        .with_max_stale_generations(10000)
        .with_fitness(NQueensFitness)
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_target_fitness_score(0)
        .with_mutate(MutateSingleGeneRandom::new(0.2))
        .with_crossover(CrossoverClone::new(true))
        .with_compete(CompeteElite::new())
        .build()
        .unwrap();

    let now = std::time::Instant::now();
    evolve.call(&mut rng);
    let duration = now.elapsed();

    println!("{}", evolve);

    if let Some(best_chromosome) = evolve.best_chromosome() {
        if let Some(fitness_score) = best_chromosome.fitness_score {
            if fitness_score == 0 {
                for gene in best_chromosome.genes {
                    let mut chars: Vec<char> = (0..BOARD_SIZE).map(|_| '.').collect();
                    chars[gene as usize] = 'X';
                    println!("{}", String::from_iter(chars));
                }
            } else {
                println!("Wrong solution with fitness score: {}", fitness_score);
            }
        } else {
            println!("Invalid solution with fitness score: None");
        }
    }
    println!("{:?}", duration);
}
