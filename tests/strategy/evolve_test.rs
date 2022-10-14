#[cfg(test)]
use crate::support::*;
use genetic_algorithm::compete::CompeteTournament;
use genetic_algorithm::crossover::{CrossoverSingleGene, CrossoverSinglePoint};
use genetic_algorithm::fitness::placeholders::{
    CountTrue, SumContinuousGenotype, SumDiscreteGenotype, SumMultiDiscreteGenotype,
    SumUniqueGenotype,
};
use genetic_algorithm::fitness::FitnessOrdering;
use genetic_algorithm::genotype::{
    BinaryGenotype, ContinuousGenotype, DiscreteGenotype, Genotype, MultiDiscreteGenotype,
    UniqueGenotype,
};
use genetic_algorithm::mutate::MutateOnce;
use genetic_algorithm::strategy::evolve::{Evolve, TryFromEvolveBuilderError};
use genetic_algorithm::strategy::Strategy;

#[test]
fn build_invalid_missing_ending_condition() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(10)
        .build()
        .unwrap();
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_population_size(100)
        .with_mutate(MutateOnce(0.1))
        .with_fitness(CountTrue)
        .with_crossover(CrossoverSingleGene(true))
        .with_compete(CompeteTournament(4))
        .build();

    assert!(evolve.is_err());
    assert_eq!(
        evolve.err(),
        Some(TryFromEvolveBuilderError(
            "Evolve requires at least a max_stale_generations or target_fitness_score ending condition"
        ))
    );
}

#[test]
fn build_invalid_require_crossover_indexes() {
    let genotype = UniqueGenotype::builder()
        .with_allele_list((0..10).collect())
        .build()
        .unwrap();
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_population_size(100)
        .with_max_stale_generations(20)
        .with_mutate(MutateOnce(0.1))
        .with_fitness(SumUniqueGenotype)
        .with_crossover(CrossoverSingleGene(true))
        .with_compete(CompeteTournament(4))
        .build();

    assert!(evolve.is_err());
    assert_eq!(
        evolve.err(),
        Some(TryFromEvolveBuilderError(
            "The provided Crossover strategy requires crossover_indexes, which the provided Genotype does not provide"
        ))
    );
}
#[test]
fn build_invalid_require_crossover_points() {
    let genotype = UniqueGenotype::builder()
        .with_allele_list((0..10).collect())
        .build()
        .unwrap();
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_population_size(100)
        .with_max_stale_generations(20)
        .with_mutate(MutateOnce(0.1))
        .with_fitness(SumUniqueGenotype)
        .with_crossover(CrossoverSinglePoint(true))
        .with_compete(CompeteTournament(4))
        .build();

    assert!(evolve.is_err());
    assert_eq!(
        evolve.err(),
        Some(TryFromEvolveBuilderError(
            "The provided Crossover strategy requires crossover_points, which the provided Genotype does not provide"
        ))
    );
}

#[test]
fn call_binary_max_stale_generations_maximize() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(10)
        .build()
        .unwrap();
    let mut rng = SmallRng::seed_from_u64(0);
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_population_size(100)
        .with_max_stale_generations(20)
        .with_mutate(MutateOnce(0.1))
        .with_fitness(CountTrue)
        .with_crossover(CrossoverSingleGene(true))
        .with_compete(CompeteTournament(4))
        .call(&mut rng)
        .unwrap();

    let best_chromosome = evolve.best_chromosome().unwrap();
    println!("{:#?}", best_chromosome);

    assert_eq!(best_chromosome.fitness_score, Some(10));
    assert_eq!(
        inspect::chromosome(&best_chromosome),
        vec![true, true, true, true, true, true, true, true, true, true]
    );
}

#[test]
fn call_binary_max_stale_generations_minimize() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(10)
        .build()
        .unwrap();
    let mut rng = SmallRng::seed_from_u64(0);
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_population_size(100)
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_max_stale_generations(20)
        .with_mutate(MutateOnce(0.1))
        .with_fitness(CountTrue)
        .with_crossover(CrossoverSingleGene(true))
        .with_compete(CompeteTournament(4))
        .call(&mut rng)
        .unwrap();

    let best_chromosome = evolve.best_chromosome().unwrap();
    println!("{:#?}", best_chromosome);

    assert_eq!(best_chromosome.fitness_score, Some(0));
    assert_eq!(
        inspect::chromosome(&best_chromosome),
        vec![false, false, false, false, false, false, false, false, false, false]
    );
}

#[test]
fn call_binary_target_fitness_score_maximize() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(10)
        .build()
        .unwrap();
    let mut rng = SmallRng::seed_from_u64(0);
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_population_size(100)
        .with_target_fitness_score(8)
        .with_mutate(MutateOnce(0.1))
        .with_fitness(CountTrue)
        .with_crossover(CrossoverSingleGene(true))
        .with_compete(CompeteTournament(4))
        .call(&mut rng)
        .unwrap();

    let best_chromosome = evolve.best_chromosome().unwrap();
    println!("{:#?}", best_chromosome);

    assert_eq!(best_chromosome.fitness_score, Some(9));
    assert_eq!(
        inspect::chromosome(&best_chromosome),
        vec![true, true, true, true, true, true, true, false, true, true]
    );
}

#[test]
fn call_binary_target_fitness_score_minimize() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(10)
        .build()
        .unwrap();
    let mut rng = SmallRng::seed_from_u64(0);
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_population_size(100)
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_target_fitness_score(0)
        .with_mutate(MutateOnce(0.1))
        .with_fitness(CountTrue)
        .with_crossover(CrossoverSingleGene(true))
        .with_compete(CompeteTournament(4))
        .call(&mut rng)
        .unwrap();

    let best_chromosome = evolve.best_chromosome().unwrap();
    println!("{:#?}", best_chromosome);

    assert_eq!(best_chromosome.fitness_score, Some(0));
    assert_eq!(
        inspect::chromosome(&best_chromosome),
        vec![false, false, false, false, false, false, false, false, false, false]
    );
}

#[test]
fn call_binary_degeneration_range() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(10)
        .build()
        .unwrap();
    let mut rng = SmallRng::seed_from_u64(0);
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_population_size(100)
        .with_target_fitness_score(8)
        .with_degeneration_range(0.0001..1.0000)
        .with_mutate(MutateOnce(0.1))
        .with_fitness(CountTrue)
        .with_crossover(CrossoverSingleGene(true))
        .with_compete(CompeteTournament(4))
        .call(&mut rng)
        .unwrap();

    let best_chromosome = evolve.best_chromosome().unwrap();
    println!("{:#?}", best_chromosome);

    assert_eq!(best_chromosome.fitness_score, Some(9));
    assert_eq!(
        inspect::chromosome(&best_chromosome),
        vec![true, true, true, true, true, true, true, false, true, true]
    );
}

#[test]
fn call_continuous() {
    let genotype = ContinuousGenotype::builder()
        .with_genes_size(10)
        .with_allele_range(0.0..1.0)
        .build()
        .unwrap();
    let mut rng = SmallRng::seed_from_u64(0);
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_population_size(100)
        .with_max_stale_generations(20)
        .with_mutate(MutateOnce(0.1))
        .with_fitness(SumContinuousGenotype(1e-3))
        .with_crossover(CrossoverSingleGene(true))
        .with_compete(CompeteTournament(4))
        .call(&mut rng)
        .unwrap();

    let best_chromosome = evolve.best_chromosome().unwrap();
    println!("{:#?}", best_chromosome);

    assert_eq!(best_chromosome.fitness_score, Some(9976));
    assert_eq!(
        inspect::chromosome(&best_chromosome),
        vec![
            0.9993049, 0.99938405, 0.99551475, 0.9943992, 0.9974326, 0.9998164, 0.99971354,
            0.9940715, 0.99899554, 0.9975089
        ]
    );
}

#[test]
fn call_discrete() {
    let genotype = DiscreteGenotype::builder()
        .with_genes_size(10)
        .with_allele_list((0..4).collect())
        .build()
        .unwrap();

    let mut rng = SmallRng::seed_from_u64(0);
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_population_size(100)
        .with_max_stale_generations(20)
        .with_mutate(MutateOnce(0.1))
        .with_fitness(SumDiscreteGenotype)
        .with_crossover(CrossoverSingleGene(true))
        .with_compete(CompeteTournament(4))
        .call(&mut rng)
        .unwrap();

    let best_chromosome = evolve.best_chromosome().unwrap();
    println!("{:#?}", best_chromosome);

    assert_eq!(best_chromosome.fitness_score, Some(30));
    assert_eq!(
        inspect::chromosome(&best_chromosome),
        vec![3, 3, 3, 3, 3, 3, 3, 3, 3, 3]
    );
}

#[test]
fn call_multi_discrete() {
    let genotype = MultiDiscreteGenotype::builder()
        .with_allele_lists(vec![
            vec![0, 1, 2, 3, 4],
            vec![0, 1],
            vec![0],
            vec![0, 1, 2, 3],
        ])
        .build()
        .unwrap();
    let mut rng = SmallRng::seed_from_u64(0);
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_population_size(100)
        .with_max_stale_generations(20)
        .with_mutate(MutateOnce(0.1))
        .with_fitness(SumMultiDiscreteGenotype)
        .with_crossover(CrossoverSingleGene(true))
        .with_compete(CompeteTournament(4))
        .call(&mut rng)
        .unwrap();

    let best_chromosome = evolve.best_chromosome().unwrap();
    println!("{:#?}", best_chromosome);

    assert_eq!(best_chromosome.fitness_score, Some(8));
    assert_eq!(inspect::chromosome(&best_chromosome), vec![4, 1, 0, 3]);
}

#[test]
fn call_multi_thread() {
    let genotype = DiscreteGenotype::builder()
        .with_genes_size(10)
        .with_allele_list((0..4).collect())
        .build()
        .unwrap();

    let mut rng = SmallRng::seed_from_u64(0);
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_population_size(100)
        .with_max_stale_generations(20)
        .with_mutate(MutateOnce(0.1))
        .with_fitness(SumDiscreteGenotype)
        .with_multithreading(true)
        .with_crossover(CrossoverSingleGene(true))
        .with_compete(CompeteTournament(4))
        .call(&mut rng)
        .unwrap();

    let best_chromosome = evolve.best_chromosome().unwrap();
    println!("{:#?}", best_chromosome);

    assert_eq!(best_chromosome.fitness_score, Some(30));
    assert_eq!(
        inspect::chromosome(&best_chromosome),
        vec![3, 3, 3, 3, 3, 3, 3, 3, 3, 3]
    );
}

#[test]
fn population_factory_binary() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(4)
        .build()
        .unwrap();
    let mut rng = SmallRng::seed_from_u64(0);
    let mut evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_population_size(8)
        .with_max_stale_generations(20)
        .with_mutate(MutateOnce(0.1))
        .with_fitness(CountTrue)
        .with_crossover(CrossoverSingleGene(true))
        .with_compete(CompeteTournament(4))
        .build()
        .unwrap();

    let population = evolve.population_factory(&mut rng);
    println!("{:#?}", population);

    assert_eq!(
        inspect::population(&population),
        vec![
            vec![true, true, false, true],
            vec![false, false, false, true],
            vec![true, false, true, false],
            vec![false, true, false, true],
            vec![true, true, false, false],
            vec![false, true, true, false],
            vec![true, false, false, true],
            vec![false, true, false, true],
        ]
    )
}
