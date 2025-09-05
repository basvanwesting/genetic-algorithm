use distance::hamming;
use genetic_algorithm::centralized::strategy::evolve::prelude::*;

// see https://en.wikipedia.org/wiki/Infinite_monkey_theorem

const TARGET_TEXT: &str =
  "Be not afraid of greatness! Some are great, some achieve greatness, and some have greatness thrust upon 'em.";

// printable chars
const MIN_CHAR: char = ' '; // 0x20;
const MAX_CHAR: char = '~'; // 0x7e;

#[derive(Clone, Debug)]
struct MonkeyFitness {
    counter: usize,
    period: usize,
}
impl MonkeyFitness {
    pub fn new(period: usize) -> Self {
        Self { counter: 0, period }
    }
}
impl Fitness for MonkeyFitness {
    type Genotype = ListGenotype<char>;
    fn calculate_for_chromosome(
        &mut self,
        chromosome: &FitnessChromosome<Self>,
        _genotype: &FitnessGenotype<Self>,
    ) -> Option<FitnessValue> {
        let string = String::from_iter(chromosome.genes.clone());
        if self.counter % self.period == 0 {
            println!("{} ({})", string, self.counter);
        }
        self.counter += 1;
        Some(hamming(&string, TARGET_TEXT).unwrap() as FitnessValue)
    }
}

fn main() {
    env_logger::init();

    let genotype = ListGenotype::builder()
        .with_genes_size(TARGET_TEXT.len())
        .with_allele_list((MIN_CHAR..MAX_CHAR).collect())
        .build()
        .unwrap();

    println!("{}", genotype);

    let mut evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(20)
        .with_max_stale_generations(10000)
        .with_fitness(MonkeyFitness::new(10000))
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_target_fitness_score(0)
        .with_mutate(MutateSingleGene::new(0.3))
        // .with_mutate(MutateSingleGeneDynamic::new(0.01, 2))
        .with_crossover(CrossoverUniform::new(0.8, 0.9))
        .with_select(SelectElite::new(0.5, 0.02))
        .with_reporter(EvolveReporterDuration::new())
        .build()
        .unwrap();

    evolve.call();
    // println!("{}", evolve);

    if let Some((best_genes, fitness_score)) = evolve.best_genes_and_fitness_score() {
        let string = String::from_iter(best_genes);
        if fitness_score == 0 {
            println!("Valid solution with fitness score: {}", fitness_score);
            println!("{}", string);
        } else {
            println!("Wrong solution with fitness score: {}", fitness_score);
            println!("{}", string);
        }
    } else {
        println!("Invalid solution with fitness score: None");
    }
}
