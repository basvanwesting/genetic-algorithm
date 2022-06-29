use criterion::*;
use genetic_algorithm::chromosome::Chromosome;
use genetic_algorithm::fitness::placeholders::{
    CountTrue, SumContinuousGenotype, SumDiscreteGenotype,
};
use genetic_algorithm::fitness::Fitness;
use genetic_algorithm::genotype::{BinaryGenotype, ContinuousGenotype, DiscreteGenotype};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("fitness");

    group.bench_function("count_true", |b| {
        let chromosome = Chromosome::<BinaryGenotype>::new(vec![true; 1000]);
        let mut sum = CountTrue;
        b.iter(|| sum.calculate_for_chromosome(black_box(&chromosome)))
    });

    group.bench_function("sum_continuous_genotype", |b| {
        let chromosome = Chromosome::<ContinuousGenotype>::new(vec![1.0; 1000]);
        let mut sum = SumContinuousGenotype(1e-5);
        b.iter(|| sum.calculate_for_chromosome(black_box(&chromosome)))
    });

    group.bench_function("sum_discrete_genotype", |b| {
        let chromosome = Chromosome::<DiscreteGenotype>::new(vec![1; 1000]);
        let mut sum = SumDiscreteGenotype;
        b.iter(|| sum.calculate_for_chromosome(black_box(&chromosome)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
