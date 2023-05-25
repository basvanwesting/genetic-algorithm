use criterion::*;
use genetic_algorithm::fitness::placeholders::Zero;
use genetic_algorithm::strategy::evolve::prelude::*;
use rand::prelude::*;
use rand::rngs::SmallRng;
use std::time::Duration;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = SmallRng::from_entropy();
    let genes_size = 100;
    let target_population_size = 100;
    let max_stale_generations = 100;

    let mut group = c.benchmark_group("evolve");
    group.warm_up_time(Duration::from_secs(5));
    group.measurement_time(Duration::from_secs(10));
    //let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    //group.plot_config(plot_config);

    group.bench_function(
        format!(
            "binary-{}-pop{}-gen{}",
            genes_size, target_population_size, max_stale_generations
        ),
        |b| {
            let genotype = BinaryGenotype::builder()
                .with_genes_size(genes_size)
                .build()
                .unwrap();

            let evolve_builder = Evolve::builder()
                .with_genotype(genotype)
                .with_target_population_size(target_population_size)
                .with_max_stale_generations(max_stale_generations)
                .with_mutate(MutateOnce::new(0.2))
                .with_fitness(Zero::new())
                .with_crossover(CrossoverSingleGene::new(true))
                .with_compete(CompeteTournament::new(4).into())
                .with_extension(ExtensionNoop::new());

            b.iter_batched(
                || evolve_builder.clone().build().unwrap(),
                |mut e| e.call(&mut rng),
                BatchSize::SmallInput,
            );
        },
    );

    group.bench_function(
        format!(
            "discrete-{}-pop{}-gen{}",
            genes_size, target_population_size, max_stale_generations
        ),
        |b| {
            let genotype = DiscreteGenotype::builder()
                .with_genes_size(genes_size)
                .with_allele_list((0..10).collect())
                .build()
                .unwrap();

            let evolve_builder = Evolve::builder()
                .with_genotype(genotype)
                .with_target_population_size(target_population_size)
                .with_max_stale_generations(max_stale_generations)
                .with_mutate(MutateOnce::new(0.2))
                .with_fitness(Zero::new())
                .with_crossover(CrossoverSingleGene::new(true))
                .with_compete(CompeteTournament::new(4).into())
                .with_extension(ExtensionNoop::new());

            b.iter_batched(
                || evolve_builder.clone().build().unwrap(),
                |mut e| e.call(&mut rng),
                BatchSize::SmallInput,
            );
        },
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
