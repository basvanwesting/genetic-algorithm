use distance::hamming;
use genetic_algorithm::chromosome::Chromosome;
use genetic_algorithm::compete::CompeteTournament;
use genetic_algorithm::crossover::CrossoverRange;
use genetic_algorithm::evolve::Evolve;
use genetic_algorithm::fitness::{Fitness, FitnessOrdering, FitnessValue};
use genetic_algorithm::genotype::{DiscreteGenotype, Genotype};
use genetic_algorithm::mutate::MutateOnce;
use rand::prelude::*;
use rand::rngs::SmallRng;

// see https://en.wikipedia.org/wiki/Infinite_monkey_theorem

const TARGET_TEXT: &str =
  "Be not afraid of greatness! Some are great, some achieve greatness, and some have greatness thrust upon 'em.";

const MIN_CHAR: u8 = 0x20;
const MAX_CHAR: u8 = 0x7e;

#[derive(Clone, Debug)]
struct MyGeneFitness;
impl Fitness for MyGeneFitness {
    type Genotype = DiscreteGenotype<u8>;
    fn call_for_chromosome(
        &mut self,
        chromosome: &Chromosome<Self::Genotype>,
    ) -> Option<FitnessValue> {
        let string = String::from_utf8(chromosome.genes.clone()).unwrap();
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
        .with_fitness(MyGeneFitness)
        .with_crossover(CrossoverRange(true))
        .with_compete(CompeteTournament(4))
        .build()
        .unwrap()
        .call(&mut rng);

    println!("{}", evolve);
}
