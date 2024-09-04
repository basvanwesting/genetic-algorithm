#[cfg(test)]
use crate::support::*;
use genetic_algorithm::compete::CompeteTournament;
use genetic_algorithm::crossover::{CrossoverSingleGene, CrossoverSinglePoint};
use genetic_algorithm::extension::{
    ExtensionMassDegeneration, ExtensionMassExtinction, ExtensionMassGenesis, ExtensionNoop,
};
use genetic_algorithm::fitness::placeholders::{CountTrue, SumGenes};
use genetic_algorithm::fitness::FitnessOrdering;
use genetic_algorithm::genotype::{
    BinaryGenotype, Genotype, ListGenotype, MultiListGenotype, RangeGenotype, UniqueGenotype,
};
use genetic_algorithm::mutate::MutateSingleGene;
use genetic_algorithm::strategy::evolve::{Evolve, EvolveReporterNoop, TryFromEvolveBuilderError};
use genetic_algorithm::strategy::Strategy;

#[test]
fn build_invalid_missing_ending_condition() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(10)
        .build()
        .unwrap();
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(100)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_fitness(CountTrue)
        .with_crossover(CrossoverSingleGene::new(0.5))
        .with_compete(CompeteTournament::new(4))
        // .with_extension(ExtensionNoop::new())
        // .with_reporter(EvolveReporterNoop::new())
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
        .with_target_population_size(100)
        .with_max_stale_generations(20)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_fitness(SumGenes::new())
        .with_crossover(CrossoverSingleGene::new(0.5))
        .with_compete(CompeteTournament::new(4))
        // .with_extension(ExtensionNoop::new())
        .with_reporter(EvolveReporterNoop::new())
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
        .with_target_population_size(100)
        .with_max_stale_generations(20)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_fitness(SumGenes::new())
        .with_crossover(CrossoverSinglePoint::new(0.5))
        .with_compete(CompeteTournament::new(4))
        .with_extension(ExtensionNoop::new())
        // .with_reporter(EvolveReporterNoop::new())
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
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(100)
        .with_max_stale_generations(20)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_fitness(CountTrue)
        .with_crossover(CrossoverSingleGene::new(0.5))
        .with_compete(CompeteTournament::new(4))
        .with_extension(ExtensionNoop::new())
        .with_reporter(EvolveReporterNoop::new())
        .with_rng_seed_from_u64(0)
        .call()
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
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(100)
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_max_stale_generations(20)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_fitness(CountTrue)
        .with_crossover(CrossoverSingleGene::new(0.5))
        .with_compete(CompeteTournament::new(4))
        .with_extension(ExtensionNoop::new())
        // .with_reporter(EvolveReporterNoop::new())
        .with_rng_seed_from_u64_option(Some(0))
        .call()
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
fn call_binary_max_stale_generations_and_valid_fitness_score_maximize() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(100)
        .build()
        .unwrap();
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(4)
        .with_max_stale_generations(2)
        .with_valid_fitness_score(75)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_fitness(CountTrue)
        .with_crossover(CrossoverSingleGene::new(0.5))
        .with_compete(CompeteTournament::new(4))
        // .with_extension(ExtensionNoop::new())
        .with_reporter(EvolveReporterNoop::new())
        .with_rng_seed_from_u64(0)
        .call()
        .unwrap();

    let best_chromosome = evolve.best_chromosome().unwrap();
    println!("{:#?}", best_chromosome);
    assert_eq!(best_chromosome.fitness_score, Some(75));
}

#[test]
fn call_binary_max_stale_generations_and_valid_fitness_score_minimize() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(100)
        .build()
        .unwrap();
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(4)
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_max_stale_generations(2)
        .with_valid_fitness_score(25)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_fitness(CountTrue)
        .with_crossover(CrossoverSingleGene::new(0.5))
        .with_compete(CompeteTournament::new(4))
        .with_extension(ExtensionNoop::new())
        // .with_reporter(EvolveReporterNoop::new())
        .with_rng_seed_from_u64(0)
        .call()
        .unwrap();

    let best_chromosome = evolve.best_chromosome().unwrap();
    println!("{:#?}", best_chromosome);

    assert_eq!(best_chromosome.fitness_score, Some(25));
}

#[test]
fn call_binary_target_fitness_score_maximize() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(10)
        .build()
        .unwrap();
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(100)
        .with_target_fitness_score(8)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_fitness(CountTrue)
        .with_crossover(CrossoverSingleGene::new(0.5))
        .with_compete(CompeteTournament::new(4))
        .with_extension(ExtensionNoop::new())
        .with_reporter(EvolveReporterNoop::new())
        .with_rng_seed_from_u64(0)
        .call()
        .unwrap();

    let best_chromosome = evolve.best_chromosome().unwrap();
    println!("{:#?}", best_chromosome);

    assert_eq!(best_chromosome.fitness_score, Some(8));
    assert_eq!(
        inspect::chromosome(&best_chromosome),
        vec![false, true, true, false, true, true, true, true, true, true]
    );
}

#[test]
fn call_binary_target_fitness_score_minimize() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(10)
        .build()
        .unwrap();
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(100)
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_target_fitness_score(0)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_fitness(CountTrue)
        .with_crossover(CrossoverSingleGene::new(0.5))
        .with_compete(CompeteTournament::new(4))
        .with_extension(ExtensionNoop::new())
        // .with_reporter(EvolveReporterNoop::new())
        .with_rng_seed_from_u64(0)
        .call()
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
fn call_binary_mass_degeneration() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(10)
        .build()
        .unwrap();
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(100)
        .with_target_fitness_score(9)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_fitness(CountTrue)
        .with_crossover(CrossoverSingleGene::new(0.5))
        .with_compete(CompeteTournament::new(4))
        .with_extension(ExtensionMassDegeneration::new(10, 10))
        .with_reporter(EvolveReporterNoop::new())
        .with_rng_seed_from_u64(0)
        .call()
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
fn call_binary_mass_extinction() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(10)
        .build()
        .unwrap();
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(100)
        .with_target_fitness_score(9)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_fitness(CountTrue)
        .with_crossover(CrossoverSingleGene::new(0.5))
        .with_compete(CompeteTournament::new(4))
        .with_extension(ExtensionMassExtinction::new(10, 0.1))
        // .with_reporter(EvolveReporterNoop::new())
        .with_rng_seed_from_u64(0)
        .call()
        .unwrap();

    let best_chromosome = evolve.best_chromosome().unwrap();
    println!("{:#?}", best_chromosome);

    assert_eq!(best_chromosome.fitness_score, Some(9));
    assert_eq!(
        inspect::chromosome(&best_chromosome),
        vec![true, true, false, true, true, true, true, true, true, true]
    );
}

#[test]
fn call_binary_mass_genesis() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(10)
        .build()
        .unwrap();
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(100)
        .with_target_fitness_score(9)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_fitness(CountTrue)
        .with_crossover(CrossoverSingleGene::new(0.5))
        .with_compete(CompeteTournament::new(4))
        .with_extension(ExtensionMassGenesis::new(10))
        .with_reporter(EvolveReporterNoop::new())
        .with_rng_seed_from_u64(0)
        .call()
        .unwrap();

    let best_chromosome = evolve.best_chromosome().unwrap();
    println!("{:#?}", best_chromosome);

    assert_eq!(best_chromosome.fitness_score, Some(9));
    assert_eq!(
        inspect::chromosome(&best_chromosome),
        vec![true, true, true, false, true, true, true, true, true, true]
    );
}

#[test]
fn call_range_f32() {
    let genotype = RangeGenotype::builder()
        .with_genes_size(10)
        .with_allele_range(0.0..=1.0)
        .build()
        .unwrap();
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(100)
        .with_max_stale_generations(20)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_fitness(SumGenes::new_with_precision(1e-3))
        .with_crossover(CrossoverSingleGene::new(0.5))
        .with_compete(CompeteTournament::new(4))
        // .with_extension(ExtensionNoop::new())
        .with_reporter(EvolveReporterNoop::new())
        .with_rng_seed_from_u64(0)
        .call()
        .unwrap();

    let best_chromosome = evolve.best_chromosome().unwrap();
    println!("{:#?}", best_chromosome);

    assert_eq!(best_chromosome.fitness_score, Some(9911));
    assert!(relative_chromosome_eq(
        inspect::chromosome(&best_chromosome),
        vec![0.978, 0.993, 0.979, 0.995, 0.999, 0.999, 0.992, 0.997, 0.980, 0.995],
        0.001
    ));
}

#[test]
fn call_range_usize() {
    let genotype = RangeGenotype::builder()
        .with_genes_size(10)
        .with_allele_range(0..=9)
        .build()
        .unwrap();
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(100)
        .with_max_stale_generations(20)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_fitness(SumGenes::new())
        .with_crossover(CrossoverSingleGene::new(0.5))
        .with_compete(CompeteTournament::new(4))
        // .with_extension(ExtensionNoop::new())
        .with_reporter(EvolveReporterNoop::new())
        .with_rng_seed_from_u64(0)
        .call()
        .unwrap();

    let best_chromosome = evolve.best_chromosome().unwrap();
    println!("{:#?}", best_chromosome);

    assert_eq!(best_chromosome.fitness_score, Some(90));
    assert_eq!(
        inspect::chromosome(&best_chromosome),
        vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9]
    );
}

#[test]
fn call_range_isize() {
    let genotype = RangeGenotype::builder()
        .with_genes_size(10)
        .with_allele_range(0..=9)
        .with_allele_mutation_range(-1..=1)
        .build()
        .unwrap();
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(100)
        .with_max_stale_generations(20)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_fitness(SumGenes::new())
        .with_crossover(CrossoverSingleGene::new(0.5))
        .with_compete(CompeteTournament::new(4))
        // .with_extension(ExtensionNoop::new())
        .with_reporter(EvolveReporterNoop::new())
        .with_rng_seed_from_u64(0)
        .call()
        .unwrap();

    let best_chromosome = evolve.best_chromosome().unwrap();
    println!("{:#?}", best_chromosome);

    assert_eq!(best_chromosome.fitness_score, Some(90));
    assert_eq!(
        inspect::chromosome(&best_chromosome),
        vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9]
    );
}

#[test]
fn call_list() {
    let genotype = ListGenotype::builder()
        .with_genes_size(10)
        .with_allele_list((0..4).collect())
        .build()
        .unwrap();

    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(100)
        .with_max_stale_generations(20)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_fitness(SumGenes::new())
        .with_crossover(CrossoverSingleGene::new(0.5))
        .with_compete(CompeteTournament::new(4))
        .with_extension(ExtensionNoop::new())
        // .with_reporter(EvolveReporterNoop::new())
        .with_rng_seed_from_u64(0)
        .call()
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
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(100)
        .with_max_stale_generations(20)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_fitness(SumGenes::new())
        .with_crossover(CrossoverSingleGene::new(0.5))
        .with_compete(CompeteTournament::new(4))
        .with_extension(ExtensionNoop::new())
        .with_reporter(EvolveReporterNoop::new())
        .with_rng_seed_from_u64(0)
        .call()
        .unwrap();

    let best_chromosome = evolve.best_chromosome().unwrap();
    println!("{:#?}", best_chromosome);

    assert_eq!(best_chromosome.fitness_score, Some(8));
    assert_eq!(inspect::chromosome(&best_chromosome), vec![4, 1, 0, 3]);
}

#[test]
fn call_par_fitness() {
    let genotype = ListGenotype::builder()
        .with_genes_size(10)
        .with_allele_list((0..4).collect())
        .build()
        .unwrap();

    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(100)
        .with_max_stale_generations(20)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_fitness(SumGenes::new())
        .with_par_fitness(true)
        .with_crossover(CrossoverSingleGene::new(0.5))
        .with_compete(CompeteTournament::new(4))
        .with_extension(ExtensionNoop::new())
        // .with_reporter(EvolveReporterNoop::new())
        .with_rng_seed_from_u64(0)
        .call()
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
    let mut evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(8)
        .with_max_stale_generations(20)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_fitness(CountTrue)
        .with_crossover(CrossoverSingleGene::new(0.5))
        .with_compete(CompeteTournament::new(4))
        .with_extension(ExtensionNoop::new())
        .with_reporter(EvolveReporterNoop::new())
        .with_rng_seed_from_u64(0)
        .build()
        .unwrap();

    evolve.init(None);
    assert_eq!(
        inspect::population(&evolve.state.population),
        vec![
            vec![true, true, true, false],
            vec![false, true, false, true],
            vec![true, false, true, false],
            vec![false, false, true, true],
            vec![true, false, false, true],
            vec![false, true, true, false],
            vec![true, false, true, false],
            vec![false, true, false, false],
        ]
    )
}
