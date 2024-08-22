use criterion::*;
use genetic_algorithm::crossover::*;
use genetic_algorithm::genotype::{BinaryAllele, BinaryGenotype, Genotype};
use genetic_algorithm::population::Population;
use genetic_algorithm::strategy::evolve::{EvolveConfig, EvolveReporterNoop, EvolveState};
use rand::prelude::*;
use rand::rngs::SmallRng;
use thread_local::ThreadLocal;
//use std::time::Duration;

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
    let population_size: usize = 1000;
    let genes_sizes = vec![10, 100, 1000, 10000];

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

    let mut group = c.benchmark_group(format!("crossovers-pop{}", population_size));
    //group.warm_up_time(Duration::from_secs(3));
    //group.measurement_time(Duration::from_secs(3));
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);

    for mut crossover in crossovers {
        for genes_size in &genes_sizes {
            group.throughput(Throughput::Elements(population_size as u64));
            let (genotype, state) = setup(*genes_size, population_size, &mut rng);
            group.bench_with_input(
                BenchmarkId::new(format!("{:?}-single-thread", crossover), genes_size),
                genes_size,
                |b, &_genes_size| {
                    b.iter_batched(
                        || state.clone(),
                        |mut data| {
                            crossover.call(
                                &genotype,
                                &mut data,
                                &config,
                                &mut reporter,
                                &mut rng,
                                None,
                            )
                        },
                        BatchSize::SmallInput,
                    )
                },
            );

            // reuse thread fitness for all runs (as in evolve loop)
            let rng_thread_local = Some(ThreadLocal::new());
            group.bench_with_input(
                BenchmarkId::new(format!("{:?}-multi-thread", crossover), genes_size),
                genes_size,
                |b, &_genes_size| {
                    b.iter_batched(
                        || state.clone(),
                        |mut data| {
                            crossover.call(
                                &genotype,
                                &mut data,
                                &config,
                                &mut reporter,
                                &mut rng,
                                rng_thread_local.as_ref(),
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
