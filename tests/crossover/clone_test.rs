#[cfg(test)]
use crate::support::*;
use genetic_algorithm::crossover::{Crossover, CrossoverClone};
use genetic_algorithm::genotype::{BinaryGenotype, Genotype};
use genetic_algorithm::population::Population;
use genetic_algorithm::strategy::evolve::{EvolveConfig, EvolveState};
use genetic_algorithm::strategy::StrategyReporterNoop;

#[test]
fn population_no_shortage() {
    let mut genotype = BinaryGenotype::builder()
        .with_genes_size(3)
        .build()
        .unwrap();

    let population: Population<BinaryChromosome> = build::population(vec![
        vec![true, true, true],
        vec![false, false, false],
        vec![true, true, true],
    ]);

    let mut state = EvolveState::new(&genotype);
    state.population = population;
    let config = EvolveConfig {
        target_population_size: 3,
        ..Default::default()
    };
    let mut reporter = StrategyReporterNoop::new();
    let mut rng = SmallRng::seed_from_u64(0);
    CrossoverClone::new().call(&mut genotype, &mut state, &config, &mut reporter, &mut rng);

    assert_eq!(
        inspect::population(&state.population),
        vec![
            vec![true, true, true],
            vec![false, false, false],
            vec![true, true, true],
        ]
    )
}

#[test]
fn population_odd_shortage() {
    let mut genotype = BinaryGenotype::builder()
        .with_genes_size(3)
        .build()
        .unwrap();

    let population: Population<BinaryChromosome> = build::population(vec![
        vec![true, true, true],
        vec![false, false, false],
        vec![true, true, true],
    ]);

    let mut state = EvolveState::new(&genotype);
    state.population = population;
    let config = EvolveConfig {
        target_population_size: 4,
        ..Default::default()
    };
    let mut reporter = StrategyReporterNoop::new();
    let mut rng = SmallRng::seed_from_u64(0);
    CrossoverClone::new().call(&mut genotype, &mut state, &config, &mut reporter, &mut rng);

    assert_eq!(
        inspect::population(&state.population),
        vec![
            vec![true, true, true],
            vec![false, false, false],
            vec![true, true, true],
            vec![true, true, true],
        ]
    )
}

#[test]
fn population_even_shortage() {
    let mut genotype = BinaryGenotype::builder()
        .with_genes_size(3)
        .build()
        .unwrap();

    let population: Population<BinaryChromosome> = build::population(vec![
        vec![true, true, true],
        vec![false, false, false],
        vec![true, true, true],
        vec![false, false, false],
    ]);

    let mut state = EvolveState::new(&genotype);
    state.population = population;
    let config = EvolveConfig {
        target_population_size: 5,
        ..Default::default()
    };
    let mut reporter = StrategyReporterNoop::new();
    let mut rng = SmallRng::seed_from_u64(0);
    CrossoverClone::new().call(&mut genotype, &mut state, &config, &mut reporter, &mut rng);

    assert_eq!(
        inspect::population(&state.population),
        vec![
            vec![true, true, true],
            vec![false, false, false],
            vec![true, true, true],
            vec![false, false, false],
            vec![true, true, true],
        ]
    )
}

#[test]
fn population_shortage_below_target() {
    let mut genotype = BinaryGenotype::builder()
        .with_genes_size(5)
        .build()
        .unwrap();

    let population: Population<BinaryChromosome> =
        build::population(vec![vec![true, false, true, false, true]]);

    let mut state = EvolveState::new(&genotype);
    state.population = population;
    let config = EvolveConfig {
        target_population_size: 4,
        ..Default::default()
    };
    let mut reporter = StrategyReporterNoop::new();
    let mut rng = SmallRng::seed_from_u64(0);
    CrossoverClone::new().call(&mut genotype, &mut state, &config, &mut reporter, &mut rng);

    assert_eq!(
        inspect::population(&state.population),
        vec![
            vec![true, false, true, false, true],
            vec![true, false, true, false, true],
            vec![true, false, true, false, true],
            vec![true, false, true, false, true],
        ]
    )
}

#[test]
fn population_excess() {
    let mut genotype = BinaryGenotype::builder()
        .with_genes_size(3)
        .build()
        .unwrap();

    let population: Population<BinaryChromosome> = build::population(vec![
        vec![true, true, true],
        vec![false, false, false],
        vec![true, true, true],
        vec![false, false, false],
        vec![true, true, true],
    ]);

    let mut state = EvolveState::new(&genotype);
    state.population = population;
    let config = EvolveConfig {
        target_population_size: 4,
        ..Default::default()
    };
    let mut reporter = StrategyReporterNoop::new();
    let mut rng = SmallRng::seed_from_u64(0);
    CrossoverClone::new().call(&mut genotype, &mut state, &config, &mut reporter, &mut rng);

    assert_eq!(
        inspect::population(&state.population),
        vec![
            vec![true, true, true],
            vec![false, false, false],
            vec![true, true, true],
            vec![false, false, false],
        ]
    )
}
