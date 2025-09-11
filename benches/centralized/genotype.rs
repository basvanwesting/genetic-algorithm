use criterion::*;
use genetic_algorithm::centralized::chromosome::ChromosomeManager;
use genetic_algorithm::centralized::genotype::*;
use rand::prelude::*;
use rand::rngs::SmallRng;
//use std::time::Duration;

const GENES_SIZE: usize = 100;
const MAX_POPULATION_SIZE: usize = 100;

pub fn mutation_benchmark(c: &mut Criterion) {
    let mut rng = SmallRng::from_entropy();

    let mut group = c.benchmark_group("genotype-mutation");
    //group.warm_up_time(Duration::from_secs(3));
    //group.measurement_time(Duration::from_secs(3));
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);

    // Binary genotype benchmarks
    {
        //group.throughput(Throughput::Elements(GENES_SIZE as u64));
        let mut genotype = StaticBinaryGenotype::<GENES_SIZE, MAX_POPULATION_SIZE>::builder()
            .with_genes_size(GENES_SIZE)
            .build()
            .unwrap();
        genotype.chromosomes_setup();
        let mut chromosome = genotype.chromosome_constructor_random(&mut rng);

        group.bench_function(
            BenchmarkId::new("binary-random-multi-10-with-duplicates", GENES_SIZE),
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
            BenchmarkId::new("binary-random-multi-10-without-duplicates", GENES_SIZE),
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
        //group.throughput(Throughput::Elements(GENES_SIZE as u64));
        let mut genotype = StaticRangeGenotype::<f32, GENES_SIZE, MAX_POPULATION_SIZE>::builder()
            .with_genes_size(GENES_SIZE)
            .with_allele_range(0.0..=1.0)
            .build()
            .unwrap();
        genotype.chromosomes_setup();
        let mut chromosome = genotype.chromosome_constructor_random(&mut rng);

        group.bench_function(
            BenchmarkId::new("range-random-multi-10-with-duplicates", GENES_SIZE),
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
            BenchmarkId::new("range-random-multi-10-without-duplicates", GENES_SIZE),
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
        let mut genotype = StaticRangeGenotype::<f32, GENES_SIZE, MAX_POPULATION_SIZE>::builder()
            .with_genes_size(GENES_SIZE)
            .with_allele_range(0.0..=1.0)
            .with_allele_mutation_range(-0.1..=0.1)
            .build()
            .unwrap();
        genotype.chromosomes_setup();
        let mut chromosome = genotype.chromosome_constructor_random(&mut rng);
        group.bench_function(
            BenchmarkId::new("range-relative-multi-10-with-duplicates", GENES_SIZE),
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
            BenchmarkId::new("range-relative-multi-10-without-duplicates", GENES_SIZE),
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
        let mut genotype = StaticRangeGenotype::<f32, GENES_SIZE, MAX_POPULATION_SIZE>::builder()
            .with_genes_size(GENES_SIZE)
            .with_allele_range(0.0..=1.0)
            .with_allele_mutation_scaled_range(vec![-0.1..=0.1, -0.01..=0.01, -0.001..=0.001])
            .build()
            .unwrap();
        genotype.chromosomes_setup();
        let mut chromosome = genotype.chromosome_constructor_random(&mut rng);

        group.bench_function(
            BenchmarkId::new("range-scaled-multi-10-with-duplicates", GENES_SIZE),
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
            BenchmarkId::new("range-scaled-multi-10-without-duplicates", GENES_SIZE),
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
