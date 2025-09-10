use criterion::*;
use genetic_algorithm::centralized::chromosome::ChromosomeManager;
use genetic_algorithm::centralized::extension::*;
use genetic_algorithm::centralized::fitness::placeholders::CountStaticTrue;
use genetic_algorithm::centralized::fitness::Fitness;
use genetic_algorithm::centralized::genotype::{Genotype, StaticBinaryGenotype};
use genetic_algorithm::centralized::population::Population;
use genetic_algorithm::centralized::strategy::evolve::{EvolveConfig, EvolveState};
use genetic_algorithm::centralized::strategy::StrategyReporterNoop;
use rand::prelude::*;
use rand::rngs::SmallRng;
//use std::time::Duration;

const GENES_SIZE_100: usize = 100;
const GENES_SIZE_10000: usize = 10000;
const MAX_POPULATION_SIZE: usize = 2000;

pub fn setup_100(
    genes_size: usize,
    population_size: usize,
    rng: &mut SmallRng,
) -> (
    StaticBinaryGenotype<GENES_SIZE_100, MAX_POPULATION_SIZE>,
    EvolveState,
) {
    let mut genotype = StaticBinaryGenotype::<GENES_SIZE_100, MAX_POPULATION_SIZE>::builder()
        .with_genes_size(genes_size)
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let chromosomes = (0..population_size)
        .map(|_| genotype.chromosome_constructor_random(rng))
        .collect();

    let mut population = Population::new(chromosomes);
    CountStaticTrue::<GENES_SIZE_100, MAX_POPULATION_SIZE>::new()
        .call_for_population(&mut population, &genotype);
    let mut state = EvolveState::new(&genotype);
    state.population = population;
    (genotype, state)
}

pub fn setup_10000(
    genes_size: usize,
    population_size: usize,
    rng: &mut SmallRng,
) -> (
    StaticBinaryGenotype<GENES_SIZE_10000, MAX_POPULATION_SIZE>,
    EvolveState,
) {
    let mut genotype = StaticBinaryGenotype::<GENES_SIZE_10000, MAX_POPULATION_SIZE>::builder()
        .with_genes_size(genes_size)
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let chromosomes = (0..population_size)
        .map(|_| genotype.chromosome_constructor_random(rng))
        .collect();

    let mut population = Population::new(chromosomes);
    CountStaticTrue::<GENES_SIZE_10000, MAX_POPULATION_SIZE>::new()
        .call_for_population(&mut population, &genotype);
    let mut state = EvolveState::new(&genotype);
    state.population = population;
    (genotype, state)
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut config = EvolveConfig::new();
    let mut rng = SmallRng::from_entropy();
    let population_size: usize = 1000;
    config.target_population_size = population_size;

    let mut group = c.benchmark_group(format!("extensions-pop{}", population_size));
    //group.warm_up_time(Duration::from_secs(3));
    //group.measurement_time(Duration::from_secs(3));
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);

    // Benchmarks for genes_size = 100
    {
        let genes_size = GENES_SIZE_100;
        let mut reporter = StrategyReporterNoop::<
            StaticBinaryGenotype<GENES_SIZE_100, MAX_POPULATION_SIZE>,
        >::new();
        let extensions: Vec<ExtensionWrapper> = vec![
            ExtensionMassGenesis::new(population_size).into(),
            ExtensionMassExtinction::new(population_size, 0.10, 0.02).into(),
            ExtensionMassDeduplication::new(population_size).into(),
            ExtensionMassDegeneration::new(population_size, 10, 0.02).into(),
        ];
        for mut extension in extensions {
            group.throughput(Throughput::Elements(population_size as u64));
            let (mut genotype, state) = setup_100(genes_size, population_size, &mut rng);
            group.bench_with_input(
                BenchmarkId::new(format!("{:?}", extension), genes_size),
                &genes_size,
                |b, &_genes_size| {
                    b.iter_batched(
                        || state.clone(),
                        |mut data| {
                            extension.call(
                                &mut genotype,
                                &mut data,
                                &config,
                                &mut reporter,
                                &mut rng,
                            )
                        },
                        BatchSize::SmallInput,
                    )
                },
            );
        }
    }

    // Benchmarks for genes_size = 10000
    {
        let genes_size = GENES_SIZE_10000;
        let mut reporter = StrategyReporterNoop::<
            StaticBinaryGenotype<GENES_SIZE_10000, MAX_POPULATION_SIZE>,
        >::new();
        let extensions: Vec<ExtensionWrapper> = vec![
            ExtensionMassGenesis::new(population_size).into(),
            ExtensionMassExtinction::new(population_size, 0.10, 0.02).into(),
            ExtensionMassDeduplication::new(population_size).into(),
            ExtensionMassDegeneration::new(population_size, 10, 0.02).into(),
        ];
        for mut extension in extensions {
            group.throughput(Throughput::Elements(population_size as u64));
            let (mut genotype, state) = setup_10000(genes_size, population_size, &mut rng);
            group.bench_with_input(
                BenchmarkId::new(format!("{:?}", extension), genes_size),
                &genes_size,
                |b, &_genes_size| {
                    b.iter_batched(
                        || state.clone(),
                        |mut data| {
                            extension.call(
                                &mut genotype,
                                &mut data,
                                &config,
                                &mut reporter,
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

