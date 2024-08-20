use criterion::*;
use genetic_algorithm::chromosome::Chromosome;
use genetic_algorithm::fitness::placeholders::{CountTrue, SumGenes};
use genetic_algorithm::fitness::Fitness;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("fitness");

    group.bench_function("count_true", |b| {
        let chromosome = Chromosome::new(vec![true; 1000]);
        let mut sum = CountTrue;
        b.iter(|| sum.calculate_for_chromosome(black_box(&chromosome)))
    });

    group.bench_function("sum_genes_with_precision", |b| {
        let chromosome = Chromosome::new(vec![1.0; 1000]);
        let mut sum = SumGenes::new_with_precision(1e-5);
        b.iter(|| sum.calculate_for_chromosome(black_box(&chromosome)))
    });

    group.bench_function("sum_genes", |b| {
        let chromosome = Chromosome::new(vec![1; 1000]);
        let mut sum = SumGenes::new();
        b.iter(|| sum.calculate_for_chromosome(black_box(&chromosome)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
