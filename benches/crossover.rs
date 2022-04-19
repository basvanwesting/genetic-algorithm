use criterion::{criterion_group, criterion_main, BatchSize, Criterion};

use genetic_algorithm::context::Context;
use genetic_algorithm::crossover;
use genetic_algorithm::crossover::Crossover;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut context = Context::new()
        .with_gene_size(100)
        .with_population_size(1000)
        .with_gene_values(vec![true, false]);
    let population = context.random_population_factory();
    println!("population size: {}", population.size());

    c.bench_function("crossover_individual", |b| {
        let crossover = crossover::Individual(false);
        b.iter_batched(
            || population.clone(),
            |data| crossover.call(&mut context, data),
            BatchSize::SmallInput,
        )
    });

    c.bench_function("crossover_all", |b| {
        let crossover = crossover::All(false);
        b.iter_batched(
            || population.clone(),
            |data| crossover.call(&mut context, data),
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
