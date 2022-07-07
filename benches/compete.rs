use criterion::*;
use genetic_algorithm::compete::*;
use genetic_algorithm::fitness::placeholders::CountTrue;
use genetic_algorithm::fitness::{Fitness, FitnessOrdering};
use genetic_algorithm::genotype::{BinaryGenotype, Genotype};
use genetic_algorithm::population::Population;
use rand::prelude::*;
use rand::rngs::SmallRng;
//use std::time::Duration;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = SmallRng::from_entropy();
    let population_sizes = vec![10, 100, 1000, 10000];
    let fitness_ordering = FitnessOrdering::Maximize;

    let competes = vec![
        CompeteDispatch(Competes::Elite, 0),
        CompeteDispatch(Competes::Tournament, 2),
        CompeteDispatch(Competes::Tournament, 4),
        CompeteDispatch(Competes::Tournament, 8),
    ];

    let mut group = c.benchmark_group("competes");
    //group.warm_up_time(Duration::from_secs(3));
    //group.measurement_time(Duration::from_secs(3));
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);

    let genotype = BinaryGenotype::builder()
        .with_genes_size(100)
        .build()
        .unwrap();

    for compete in competes {
        for population_size in &population_sizes {
            group.throughput(Throughput::Elements(*population_size as u64));

            let target_population_size = population_size;
            let source_population_size = population_size * 2;

            let chromosomes = (0..source_population_size)
                .map(|_| genotype.chromosome_factory(&mut rng))
                .collect();
            let population = &mut Population::new(chromosomes);
            CountTrue.call_for_population(population, None);

            group.bench_with_input(
                BenchmarkId::new(format!("{:?}-{}", compete.0, compete.1), population_size),
                population_size,
                |b, &_population_size| {
                    b.iter_batched(
                        || population.clone(),
                        |mut data| {
                            compete.call(
                                &mut data,
                                fitness_ordering,
                                *target_population_size,
                                &mut rng,
                            )
                        },
                        BatchSize::SmallInput,
                    )
                },
            );
        }
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
