use criterion::*;
use genetic_algorithm::chromosome::Chromosome;
use genetic_algorithm::fitness::placeholders::{CountTrue, CountTrueWithSleep, SumGenes};
use genetic_algorithm::fitness::Fitness;
use genetic_algorithm::genotype::{BinaryGenotype, Genotype};
use genetic_algorithm::population::Population;
use rand::prelude::*;
use rand::rngs::SmallRng;
use thread_local::ThreadLocal;

pub fn placeholders_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("fitness-placeholders");

    group.bench_function("count_true", |b| {
        let chromosome = Chromosome::new(vec![true; 1000]);
        let mut fitness = CountTrue;
        b.iter(|| fitness.calculate_for_chromosome(black_box(&chromosome)))
    });

    group.bench_function("sum_genes_with_precision", |b| {
        let chromosome = Chromosome::new(vec![1.0; 1000]);
        let mut fitness = SumGenes::new_with_precision(1e-5);
        b.iter(|| fitness.calculate_for_chromosome(black_box(&chromosome)))
    });

    group.bench_function("sum_genes", |b| {
        let chromosome = Chromosome::new(vec![1; 1000]);
        let mut fitness = SumGenes::new();
        b.iter(|| fitness.calculate_for_chromosome(black_box(&chromosome)))
    });
}

pub fn multithreading_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("fitness-multithreading");

    let mut rng = SmallRng::from_entropy();
    let genotype = BinaryGenotype::builder()
        .with_genes_size(100)
        .build()
        .unwrap();

    let chromosomes = (0..100)
        .map(|_| genotype.chromosome_factory(&mut rng))
        .collect();
    let population = Population::new(chromosomes);

    group.sample_size(30);
    group.bench_function("fitness-CountTrueWithSleep-single-threaded", |b| {
        let mut fitness = CountTrueWithSleep::new(1000, true);
        b.iter_batched(
            || population.clone(),
            |mut data| {
                fitness.call_for_population(&mut data, None);
            },
            BatchSize::SmallInput,
        );
    });

    // reuse thread fitness for all runs (as in evolve loop)
    let fitness_thread_local = Some(ThreadLocal::new());
    group.sample_size(300);
    group.bench_function("fitness-CountTrueWithSleep-multi-threaded", |b| {
        let mut fitness = CountTrueWithSleep::new(1000, true);
        b.iter_batched(
            || population.clone(),
            |mut data| {
                // println!("run benchmark batch");
                fitness.call_for_population(&mut data, fitness_thread_local.as_ref());
            },
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, placeholders_benchmark, multithreading_benchmark);
criterion_main!(benches);
