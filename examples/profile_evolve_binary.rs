extern crate criterion;
use criterion::{criterion_group, criterion_main, Criterion};

extern crate pprof;
use pprof::criterion::{Output, PProfProfiler};

use genetic_algorithm::compete;
use genetic_algorithm::context::Context;
use genetic_algorithm::crossover;
use genetic_algorithm::evolve::Evolve;
use genetic_algorithm::fitness;
use genetic_algorithm::mutate;

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
    let context = Context::new()
        .with_gene_size(100)
        .with_gene_values(vec![true, false])
        .with_population_size(1000);

    let _evolve = Evolve::new(context)
        .with_max_stale_generations(20)
        .with_target_fitness_score(100)
        .with_mutate(mutate::SingleGene(0.2))
        .with_fitness(fitness::SimpleSum)
        .with_crossover(crossover::Individual)
        //.with_compete(compete::Tournament(4))
        .with_compete(compete::Elite)
        .call();
}
