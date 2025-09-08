use criterion::*;
use genetic_algorithm::centralized::chromosome::ChromosomeManager;
use genetic_algorithm::centralized::fitness::placeholders::CountTrue;
use genetic_algorithm::centralized::fitness::Fitness;
use genetic_algorithm::centralized::genotype::{BinaryGenotype, Genotype};
use genetic_algorithm::centralized::population::Population;
use rand::prelude::*;
use rand::rngs::SmallRng;
//use std::time::Duration;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = SmallRng::from_entropy();

    let mut group = c.benchmark_group("population");
    //group.warm_up_time(Duration::from_secs(3));
    //group.measurement_time(Duration::from_secs(3));
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);

    let mut genotype = BinaryGenotype::builder()
        .with_genes_size(100)
        .with_genes_hashing(true)
        .build()
        .unwrap();

    // Benchmarks for population_size = 100
    {
        let population_size = 100;
        group.throughput(Throughput::Elements(population_size as u64));

        let chromosomes = (0..population_size)
            .map(|_| genotype.chromosome_constructor_random(&mut rng))
            .collect();
        let population = &mut Population::new(chromosomes);
        CountTrue.call_for_population(population, &genotype, None, None);

        group.bench_with_input(
            BenchmarkId::new(
                "fitness_score_cardinality (known score), low",
                population_size,
            ),
            &population_size,
            |b, &_population_size| {
                b.iter_batched(
                    || population.clone(),
                    |data| data.fitness_score_cardinality(),
                    BatchSize::SmallInput,
                )
            },
        );

        let random_chromosome = population.chromosomes.choose(&mut rng).unwrap();
        let chromosomes = (0..population_size)
            .map(|_| random_chromosome.clone())
            .collect();
        let population = &mut Population::new(chromosomes);
        CountTrue.call_for_population(population, &genotype, None, None);

        group.bench_with_input(
            BenchmarkId::new(
                "fitness_score_cardinality (known score), high",
                population_size,
            ),
            &population_size,
            |b, &_population_size| {
                b.iter_batched(
                    || population.clone(),
                    |data| data.fitness_score_cardinality(),
                    BatchSize::SmallInput,
                )
            },
        );

        let chromosomes = (0..population_size)
            .map(|_| genotype.chromosome_constructor_random(&mut rng))
            .collect();
        let population = &mut Population::new(chromosomes);

        group.bench_with_input(
            BenchmarkId::new("genes_cardinality (unknown hash), low", population_size),
            &population_size,
            |b, &_population_size| {
                b.iter_batched(
                    || population.clone(),
                    |mut data| {
                        data.chromosomes.iter_mut().for_each(|c| {
                            genotype.calculate_genes_hash(c);
                        });
                        data.genes_cardinality()
                    },
                    BatchSize::SmallInput,
                )
            },
        );

        let random_chromosome = population.chromosomes.choose(&mut rng).unwrap();
        let chromosomes = (0..population_size)
            .map(|_| random_chromosome.clone())
            .collect();
        let population = &mut Population::new(chromosomes);

        group.bench_with_input(
            BenchmarkId::new("genes_cardinality (unknown hash), high", population_size),
            &population_size,
            |b, &_population_size| {
                b.iter_batched(
                    || population.clone(),
                    |mut data| {
                        data.chromosomes.iter_mut().for_each(|c| {
                            genotype.calculate_genes_hash(c);
                        });
                        data.genes_cardinality()
                    },
                    BatchSize::SmallInput,
                )
            },
        );
    }

    // Benchmarks for population_size = 1000
    {
        let population_size = 1000;
        group.throughput(Throughput::Elements(population_size as u64));

        let chromosomes = (0..population_size)
            .map(|_| genotype.chromosome_constructor_random(&mut rng))
            .collect();
        let population = &mut Population::new(chromosomes);
        CountTrue.call_for_population(population, &genotype, None, None);

        group.bench_with_input(
            BenchmarkId::new(
                "fitness_score_cardinality (known score), low",
                population_size,
            ),
            &population_size,
            |b, &_population_size| {
                b.iter_batched(
                    || population.clone(),
                    |data| data.fitness_score_cardinality(),
                    BatchSize::SmallInput,
                )
            },
        );

        let random_chromosome = population.chromosomes.choose(&mut rng).unwrap();
        let chromosomes = (0..population_size)
            .map(|_| random_chromosome.clone())
            .collect();
        let population = &mut Population::new(chromosomes);
        CountTrue.call_for_population(population, &genotype, None, None);

        group.bench_with_input(
            BenchmarkId::new(
                "fitness_score_cardinality (known score), high",
                population_size,
            ),
            &population_size,
            |b, &_population_size| {
                b.iter_batched(
                    || population.clone(),
                    |data| data.fitness_score_cardinality(),
                    BatchSize::SmallInput,
                )
            },
        );

        let chromosomes = (0..population_size)
            .map(|_| genotype.chromosome_constructor_random(&mut rng))
            .collect();
        let population = &mut Population::new(chromosomes);

        group.bench_with_input(
            BenchmarkId::new("genes_cardinality (unknown hash), low", population_size),
            &population_size,
            |b, &_population_size| {
                b.iter_batched(
                    || population.clone(),
                    |mut data| {
                        data.chromosomes.iter_mut().for_each(|c| {
                            genotype.calculate_genes_hash(c);
                        });
                        data.genes_cardinality()
                    },
                    BatchSize::SmallInput,
                )
            },
        );

        let random_chromosome = population.chromosomes.choose(&mut rng).unwrap();
        let chromosomes = (0..population_size)
            .map(|_| random_chromosome.clone())
            .collect();
        let population = &mut Population::new(chromosomes);

        group.bench_with_input(
            BenchmarkId::new("genes_cardinality (unknown hash), high", population_size),
            &population_size,
            |b, &_population_size| {
                b.iter_batched(
                    || population.clone(),
                    |mut data| {
                        data.chromosomes.iter_mut().for_each(|c| {
                            genotype.calculate_genes_hash(c);
                        });
                        data.genes_cardinality()
                    },
                    BatchSize::SmallInput,
                )
            },
        );
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);