#[cfg(test)]
use crate::support::*;
use genetic_algorithm::fitness::placeholders::{CountTrue, SumContinuousGenotype};
use genetic_algorithm::fitness::FitnessOrdering;
use genetic_algorithm::genotype::{BinaryGenotype, ContinuousGenotype, Genotype};
use genetic_algorithm::strategy::hill_climb::{
    HillClimb, HillClimbReporterNoop, HillClimbVariant, TryFromHillClimbBuilderError,
};
use genetic_algorithm::strategy::Strategy;

#[test]
fn build_invalid_missing_ending_condition() {
    let genotype = ContinuousGenotype::builder()
        .with_genes_size(10)
        .with_allele_range(0.0..1.0)
        .with_allele_neighbour_range(-0.1..0.1)
        .build()
        .unwrap();

    let hill_climb = HillClimb::builder()
        .with_genotype(genotype)
        .with_fitness(SumContinuousGenotype(1e-3))
        // .with_reporter(HillClimbReporterNoop::new())
        .build();

    assert!(hill_climb.is_err());
    assert_eq!(
        hill_climb.err(),
        Some(TryFromHillClimbBuilderError(
            "HillClimb requires at least a max_stale_generations, target_fitness_score or scaling ending condition"
        ))
    );
}

#[test]
fn call_continuous_max_stale_generations_maximize() {
    let genotype = ContinuousGenotype::builder()
        .with_genes_size(10)
        .with_allele_range(0.0..1.0)
        .with_allele_neighbour_range(-0.1..0.1)
        .build()
        .unwrap();
    let mut rng = SmallRng::seed_from_u64(0);
    let hill_climb = HillClimb::builder()
        .with_genotype(genotype)
        .with_max_stale_generations(1000)
        .with_fitness(SumContinuousGenotype(1e-3))
        .with_reporter(HillClimbReporterNoop::new())
        .call(&mut rng)
        .unwrap();

    let best_chromosome = hill_climb.best_chromosome().unwrap();
    println!("{:#?}", best_chromosome);

    assert_eq!(best_chromosome.fitness_score, Some(9999));
    assert_eq!(
        inspect::chromosome(&best_chromosome),
        vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,]
    );
}

#[test]
fn call_continuous_max_stale_generations_minimize() {
    let genotype = ContinuousGenotype::builder()
        .with_genes_size(10)
        .with_allele_range(0.0..1.0)
        .with_allele_neighbour_range(-0.1..0.1)
        .build()
        .unwrap();
    let mut rng = SmallRng::seed_from_u64(0);
    let hill_climb = HillClimb::builder()
        .with_genotype(genotype)
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_max_stale_generations(100)
        .with_fitness(SumContinuousGenotype(1e-3))
        // .with_reporter(HillClimbReporterNoop::new())
        .call(&mut rng)
        .unwrap();

    let best_chromosome = hill_climb.best_chromosome().unwrap();
    println!("{:#?}", best_chromosome);

    assert_eq!(best_chromosome.fitness_score, Some(0));
    assert_eq!(
        inspect::chromosome(&best_chromosome),
        vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,]
    );
}

#[test]
fn call_continuous_max_stale_generations_and_valid_fitness_score_maximize() {
    let genotype = ContinuousGenotype::builder()
        .with_genes_size(100)
        .with_allele_range(0.0..1.0)
        .with_allele_neighbour_range(-0.1..0.1)
        .build()
        .unwrap();
    let mut rng = SmallRng::seed_from_u64(0);
    let hill_climb = HillClimb::builder()
        .with_genotype(genotype)
        .with_max_stale_generations(10)
        .with_valid_fitness_score(75000)
        .with_fitness(SumContinuousGenotype(1e-3))
        .with_reporter(HillClimbReporterNoop::new())
        .call(&mut rng)
        .unwrap();

    let best_chromosome = hill_climb.best_chromosome().unwrap();
    println!("{:#?}", best_chromosome);

    assert_eq!(best_chromosome.fitness_score, Some(76681));
}

#[test]
fn call_continuous_max_stale_generations_and_valid_fitness_score_minimize() {
    let genotype = ContinuousGenotype::builder()
        .with_genes_size(100)
        .with_allele_range(0.0..1.0)
        .with_allele_neighbour_range(-0.1..0.1)
        .build()
        .unwrap();
    let mut rng = SmallRng::seed_from_u64(0);
    let hill_climb = HillClimb::builder()
        .with_genotype(genotype)
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_max_stale_generations(10)
        .with_valid_fitness_score(25000)
        .with_fitness(SumContinuousGenotype(1e-3))
        // .with_reporter(HillClimbReporterNoop::new())
        .call(&mut rng)
        .unwrap();

    let best_chromosome = hill_climb.best_chromosome().unwrap();
    println!("{:#?}", best_chromosome);

    assert_eq!(best_chromosome.fitness_score, Some(24930));
}

#[test]
fn call_continuous_target_fitness_score_maximize() {
    let genotype = ContinuousGenotype::builder()
        .with_genes_size(10)
        .with_allele_range(0.0..1.0)
        .with_allele_neighbour_range(-0.1..0.1)
        .build()
        .unwrap();
    let mut rng = SmallRng::seed_from_u64(0);
    let hill_climb = HillClimb::builder()
        .with_genotype(genotype)
        .with_target_fitness_score(8000)
        .with_fitness(SumContinuousGenotype(1e-3))
        .with_reporter(HillClimbReporterNoop::new())
        .call(&mut rng)
        .unwrap();

    let best_chromosome = hill_climb.best_chromosome().unwrap();
    println!("{:#?}", best_chromosome);

    assert_eq!(best_chromosome.fitness_score, Some(8088));
    assert_eq!(
        inspect::chromosome(&best_chromosome),
        vec![
            0.673274, 0.62921375, 1.0, 0.7220475, 1.0, 1.0, 0.73748976, 0.7359946, 0.5902894, 1.0,
        ]
    );
}

#[test]
fn call_continuous_target_fitness_score_minimize() {
    let genotype = ContinuousGenotype::builder()
        .with_genes_size(10)
        .with_allele_range(0.0..1.0)
        .with_allele_neighbour_range(-0.1..0.1)
        .build()
        .unwrap();
    let mut rng = SmallRng::seed_from_u64(0);
    let hill_climb = HillClimb::builder()
        .with_genotype(genotype)
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_target_fitness_score(1000)
        .with_fitness(SumContinuousGenotype(1e-3))
        // .with_reporter(HillClimbReporterNoop::new())
        .call(&mut rng)
        .unwrap();

    let best_chromosome = hill_climb.best_chromosome().unwrap();
    println!("{:#?}", best_chromosome);

    assert_eq!(best_chromosome.fitness_score, Some(964));
    assert_eq!(
        inspect::chromosome(&best_chromosome),
        vec![
            0.0,
            0.0,
            0.17363752,
            0.0,
            0.62618715,
            0.0061164834,
            0.0,
            0.0,
            0.0,
            0.15902928,
        ]
    );
}

#[test]
fn call_continuous_multi_thread() {
    let genotype = ContinuousGenotype::builder()
        .with_genes_size(10)
        .with_allele_range(0.0..1.0)
        .with_allele_neighbour_range(-0.1..0.1)
        .build()
        .unwrap();
    let mut rng = SmallRng::seed_from_u64(0);
    let hill_climb = HillClimb::builder()
        .with_genotype(genotype)
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_multithreading(true)
        .with_target_fitness_score(1000)
        .with_fitness(SumContinuousGenotype(1e-3))
        .with_reporter(HillClimbReporterNoop::new())
        .call(&mut rng)
        .unwrap();

    let best_chromosome = hill_climb.best_chromosome().unwrap();
    println!("{:#?}", best_chromosome);

    assert_eq!(best_chromosome.fitness_score, Some(964));
    assert_eq!(
        inspect::chromosome(&best_chromosome),
        vec![
            0.0,
            0.0,
            0.17363752,
            0.0,
            0.62618715,
            0.0061164834,
            0.0,
            0.0,
            0.0,
            0.15902928,
        ]
    );
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
