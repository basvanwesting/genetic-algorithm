use criterion::{criterion_group, criterion_main, BatchSize, Criterion};

use genetic_algorithm::compete;
use genetic_algorithm::compete::Compete;
use genetic_algorithm::fitness;
use genetic_algorithm::fitness::Fitness;
use genetic_algorithm::genotype::Genotype;
use genetic_algorithm::population::Population;
use rand::prelude::*;
use rand::rngs::SmallRng;

pub fn criterion_benchmark(c: &mut Criterion) {
    let source_population_size = 2000;
    let target_population_size = 1000;
    let mut rng = SmallRng::from_entropy();
    let genotype = Genotype::new()
        .with_gene_size(10)
        .with_gene_values(vec![true, false]);

    let chromosomes = (0..source_population_size)
        .map(|_| genotype.random_chromosome_factory(&mut rng))
        .collect();

    let population = Population::new(chromosomes);
    let population = fitness::SimpleSum.call_for_population(population);

    println!(
        "start population size: {}, target population size: {}",
        population.size(),
        target_population_size,
    );

    c.bench_function("compete_tournament", |b| {
        let compete = compete::Tournament(4);
        b.iter_batched(
            || population.clone(),
            |data| compete.call(&genotype, data, target_population_size, &mut rng),
            BatchSize::SmallInput,
        )
    });

    c.bench_function("compete_elite", |b| {
        let compete = compete::Elite;
        b.iter_batched(
            || population.clone(),
            |data| compete.call(&genotype, data, target_population_size, &mut rng),
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
