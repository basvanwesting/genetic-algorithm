use criterion::{criterion_group, criterion_main, BatchSize, Criterion};

use genetic_algorithm::compete;
use genetic_algorithm::compete::Compete;
use genetic_algorithm::context::Context;
use genetic_algorithm::fitness;
use genetic_algorithm::fitness::Fitness;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut context = Context::new()
        .with_gene_size(10)
        .with_gene_values(vec![true, false]);
    let mut population = context.permutation_population_factory();
    let mut population_overshoot = context.permutation_population_factory();
    population.merge(&mut population_overshoot);
    fitness::SimpleSum.call_for_population(&mut population);
    println!("population size: {}", population.size());

    c.bench_function("compete_tournament", |b| {
        let tournament = compete::Tournament(4);
        b.iter_batched(
            || population.clone(),
            |data| tournament.call(&mut context, data),
            BatchSize::SmallInput,
        )
    });

    c.bench_function("compete_elite", |b| {
        let elite = compete::Elite;
        b.iter_batched(
            || population.clone(),
            |data| elite.call(&mut context, data),
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
