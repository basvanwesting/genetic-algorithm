use criterion::{criterion_group, criterion_main, Criterion};
use pprof::criterion::{Output, PProfProfiler};

use distance::hamming;
use genetic_algorithm::evolve::prelude::*;
use rand::prelude::*;
use rand::rngs::SmallRng;

// see https://en.wikipedia.org/wiki/Infinite_monkey_theorem

const TARGET_TEXT: &str =
    "Some are great, some achieve greatness, and some have greatness thrust upon 'em.";

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
        Some(hamming(&string, TARGET_TEXT).unwrap() as FitnessValue)
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("profile_evolve_monkeys", |b| b.iter(|| run()));
}

criterion_group! {
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = criterion_benchmark
}
criterion_main!(benches);

fn run() {
    let mut rng = SmallRng::from_entropy();
    let genotype = DiscreteGenotype::builder()
        .with_genes_size(TARGET_TEXT.len())
        .with_allele_values((MIN_CHAR..=MAX_CHAR).collect())
        .build()
        .unwrap();

    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_population_size(100)
        .with_max_stale_generations(1000)
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_target_fitness_score(0)
        .with_mutate(MutateOnce(0.2))
        .with_fitness(MonkeyFitness)
        .with_crossover(CrossoverSinglePoint(true))
        .with_compete(CompeteTournament(4))
        .call(&mut rng)
        .unwrap();

    println!("{}", evolve);
}
