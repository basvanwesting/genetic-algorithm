use criterion::*;
use genetic_algorithm::fitness::placeholders::CountTrue;
use genetic_algorithm::fitness::Fitness;
use genetic_algorithm::genotype::{BinaryAllele, BinaryGenotype, Genotype};
use genetic_algorithm::mutate::*;
use genetic_algorithm::population::Population;
use genetic_algorithm::strategy::evolve::{EvolveConfig, EvolveReporterNoop, EvolveState};
use rand::prelude::*;
use rand::rngs::SmallRng;
//use std::time::Duration;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut reporter = EvolveReporterNoop::<BinaryAllele>::new();
    let mut rng = SmallRng::from_entropy();

    let population_sizes = vec![250, 500, 1000, 2000];

    let mut group = c.benchmark_group("mutates");
    //group.warm_up_time(Duration::from_secs(3));
    //group.measurement_time(Duration::from_secs(3));
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);

    let genotype = BinaryGenotype::builder()
        .with_genes_size(100)
        .build()
        .unwrap();

    for population_size in &population_sizes {
        let mutates: Vec<MutateWrapper> = vec![
            MutateSingleGene::new(0.2).into(),
            // MutateMultiGene::new(1, 0.2).into(),
            MutateMultiGene::new(2, 0.2).into(),
            // MutateMultiGene::new(3, 0.2).into(),
            MutateSingleGeneDynamic::new(0.2, population_size / 2).into(),
            MutateMultiGeneDynamic::new(2, 0.2, population_size / 2).into(),
        ];
        for mut mutate in mutates {
            group.throughput(Throughput::Elements(*population_size as u64));

            let chromosomes = (0..*population_size)
                .map(|_| genotype.chromosome_factory(&mut rng))
                .collect();
            let population = Population::new(chromosomes);
            let mut state = EvolveState::new(&genotype, population);
            let config = EvolveConfig::new();
            CountTrue.call_for_population(&mut state.population, None);

            group.bench_with_input(
                BenchmarkId::new(format!("{:?}", mutate), population_size),
                population_size,
                |b, &_population_size| {
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
