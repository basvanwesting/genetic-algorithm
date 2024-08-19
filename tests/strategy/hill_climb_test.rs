#[cfg(test)]
use crate::support::*;
use genetic_algorithm::fitness::placeholders::{CountTrue, SumGenes};
use genetic_algorithm::fitness::FitnessOrdering;
use genetic_algorithm::genotype::{BinaryGenotype, RangeGenotype, Genotype};
use genetic_algorithm::strategy::hill_climb::{
    HillClimb, HillClimbReporterNoop, HillClimbVariant, TryFromHillClimbBuilderError,
};
use genetic_algorithm::strategy::Strategy;

#[test]
fn build_invalid_missing_ending_condition() {
    let genotype = RangeGenotype::builder()
        .with_genes_size(10)
        .with_allele_range(0.0..=1.0)
        .with_allele_neighbour_range(-0.1..=0.1)
        .build()
        .unwrap();

    let hill_climb = HillClimb::builder()
        .with_genotype(genotype)
        .with_fitness(SumGenes::new_with_precision(1e-3))
        // .with_reporter(HillClimbReporterNoop::new())
        .build();

    assert!(hill_climb.is_err());
    assert_eq!(
        hill_climb.err(),
        Some(TryFromHillClimbBuilderError(
            "HillClimb requires at least a max_stale_generations or target_fitness_score ending condition"
        ))
    );
}

#[test]
fn call_range_max_stale_generations_maximize() {
    let genotype = RangeGenotype::builder()
        .with_genes_size(10)
        .with_allele_range(0.0..=1.0)
        .with_allele_neighbour_range(-0.1..=0.1)
        .build()
        .unwrap();
    let mut rng = SmallRng::seed_from_u64(0);
    let hill_climb = HillClimb::builder()
        .with_genotype(genotype)
        .with_max_stale_generations(1000)
        .with_fitness(SumGenes::new_with_precision(1e-3))
        .with_reporter(HillClimbReporterNoop::new())
        .call(&mut rng)
        .unwrap();

    let best_chromosome = hill_climb.best_chromosome().unwrap();
    println!("{:#?}", best_chromosome);

    assert_eq!(best_chromosome.fitness_score, Some(10000));
    assert!(relative_chromosome_eq(
        inspect::chromosome(&best_chromosome),
        vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,],
        0.001
    ));
}

#[test]
fn call_range_max_stale_generations_minimize() {
    let genotype = RangeGenotype::builder()
        .with_genes_size(10)
        .with_allele_range(0.0..=1.0)
        .with_allele_neighbour_range(-0.1..=0.1)
        .build()
        .unwrap();
    let mut rng = SmallRng::seed_from_u64(0);
    let hill_climb = HillClimb::builder()
        .with_genotype(genotype)
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_max_stale_generations(100)
        .with_fitness(SumGenes::new_with_precision(1e-3))
        // .with_reporter(HillClimbReporterNoop::new())
        .call(&mut rng)
        .unwrap();

    let best_chromosome = hill_climb.best_chromosome().unwrap();
    println!("{:#?}", best_chromosome);

    assert_eq!(best_chromosome.fitness_score, Some(0));
    assert!(relative_chromosome_eq(
        inspect::chromosome(&best_chromosome),
        vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,],
        0.001
    ));
}

#[test]
fn call_range_max_stale_generations_and_valid_fitness_score_maximize() {
    let genotype = RangeGenotype::builder()
        .with_genes_size(100)
        .with_allele_range(0.0..=1.0)
        .with_allele_neighbour_range(-0.1..=0.1)
        .build()
        .unwrap();
    let mut rng = SmallRng::seed_from_u64(0);
    let hill_climb = HillClimb::builder()
        .with_genotype(genotype)
        .with_max_stale_generations(10)
        .with_valid_fitness_score(75000)
        .with_fitness(SumGenes::new_with_precision(1e-3))
        .with_reporter(HillClimbReporterNoop::new())
        .call(&mut rng)
        .unwrap();

    let best_chromosome = hill_climb.best_chromosome().unwrap();
    println!("{:#?}", best_chromosome);

    assert_eq!(best_chromosome.fitness_score, Some(76681));
}

#[test]
fn call_range_max_stale_generations_and_valid_fitness_score_minimize() {
    let genotype = RangeGenotype::builder()
        .with_genes_size(100)
        .with_allele_range(0.0..=1.0)
        .with_allele_neighbour_range(-0.1..=0.1)
        .build()
        .unwrap();
    let mut rng = SmallRng::seed_from_u64(0);
    let hill_climb = HillClimb::builder()
        .with_genotype(genotype)
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_max_stale_generations(10)
        .with_valid_fitness_score(25000)
        .with_fitness(SumGenes::new_with_precision(1e-3))
        // .with_reporter(HillClimbReporterNoop::new())
        .call(&mut rng)
        .unwrap();

    let best_chromosome = hill_climb.best_chromosome().unwrap();
    println!("{:#?}", best_chromosome);

    assert_eq!(best_chromosome.fitness_score, Some(24930));
}

#[test]
fn call_range_target_fitness_score_maximize() {
    let genotype = RangeGenotype::builder()
        .with_genes_size(10)
        .with_allele_range(0.0..=1.0)
        .with_allele_neighbour_range(-0.1..=0.1)
        .build()
        .unwrap();
    let mut rng = SmallRng::seed_from_u64(0);
    let hill_climb = HillClimb::builder()
        .with_genotype(genotype)
        .with_target_fitness_score(8000)
        .with_fitness(SumGenes::new_with_precision(1e-3))
        .with_reporter(HillClimbReporterNoop::new())
        .call(&mut rng)
        .unwrap();

    let best_chromosome = hill_climb.best_chromosome().unwrap();
    println!("{:#?}", best_chromosome);

    assert_eq!(best_chromosome.fitness_score, Some(8088));
    assert!(relative_chromosome_eq(
        inspect::chromosome(&best_chromosome),
        vec![0.673, 0.629, 1.0, 0.722, 1.0, 1.0, 0.737, 0.735, 0.590, 1.0,],
        0.001
    ));
}

#[test]
fn call_range_target_fitness_score_minimize() {
    let genotype = RangeGenotype::builder()
        .with_genes_size(10)
        .with_allele_range(0.0..=1.0)
        .with_allele_neighbour_range(-0.1..=0.1)
        .build()
        .unwrap();
    let mut rng = SmallRng::seed_from_u64(0);
    let hill_climb = HillClimb::builder()
        .with_genotype(genotype)
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_target_fitness_score(1000)
        .with_fitness(SumGenes::new_with_precision(1e-3))
        // .with_reporter(HillClimbReporterNoop::new())
        .call(&mut rng)
        .unwrap();

    let best_chromosome = hill_climb.best_chromosome().unwrap();
    println!("{:#?}", best_chromosome);

    assert_eq!(best_chromosome.fitness_score, Some(964));
    assert!(relative_chromosome_eq(
        inspect::chromosome(&best_chromosome),
        vec![0.0, 0.0, 0.173, 0.0, 0.626, 0.006, 0.0, 0.0, 0.0, 0.159,],
        0.001
    ));
}

#[test]
fn call_range_multi_thread() {
    let genotype = RangeGenotype::builder()
        .with_genes_size(10)
        .with_allele_range(0.0..=1.0)
        .with_allele_neighbour_range(-0.1..=0.1)
        .build()
        .unwrap();
    let mut rng = SmallRng::seed_from_u64(0);
    let hill_climb = HillClimb::builder()
        .with_genotype(genotype)
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_multithreading(true)
        .with_target_fitness_score(1000)
        .with_fitness(SumGenes::new_with_precision(1e-3))
        .with_reporter(HillClimbReporterNoop::new())
        .call(&mut rng)
        .unwrap();

    let best_chromosome = hill_climb.best_chromosome().unwrap();
    println!("{:#?}", best_chromosome);

    assert_eq!(best_chromosome.fitness_score, Some(964));
    assert!(relative_chromosome_eq(
        inspect::chromosome(&best_chromosome),
        vec![0.0, 0.0, 0.173, 0.0, 0.626, 0.006, 0.0, 0.0, 0.0, 0.159,],
        0.001
    ));
}

#[test]
fn call_binary_stochastic() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(100)
        .build()
        .unwrap();
    let mut rng = SmallRng::seed_from_u64(0);
    let hill_climb = HillClimb::builder()
        .with_genotype(genotype)
        .with_variant(HillClimbVariant::Stochastic)
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_target_fitness_score(0)
        .with_fitness(CountTrue)
        // .with_reporter(HillClimbReporterNoop::new())
        .call(&mut rng)
        .unwrap();

    let best_chromosome = hill_climb.best_chromosome().unwrap();
    println!("{:#?}", best_chromosome);
    assert_eq!(best_chromosome.fitness_score, Some(0));
}

#[test]
fn call_binary_stochastic_secondary() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(100)
        .build()
        .unwrap();
    let mut rng = SmallRng::seed_from_u64(0);
    let hill_climb = HillClimb::builder()
        .with_genotype(genotype)
        .with_variant(HillClimbVariant::StochasticSecondary)
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_target_fitness_score(0)
        .with_fitness(CountTrue)
        .with_reporter(HillClimbReporterNoop::new())
        .call(&mut rng)
        .unwrap();

    let best_chromosome = hill_climb.best_chromosome().unwrap();
    println!("{:#?}", best_chromosome);
    assert_eq!(best_chromosome.fitness_score, Some(0));
}

#[test]
fn call_binary_steepest_ascent() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(100)
        .build()
        .unwrap();
    let mut rng = SmallRng::seed_from_u64(0);
    let hill_climb = HillClimb::builder()
        .with_genotype(genotype)
        .with_variant(HillClimbVariant::SteepestAscent)
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_target_fitness_score(0)
        .with_fitness(CountTrue)
        .with_reporter(HillClimbReporterNoop::new())
        .call(&mut rng)
        .unwrap();

    let best_chromosome = hill_climb.best_chromosome().unwrap();
    println!("{:#?}", best_chromosome);
    assert_eq!(best_chromosome.fitness_score, Some(0));
}

#[test]
fn call_binary_steepest_ascent_secondary() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(50)
        .build()
        .unwrap();
    let mut rng = SmallRng::seed_from_u64(0);
    let hill_climb = HillClimb::builder()
        .with_genotype(genotype)
        .with_variant(HillClimbVariant::SteepestAscentSecondary)
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_target_fitness_score(0)
        .with_fitness(CountTrue)
        .with_reporter(HillClimbReporterNoop::new())
        .call(&mut rng)
        .unwrap();

    let best_chromosome = hill_climb.best_chromosome().unwrap();
    println!("{:#?}", best_chromosome);
    assert_eq!(best_chromosome.fitness_score, Some(0));
}
