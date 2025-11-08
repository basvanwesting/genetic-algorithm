use criterion::*;
use genetic_algorithm::chromosome::Chromosome;
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
        let genotype = BinaryGenotype::builder()
            .with_genes_size(*genes_size)
            .build()
            .unwrap();
        let mut chromosome = Chromosome::new(genotype.random_genes_factory(&mut rng));

        group.bench_function(
            BenchmarkId::new("binary-random-multi-10-with-duplicates", genes_size),
            |b| {
                b.iter(|| {
                    genotype.mutate_chromosome_genes(10, true, black_box(&mut chromosome), &mut rng)
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
                        &mut rng,
                    )
                })
            },
        );
    }

    for genes_size in &genes_sizes {
        //group.throughput(Throughput::Elements(*genes_size as u64));
        let genotype = RangeGenotype::builder()
            .with_genes_size(*genes_size)
            .with_allele_range(0.0..=1.0)
            .build()
            .unwrap();
        let mut chromosome = Chromosome::new(genotype.random_genes_factory(&mut rng));

        group.bench_function(
            BenchmarkId::new("range-random-multi-10-with-duplicates", genes_size),
            |b| {
                b.iter(|| {
                    genotype.mutate_chromosome_genes(10, true, black_box(&mut chromosome), &mut rng)
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
                        &mut rng,
                    )
                })
            },
        );

        let genotype = RangeGenotype::builder()
            .with_genes_size(*genes_size)
            .with_allele_range(0.0..=1.0)
            .with_mutation_type(MutationType::Range(0.1))
            .build()
            .unwrap();
        let mut chromosome = Chromosome::new(genotype.random_genes_factory(&mut rng));
        group.bench_function(
            BenchmarkId::new("range-relative-multi-10-with-duplicates", genes_size),
            |b| {
                b.iter(|| {
                    genotype.mutate_chromosome_genes(10, true, black_box(&mut chromosome), &mut rng)
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
                        &mut rng,
                    )
                })
            },
        );

        let mut genotype = RangeGenotype::builder()
            .with_genes_size(*genes_size)
            .with_allele_range(0.0..=1.0)
            .with_mutation_type(MutationType::StepScaled(vec![0.1, 0.01, 0.001]))
            .build()
            .unwrap();
        let mut chromosome = Chromosome::new(genotype.random_genes_factory(&mut rng));
        genotype.increment_scale_index();

        group.bench_function(
            BenchmarkId::new("range-scaled-multi-10-with-duplicates", genes_size),
            |b| {
                b.iter(|| {
                    genotype.mutate_chromosome_genes(10, true, black_box(&mut chromosome), &mut rng)
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
                        &mut rng,
                    )
                })
            },
        );
    }
}

criterion_group!(benches, mutation_benchmark);
criterion_main!(benches);
