use criterion::{black_box, criterion_group, criterion_main, Criterion};

use genetic_algorithm::chromosome::Chromosome;
use genetic_algorithm::fitness;
use genetic_algorithm::fitness::Fitness;
use genetic_algorithm::gene::DiscreteGene;
use genetic_algorithm::genotype::{BinaryGenotype, ContinuousGenotype, DiscreteGenotype};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fitness_simple_sum_binary", |b| {
        let chromosome = Chromosome::<BinaryGenotype>::new(vec![true; 1000]);
        let simple_sum_binary = fitness::SimpleSumBinaryGenotype;
        b.iter(|| simple_sum_binary.call_for_chromosome(black_box(&chromosome)))
    });

    c.bench_function("fitness_simple_sum_discrete", |b| {
        let chromosome = Chromosome::<DiscreteGenotype<DiscreteGene>>::new(vec![1; 1000]);
        let simple_sum_discrete = fitness::SimpleSumDiscreteGenotypeDiscreteGene;
        b.iter(|| simple_sum_discrete.call_for_chromosome(black_box(&chromosome)))
    });

    c.bench_function("fitness_simple_sum_continuous", |b| {
        let chromosome = Chromosome::<ContinuousGenotype>::new(vec![1.0; 1000]);
        let simple_sum_continuous = fitness::SimpleSumContinuousGenotype;
        b.iter(|| simple_sum_continuous.call_for_chromosome(black_box(&chromosome)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
