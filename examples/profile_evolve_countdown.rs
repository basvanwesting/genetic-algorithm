use genetic_algorithm::fitness::placeholders::CountdownNoisy;
use genetic_algorithm::strategy::evolve::prelude::*;
use std::ops::RangeInclusive;

const GENES_SIZE: usize = 100_000;
#[allow(dead_code)]
const ALLELE_RANGE: RangeInclusive<f32> = -150.0..=120.0;
const SELECTION_RATE: f32 = 0.8;
const POPULATION_SIZE: usize = 200;
const TARGET_GENERATION: usize = (500_f32 * SELECTION_RATE) as usize;
const TOURNAMENT_SIZE: usize = 20;
const MUTATIONS_PER_CHROMOSOME: usize = 50;
const USE_CHROMOSOME_STACK: bool = true;

// Crossover is where the main work is taking place in the base loop

fn main() {
    let genotype = RangeGenotype::builder()
        .with_genes_size(GENES_SIZE)
        .with_allele_range(ALLELE_RANGE)
        .with_chromosome_stack(USE_CHROMOSOME_STACK)
        .build()
        .unwrap();
    // let genotype = MatrixGenotype::<f32, GENES_SIZE, { POPULATION_SIZE + 2 }>::builder()
    //     .with_genes_size(GENES_SIZE)
    //     .with_allele_range(ALLELE_RANGE)
    //     .build()
    //     .unwrap();
    // let genotype = BinaryGenotype::builder()
    //     .with_genes_size(GENES_SIZE)
    //     .with_chromosome_stack(USE_CHROMOSOME_STACK)
    //     .build()
    //     .unwrap();
    // let genotype = BitGenotype::builder()
    //     .with_genes_size(GENES_SIZE)
    //     .with_chromosome_stack(USE_CHROMOSOME_STACK)
    //     .build()
    //     .unwrap();

    let evolve_builder = Evolve::builder()
        .with_genotype(genotype)
        // .with_select(SelectElite)
        .with_select(SelectTournament::new(TOURNAMENT_SIZE, 0.8))
        // .with_crossover(CrossoverClone::new())
        .with_crossover(CrossoverMultiPoint::new(9, false))
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
//         .with_select(SelectTournament::new(TOURNAMENT_SIZE))
//         .with_crossover(CrossoverMultiPoint::new(9, false))
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
