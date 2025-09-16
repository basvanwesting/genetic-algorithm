use criterion::*;
use genetic_algorithm::chromosome::Chromosome;
use genetic_algorithm::crossover::*;
use genetic_algorithm::genotype::{BinaryGenotype, Genotype};
use genetic_algorithm::population::Population;
use genetic_algorithm::strategy::evolve::{EvolveConfig, EvolveState};
use genetic_algorithm::strategy::StrategyReporterNoop;
use rand::prelude::*;
use rand::rngs::SmallRng;
//use std::time::Duration;

pub fn setup(
    genes_size: usize,
    population_size: usize,
    rng: &mut SmallRng,
) -> (BinaryGenotype, EvolveState<BinaryGenotype>) {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(genes_size)
        .build()
        .unwrap();

    let chromosomes = (0..population_size)
        .map(|_| Chromosome::new(genotype.random_genes_factory(rng)))
        .collect();

    let population = Population::new(chromosomes);
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

    let mut group = c.benchmark_group(format!("crossovers-pop{}", population_size));
    //group.warm_up_time(Duration::from_secs(3));
    //group.measurement_time(Duration::from_secs(3));
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);

    for genes_size in &genes_sizes {
        let crossovers: Vec<CrossoverWrapper> = vec![
            // CrossoverClone::new(0.7).into(), //clones
            // CrossoverRejuvenate::new(0.7).into(), //noop
            // CrossoverSingleGene::new(0.7, 0.8).into(),
            // CrossoverSinglePoint::new(0.7, 0.8).into(),
            // CrossoverMultiGene::new(0.7, 0.8, genes_size / 2, false).into(),
            // CrossoverMultiGene::new(0.7, 0.8, genes_size / 2, true).into(),
            CrossoverMultiPoint::new(0.7, 0.8, genes_size / 10, false).into(),
            CrossoverMultiPoint::new(0.7, 0.8, genes_size / 10, true).into(),
        ];
        for mut crossover in crossovers {
            group.throughput(Throughput::Elements(population_size as u64));
            let (genotype, state) = setup(*genes_size, population_size, &mut rng);
            group.bench_with_input(
                BenchmarkId::new(format!("{:?}", crossover), genes_size),
                genes_size,
                |b, &_genes_size| {
                    b.iter_batched(
                        || state.clone(),
                        |mut data| {
                            crossover.call(&genotype, &mut data, &config, &mut reporter, &mut rng)
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
