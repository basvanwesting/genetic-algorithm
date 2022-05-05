use criterion::{black_box, criterion_group, criterion_main, Criterion};

use genetic_algorithm::chromosome::Chromosome;
use genetic_algorithm::fitness::{
    Fitness, FitnessSimpleCount, FitnessSimpleSumContinuousGenotype, FitnessSimpleSumIndexGenotype,
};
use genetic_algorithm::genotype::{BinaryGenotype, ContinuousGenotype, IndexGenotype};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fitness_simple_sum_binary", |b| {
        let chromosome = Chromosome::<BinaryGenotype>::new(vec![true; 1000]);
        let mut simple_sum = FitnessSimpleCount;
        b.iter(|| simple_sum.call_for_chromosome(black_box(&chromosome)))
    });

    c.bench_function("fitness_simple_sum_index", |b| {
        let chromosome = Chromosome::<IndexGenotype>::new(vec![1; 1000]);
        let mut simple_sum = FitnessSimpleSumIndexGenotype;
        b.iter(|| simple_sum.call_for_chromosome(black_box(&chromosome)))
    });

    c.bench_function("fitness_simple_sum_continuous", |b| {
        let chromosome = Chromosome::<ContinuousGenotype>::new(vec![1.0; 1000]);
        let mut simple_sum = FitnessSimpleSumContinuousGenotype;
        b.iter(|| simple_sum.call_for_chromosome(black_box(&chromosome)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
