use criterion::*;
use pprof::criterion::*;

use genetic_algorithm::fitness::placeholders::CountTrue;
use genetic_algorithm::strategy::evolve::prelude::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(1000)
        .build()
        .unwrap();

    let evolve_builder = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(1000)
        .with_max_stale_generations(1000)
        .with_target_fitness_score(0)
        .with_fitness(CountTrue)
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_mutate(MutateSingleGene::new(0.2))
        .with_crossover(CrossoverSinglePoint::new(0.5))
        .with_compete(CompeteTournament::new(4));

    c.bench_function("profile_evolve_binary", |b| {
        b.iter_batched(
            || evolve_builder.clone().build().unwrap(),
            |mut e| e.call(),
            BatchSize::SmallInput,
        );
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = criterion_benchmark
}

criterion_main!(benches);
