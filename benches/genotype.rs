use criterion::*;
use genetic_algorithm::genotype::*;
use rand::prelude::*;
use rand::rngs::SmallRng;
//use std::time::Duration;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = SmallRng::from_entropy();
    let gene_sizes = vec![10, 100, 1000, 10000];

    let mut group = c.benchmark_group("genotype-mutate");
    //group.warm_up_time(Duration::from_secs(3));
    //group.measurement_time(Duration::from_secs(3));
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);

    for gene_size in &gene_sizes {
        //group.throughput(Throughput::Elements(*gene_size as u64));
        group.bench_with_input(
            BenchmarkId::new("binary", gene_size),
            gene_size,
            |b, &gene_size| {
                let genotype = BinaryGenotype::builder()
                    .with_gene_size(gene_size)
                    .build()
                    .unwrap();
                let mut chromosome = genotype.chromosome_factory(&mut rng);
                b.iter(|| genotype.mutate_chromosome(black_box(&mut chromosome), &mut rng))
            },
        );
    }

    for gene_size in &gene_sizes {
        //group.throughput(Throughput::Elements(*gene_size as u64));
        group.bench_with_input(
            BenchmarkId::new("continuous", gene_size),
            gene_size,
            |b, &gene_size| {
                let genotype = ContinuousGenotype::builder()
                    .with_gene_size(gene_size)
                    .with_gene_range(0.0..1.0)
                    .build()
                    .unwrap();
                let mut chromosome = genotype.chromosome_factory(&mut rng);
                b.iter(|| genotype.mutate_chromosome(black_box(&mut chromosome), &mut rng))
            },
        );
    }

    for gene_size in &gene_sizes {
        //group.throughput(Throughput::Elements(*gene_size as u64));
        group.bench_with_input(
            BenchmarkId::new("discrete", gene_size),
            gene_size,
            |b, &gene_size| {
                let genotype = DiscreteGenotype::builder()
                    .with_gene_size(gene_size)
                    .with_gene_values((0..10).collect())
                    .build()
                    .unwrap();
                let mut chromosome = genotype.chromosome_factory(&mut rng);
                b.iter(|| genotype.mutate_chromosome(black_box(&mut chromosome), &mut rng))
            },
        );
    }

    for gene_size in &gene_sizes {
        //group.throughput(Throughput::Elements(*gene_size as u64));
        group.bench_with_input(
            BenchmarkId::new("multi_continuous", gene_size),
            gene_size,
            |b, &gene_size| {
                let genotype = MultiContinuousGenotype::builder()
                    .with_gene_multi_range((0..gene_size).map(|_| (0.0..1.0)).collect())
                    .build()
                    .unwrap();
                let mut chromosome = genotype.chromosome_factory(&mut rng);
                b.iter(|| genotype.mutate_chromosome(black_box(&mut chromosome), &mut rng))
            },
        );
    }

    for gene_size in &gene_sizes {
        //group.throughput(Throughput::Elements(*gene_size as u64));
        group.bench_with_input(
            BenchmarkId::new("multi_discrete", gene_size),
            gene_size,
            |b, &gene_size| {
                let genotype = MultiDiscreteGenotype::builder()
                    .with_gene_multi_values((0..gene_size).map(|_| (0..10).collect()).collect())
                    .build()
                    .unwrap();
                let mut chromosome = genotype.chromosome_factory(&mut rng);
                b.iter(|| genotype.mutate_chromosome(black_box(&mut chromosome), &mut rng))
            },
        );
    }

    for gene_size in &gene_sizes {
        //group.throughput(Throughput::Elements(*gene_size as u64));
        group.bench_with_input(
            BenchmarkId::new("unique_discrete", gene_size),
            gene_size,
            |b, &gene_size| {
                let genotype = UniqueDiscreteGenotype::builder()
                    .with_gene_size(gene_size)
                    .with_gene_values((0..10).collect())
                    .build()
                    .unwrap();
                let mut chromosome = genotype.chromosome_factory(&mut rng);
                b.iter(|| genotype.mutate_chromosome(black_box(&mut chromosome), &mut rng))
            },
        );
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
