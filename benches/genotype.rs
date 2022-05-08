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
            .build()
            .unwrap();
        let mut chromosome = genotype.chromosome_factory(&mut rng);
        b.iter(|| genotype.mutate_chromosome(black_box(&mut chromosome), &mut rng))
    });

    c.bench_function("mutate_chromosome_index", |b| {
        let mut rng = SmallRng::from_entropy();
        let genotype = IndexGenotype::builder()
            .with_gene_size(100)
            .with_gene_value_size(10)
            .build()
            .unwrap();
        let mut chromosome = genotype.chromosome_factory(&mut rng);
        b.iter(|| genotype.mutate_chromosome(black_box(&mut chromosome), &mut rng))
    });

    c.bench_function("mutate_chromosome_unique_index", |b| {
        let mut rng = SmallRng::from_entropy();
        let genotype = UniqueIndexGenotype::builder()
            .with_gene_value_size(10)
            .build()
            .unwrap();
        let mut chromosome = genotype.chromosome_factory(&mut rng);
        b.iter(|| genotype.mutate_chromosome(black_box(&mut chromosome), &mut rng))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
