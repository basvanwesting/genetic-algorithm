use distance::hamming;
use genetic_algorithm::strategy::evolve::prelude::*;
use rand::prelude::*;
use rand::rngs::SmallRng;

// see https://en.wikipedia.org/wiki/Infinite_monkey_theorem

const TARGET_TEXT: &str =
  "Be not afraid of greatness! Some are great, some achieve greatness, and some have greatness thrust upon 'em.";

// printable chars
const MIN_CHAR: char = ' '; // 0x20;
const MAX_CHAR: char = '~'; // 0x7e;

#[derive(Clone, Debug)]
struct MonkeyFitness;
impl Fitness for MonkeyFitness {
    type Genotype = DiscreteGenotype<char>;
    fn calculate_for_chromosome(
        &mut self,
        chromosome: &Chromosome<Self::Genotype>,
    ) -> Option<FitnessValue> {
        let string = String::from_iter(chromosome.genes.clone());
        println!("{}", string);
        Some(hamming(&string, TARGET_TEXT).unwrap() as FitnessValue)
    }
}

fn main() {
    env_logger::init();

    let mut rng = SmallRng::from_entropy();
    let genotype = DiscreteGenotype::builder()
        .with_genes_size(TARGET_TEXT.len())
        .with_allele_list((MIN_CHAR..=MAX_CHAR).collect())
        .build()
        .unwrap();

    println!("{}", genotype);

    let mut evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(20)
        .with_max_stale_generations(10000)
        .with_fitness(MonkeyFitness)
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_target_fitness_score(0)
        .with_mutate(MutateOnce::new(0.3))
        .with_crossover(CrossoverClone::new(true))
        .with_compete(CompeteElite)
        .with_extension(ExtensionNoop)
        .build()
        .unwrap();

    let now = std::time::Instant::now();
    evolve.call(&mut rng);
    let duration = now.elapsed();

    println!("{}", evolve);

    if let Some(best_chromosome) = evolve.best_chromosome() {
        if let Some(fitness_score) = best_chromosome.fitness_score {
            if fitness_score == 0 {
                let string = String::from_iter(best_chromosome.genes);
                println!("{}", string);
            } else {
                println!("Wrong solution with fitness score: {}", fitness_score);
            }
        } else {
            println!("Invalid solution with fitness score: None");
        }
    }
    println!("{:?}", duration);
}
