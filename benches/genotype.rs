use criterion::*;
use genetic_algorithm::chromosome::ChromosomeManager;
use genetic_algorithm::genotype::*;
use rand::prelude::*;
use rand::rngs::SmallRng;
//use std::time::Duration;

pub fn mutation_benchmark(c: &mut Criterion) {
    let mut rng = SmallRng::from_entropy();
    //let genes_sizes = vec![10, 100, 1000, 10000];
    let genes_sizes = vec![100];

    let mut group = c.benchmark_group("genotype-mutation");
    //group.warm_up_time(Duration::from_secs(3));
    //group.measurement_time(Duration::from_secs(3));
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);

    for genes_size in &genes_sizes {
        //group.throughput(Throughput::Elements(*genes_size as u64));
        let mut genotype = BinaryGenotype::builder()
            .with_genes_size(*genes_size)
            .build()
            .unwrap();
        let mut chromosome = genotype.chromosome_constructor(&mut rng);

        group.bench_function(
            BenchmarkId::new("binary-random-multi-10-with-duplicates", genes_size),
            |b| {
                b.iter(|| {
                    genotype.mutate_chromosome_genes(
                        10,
                        true,
                        black_box(&mut chromosome),
                        None,
                        &mut rng,
                    )
                })
            },
        );
        group.bench_function(
            BenchmarkId::new("binary-random-multi-10-without-duplicates", genes_size),
            |b| {
                b.iter(|| {
                    genotype.mutate_chromosome_genes(
                        10,
                        false,
                        black_box(&mut chromosome),
                        None,
                        &mut rng,
                    )
                })
            },
        );
    }

    for genes_size in &genes_sizes {
        //group.throughput(Throughput::Elements(*genes_size as u64));
        let mut genotype = ListGenotype::builder()
            .with_genes_size(*genes_size)
            .with_allele_list((0..10).collect())
            .build()
            .unwrap();
        let mut chromosome = genotype.chromosome_constructor(&mut rng);

        group.bench_function(
            BenchmarkId::new("list-random-multi-10-with-duplicates", genes_size),
            |b| {
                b.iter(|| {
                    genotype.mutate_chromosome_genes(
                        10,
                        true,
                        black_box(&mut chromosome),
                        None,
                        &mut rng,
                    )
                })
            },
        );
        group.bench_function(
            BenchmarkId::new("list-random-multi-10-without-duplicates", genes_size),
            |b| {
                b.iter(|| {
                    genotype.mutate_chromosome_genes(
                        10,
                        false,
                        black_box(&mut chromosome),
                        None,
                        &mut rng,
                    )
                })
            },
        );
    }

    for genes_size in &genes_sizes {
        //group.throughput(Throughput::Elements(*genes_size as u64));
        let mut genotype = RangeGenotype::builder()
            .with_genes_size(*genes_size)
            .with_allele_range(0.0..=1.0)
            .build()
            .unwrap();
        let mut chromosome = genotype.chromosome_constructor(&mut rng);

        group.bench_function(
            BenchmarkId::new("range-random-multi-10-with-duplicates", genes_size),
            |b| {
                b.iter(|| {
                    genotype.mutate_chromosome_genes(
                        10,
                        true,
                        black_box(&mut chromosome),
                        None,
                        &mut rng,
                    )
                })
            },
        );
        group.bench_function(
            BenchmarkId::new("range-random-multi-10-without-duplicates", genes_size),
            |b| {
                b.iter(|| {
                    genotype.mutate_chromosome_genes(
                        10,
                        false,
                        black_box(&mut chromosome),
                        None,
                        &mut rng,
                    )
                })
            },
        );

        let mut genotype = RangeGenotype::builder()
            .with_genes_size(*genes_size)
            .with_allele_range(0.0..=1.0)
            .with_allele_mutation_range(-0.1..=0.1)
            .build()
            .unwrap();
        let mut chromosome = genotype.chromosome_constructor(&mut rng);
        group.bench_function(
            BenchmarkId::new("range-relative-multi-10-with-duplicates", genes_size),
            |b| {
                b.iter(|| {
                    genotype.mutate_chromosome_genes(
                        10,
                        true,
                        black_box(&mut chromosome),
                        None,
                        &mut rng,
                    )
                })
            },
        );
        group.bench_function(
            BenchmarkId::new("range-relative-multi-10-without-duplicates", genes_size),
            |b| {
                b.iter(|| {
                    genotype.mutate_chromosome_genes(
                        10,
                        false,
                        black_box(&mut chromosome),
                        None,
                        &mut rng,
                    )
                })
            },
        );

        let mut genotype = RangeGenotype::builder()
            .with_genes_size(*genes_size)
            .with_allele_range(0.0..=1.0)
            .with_allele_mutation_scaled_range(vec![-0.1..=0.1, -0.01..=0.01, -0.001..=0.001])
            .build()
            .unwrap();
        let mut chromosome = genotype.chromosome_constructor(&mut rng);

        group.bench_function(
            BenchmarkId::new("range-scaled-multi-10-with-duplicates", genes_size),
            |b| {
                b.iter(|| {
                    genotype.mutate_chromosome_genes(
                        10,
                        true,
                        black_box(&mut chromosome),
                        Some(1),
                        &mut rng,
                    )
                })
            },
        );
        group.bench_function(
            BenchmarkId::new("range-scaled-multi-10-without-duplicates", genes_size),
            |b| {
                b.iter(|| {
                    genotype.mutate_chromosome_genes(
                        10,
                        false,
                        black_box(&mut chromosome),
                        Some(1),
                        &mut rng,
                    )
                })
            },
        );
    }

    for genes_size in &genes_sizes {
        //group.throughput(Throughput::Elements(*genes_size as u64));
        let mut genotype = UniqueGenotype::builder()
            .with_allele_list((0..*genes_size).collect())
            .build()
            .unwrap();
        let mut chromosome = genotype.chromosome_constructor(&mut rng);

        group.bench_function(
            BenchmarkId::new("unique-random-multi-10-with-duplicates", genes_size),
            |b| {
                b.iter(|| {
                    genotype.mutate_chromosome_genes(
                        10,
                        true,
                        black_box(&mut chromosome),
                        None,
                        &mut rng,
                    )
                })
            },
        );
        group.bench_function(
            BenchmarkId::new("unique-random-multi-10-without-duplicates", genes_size),
            |b| {
                b.iter(|| {
                    genotype.mutate_chromosome_genes(
                        10,
                        false,
                        black_box(&mut chromosome),
                        None,
                        &mut rng,
                    )
                })
            },
        );
    }

    for genes_size in &genes_sizes {
        //group.throughput(Throughput::Elements(*genes_size as u64));
        let mut genotype = MultiListGenotype::builder()
            .with_allele_lists((0..*genes_size).map(|_| (0..10).collect()).collect())
            .build()
            .unwrap();
        let mut chromosome = genotype.chromosome_constructor(&mut rng);

        group.bench_function(
            BenchmarkId::new("multi_list-random-multi-10-with-duplicates", genes_size),
            |b| {
                b.iter(|| {
                    genotype.mutate_chromosome_genes(
                        10,
                        true,
                        black_box(&mut chromosome),
                        None,
                        &mut rng,
                    )
                })
            },
        );
        group.bench_function(
            BenchmarkId::new("multi_list-random-multi-10-without-duplicates", genes_size),
            |b| {
                b.iter(|| {
                    genotype.mutate_chromosome_genes(
                        10,
                        false,
                        black_box(&mut chromosome),
                        None,
                        &mut rng,
                    )
                })
            },
        );
    }

    for genes_size in &genes_sizes {
        //group.throughput(Throughput::Elements(*genes_size as u64));
        let mut genotype = MultiRangeGenotype::builder()
            .with_allele_ranges((0..*genes_size).map(|_| (0.0..=1.0)).collect())
            .build()
            .unwrap();
        let mut chromosome = genotype.chromosome_constructor(&mut rng);
        group.bench_function(
            BenchmarkId::new("multi_range-random-multi-10-with-duplicates", genes_size),
            |b| {
                b.iter(|| {
                    genotype.mutate_chromosome_genes(
                        10,
                        true,
                        black_box(&mut chromosome),
                        None,
                        &mut rng,
                    )
                })
            },
        );
        group.bench_function(
            BenchmarkId::new("multi_range-random-multi-10-without-duplicates", genes_size),
            |b| {
                b.iter(|| {
                    genotype.mutate_chromosome_genes(
                        10,
                        false,
                        black_box(&mut chromosome),
                        None,
                        &mut rng,
                    )
                })
            },
        );

        let mut genotype = MultiRangeGenotype::builder()
            .with_allele_ranges((0..*genes_size).map(|_| (0.0..=1.0)).collect())
            .with_allele_mutation_ranges((0..*genes_size).map(|_| (-0.1..=0.1)).collect())
            .build()
            .unwrap();
        let mut chromosome = genotype.chromosome_constructor(&mut rng);
        group.bench_function(
            BenchmarkId::new("multi_range-relative-multi-10-with-duplicates", genes_size),
            |b| {
                b.iter(|| {
                    genotype.mutate_chromosome_genes(
                        10,
                        true,
                        black_box(&mut chromosome),
                        None,
                        &mut rng,
                    )
                })
            },
        );
        group.bench_function(
            BenchmarkId::new(
                "multi_range-relative-multi-10-without-duplicates",
                genes_size,
            ),
            |b| {
                b.iter(|| {
                    genotype.mutate_chromosome_genes(
                        10,
                        false,
                        black_box(&mut chromosome),
                        None,
                        &mut rng,
                    )
                })
            },
        );

        let mut genotype = MultiRangeGenotype::builder()
            .with_allele_ranges((0..*genes_size).map(|_| (0.0..=1.0)).collect())
            .with_allele_mutation_scaled_ranges(vec![
                (0..*genes_size).map(|_| (-0.1..=0.1)).collect(),
                (0..*genes_size).map(|_| (-0.01..=0.01)).collect(),
                (0..*genes_size).map(|_| (-0.001..=0.001)).collect(),
            ])
            .build()
            .unwrap();
        let mut chromosome = genotype.chromosome_constructor(&mut rng);
        group.bench_function(
            BenchmarkId::new("multi_range-scaled-multi-10-with-duplicates", genes_size),
            |b| {
                b.iter(|| {
                    genotype.mutate_chromosome_genes(
                        10,
                        true,
                        black_box(&mut chromosome),
                        Some(1),
                        &mut rng,
                    )
                })
            },
        );
        group.bench_function(
            BenchmarkId::new("multi_range-scaled-multi-10-without-duplicates", genes_size),
            |b| {
                b.iter(|| {
                    genotype.mutate_chromosome_genes(
                        10,
                        false,
                        black_box(&mut chromosome),
                        Some(1),
                        &mut rng,
                    )
                })
            },
        );
    }

    for genes_size in &genes_sizes {
        //group.throughput(Throughput::Elements(*genes_size as u64));
        let mut genotype = MultiUniqueGenotype::builder()
            .with_allele_lists(vec![
                (0..*genes_size / 10).collect(),
                (0..*genes_size / 10).collect(),
                (0..*genes_size / 10).collect(),
                (0..*genes_size / 10).collect(),
                (0..*genes_size / 10).collect(),
                (0..*genes_size / 10).collect(),
                (0..*genes_size / 10).collect(),
                (0..*genes_size / 10).collect(),
                (0..*genes_size / 10).collect(),
                (0..*genes_size / 10).collect(),
            ])
            .build()
            .unwrap();
        let mut chromosome = genotype.chromosome_constructor(&mut rng);
        group.bench_function(
            BenchmarkId::new("multi_unique-random-multi-10-with-duplicates", genes_size),
            |b| {
                b.iter(|| {
                    genotype.mutate_chromosome_genes(
                        10,
                        true,
                        black_box(&mut chromosome),
                        None,
                        &mut rng,
                    )
                })
            },
        );
        group.bench_function(
            BenchmarkId::new(
                "multi_unique-random-multi-10-without-duplicates",
                genes_size,
            ),
            |b| {
                b.iter(|| {
                    genotype.mutate_chromosome_genes(
                        10,
                        false,
                        black_box(&mut chromosome),
                        None,
                        &mut rng,
                    )
                })
            },
        );
    }
}

criterion_group!(benches, mutation_benchmark);
criterion_main!(benches);
