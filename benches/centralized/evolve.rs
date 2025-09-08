use criterion::*;
use genetic_algorithm::centralized::fitness::placeholders::StaticZero;
use genetic_algorithm::centralized::genotype::StaticBinaryGenotype;
use genetic_algorithm::centralized::strategy::evolve::prelude::*;
use std::time::Duration;

pub fn criterion_benchmark(c: &mut Criterion) {
    const GENES_SIZE: usize = 100;
    const TARGET_POPULATION_SIZE: usize = 100;
    const MAX_POPULATION_SIZE: usize = 300; // For static genotype capacity
    let max_stale_generations = 100;

    let mut group = c.benchmark_group("evolve");
    group.warm_up_time(Duration::from_secs(5));
    group.measurement_time(Duration::from_secs(10));
    //let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    //group.plot_config(plot_config);

    group.bench_function(
        format!(
            "binary-{}-pop{}-gen{}",
            GENES_SIZE, TARGET_POPULATION_SIZE, max_stale_generations
        ),
        |b| {
            let genotype = StaticBinaryGenotype::<GENES_SIZE, MAX_POPULATION_SIZE>::builder()
                .with_genes_size(GENES_SIZE)
                .build()
                .unwrap();

            let evolve_builder = Evolve::builder()
                .with_genotype(genotype)
                .with_target_population_size(TARGET_POPULATION_SIZE)
                .with_max_stale_generations(max_stale_generations)
                .with_mutate(MutateSingleGene::new(0.2))
                .with_fitness(StaticZero::new())
                .with_crossover(CrossoverSingleGene::new(0.7, 0.8))
                .with_select(SelectTournament::new(0.5, 0.02, 4));

            b.iter_batched(
                || evolve_builder.clone().build().unwrap(),
                |mut e| e.call(),
                BatchSize::SmallInput,
            );
        },
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
