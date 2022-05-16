use distance::hamming;
use genetic_algorithm::evolve::prelude::*;
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
    let mut rng = SmallRng::from_entropy();
    let genotype = DiscreteGenotype::builder()
        .with_gene_size(TARGET_TEXT.len())
        .with_gene_values((MIN_CHAR..=MAX_CHAR).collect())
        .build()
        .unwrap();

    println!("{}", genotype);

    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_population_size(100)
        .with_max_stale_generations(1000)
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_target_fitness_score(0)
        .with_mutate(MutateOnce(0.2))
        .with_fitness(MonkeyFitness)
        .with_crossover(CrossoverRange(true))
        .with_compete(CompeteTournament(4))
        .call(&mut rng)
        .unwrap();

    println!("{}", evolve);

    if let Some(best_chromosome) = evolve.best_chromosome {
        let string = String::from_iter(best_chromosome.genes);
        println!("{}", string);
    }
}
