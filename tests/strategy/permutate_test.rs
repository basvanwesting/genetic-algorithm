#[cfg(test)]
use crate::support::*;
use genetic_algorithm::fitness::placeholders::{CountTrue, SumGenes};
use genetic_algorithm::strategy::permutate::prelude::*;

//#[test]
//build_invalid cannot be tested because invalid doesn't even have a type

#[test]
fn call_binary_maximize() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(5)
        .build()
        .unwrap();

    let permutate = Permutate::builder()
        .with_genotype(genotype)
        .with_fitness(CountTrue)
        .with_reporter(StrategyReporterNoop::new())
        .call()
        .unwrap();

    println!("{:#?}", permutate.best_genes());
    assert_eq!(permutate.best_fitness_score(), Some(5));
    assert_eq!(
        permutate.best_genes().unwrap(),
        vec![true, true, true, true, true]
    );
}

#[test]
fn call_binary_minimize() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(5)
        .build()
        .unwrap();

    let permutate = Permutate::builder()
        .with_genotype(genotype)
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_fitness(CountTrue)
        // .with_reporter(StrategyReporterNoop::new())
        .call()
        .unwrap();

    println!("{:#?}", permutate.best_genes());
    assert_eq!(permutate.best_fitness_score(), Some(0));
    assert_eq!(
        permutate.best_genes().unwrap(),
        vec![false, false, false, false, false]
    );
}

#[test]
fn call_list() {
    let genotype = ListGenotype::builder()
        .with_genes_size(5)
        .with_allele_list((0..10).collect())
        .build()
        .unwrap();

    let permutate = Permutate::builder()
        .with_genotype(genotype)
        .with_fitness(SumGenes::new())
        .with_reporter(StrategyReporterNoop::new())
        .call()
        .unwrap();

    println!("{:#?}", permutate.best_genes());
    assert_eq!(permutate.best_fitness_score(), Some(45));
    assert_eq!(permutate.best_genes().unwrap(), vec![9, 9, 9, 9, 9]);
}

#[test]
fn call_multi_list() {
    let genotype = MultiListGenotype::builder()
        .with_allele_lists(vec![
            vec![0, 1, 2, 3, 4],
            vec![0, 1],
            vec![0],
            vec![0, 1, 2, 3],
        ])
        .build()
        .unwrap();

    let permutate = Permutate::builder()
        .with_genotype(genotype)
        .with_fitness(SumGenes::new())
        // .with_reporter(StrategyReporterNoop::new())
        .call()
        .unwrap();

    println!("{:#?}", permutate.best_genes());
    assert_eq!(permutate.best_fitness_score(), Some(8));
    assert_eq!(permutate.best_genes().unwrap(), vec![4, 1, 0, 3]);
}

#[test]
fn call_range_f32_scaled() {
    let genotype = RangeGenotype::builder()
        .with_genes_size(4)
        .with_allele_range(0.0..=1.0)
        .with_mutation_type(MutationType::StepScaled(vec![0.1, 0.01, 0.001]))
        .build()
        .unwrap();

    let permutate = Permutate::builder()
        .with_genotype(genotype)
        .with_fitness(SumGenes::new_with_precision(1e-3))
        .with_reporter(StrategyReporterNoop::new())
        .call()
        .unwrap();

    println!("{:#?}", permutate.best_genes());
    assert_eq!(permutate.best_fitness_score(), Some(4000));
    assert!(relative_chromosome_eq(
        permutate.best_genes().unwrap(),
        vec![1.0, 1.0, 1.0, 1.0],
        0.001
    ));
}

#[test]
fn call_range_usize_scaled() {
    let genotype = RangeGenotype::builder()
        .with_genes_size(4)
        .with_allele_range(0..=100)
        .with_mutation_type(MutationType::StepScaled(vec![10, 1]))
        .build()
        .unwrap();

    let permutate = Permutate::builder()
        .with_genotype(genotype)
        .with_fitness(SumGenes::new())
        .with_reporter(StrategyReporterNoop::new())
        .call()
        .unwrap();

    println!("{:#?}", permutate.best_genes());
    assert_eq!(permutate.best_fitness_score(), Some(400));
    assert_eq!(permutate.best_genes().unwrap(), vec![100, 100, 100, 100]);
}

#[test]
fn call_range_f32_random_invalid() {
    let genotype = RangeGenotype::builder()
        .with_genes_size(4)
        .with_allele_range(0.0..=1.0)
        .build()
        .unwrap();

    let permutate = Permutate::builder()
        .with_genotype(genotype)
        .with_fitness(SumGenes::new_with_precision(1e-3))
        .with_reporter(StrategyReporterNoop::new())
        .build();

    assert!(permutate.is_err());
    assert_eq!(
        permutate.err(),
        Some(TryFromPermutateBuilderError(
            "The Genotype's mutation_type does not allow permutation. RangeGenotype/MultiRangeGenotype require MutationType::Step, StepScaled, or Discrete for permutation"
        ))
    );
}

#[test]
fn call_par_fitness() {
    let genotype = ListGenotype::builder()
        .with_genes_size(5)
        .with_allele_list((0..10).collect())
        .build()
        .unwrap();

    let permutate = Permutate::builder()
        .with_genotype(genotype)
        .with_fitness(SumGenes::new())
        .with_par_fitness(true)
        .with_reporter(StrategyReporterNoop::new())
        .call()
        .unwrap();

    println!("{:#?}", permutate.best_genes());
    assert_eq!(permutate.best_fitness_score(), Some(45));
    assert_eq!(permutate.best_genes().unwrap(), vec![9, 9, 9, 9, 9]);
}

#[test]
fn call_binary_target_fitness_score_stops_early() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(5)
        .build()
        .unwrap();

    let permutate = Permutate::builder()
        .with_genotype(genotype)
        .with_fitness(CountTrue)
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_target_fitness_score(4)
        .call()
        .unwrap();

    // setup evaluates the first permutation (all true => 5); the second (4 true => 4) reaches the
    // target and breaks the inner loop, far short of exhausting all 32 permutations.
    assert_eq!(permutate.best_fitness_score(), Some(4));
    assert_eq!(permutate.state.current_generation, 2);
}

#[test]
fn call_binary_target_fitness_score_stops_early_par() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(12)
        .build()
        .unwrap();

    let permutate = Permutate::builder()
        .with_genotype(genotype)
        .with_fitness(CountTrue)
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_par_fitness(true)
        .with_target_fitness_score(10)
        .call()
        .unwrap();

    // target reached, and we stopped well before exhausting all 4096 permutations
    assert!(permutate.best_fitness_score().unwrap() <= 10);
    assert!(permutate.state.current_generation < 4096);
}

#[test]
fn call_binary_abort_flag_preset_returns_immediately() {
    use std::sync::atomic::AtomicBool;
    use std::sync::Arc;

    let genotype = BinaryGenotype::builder()
        .with_genes_size(5)
        .build()
        .unwrap();

    let abort_flag = Arc::new(AtomicBool::new(true));
    let permutate = Permutate::builder()
        .with_genotype(genotype)
        .with_fitness(CountTrue)
        .with_abort_flag(abort_flag.clone())
        .call()
        .unwrap();

    // the outer loop never runs: only setup evaluated the first permutation (all true => 5)
    assert_eq!(permutate.state.current_generation, 0);
    assert_eq!(permutate.best_fitness_score(), Some(5));
}

#[test]
fn call_with_reporter_period_zero() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(5)
        .build()
        .unwrap();
    let mut strategy = Permutate::builder()
        .with_genotype(genotype)
        .with_fitness(CountTrue)
        .with_reporter(PermutateReporterSimple::new_with_buffer(0))
        .call()
        .unwrap();

    let mut buffer: Vec<u8> = vec![];
    strategy.flush_reporter(&mut buffer);
    let output = String::from_utf8(buffer).unwrap();
    assert!(output.contains("enter - "));
    assert!(!output.contains("periodic - "));
}
