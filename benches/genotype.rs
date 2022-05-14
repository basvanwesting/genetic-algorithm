use criterion::{black_box, criterion_group, criterion_main, Criterion};

use genetic_algorithm::genotype::*;
use rand::prelude::*;
use rand::rngs::SmallRng;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("mutate_chromosome_binary", |b| {
        let mut rng = SmallRng::from_entropy();
        let genotype = BinaryGenotype::builder()
            .with_gene_size(100)
            .build()
            .unwrap();
        let mut chromosome = genotype.chromosome_factory(&mut rng);
        b.iter(|| genotype.mutate_chromosome(black_box(&mut chromosome), &mut rng))
    });

    c.bench_function("mutate_chromosome_continuous", |b| {
        let mut rng = SmallRng::from_entropy();
        let genotype = ContinuousGenotype::builder()
            .with_gene_size(100)
            .with_gene_range(0.0..1.0)
            .build()
            .unwrap();
        let mut chromosome = genotype.chromosome_factory(&mut rng);
        b.iter(|| genotype.mutate_chromosome(black_box(&mut chromosome), &mut rng))
    });

    c.bench_function("mutate_chromosome_discrete", |b| {
        let mut rng = SmallRng::from_entropy();
        let genotype = DiscreteGenotype::builder()
            .with_gene_size(100)
            .with_gene_values((0..10).collect())
            .build()
            .unwrap();
        let mut chromosome = genotype.chromosome_factory(&mut rng);
        b.iter(|| genotype.mutate_chromosome(black_box(&mut chromosome), &mut rng))
    });

    c.bench_function("mutate_chromosome_multi_continuous", |b| {
        let mut rng = SmallRng::from_entropy();
        let genotype = MultiContinuousGenotype::builder()
            .with_gene_multi_range(vec![
                (0.0..1.0),
                (0.0..2.0),
                (0.0..3.0),
                (0.0..4.0),
                (0.0..5.0),
                (0.0..6.0),
                (0.0..7.0),
                (0.0..8.0),
                (0.0..9.0),
                (0.0..10.0),
            ])
            .build()
            .unwrap();
        let mut chromosome = genotype.chromosome_factory(&mut rng);
        b.iter(|| genotype.mutate_chromosome(black_box(&mut chromosome), &mut rng))
    });

    c.bench_function("mutate_chromosome_multi_discrete", |b| {
        let mut rng = SmallRng::from_entropy();
        let genotype = MultiDiscreteGenotype::builder()
            .with_gene_multi_values(vec![
                (0..1).collect(),
                (0..2).collect(),
                (0..3).collect(),
                (0..4).collect(),
                (0..5).collect(),
                (0..6).collect(),
                (0..7).collect(),
                (0..8).collect(),
                (0..9).collect(),
                (0..10).collect(),
            ])
            .build()
            .unwrap();
        let mut chromosome = genotype.chromosome_factory(&mut rng);
        b.iter(|| genotype.mutate_chromosome(black_box(&mut chromosome), &mut rng))
    });

    c.bench_function("mutate_chromosome_unique_discrete", |b| {
        let mut rng = SmallRng::from_entropy();
        let genotype = UniqueDiscreteGenotype::builder()
            .with_gene_values((0..10).collect())
            .build()
            .unwrap();
        let mut chromosome = genotype.chromosome_factory(&mut rng);
        b.iter(|| genotype.mutate_chromosome(black_box(&mut chromosome), &mut rng))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
