use criterion::*;
use genetic_algorithm::fitness::placeholders::CountTrue;
use genetic_algorithm::fitness::Fitness;
use genetic_algorithm::genotype::{BinaryGenotype, Genotype};
use genetic_algorithm::mutate::*;
use genetic_algorithm::population::Population;
use rand::prelude::*;
use rand::rngs::SmallRng;
//use std::time::Duration;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = SmallRng::from_entropy();
    let population_sizes = vec![250, 500, 1000, 2000];

    let mutates: Vec<MutateWrapper> = vec![
        MutateSingleGeneRandom::new(0.2).into(),
        MutateTwice::new(0.2).into(),
        MutateDynamicOnce::new(0.2, 0.5).into(),
        MutateDynamicRounds::new(0.2, 0.5).into(),
    ];

    let mut group = c.benchmark_group("mutates");
    //group.warm_up_time(Duration::from_secs(3));
    //group.measurement_time(Duration::from_secs(3));
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);

    let genotype = BinaryGenotype::builder()
        .with_genes_size(100)
        .build()
        .unwrap();

    for mut mutate in mutates {
        for population_size in &population_sizes {
            group.throughput(Throughput::Elements(*population_size as u64));

            let chromosomes = (0..*population_size)
                .map(|_| genotype.chromosome_factory(&mut rng))
                .collect();
            let population = &mut Population::new(chromosomes);
            CountTrue.call_for_population(population, None);

            group.bench_with_input(
                BenchmarkId::new(format!("{:?}", mutate), population_size),
                population_size,
                |b, &_population_size| {
                    b.iter_batched(
                        || population.clone(),
                        |mut data| mutate.call(&genotype, &mut data, &mut rng),
                        BatchSize::SmallInput,
                    )
                },
            );
        }
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
