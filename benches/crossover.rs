use criterion::{criterion_group, criterion_main, BatchSize, Criterion};

use genetic_algorithm::crossover;
use genetic_algorithm::crossover::Crossover;
use genetic_algorithm::genotype::{BinaryGenotype, Genotype};
use genetic_algorithm::population::Population;
use rand::prelude::*;
use rand::rngs::SmallRng;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = SmallRng::from_entropy();
    let genotype = BinaryGenotype::new().with_gene_size(100).build();

    let chromosomes = (0..1000)
        .map(|_| genotype.chromosome_factory(&mut rng))
        .collect();

    let population = Population::new(chromosomes);

    println!("population size: {}", population.size());

    c.bench_function("crossover_individual", |b| {
        let crossover = CrossoverIndividual(false);
        b.iter_batched(
            || population.clone(),
            |data| crossover.call(&genotype, data, &mut rng),
            BatchSize::SmallInput,
        )
    });

    c.bench_function("crossover_all", |b| {
        let crossover = CrossoverAll(false);
        b.iter_batched(
            || population.clone(),
            |data| crossover.call(&genotype, data, &mut rng),
            BatchSize::SmallInput,
        )
    });

    c.bench_function("crossover_range", |b| {
        let crossover = CrossoverRange(false);
        b.iter_batched(
            || population.clone(),
            |data| crossover.call(&genotype, data, &mut rng),
            BatchSize::SmallInput,
        )
    });

    c.bench_function("crossover_cloning", |b| {
        let crossover = CrossoverCloning(false);
        b.iter_batched(
            || population.clone(),
            |data| crossover.call(&genotype, data, &mut rng),
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
