use criterion::{black_box, criterion_group, criterion_main, Criterion};

use genetic_algorithm::chromosome::Chromosome;
use genetic_algorithm::fitness::{Fitness, FitnessCountTrue, FitnessSumContinuousGenotype};
use genetic_algorithm::genotype::{BinaryGenotype, ContinuousGenotype};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fitness_count_true_binary", |b| {
        let chromosome = Chromosome::<BinaryGenotype>::new(vec![true; 1000]);
        let mut sum = FitnessCountTrue;
        b.iter(|| sum.call_for_chromosome(black_box(&chromosome)))
    });

    c.bench_function("fitness_sum_continuous", |b| {
        let chromosome = Chromosome::<ContinuousGenotype>::new(vec![1.0; 1000]);
        let mut sum = FitnessSumContinuousGenotype;
        b.iter(|| sum.call_for_chromosome(black_box(&chromosome)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
