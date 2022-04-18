use criterion::{black_box, criterion_group, criterion_main, Criterion};

use genetic_algorithm::chromosome::Chromosome;
use genetic_algorithm::fitness;
use genetic_algorithm::fitness::Fitness;
use genetic_algorithm::gene::{BinaryGene, ContinuousGene, DiscreteGene};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fitness_simple_sum_binary", |b| {
        let chromosome = Chromosome::<BinaryGene>::new(vec![true; 1000]);
        let simple_sum_binary = fitness::SimpleSum;
        b.iter(|| simple_sum_binary.call_for_chromosome(black_box(&chromosome)))
    });

    c.bench_function("fitness_simple_sum_discrete", |b| {
        let chromosome = Chromosome::<DiscreteGene>::new(vec![1; 1000]);
        let simple_sum_discrete = fitness::SimpleSum;
        b.iter(|| simple_sum_discrete.call_for_chromosome(black_box(&chromosome)))
    });

    c.bench_function("fitness_simple_sum_continuous", |b| {
        let chromosome = Chromosome::<ContinuousGene>::new(vec![1.0; 1000]);
        let simple_sum_continuous = fitness::SimpleSum;
        b.iter(|| simple_sum_continuous.call_for_chromosome(black_box(&chromosome)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
