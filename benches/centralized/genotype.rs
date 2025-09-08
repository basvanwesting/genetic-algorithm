use criterion::*;
use genetic_algorithm::centralized::chromosome::ChromosomeManager;
use genetic_algorithm::centralized::genotype::*;
use rand::prelude::*;
use rand::rngs::SmallRng;
//use std::time::Duration;

pub fn mutation_benchmark(c: &mut Criterion) {
    let mut rng = SmallRng::from_entropy();
    let genes_size = 100;

    let mut group = c.benchmark_group("genotype-mutation");
    //group.warm_up_time(Duration::from_secs(3));
    //group.measurement_time(Duration::from_secs(3));
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);

    // Binary genotype benchmarks
    {
        //group.throughput(Throughput::Elements(genes_size as u64));
        let mut genotype = BinaryGenotype::builder()
            .with_genes_size(genes_size)
            .build()
            .unwrap();
        let mut chromosome = genotype.chromosome_constructor_random(&mut rng);

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

    // Range genotype benchmarks - random mutation
    {
        //group.throughput(Throughput::Elements(genes_size as u64));
        let mut genotype = RangeGenotype::builder()
            .with_genes_size(genes_size)
            .with_allele_range(0.0..=1.0)
            .build()
            .unwrap();
        let mut chromosome = genotype.chromosome_constructor_random(&mut rng);

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
    }

    // Range genotype benchmarks - relative mutation
    {
        let mut genotype = RangeGenotype::builder()
            .with_genes_size(genes_size)
            .with_allele_range(0.0..=1.0)
            .with_allele_mutation_range(-0.1..=0.1)
            .build()
            .unwrap();
        let mut chromosome = genotype.chromosome_constructor_random(&mut rng);
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
    }

    // Range genotype benchmarks - scaled mutation
    {
        let mut genotype = RangeGenotype::builder()
            .with_genes_size(genes_size)
            .with_allele_range(0.0..=1.0)
            .with_allele_mutation_scaled_range(vec![-0.1..=0.1, -0.01..=0.01, -0.001..=0.001])
            .build()
            .unwrap();
        let mut chromosome = genotype.chromosome_constructor_random(&mut rng);

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
}

criterion_group!(benches, mutation_benchmark);
criterion_main!(benches);