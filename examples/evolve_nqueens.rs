use genetic_algorithm::strategy::evolve::prelude::*;

// see https://en.wikipedia.org/wiki/Eight_queens_puzzle
#[derive(Clone, Debug)]
struct NQueensFitness;
impl Fitness for NQueensFitness {
    type Genotype = UniqueGenotype<u8>;
    fn calculate_for_chromosome(
        &mut self,
        chromosome: &FitnessChromosome<Self>,
        _genotype: &FitnessGenotype<Self>,
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
        .with_replace_on_equal_fitness(true)
        .with_mutate(MutateSingleGene::new(1.0))
        .with_crossover(CrossoverRejuvenate::new(1.0))
        .with_select(SelectElite::new(1.0, 0.0))
        .with_reporter(EvolveReporterSimple::new(100))
        .build()
        .unwrap();

    evolve.call();
    // println!("{}", evolve);

    if let Some((best_genes, fitness_score)) = evolve.best_genes_and_fitness_score() {
        if fitness_score == 0 {
            for gene in best_genes {
                let mut chars: Vec<char> = (0..BOARD_SIZE).map(|_| '.').collect();
                chars[gene as usize] = 'X';
                println!("{}", String::from_iter(chars));
            }
            println!("Valid solution with fitness score: {}", fitness_score);
        } else {
            println!("Wrong solution with fitness score: {}", fitness_score);
        }
    } else {
        println!("Invalid solution with fitness score: None");
    }

    println!("The Evolve strategy is very inefficient for this problem as there is only mutation and no crossover. HillClimb works much better for this case");
}
