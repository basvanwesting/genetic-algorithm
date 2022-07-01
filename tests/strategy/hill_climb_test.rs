#[cfg(test)]
use crate::support::*;
use genetic_algorithm::fitness::placeholders::SumContinuousGenotype;
use genetic_algorithm::fitness::FitnessOrdering;
use genetic_algorithm::genotype::{ContinuousGenotype, Genotype};
use genetic_algorithm::strategy::hill_climb::{HillClimb, TryFromHillClimbBuilderError};
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
        .with_max_stale_generations(100)
        .with_fitness(SumContinuousGenotype(1e-3))
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
        .call(&mut rng)
        .unwrap();

    let best_chromosome = hill_climb.best_chromosome().unwrap();
    println!("{:#?}", best_chromosome);

    assert_eq!(best_chromosome.fitness_score, Some(8032));
    assert_eq!(
        inspect::chromosome(&best_chromosome),
        vec![
            0.69105303, 0.6415004, 1.0, 0.7007805, 1.0, 1.0, 0.7997122, 0.69759524, 0.5018855, 1.0,
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
        .call(&mut rng)
        .unwrap();

    let best_chromosome = hill_climb.best_chromosome().unwrap();
    println!("{:#?}", best_chromosome);

    assert_eq!(best_chromosome.fitness_score, Some(998));
    assert_eq!(
        inspect::chromosome(&best_chromosome),
        vec![
            0.0,
            0.0,
            0.019872978,
            0.0,
            0.11975294,
            0.6136266,
            0.0,
            0.0,
            0.0,
            0.24557132
        ]
    );
}

//#[test]
//fn call_discrete() {
//let genotype = DiscreteGenotype::builder()
//.with_genes_size(10)
//.with_allele_list((0..4).collect())
//.build()
//.unwrap();

//let mut rng = SmallRng::seed_from_u64(0);
//let hill_climb = HillClimb::builder()
//.with_genotype(genotype)
//.with_max_stale_generations(100)
//.with_fitness(SumDiscreteGenotype)
//.call(&mut rng)
//.unwrap();

//let best_chromosome = hill_climb.best_chromosome().unwrap();
//println!("{:#?}", best_chromosome);

//assert_eq!(best_chromosome.fitness_score, Some(30));
//assert_eq!(
//inspect::chromosome(&best_chromosome),
//vec![3, 3, 3, 3, 3, 3, 3, 3, 3, 3]
//);
//}

//#[test]
//fn call_multi_discrete() {
//let genotype = MultiDiscreteGenotype::builder()
//.with_allele_lists(vec![
//vec![0, 1, 2, 3, 4],
//vec![0, 1],
//vec![0],
//vec![0, 1, 2, 3],
//])
//.build()
//.unwrap();
//let mut rng = SmallRng::seed_from_u64(0);
//let hill_climb = HillClimb::builder()
//.with_genotype(genotype)
//.with_max_stale_generations(100)
//.with_fitness(SumMultiDiscreteGenotype)
//.call(&mut rng)
//.unwrap();

//let best_chromosome = hill_climb.best_chromosome().unwrap();
//println!("{:#?}", best_chromosome);

//assert_eq!(best_chromosome.fitness_score, Some(8));
//assert_eq!(inspect::chromosome(&best_chromosome), vec![4, 1, 0, 3]);
//}
