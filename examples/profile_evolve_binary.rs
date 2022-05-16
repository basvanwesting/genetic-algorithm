use criterion::{criterion_group, criterion_main, Criterion};
use pprof::criterion::{Output, PProfProfiler};

use genetic_algorithm::evolve::prelude::*;
use genetic_algorithm::fitness::placeholders::CountTrue;
use rand::prelude::*;
use rand::rngs::SmallRng;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("profile_evolve_binary", |b| b.iter(|| run()));
}

criterion_group! {
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = criterion_benchmark
}
criterion_main!(benches);

fn run() {
    let mut rng = SmallRng::from_entropy();
    let genotype = BinaryGenotype::builder()
        .with_gene_size(100)
        .build()
        .unwrap();

    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_population_size(100)
        .with_max_stale_generations(1000)
        .with_target_fitness_score(100)
        .with_mutate(MutateOnce(0.2))
        .with_fitness(CountTrue)
        .with_crossover(CrossoverAll(true))
        .with_compete(CompeteTournament(4))
        .call(&mut rng)
        .unwrap();

    println!("{}", evolve);
}
