use criterion::*;
use genetic_algorithm::centralized::chromosome::ChromosomeManager;
use genetic_algorithm::centralized::fitness::placeholders::CountStaticTrue;
use genetic_algorithm::centralized::fitness::Fitness;
use genetic_algorithm::centralized::genotype::{Genotype, StaticBinaryGenotype};
use genetic_algorithm::centralized::population::Population;
use rand::prelude::*;
use rand::rngs::SmallRng;
//use std::time::Duration;

const GENES_SIZE: usize = 100;
const MAX_POPULATION_SIZE_100: usize = 200;
const MAX_POPULATION_SIZE_1000: usize = 2000;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = SmallRng::from_entropy();

    let mut group = c.benchmark_group("population");
    //group.warm_up_time(Duration::from_secs(3));
    //group.measurement_time(Duration::from_secs(3));
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);

    // Benchmarks for population_size = 100
    {
        let population_size = 100;
        let mut genotype = StaticBinaryGenotype::<GENES_SIZE, MAX_POPULATION_SIZE_100>::builder()
            .with_genes_size(GENES_SIZE)
            .with_genes_hashing(true)
            .build()
            .unwrap();
        genotype.chromosomes_setup();

        group.throughput(Throughput::Elements(population_size as u64));

        let chromosomes = (0..population_size)
            .map(|_| genotype.chromosome_constructor_random(&mut rng))
            .collect();
        let population = &mut Population::new(chromosomes);
        CountStaticTrue::<GENES_SIZE, MAX_POPULATION_SIZE_100>::new()
            .call_for_population(population, &genotype);

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
        CountStaticTrue::<GENES_SIZE, MAX_POPULATION_SIZE_100>::new()
            .call_for_population(population, &genotype);

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
        let mut genotype = StaticBinaryGenotype::<GENES_SIZE, MAX_POPULATION_SIZE_1000>::builder()
            .with_genes_size(GENES_SIZE)
            .with_genes_hashing(true)
            .build()
            .unwrap();
        genotype.chromosomes_setup();

        group.throughput(Throughput::Elements(population_size as u64));

        let chromosomes = (0..population_size)
            .map(|_| genotype.chromosome_constructor_random(&mut rng))
            .collect();
        let population = &mut Population::new(chromosomes);
        CountStaticTrue::<GENES_SIZE, MAX_POPULATION_SIZE_1000>::new()
            .call_for_population(population, &genotype);

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
        CountStaticTrue::<GENES_SIZE, MAX_POPULATION_SIZE_1000>::new()
            .call_for_population(population, &genotype);

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
