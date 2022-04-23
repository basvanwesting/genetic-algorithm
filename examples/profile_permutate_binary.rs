extern crate criterion;
use criterion::{criterion_group, criterion_main, Criterion};

extern crate pprof;
use pprof::criterion::{Output, PProfProfiler};

use genetic_algorithm::genotype::Genotype;
use genetic_algorithm::fitness;
use genetic_algorithm::permutate::Permutate;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("profile_permutate_binary", |b| b.iter(|| run()));
}

criterion_group! {
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = criterion_benchmark
}
criterion_main!(benches);

fn run() {
    let genotype = Genotype::new()
        .with_gene_size(16)
        .with_gene_values(vec![true, false]);

    let permutate = Permutate::new(genotype)
        .with_fitness(fitness::SimpleSum)
        .call();

    println!("{}", permutate);
}
