use criterion::*;
use genetic_algorithm::fitness::placeholders::CountTrue;
use genetic_algorithm::fitness::Fitness;
use genetic_algorithm::genotype::{BinaryGenotype, Genotype};
use genetic_algorithm::population::Population;
use rand::prelude::*;
use rand::rngs::SmallRng;
//use std::time::Duration;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = SmallRng::from_entropy();
    let population_sizes = vec![100, 1000];

    let mut group = c.benchmark_group("population");
    //group.warm_up_time(Duration::from_secs(3));
    //group.measurement_time(Duration::from_secs(3));
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);

    let mut genotype = BinaryGenotype::builder()
        .with_genes_size(100)
        .build()
        .unwrap();

    for population_size in &population_sizes {
        group.throughput(Throughput::Elements(*population_size as u64));

        let chromosomes = (0..*population_size)
            .map(|_| genotype.chromosome_constructor(&mut rng))
            .collect();
        let population = &mut Population::new(chromosomes);
        CountTrue.call_for_population(population, &mut genotype, None);

        group.bench_with_input(
            BenchmarkId::new("fitness_score_cardinality, low", population_size),
            population_size,
            |b, &_population_size| {
                b.iter_batched(
                    || population.clone(),
                    |data| data.fitness_score_cardinality(),
                    BatchSize::SmallInput,
                )
            },
        );

        let random_chromosome = population.chromosomes.choose(&mut rng).unwrap();
        let chromosomes = (0..*population_size)
            .map(|_| random_chromosome.clone())
            .collect();
        let population = &mut Population::new(chromosomes);
        CountTrue.call_for_population(population, &mut genotype, None);

        group.bench_with_input(
            BenchmarkId::new("fitness_score_cardinality, high", population_size),
            population_size,
            |b, &_population_size| {
                b.iter_batched(
                    || population.clone(),
                    |data| data.fitness_score_cardinality(),
                    BatchSize::SmallInput,
                )
            },
        );
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
