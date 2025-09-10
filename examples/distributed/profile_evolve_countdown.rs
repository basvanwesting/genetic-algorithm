use genetic_algorithm::distributed::fitness::placeholders::CountdownNoisy;
use genetic_algorithm::distributed::strategy::evolve::prelude::*;
use std::ops::RangeInclusive;

const GENES_SIZE: usize = 40_695;
#[allow(dead_code)]
const ALLELE_RANGE: RangeInclusive<f32> = -150.0..=120.0;
const REPLACEMENT_RATE: f32 = 0.5;
const ELITISM_RATE: f32 = 0.02;
const SELECTION_RATE: f32 = 0.4;
const CROSSOVER_RATE: f32 = 0.8;
const POPULATION_SIZE: usize = 225;
const TARGET_GENERATION: usize = (500_f32 * SELECTION_RATE) as usize;
const TOURNAMENT_SIZE: usize = 20;
const MUTATIONS_PER_CHROMOSOME: usize = 50;

// Crossover is where the main work is taking place in the base loop

fn main() {
    let genotype = RangeGenotype::builder()
        .with_genes_size(GENES_SIZE)
        .with_allele_range(ALLELE_RANGE)
        .build()
        .unwrap();
    // let genotype = BinaryGenotype::builder()
    //     .with_genes_size(GENES_SIZE)
    //     .build()
    //     .unwrap();

    println!("{}", genotype);

    let evolve_builder = Evolve::builder()
        .with_genotype(genotype)
        // .with_select(SelectElite::new(REPLACEMENT_RATE, ELITISM_RATE))
        .with_select(SelectTournament::new(
            REPLACEMENT_RATE,
            ELITISM_RATE,
            TOURNAMENT_SIZE,
        ))
        // .with_crossover(CrossoverClone::new(SELECTION_RATE))
        // .with_crossover(CrossoverRejuvenate::new(1.0))
        .with_crossover(CrossoverMultiPoint::new(
            SELECTION_RATE,
            CROSSOVER_RATE,
            9,
            false,
        ))
        .with_mutate(MutateMultiGene::new(MUTATIONS_PER_CHROMOSOME, 0.2))
        .with_reporter(EvolveReporterSimple::new(100))
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
//         .with_select(SelectTournament::new(REPLACEMENT_RATE, ELITISM_RATE, TOURNAMENT_SIZE))
//         .with_crossover(CrossoverMultiPoint::new(1.0))
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
