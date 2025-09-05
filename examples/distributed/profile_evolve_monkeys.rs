use criterion::*;
use pprof::criterion::*;

use distance::hamming;
use genetic_algorithm::distributed::strategy::evolve::prelude::*;

// see https://en.wikipedia.org/wiki/Infinite_monkey_theorem

const TARGET_TEXT: &str =
  "Be not afraid of greatness! Some are great, some achieve greatness, and some have greatness thrust upon 'em.";

// printable chars
const MIN_CHAR: char = ' '; // 0x20;
const MAX_CHAR: char = '~'; // 0x7e;

#[derive(Clone, Debug)]
struct MonkeyFitness;
impl Fitness for MonkeyFitness {
    type Genotype = ListGenotype<char>;
    fn calculate_for_chromosome(
        &mut self,
        chromosome: &FitnessChromosome<Self>,
        _genotype: &FitnessGenotype<Self>,
    ) -> Option<FitnessValue> {
        let string = String::from_iter(chromosome.genes.clone());
        Some(hamming(&string, TARGET_TEXT).unwrap() as FitnessValue)
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let genotype = ListGenotype::builder()
        .with_genes_size(TARGET_TEXT.len())
        .with_allele_list((MIN_CHAR..MAX_CHAR).collect())
        .build()
        .unwrap();

    let evolve_builder = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(100)
        .with_max_stale_generations(1000)
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_target_fitness_score(0)
        .with_mutate(MutateSingleGene::new(0.2))
        .with_fitness(MonkeyFitness)
        .with_crossover(CrossoverSinglePoint::new(0.7, 0.8))
        .with_select(SelectTournament::new(0.5, 0.02, 4));

    c.bench_function("profile_evolve_monkeys", |b| {
        b.iter_batched(
            || evolve_builder.clone().build().unwrap(),
            |mut e| e.call(),
            BatchSize::SmallInput,
        );
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = criterion_benchmark
}

criterion_main!(benches);
