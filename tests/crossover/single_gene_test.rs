#[cfg(test)]
use crate::support::*;
use genetic_algorithm::crossover::{Crossover, CrossoverSingleGene};
use genetic_algorithm::genotype::{BinaryGenotype, Genotype};
use genetic_algorithm::population::Population;
use genetic_algorithm::strategy::evolve::{EvolveConfig, EvolveState};
use genetic_algorithm::strategy::StrategyReporterNoop;

#[test]
fn standard_crossover() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(5)
        .build()
        .unwrap();

    let population: Population<bool> = build::population_with_age(vec![
        (vec![true, true, true, true, true], 1),
        (vec![false, false, false, false, false], 1),
        (vec![true, true, true, true, true], 1),
        (vec![false, false, false, false, false], 1),
    ]);
    assert_eq!(population.chromosomes.capacity(), 4);

    let mut state = EvolveState::new(&genotype);
    state.population = population;
    let config = EvolveConfig {
        target_population_size: 4,
        ..Default::default()
    };
    let mut reporter = StrategyReporterNoop::new();
    let mut rng = SmallRng::seed_from_u64(0);
    CrossoverSingleGene::new(0.5, 1.0).call(
        &genotype,
        &mut state,
        &config,
        &mut reporter,
        &mut rng,
    );

    assert_eq!(
        inspect::population_with_age(&state.population),
        vec![
            (vec![true, true, true, true, true], 1),
            (vec![false, false, false, false, false], 1),
            (vec![true, true, true, true, true], 1),
            (vec![false, false, false, false, false], 1),
            (vec![true, true, false, true, true], 0),
            (vec![false, false, true, false, false], 0),
        ]
    );
    assert_eq!(state.population.chromosomes.capacity(), 8);
}

#[test]
fn zero_crossover_rate() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(5)
        .build()
        .unwrap();

    let population: Population<bool> = build::population_with_age(vec![
        (vec![true, true, true, true, true], 1),
        (vec![false, false, false, false, false], 1),
        (vec![true, true, true, true, true], 1),
        (vec![false, false, false, false, false], 1),
    ]);
    assert_eq!(population.chromosomes.capacity(), 4);

    let mut state = EvolveState::new(&genotype);
    state.population = population;
    let config = EvolveConfig {
        target_population_size: 4,
        ..Default::default()
    };
    let mut reporter = StrategyReporterNoop::new();
    let mut rng = SmallRng::seed_from_u64(0);
    CrossoverSingleGene::new(0.5, 0.0).call(
        &genotype,
        &mut state,
        &config,
        &mut reporter,
        &mut rng,
    );

    assert_eq!(
        inspect::population_with_age(&state.population),
        vec![
            (vec![true, true, true, true, true], 1),
            (vec![false, false, false, false, false], 1),
            (vec![true, true, true, true, true], 1),
            (vec![false, false, false, false, false], 1),
            (vec![true, true, true, true, true], 0),
            (vec![false, false, false, false, false], 0),
        ]
    );
    assert_eq!(state.population.chromosomes.capacity(), 8);
}

#[test]
fn odd_selection_size() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(5)
        .build()
        .unwrap();

    let population: Population<bool> = build::population_with_age(vec![
        (vec![true, true, true, true, true], 1),
        (vec![false, false, false, false, false], 1),
        (vec![true, true, true, true, true], 1),
        (vec![false, false, false, false, false], 1),
    ]);
    assert_eq!(population.chromosomes.capacity(), 4);

    let mut state = EvolveState::new(&genotype);
    state.population = population;
    let config = EvolveConfig {
        target_population_size: 4,
        ..Default::default()
    };
    let mut reporter = StrategyReporterNoop::new();
    let mut rng = SmallRng::seed_from_u64(0);
    CrossoverSingleGene::new(0.6, 0.8).call(
        &genotype,
        &mut state,
        &config,
        &mut reporter,
        &mut rng,
    );

    assert_eq!(
        inspect::population_with_age(&state.population),
        vec![
            (vec![true, true, true, true, true], 1),
            (vec![false, false, false, false, false], 1),
            (vec![true, true, true, true, true], 1),
            (vec![false, false, false, false, false], 1),
            (vec![true, true, false, true, true], 0),
            (vec![false, false, true, false, false], 0),
            (vec![true, true, true, true, true], 0),
        ]
    );
    assert_eq!(state.population.chromosomes.capacity(), 8);
}
