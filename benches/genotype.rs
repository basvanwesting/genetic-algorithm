use criterion::*;
use genetic_algorithm::genotype::*;
use rand::prelude::*;
use rand::rngs::SmallRng;
//use std::time::Duration;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = SmallRng::from_entropy();
    //let genes_sizes = vec![10, 100, 1000, 10000];
    let genes_sizes = vec![100];

    let mut group = c.benchmark_group("genotype-mutate");
    //group.warm_up_time(Duration::from_secs(3));
    //group.measurement_time(Duration::from_secs(3));
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);

    for genes_size in &genes_sizes {
        //group.throughput(Throughput::Elements(*genes_size as u64));
        group.bench_with_input(
            BenchmarkId::new("binary-random", genes_size),
            genes_size,
            |b, &genes_size| {
                let genotype = BinaryGenotype::builder()
                    .with_genes_size(genes_size)
                    .build()
                    .unwrap();
                let mut chromosome = genotype.chromosome_factory(&mut rng);
                b.iter(|| genotype.mutate_chromosome_random(black_box(&mut chromosome), &mut rng))
            },
        );
    }

    for genes_size in &genes_sizes {
        //group.throughput(Throughput::Elements(*genes_size as u64));
        group.bench_with_input(
            BenchmarkId::new("continuous-random", genes_size),
            genes_size,
            |b, &genes_size| {
                let genotype = ContinuousGenotype::builder()
                    .with_genes_size(genes_size)
                    .with_allele_range(0.0..1.0)
                    .build()
                    .unwrap();
                let mut chromosome = genotype.chromosome_factory(&mut rng);
                b.iter(|| genotype.mutate_chromosome_random(black_box(&mut chromosome), &mut rng))
            },
        );
        group.bench_with_input(
            BenchmarkId::new("continuous-neighbour", genes_size),
            genes_size,
            |b, &genes_size| {
                let genotype = ContinuousGenotype::builder()
                    .with_genes_size(genes_size)
                    .with_allele_range(0.0..1.0)
                    .with_allele_neighbour_range(-0.1..0.1)
                    .build()
                    .unwrap();
                let mut chromosome = genotype.chromosome_factory(&mut rng);
                b.iter(|| {
                    genotype.mutate_chromosome_neighbour(
                        black_box(&mut chromosome),
                        Some(1.0),
                        &mut rng,
                    )
                })
            },
        );
    }

    for genes_size in &genes_sizes {
        //group.throughput(Throughput::Elements(*genes_size as u64));
        group.bench_with_input(
            BenchmarkId::new("discrete-random", genes_size),
            genes_size,
            |b, &genes_size| {
                let genotype = DiscreteGenotype::builder()
                    .with_genes_size(genes_size)
                    .with_allele_list((0..10).collect())
                    .build()
                    .unwrap();
                let mut chromosome = genotype.chromosome_factory(&mut rng);
                b.iter(|| genotype.mutate_chromosome_random(black_box(&mut chromosome), &mut rng))
            },
        );
    }

    for genes_size in &genes_sizes {
        //group.throughput(Throughput::Elements(*genes_size as u64));
        group.bench_with_input(
            BenchmarkId::new("multi_continuous-random", genes_size),
            genes_size,
            |b, &genes_size| {
                let genotype = MultiContinuousGenotype::builder()
                    .with_allele_ranges((0..genes_size).map(|_| (0.0..1.0)).collect())
                    .build()
                    .unwrap();
                let mut chromosome = genotype.chromosome_factory(&mut rng);
                b.iter(|| genotype.mutate_chromosome_random(black_box(&mut chromosome), &mut rng))
            },
        );
        group.bench_with_input(
            BenchmarkId::new("multi_continuous-neighbour", genes_size),
            genes_size,
            |b, &genes_size| {
                let genotype = MultiContinuousGenotype::builder()
                    .with_allele_ranges((0..genes_size).map(|_| (0.0..1.0)).collect())
                    .with_allele_neighbour_ranges(
                        (0..genes_size).map(|_| (-0.1..0.1)).collect(),
                    )
                    .build()
                    .unwrap();
                let mut chromosome = genotype.chromosome_factory(&mut rng);
                b.iter(|| {
                    genotype.mutate_chromosome_neighbour(
                        black_box(&mut chromosome),
                        Some(1.0),
                        &mut rng,
                    )
                })
            },
        );
    }

    for genes_size in &genes_sizes {
        //group.throughput(Throughput::Elements(*genes_size as u64));
        group.bench_with_input(
            BenchmarkId::new("multi_discrete-random", genes_size),
            genes_size,
            |b, &genes_size| {
                let genotype = MultiDiscreteGenotype::builder()
                    .with_allele_lists((0..genes_size).map(|_| (0..10).collect()).collect())
                    .build()
                    .unwrap();
                let mut chromosome = genotype.chromosome_factory(&mut rng);
                b.iter(|| genotype.mutate_chromosome_random(black_box(&mut chromosome), &mut rng))
            },
        );
    }

    for genes_size in &genes_sizes {
        //group.throughput(Throughput::Elements(*genes_size as u64));
        group.bench_with_input(
            BenchmarkId::new("unique-random", genes_size),
            genes_size,
            |b, &genes_size| {
                let genotype = UniqueGenotype::builder()
                    .with_genes_size(genes_size)
                    .with_allele_list((0..10).collect())
                    .build()
                    .unwrap();
                let mut chromosome = genotype.chromosome_factory(&mut rng);
                b.iter(|| genotype.mutate_chromosome_random(black_box(&mut chromosome), &mut rng))
            },
        );
        group.bench_with_input(
            BenchmarkId::new("unique-neighbour", genes_size),
            genes_size,
            |b, &genes_size| {
                let genotype = UniqueGenotype::builder()
                    .with_genes_size(genes_size)
                    .with_allele_list((0..10).collect())
                    .build()
                    .unwrap();
                let mut chromosome = genotype.chromosome_factory(&mut rng);
                b.iter(|| {
                    genotype.mutate_chromosome_neighbour(black_box(&mut chromosome), None, &mut rng)
                })
            },
        );
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
