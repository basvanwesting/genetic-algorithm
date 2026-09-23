#[cfg(test)]
use crate::support::*;
use genetic_algorithm::fitness::placeholders::{CountTrue, SumGenes};
use genetic_algorithm::genotype::HillClimbGenotype;
use genetic_algorithm::strategy::hill_climb::prelude::*;

#[test]
fn build_invalid_missing_ending_condition() {
    let genotype = RangeGenotype::builder()
        .with_genes_size(10)
        .with_allele_range(0.0..=1.0)
        .with_mutation_type(MutationType::Range(0.1))
        .build()
        .unwrap();

    let hill_climb = HillClimb::builder()
        .with_genotype(genotype)
        .with_fitness(SumGenes::new_with_precision(1e-3))
        // .with_reporter(StrategyReporterNoop::new())
        .build();

    assert!(hill_climb.is_err());
    assert_eq!(
        hill_climb.err(),
        Some(TryFromHillClimbBuilderError(
            "HillClimb requires at least a max_stale_generations, max_generations or target_fitness_score ending condition"
        ))
    );
}

#[test]
fn call_range_max_stale_generations_maximize() {
    let genotype = RangeGenotype::builder()
        .with_genes_size(10)
        .with_allele_range(0.0..=1.0)
        .with_mutation_type(MutationType::Range(0.1))
        .build()
        .unwrap();
    let hill_climb = HillClimb::builder()
        .with_genotype(genotype)
        .with_max_stale_generations(1000)
        .with_fitness(SumGenes::new_with_precision(1e-3))
        .with_reporter(StrategyReporterNoop::new())
        .with_rng_seed_from_u64(0)
        .call()
        .unwrap();

    println!("{:#?}", hill_climb.best_genes());
    assert_eq!(hill_climb.best_fitness_score(), Some(10000));
    assert!(relative_chromosome_eq(
        hill_climb.best_genes().unwrap(),
        vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,],
        0.001
    ));
}

#[test]
fn call_range_max_stale_generations_minimize() {
    let genotype = RangeGenotype::builder()
        .with_genes_size(10)
        .with_allele_range(0.0..=1.0)
        .with_mutation_type(MutationType::Range(0.1))
        .build()
        .unwrap();
    let hill_climb = HillClimb::builder()
        .with_genotype(genotype)
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_max_stale_generations(100)
        .with_fitness(SumGenes::new_with_precision(1e-3))
        // .with_reporter(StrategyReporterNoop::new())
        .with_rng_seed_from_u64(0)
        .call()
        .unwrap();

    println!("{:#?}", hill_climb.best_genes());
    assert_eq!(hill_climb.best_fitness_score(), Some(0));
    assert!(relative_chromosome_eq(
        hill_climb.best_genes().unwrap(),
        vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,],
        0.001
    ));
}

#[test]
fn call_range_max_generations_maximize() {
    let genotype = RangeGenotype::builder()
        .with_genes_size(10)
        .with_allele_range(0.0..=1.0)
        .with_mutation_type(MutationType::Range(0.1))
        .build()
        .unwrap();
    let hill_climb = HillClimb::builder()
        .with_genotype(genotype)
        .with_max_generations(1000)
        .with_fitness(SumGenes::new_with_precision(1e-3))
        .with_reporter(StrategyReporterNoop::new())
        .with_rng_seed_from_u64(0)
        .call()
        .unwrap();

    println!("{:#?}", hill_climb.best_genes());
    assert_eq!(hill_climb.best_fitness_score(), Some(10000));
    assert!(relative_chromosome_eq(
        hill_climb.best_genes().unwrap(),
        vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,],
        0.001
    ));
}

#[test]
fn call_range_max_stale_generations_and_valid_fitness_score_maximize() {
    let genotype = RangeGenotype::builder()
        .with_genes_size(100)
        .with_allele_range(0.0..=1.0)
        .with_mutation_type(MutationType::Range(0.1))
        .build()
        .unwrap();
    let hill_climb = HillClimb::builder()
        .with_genotype(genotype)
        .with_max_stale_generations(10)
        .with_valid_fitness_score(75000)
        .with_fitness(SumGenes::new_with_precision(1e-3))
        .with_reporter(StrategyReporterNoop::new())
        .with_rng_seed_from_u64(0)
        .call()
        .unwrap();

    println!("{:#?}", hill_climb.best_genes());
    assert_eq!(hill_climb.best_fitness_score(), Some(81575));
}

#[test]
fn call_range_max_stale_generations_and_valid_fitness_score_minimize() {
    let genotype = RangeGenotype::builder()
        .with_genes_size(100)
        .with_allele_range(0.0..=1.0)
        .with_mutation_type(MutationType::Range(0.1))
        .build()
        .unwrap();
    let hill_climb = HillClimb::builder()
        .with_genotype(genotype)
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_max_stale_generations(10)
        .with_valid_fitness_score(25000)
        .with_fitness(SumGenes::new_with_precision(1e-3))
        // .with_reporter(StrategyReporterNoop::new())
        .with_rng_seed_from_u64(0)
        .call()
        .unwrap();

    println!("{:#?}", hill_climb.best_genes());
    assert_eq!(hill_climb.best_fitness_score(), Some(23706));
}

#[test]
fn call_range_target_fitness_score_maximize() {
    let genotype = RangeGenotype::builder()
        .with_genes_size(10)
        .with_allele_range(0.0..=1.0)
        .with_mutation_type(MutationType::Range(0.1))
        .build()
        .unwrap();
    let hill_climb = HillClimb::builder()
        .with_genotype(genotype)
        .with_target_fitness_score(8000)
        .with_fitness(SumGenes::new_with_precision(1e-3))
        .with_reporter(StrategyReporterNoop::new())
        .with_rng_seed_from_u64(0)
        .call()
        .unwrap();

    println!("{:#?}", hill_climb.best_genes());
    assert_eq!(hill_climb.best_fitness_score(), Some(8008));
    assert!(relative_chromosome_eq(
        hill_climb.best_genes().unwrap(),
        vec![0.567, 0.651, 1.0, 0.696, 1.0, 1.0, 0.785, 0.899, 0.490, 0.918],
        0.001
    ));
}

#[test]
fn call_range_target_fitness_score_minimize() {
    let genotype = RangeGenotype::builder()
        .with_genes_size(10)
        .with_allele_range(0.0..=1.0)
        .with_mutation_type(MutationType::Range(0.1))
        .build()
        .unwrap();
    let hill_climb = HillClimb::builder()
        .with_genotype(genotype)
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_target_fitness_score(1000)
        .with_fitness(SumGenes::new_with_precision(1e-3))
        // .with_reporter(StrategyReporterNoop::new())
        .with_rng_seed_from_u64(0)
        .call()
        .unwrap();

    println!("{:#?}", hill_climb.best_genes());
    assert_eq!(hill_climb.best_fitness_score(), Some(972));
    assert!(relative_chromosome_eq(
        hill_climb.best_genes().unwrap(),
        vec![0.0, 0.0, 0.395, 0.0, 0.364, 0.0, 0.0, 0.0, 0.0, 0.211],
        0.001
    ));
}

#[test]
fn call_range_par_fitness() {
    let genotype = RangeGenotype::builder()
        .with_genes_size(10)
        .with_allele_range(0.0..=1.0)
        .with_mutation_type(MutationType::Range(0.1))
        .build()
        .unwrap();
    let hill_climb = HillClimb::builder()
        .with_genotype(genotype)
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_par_fitness(true)
        .with_target_fitness_score(1000)
        .with_fitness(SumGenes::new_with_precision(1e-3))
        .with_reporter(StrategyReporterNoop::new())
        .with_rng_seed_from_u64(0)
        .call()
        .unwrap();

    println!("{:#?}", hill_climb.best_genes());
    assert_eq!(hill_climb.best_fitness_score(), Some(972));
    assert!(relative_chromosome_eq(
        hill_climb.best_genes().unwrap(),
        vec![0.0, 0.0, 0.395, 0.0, 0.364, 0.0, 0.0, 0.0, 0.0, 0.211],
        0.001
    ));
}

#[test]
fn call_binary_stochastic() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(100)
        .build()
        .unwrap();
    let hill_climb = HillClimb::builder()
        .with_genotype(genotype)
        .with_variant(HillClimbVariant::Stochastic)
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_target_fitness_score(0)
        .with_fitness(CountTrue)
        // .with_reporter(StrategyReporterNoop::new())
        .with_rng_seed_from_u64(0)
        .call()
        .unwrap();

    println!("{:#?}", hill_climb.best_genes());
    assert_eq!(hill_climb.best_fitness_score(), Some(0));
}

#[test]
fn call_binary_steepest_ascent() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(100)
        .build()
        .unwrap();
    assert_eq!(
        genotype.neighbouring_population_size(),
        BigUint::from(100_u32)
    );
    let hill_climb = HillClimb::builder()
        .with_genotype(genotype)
        .with_variant(HillClimbVariant::SteepestAscent)
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_target_fitness_score(0)
        .with_fitness(CountTrue)
        .with_reporter(StrategyReporterNoop::new())
        .with_rng_seed_from_u64(0)
        .call()
        .unwrap();

    println!("{:#?}", hill_climb.best_genes());
    assert_eq!(hill_climb.best_fitness_score(), Some(0));
}

#[test]
fn call_abort_flag_preset_returns_immediately() {
    use std::sync::atomic::AtomicBool;
    use std::sync::Arc;

    let genotype = BinaryGenotype::builder()
        .with_genes_size(10)
        .build()
        .unwrap();

    let abort_flag = Arc::new(AtomicBool::new(true));
    let hill_climb = HillClimb::builder()
        .with_genotype(genotype)
        .with_max_stale_generations(1000)
        .with_fitness(CountTrue)
        .with_abort_flag(abort_flag.clone())
        .with_rng_seed_from_u64(0)
        .call()
        .unwrap();

    // is_aborted() short-circuits the loop before any generation runs; best comes from setup
    assert_eq!(hill_climb.state.current_generation, 0);
    assert!(hill_climb.best_fitness_score().is_some());
}

#[test]
fn call_repeatedly_abort_flag_preset_stops_after_first_run() {
    use std::sync::atomic::AtomicBool;
    use std::sync::Arc;

    let genotype = BinaryGenotype::builder()
        .with_genes_size(10)
        .build()
        .unwrap();

    let abort_flag = Arc::new(AtomicBool::new(true));
    let (_best_run, other_runs) = HillClimb::builder()
        .with_genotype(genotype)
        .with_max_stale_generations(1000)
        .with_fitness(CountTrue)
        .with_abort_flag(abort_flag.clone())
        .with_rng_seed_from_u64(0)
        .call_repeatedly(5)
        .unwrap();

    // the abort flag short-circuits the repeat loop after the first run, so no contenders remain
    assert_eq!(other_runs.len(), 0);
}

#[test]
fn call_repeatedly_with_rng_seed_gives_distinct_deterministic_runs() {
    let run = || {
        let genotype = RangeGenotype::builder()
            .with_genes_size(5)
            .with_allele_range(0.0..=1.0)
            .build()
            .unwrap();
        let (best_run, other_runs) = HillClimb::builder()
            .with_genotype(genotype)
            .with_max_stale_generations(20)
            .with_fitness(SumGenes::new_with_precision(1e-3))
            .with_rng_seed_from_u64(0)
            .call_repeatedly(4)
            .unwrap();
        let mut runs: Vec<(usize, Vec<f32>)> = std::iter::once(best_run)
            .chain(other_runs)
            .map(|run| (run.state.current_iteration, run.best_genes().unwrap()))
            .collect();
        runs.sort_by_key(|(iteration, _)| *iteration);
        runs
    };

    let runs = run();
    assert_eq!(runs.len(), 4);
    // each iteration has its own rng, so the runs differ
    for i in 1..runs.len() {
        assert_ne!(runs[0].1, runs[i].1);
    }
    // but they are still deterministic
    assert_eq!(runs, run());
}
