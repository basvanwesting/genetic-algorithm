use criterion::*;
use genetic_algorithm::evolve::prelude::*;
use genetic_algorithm::fitness::placeholders::Zero;
use rand::prelude::*;
use rand::rngs::SmallRng;
use std::time::Duration;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = SmallRng::from_entropy();
    let genes_size = 100;
    let population_size = 100;
    let max_stale_generations = 100;

    let mut group = c.benchmark_group("evolve");
    group.warm_up_time(Duration::from_secs(5));
    group.measurement_time(Duration::from_secs(10));
    //let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    //group.plot_config(plot_config);

    group.bench_function(
        format!(
            "binary-{}-pop{}-gen{}",
            genes_size, population_size, max_stale_generations
        ),
        |b| {
            let genotype = BinaryGenotype::builder()
                .with_genes_size(genes_size)
                .build()
                .unwrap();

            let evolve_builder = Evolve::builder()
                .with_genotype(genotype)
                .with_population_size(population_size)
                .with_max_stale_generations(max_stale_generations)
                .with_mutate(MutateOnce(0.2))
                .with_fitness(Zero::new())
                .with_crossover(CrossoverSingleGene(true))
                .with_compete(CompeteTournament(4));

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
            genes_size, population_size, max_stale_generations
        ),
        |b| {
            let genotype = DiscreteGenotype::builder()
                .with_genes_size(genes_size)
                .with_allele_values((0..10).collect())
                .build()
                .unwrap();

            let evolve_builder = Evolve::builder()
                .with_genotype(genotype)
                .with_population_size(population_size)
                .with_max_stale_generations(max_stale_generations)
                .with_mutate(MutateOnce(0.2))
                .with_fitness(Zero::new())
                .with_crossover(CrossoverSingleGene(true))
                .with_compete(CompeteTournament(4));

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
