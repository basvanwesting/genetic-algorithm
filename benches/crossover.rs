use criterion::*;
use genetic_algorithm::crossover::*;
use genetic_algorithm::genotype::{BinaryAllele, BinaryGenotype, Genotype};
use genetic_algorithm::population::Population;
use genetic_algorithm::strategy::evolve::{EvolveConfig, EvolveReporterNoop, EvolveState};
use rand::prelude::*;
use rand::rngs::SmallRng;

pub fn setup(
    genes_size: usize,
    population_size: usize,
    rng: &mut SmallRng,
) -> (BinaryGenotype, EvolveState<BinaryAllele>) {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(genes_size)
        .build()
        .unwrap();

    let chromosomes = (0..population_size)
        .map(|_| genotype.chromosome_factory(rng))
        .collect();

    let population = Population::new(chromosomes);
    let state = EvolveState::new(&genotype, population);
    (genotype, state)
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let config = EvolveConfig::new();
    let mut reporter = EvolveReporterNoop::<BinaryAllele>::new();
    let mut rng = SmallRng::from_entropy();
    let population_sizes = vec![100, 1000];
    let genes_sizes = vec![10, 100];

    let crossovers: Vec<CrossoverWrapper> = vec![
        // CrossoverSingleGene::new(true).into(),
        // CrossoverSingleGene::new(false).into(),
        // CrossoverUniform::new(true).into(),
        CrossoverUniform::new(false).into(),
        // CrossoverSinglePoint::new(true).into(),
        // CrossoverSinglePoint::new(false).into(),
        // CrossoverClone::new(true).into(),
        //CrossoverClone::new(false).into(), //noop
    ];

    let mut group = c.benchmark_group("crossovers");
    //group.warm_up_time(Duration::from_secs(3));
    //group.measurement_time(Duration::from_secs(3));
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);

    for mut crossover in crossovers {
        for population_size in &population_sizes {
            for genes_size in &genes_sizes {
                let (genotype, state) = setup(*genes_size, *population_size, &mut rng);
                group.bench_function(
                    BenchmarkId::new(
                        format!("{:?}-single-thread", crossover),
                        format!("{:?}-{:?}", genes_size, population_size),
                    ),
                    |b| {
                        b.iter_batched(
                            || state.clone(),
                            |mut data| {
                                crossover.call(
                                    &genotype,
                                    &mut data,
                                    &config,
                                    &mut reporter,
                                    &mut rng,
                                    false,
                                )
                            },
                            BatchSize::SmallInput,
                        )
                    },
                );

                // reuse thread fitness for all runs (as in evolve loop)
                group.bench_function(
                    BenchmarkId::new(
                        format!("{:?}-multi-thread", crossover),
                        format!("{:?}-{:?}", genes_size, population_size),
                    ),
                    |b| {
                        b.iter_batched(
                            || state.clone(),
                            |mut data| {
                                crossover.call(
                                    &genotype,
                                    &mut data,
                                    &config,
                                    &mut reporter,
                                    &mut rng,
                                    true,
                                )
                            },
                            BatchSize::SmallInput,
                        )
                    },
                );
            }
        }
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
