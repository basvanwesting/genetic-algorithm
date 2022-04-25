use criterion::{black_box, criterion_group, criterion_main, Criterion};

use genetic_algorithm::genotype::*;
use rand::prelude::*;
use rand::rngs::SmallRng;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("mutate_chromosome binary", |b| {
        let mut rng = SmallRng::from_entropy();
        let genotype = BinaryGenotype::new().with_gene_size(100).build();
        let mut chromosome = genotype.chromosome_factory(&mut rng);
        b.iter(|| genotype.mutate_chromosome(black_box(&mut chromosome), &mut rng))
    });

    c.bench_function("mutate_chromosome discrete", |b| {
        let mut rng = SmallRng::from_entropy();
        let genotype = DiscreteGenotype::new()
            .with_gene_size(100)
            .with_gene_values(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9])
            .build();
        let mut chromosome = genotype.chromosome_factory(&mut rng);
        b.iter(|| genotype.mutate_chromosome(black_box(&mut chromosome), &mut rng))
    });

    c.bench_function("mutate_chromosome discrete_unique", |b| {
        let mut rng = SmallRng::from_entropy();
        let genotype = DiscreteUniqueGenotype::new()
            .with_gene_values(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9])
            .build();
        let mut chromosome = genotype.chromosome_factory(&mut rng);
        b.iter(|| genotype.mutate_chromosome(black_box(&mut chromosome), &mut rng))
    });

    c.bench_function("mutate_chromosome range", |b| {
        let mut rng = SmallRng::from_entropy();
        let genotype = RangeGenotype::new()
            .with_gene_size(100)
            .with_gene_range(0..100)
            .build();
        let mut chromosome = genotype.chromosome_factory(&mut rng);
        b.iter(|| genotype.mutate_chromosome(black_box(&mut chromosome), &mut rng))
    });

    c.bench_function("mutate_chromosome range_unique", |b| {
        let mut rng = SmallRng::from_entropy();
        let genotype = RangeUniqueGenotype::new().with_gene_range(0..100).build();
        let mut chromosome = genotype.chromosome_factory(&mut rng);
        b.iter(|| genotype.mutate_chromosome(black_box(&mut chromosome), &mut rng))
    });

    c.bench_function("mutate_chromosome continuous", |b| {
        let mut rng = SmallRng::from_entropy();
        let genotype = ContinuousGenotype::new().with_gene_size(100).build();
        let mut chromosome = genotype.chromosome_factory(&mut rng);
        b.iter(|| genotype.mutate_chromosome(black_box(&mut chromosome), &mut rng))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
