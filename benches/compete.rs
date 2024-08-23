use criterion::*;
use genetic_algorithm::compete::*;
use genetic_algorithm::fitness::placeholders::CountTrue;
use genetic_algorithm::fitness::{Fitness, FitnessOrdering};
use genetic_algorithm::genotype::{BinaryAllele, BinaryGenotype, Genotype};
use genetic_algorithm::population::Population;
use genetic_algorithm::strategy::evolve::{EvolveConfig, EvolveReporterNoop, EvolveState};
use rand::prelude::*;
use rand::rngs::SmallRng;
//use std::time::Duration;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut reporter = EvolveReporterNoop::<BinaryAllele>::new();
    let mut rng = SmallRng::from_entropy();
    let population_sizes = vec![100, 1000];
    let fitness_ordering = FitnessOrdering::Minimize;

    let competes: Vec<CompeteWrapper> = vec![
        CompeteElite::new().into(),
        CompeteTournament::new(4).into(),
        CompeteTournament::new(8).into(),
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

    for mut compete in competes {
        for population_size in &population_sizes {
            group.throughput(Throughput::Elements(*population_size as u64));

            let target_population_size = population_size;
            let source_population_size = population_size * 2;

            let config = EvolveConfig {
                target_population_size: *target_population_size,
                fitness_ordering,
                ..Default::default()
            };

            let chromosomes = (0..source_population_size)
                .map(|_| genotype.chromosome_factory(&mut rng))
                .collect();
            let population = Population::new(chromosomes);
            let mut state = EvolveState::new(&genotype, population);
            CountTrue.call_for_population(&mut state.population, None);

            group.bench_with_input(
                BenchmarkId::new(format!("{:?}-single-threaded", compete), population_size),
                population_size,
                |b, &_population_size| {
                    b.iter_batched(
                        || state.clone(),
                        |mut data| compete.call(&mut data, &config, &mut reporter, &mut rng, false),
                        BatchSize::SmallInput,
                    )
                },
            );

            group.bench_with_input(
                BenchmarkId::new(format!("{:?}-multi-threaded", compete), population_size),
                population_size,
                |b, &_population_size| {
                    b.iter_batched(
                        || state.clone(),
                        |mut data| compete.call(&mut data, &config, &mut reporter, &mut rng, true),
                        BatchSize::SmallInput,
                    )
                },
            );
        }
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
