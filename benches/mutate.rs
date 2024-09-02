use criterion::*;
use genetic_algorithm::fitness::placeholders::CountTrue;
use genetic_algorithm::fitness::Fitness;
use genetic_algorithm::genotype::{BinaryGenotype, Genotype};
use genetic_algorithm::mutate::*;
use genetic_algorithm::population::Population;
use genetic_algorithm::strategy::evolve::{EvolveConfig, EvolveReporterNoop, EvolveState};
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
        .map(|_| genotype.chromosome_factory(rng))
        .collect();

    let mut population = Population::new(chromosomes);
    CountTrue.call_for_population(&mut population, None);
    let mut state = EvolveState::new(&genotype);
    state.population = population;
    (genotype, state)
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let config = EvolveConfig::new();
    let mut reporter = EvolveReporterNoop::<BinaryGenotype>::new();
    let mut rng = SmallRng::from_entropy();
    let population_size: usize = 1000;
    let genes_sizes = vec![100, 10000];

    let mut group = c.benchmark_group(format!("mutates-pop{}", population_size));
    //group.warm_up_time(Duration::from_secs(3));
    //group.measurement_time(Duration::from_secs(3));
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);

    for genes_size in &genes_sizes {
        let mutates: Vec<MutateWrapper> = vec![
            MutateSingleGene::new(0.2).into(),
            MutateMultiGene::new(10, 0.2).into(),
            MutateSingleGeneDynamic::new(0.2, population_size / 2).into(),
            MutateMultiGeneDynamic::new(10, 0.2, population_size / 2).into(),
        ];
        for mut mutate in mutates {
            group.throughput(Throughput::Elements(population_size as u64));
            let (genotype, state) = setup(*genes_size, population_size, &mut rng);
            group.bench_with_input(
                BenchmarkId::new(format!("{:?}", mutate), genes_size),
                genes_size,
                |b, &_genes_size| {
                    b.iter_batched(
                        || state.clone(),
                        |mut data| {
                            mutate.call(&genotype, &mut data, &config, &mut reporter, &mut rng)
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
