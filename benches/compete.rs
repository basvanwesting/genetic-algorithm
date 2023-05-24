use criterion::*;
use genetic_algorithm::compete::*;
use genetic_algorithm::fitness::placeholders::CountTrue;
use genetic_algorithm::fitness::{Fitness, FitnessOrdering};
use genetic_algorithm::genotype::{BinaryGenotype, Genotype};
use genetic_algorithm::population::Population;
use genetic_algorithm::strategy::evolve::EvolveConfig;
use rand::prelude::*;
use rand::rngs::SmallRng;
//use std::time::Duration;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = SmallRng::from_entropy();
    let population_sizes = vec![250, 500, 1000, 2000];
    let fitness_ordering = FitnessOrdering::Minimize;

    let competes = vec![
        CompeteElite::new_dispatch(),
        CompeteTournament::new_dispatch(4),
        CompeteTournament::new_dispatch(8),
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

            let evolve_config = EvolveConfig {
                target_population_size: *target_population_size,
                fitness_ordering,
                ..Default::default()
            };

            let chromosomes = (0..source_population_size)
                .map(|_| genotype.chromosome_factory(&mut rng))
                .collect();
            let population = &mut Population::new(chromosomes);
            CountTrue.call_for_population(population, None);

            group.bench_with_input(
                BenchmarkId::new(
                    format!("{:?}-{}", compete.compete, compete.tournament_size),
                    population_size,
                ),
                population_size,
                |b, &_population_size| {
                    b.iter_batched(
                        || population.clone(),
                        |mut data| compete.call(&mut data, &evolve_config, &mut rng),
                        BatchSize::SmallInput,
                    )
                },
            );
        }
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
