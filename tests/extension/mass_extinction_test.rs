#[cfg(test)]
use crate::support::*;
use genetic_algorithm::extension::{Extension, ExtensionMassExtinction};
use genetic_algorithm::genotype::{BinaryGenotype, Genotype};
use genetic_algorithm::population::Population;
use genetic_algorithm::strategy::evolve::{EvolveConfig, EvolveState};
use genetic_algorithm::strategy::StrategyReporterNoop;

#[test]
fn removes_randomly() {
    let mut genotype = BinaryGenotype::builder()
        .with_genes_size(3)
        .build()
        .unwrap();

    let mut population: Population<BinaryChromosome> = build::population_with_fitness_scores(vec![
        (vec![false, false, false], Some(4)),
        (vec![false, false, true], Some(3)),
        (vec![false, true, false], Some(2)),
        (vec![false, true, true], Some(1)),
        (vec![true, false, false], Some(8)),
        (vec![true, false, true], Some(7)),
        (vec![true, true, false], Some(6)),
        (vec![true, true, true], Some(5)),
    ]);
    population.chromosomes.reserve_exact(2);
    assert_eq!(population.chromosomes.capacity(), 10);

    let mut state = EvolveState::new(&genotype);
    state.population = population;
    state.population_cardinality = Some(6);
    let config = EvolveConfig::new();
    let mut reporter = StrategyReporterNoop::new();
    let mut rng = SmallRng::seed_from_u64(0);
    ExtensionMassExtinction::new(7, 0.50, 0.20).call(
        &mut genotype,
        &mut state,
        &config,
        &mut reporter,
        &mut rng,
    );

    assert_eq!(
        inspect::population_with_fitness_scores(&state.population),
        vec![
            // elite
            (vec![true, false, false], Some(8)),
            (vec![true, false, true], Some(7)),
            // normal
            (vec![false, false, false], Some(4)),
            (vec![false, true, true], Some(1)),
        ]
    );
    assert_eq!(state.population.chromosomes.capacity(), 10);
}

#[test]
fn never_leaves_less_than_two_no_elite() {
    let mut genotype = BinaryGenotype::builder()
        .with_genes_size(3)
        .build()
        .unwrap();

    let population: Population<BinaryChromosome> = build::population_with_fitness_scores(vec![
        (vec![false, false, false], Some(4)),
        (vec![false, false, true], Some(3)),
        (vec![false, true, false], Some(2)),
        (vec![false, true, true], Some(1)),
        (vec![true, false, true], Some(7)),
        (vec![true, true, false], Some(6)),
        (vec![true, false, false], Some(8)),
        (vec![true, true, true], Some(5)),
    ]);

    let mut state = EvolveState::new(&genotype);
    state.population = population;
    state.population_cardinality = Some(6);
    let config = EvolveConfig::new();
    let mut reporter = StrategyReporterNoop::new();
    let mut rng = SmallRng::seed_from_u64(0);
    ExtensionMassExtinction::new(7, 0.01, 0.0).call(
        &mut genotype,
        &mut state,
        &config,
        &mut reporter,
        &mut rng,
    );

    assert_eq!(
        inspect::population_with_fitness_scores(&state.population),
        vec![
            (vec![true, false, true], Some(7)),
            (vec![false, false, true], Some(3)),
        ]
    );
}

#[test]
fn never_leaves_less_than_two_one_elite() {
    let mut genotype = BinaryGenotype::builder()
        .with_genes_size(3)
        .build()
        .unwrap();

    let population: Population<BinaryChromosome> = build::population_with_fitness_scores(vec![
        (vec![false, false, false], Some(4)),
        (vec![false, false, true], Some(3)),
        (vec![false, true, false], Some(2)),
        (vec![false, true, true], Some(1)),
        (vec![true, false, true], Some(7)),
        (vec![true, true, false], Some(6)),
        (vec![true, false, false], Some(8)),
        (vec![true, true, true], Some(5)),
    ]);

    let mut state = EvolveState::new(&genotype);
    state.population = population;
    state.population_cardinality = Some(6);
    let config = EvolveConfig::new();
    let mut reporter = StrategyReporterNoop::new();
    let mut rng = SmallRng::seed_from_u64(0);
    ExtensionMassExtinction::new(7, 0.01, 0.01).call(
        &mut genotype,
        &mut state,
        &config,
        &mut reporter,
        &mut rng,
    );

    assert_eq!(
        inspect::population_with_fitness_scores(&state.population),
        vec![
            (vec![true, false, false], Some(8)),
            (vec![true, true, true], Some(5))
        ]
    );
}
