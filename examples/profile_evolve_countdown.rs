use genetic_algorithm::fitness::placeholders::CountdownNoisy;
use genetic_algorithm::strategy::evolve::prelude::*;

const GENES_SIZE: usize = 50; // assumption
const POPULATION_SIZE: usize = 150;
const TARGET_GENERATION: usize = 500;

fn main() {
    let genotype = RangeGenotype::builder()
        .with_genes_size(GENES_SIZE)
        .with_allele_range(-1.0..=1.0)
        .build()
        .unwrap();

    let evolve_builder = Evolve::builder()
        .with_genotype(genotype)
        .with_compete(CompeteTournament::new(4))
        // .with_crossover(CrossoverSinglePoint::new(true))
        .with_crossover(CrossoverMultiGene::new(GENES_SIZE / 2, true))
        .with_mutate(MutateMultiGene::new(2, 0.2))
        // .with_reporter(EvolveReporterSimple::new(100))
        .with_fitness(CountdownNoisy::new(
            POPULATION_SIZE * TARGET_GENERATION,
            POPULATION_SIZE * 10,
            1..(POPULATION_SIZE * 10),
        ))
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_max_stale_generations(100)
        .with_target_fitness_score(0)
        .with_target_population_size(POPULATION_SIZE);

    let now = std::time::Instant::now();
    let evolve = evolve_builder.call().unwrap();
    let duration = now.elapsed();

    println!(
        "best_generation: {:?}, best fitness score: {:?}, duration: {:?}",
        evolve.best_generation(),
        evolve.best_fitness_score(),
        duration
    );
}

// best_generation: 500, best fitness score: Some(0), duration: 28.202579ms
// best_generation: 500, best fitness score: Some(0), duration: 21.239145ms
// best_generation: 500, best fitness score: Some(0), duration: 23.615032ms
// best_generation: 500, best fitness score: Some(0), duration: 23.711847ms
// best_generation: 500, best fitness score: Some(0), duration: 24.961629ms
// best_generation: 500, best fitness score: Some(0), duration: 21.501972ms
// best_generation: 500, best fitness score: Some(0), duration: 25.519464ms

// use criterion::*;
// use pprof::criterion::*;
// pub fn criterion_benchmark(c: &mut Criterion) {
//     let genotype = RangeGenotype::builder()
//         .with_genes_size(GENES_SIZE)
//         .with_allele_range(-1.0..=1.0)
//         .build()
//         .unwrap();
//
//     let evolve_builder = Evolve::builder()
//         .with_genotype(genotype)
//         .with_compete(CompeteTournament::new(4))
//         // .with_crossover(CrossoverSinglePoint::new(true))
//         .with_crossover(CrossoverMultiGene::new(GENES_SIZE / 2, true))
//         .with_mutate(MutateMultiGene::new(2, 0.2))
//         // .with_reporter(EvolveReporterSimple::new(100))
//         .with_fitness(CountdownNoisy::new(
//             POPULATION_SIZE * TARGET_GENERATION,
//             POPULATION_SIZE * 10,
//             1..(POPULATION_SIZE * 10),
//         ))
//         .with_fitness_ordering(FitnessOrdering::Minimize)
//         .with_max_stale_generations(100)
//         .with_target_fitness_score(0)
//         .with_target_population_size(POPULATION_SIZE);
//
//     c.bench_function("profile_evolve_countdown", |b| {
//         b.iter_batched(
//             || evolve_builder.clone().build().unwrap(),
//             |mut e| e.call(),
//             BatchSize::SmallInput,
//         );
//     });
// }
// criterion_group! {
//     name = benches;
//     config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
//     targets = criterion_benchmark
// }
// criterion_main!(benches);
