use criterion::*;
use genetic_algorithm::centralized::chromosome::ChromosomeManager;
use genetic_algorithm::centralized::extension::*;
use genetic_algorithm::centralized::fitness::placeholders::CountTrue;
use genetic_algorithm::centralized::fitness::Fitness;
use genetic_algorithm::centralized::genotype::{BinaryGenotype, Genotype};
use genetic_algorithm::centralized::population::Population;
use genetic_algorithm::centralized::strategy::evolve::{EvolveConfig, EvolveState};
use genetic_algorithm::centralized::strategy::StrategyReporterNoop;
use rand::prelude::*;
use rand::rngs::SmallRng;
//use std::time::Duration;

pub fn setup(
    genes_size: usize,
    population_size: usize,
    rng: &mut SmallRng,
) -> (BinaryGenotype, EvolveState<BinaryGenotype>) {
    let mut genotype = BinaryGenotype::builder()
        .with_genes_size(genes_size)
        .build()
        .unwrap();

    let chromosomes = (0..population_size)
        .map(|_| genotype.chromosome_constructor_random(rng))
        .collect();

    let mut population = Population::new(chromosomes);
    CountTrue.call_for_population(&mut population, &genotype, None, None);
    let mut state = EvolveState::new(&genotype);
    state.population = population;
    (genotype, state)
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut config = EvolveConfig::new();
    let mut reporter = StrategyReporterNoop::<BinaryGenotype>::new();
    let mut rng = SmallRng::from_entropy();
    let population_size: usize = 1000;
    let genes_sizes = vec![100, 10000];
    config.target_population_size = population_size;

    let mut group = c.benchmark_group(format!("extensions-pop{}", population_size));
    //group.warm_up_time(Duration::from_secs(3));
    //group.measurement_time(Duration::from_secs(3));
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);

    for genes_size in &genes_sizes {
        let extensions: Vec<ExtensionWrapper> = vec![
            ExtensionMassGenesis::new(population_size).into(),
            ExtensionMassExtinction::new(population_size, 0.10, 0.02).into(),
            ExtensionMassDeduplication::new(population_size).into(),
            ExtensionMassDegeneration::new(population_size, 10, 0.02).into(),
        ];
        for mut extension in extensions {
            group.throughput(Throughput::Elements(population_size as u64));
            let (mut genotype, state) = setup(*genes_size, population_size, &mut rng);
            group.bench_with_input(
                BenchmarkId::new(format!("{:?}", extension), genes_size),
                genes_size,
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
