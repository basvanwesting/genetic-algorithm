use genetic_algorithm::fitness::placeholders::CountdownNoisy;
use genetic_algorithm::strategy::evolve::prelude::*;
use std::ops::RangeInclusive;

const GENES_SIZE: usize = 40695;
const ALLELE_RANGE: RangeInclusive<f32> = -150.0..=120.0;
const POPULATION_SIZE: usize = 150;
const TARGET_GENERATION: usize = 500;
const TOURNAMENT_SIZE: usize = 20;
const MUTATIONS_PER_CHROMOSOME: usize = 50;

// Crossover is where the main work is taking place in the base loop
// * Total base runtime (60ms) using crossover with 1 gene, don't keep parents
//   * crossover single point adds about 1s. Because the genes are partially cloned which is expensive
//   * crossover number of genes scale linearly from 0 to 5s for 50% of the genes size.
//   * crossover deny duplicates scales linearly from 0 to 5s for 50% of the genes size (so unique
//     sampling is as expensive as swapping genes).
//   * crossover keep_parents (+8s, cloning full population is very expensive)
//   * mutation number of genes scale linearly from 0 to 5s for 50% of the genes size.
//   * compete not a factor, it's basically some form of in-place sorting of some kind

fn main() {
    let genotype = RangeGenotype::builder()
        .with_genes_size(GENES_SIZE)
        .with_allele_range(ALLELE_RANGE)
        .build()
        .unwrap();

    let evolve_builder = Evolve::builder()
        .with_genotype(genotype)
        // .with_compete(CompeteElite)
        .with_compete(CompeteTournament::new(TOURNAMENT_SIZE))
        .with_crossover(CrossoverMultiPoint::new(9, false, false))
        // .with_crossover(CrossoverParMultiPoint::new(10, false, false))
        .with_mutate(MutateMultiGene::new(MUTATIONS_PER_CHROMOSOME, 0.2))
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

// use criterion::*;
// use pprof::criterion::*;
// pub fn criterion_benchmark(c: &mut Criterion) {
//     let genotype = RangeGenotype::builder()
//         .with_genes_size(GENES_SIZE)
//         .with_allele_range(ALLELE_RANGE)
//         .build()
//         .unwrap();
//
//     let evolve_builder = Evolve::builder()
//         .with_genotype(genotype)
//         .with_compete(CompeteTournament::new(TOURNAMENT_SIZE))
//         .with_crossover(CrossoverMultiPoint::new(9, false, false))
//         // .with_crossover(CrossoverParMultiPoint::new(10, false, false))
//         .with_mutate(MutateMultiGene::new(MUTATIONS_PER_CHROMOSOME, 0.2))
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
