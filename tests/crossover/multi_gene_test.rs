#[cfg(test)]
use crate::support::*;
use genetic_algorithm::crossover::{Crossover, CrossoverMultiGene};
use genetic_algorithm::genotype::{BinaryGenotype, Genotype};
use genetic_algorithm::strategy::evolve::{EvolveConfig, EvolveReporterNoop, EvolveState};

#[test]
fn population_even() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(10)
        .build()
        .unwrap();

    let population = build::population(vec![
        vec![true; 10],
        vec![false; 10],
        vec![true; 10],
        vec![false; 10],
    ]);

    let mut state = EvolveState::new(&genotype, population);
    let config = EvolveConfig::new();
    let mut reporter = EvolveReporterNoop::new();
    let mut rng = SmallRng::seed_from_u64(0);
    CrossoverMultiGene::new(4, true, false).call(
        &genotype,
        &mut state,
        &config,
        &mut reporter,
        &mut rng,
    );

    assert_eq!(
        inspect::population(&state.population),
        vec![
            vec![true, true, true, true, false, true, true, true, true, false],
            vec![false, false, false, false, true, false, false, false, false, true],
            vec![true, true, true, true, false, false, true, true, false, false],
            vec![false, false, false, false, true, true, false, false, true, true],
        ]
    )
}

#[test]
fn population_odd() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(10)
        .build()
        .unwrap();

    let population = build::population(vec![
        vec![true; 10],
        vec![false; 10],
        vec![true; 10],
        vec![false; 10],
        vec![true; 10],
    ]);

    let mut state = EvolveState::new(&genotype, population);
    let config = EvolveConfig::new();
    let mut reporter = EvolveReporterNoop::new();
    let mut rng = SmallRng::seed_from_u64(0);
    CrossoverMultiGene::new(4, true, false).call(
        &genotype,
        &mut state,
        &config,
        &mut reporter,
        &mut rng,
    );

    assert_eq!(
        inspect::population(&state.population),
        vec![
            vec![true, true, true, true, false, true, true, true, true, false],
            vec![false, false, false, false, true, false, false, false, false, true],
            vec![true, true, true, true, false, false, true, true, false, false],
            vec![false, false, false, false, true, true, false, false, true, true],
            vec![true; 10],
        ]
    )
}

#[test]
fn population_even_keep_parents() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(10)
        .build()
        .unwrap();

    let population = build::population(vec![
        vec![true; 10],
        vec![false; 10],
        vec![true; 10],
        vec![false; 10],
    ]);

    let mut state = EvolveState::new(&genotype, population);
    let config = EvolveConfig::new();
    let mut reporter = EvolveReporterNoop::new();
    let mut rng = SmallRng::seed_from_u64(0);
    CrossoverMultiGene::new(4, true, true).call(
        &genotype,
        &mut state,
        &config,
        &mut reporter,
        &mut rng,
    );

    assert_eq!(
        inspect::population(&state.population),
        vec![
            vec![true, true, true, true, false, true, true, true, true, false],
            vec![false, false, false, false, true, false, false, false, false, true],
            vec![true, true, true, true, false, false, true, true, false, false],
            vec![false, false, false, false, true, true, false, false, true, true],
            vec![true; 10],
            vec![false; 10],
            vec![true; 10],
            vec![false; 10],
        ]
    )
}

#[test]
fn population_odd_keep_parents() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(10)
        .build()
        .unwrap();

    let population = build::population(vec![
        vec![true; 10],
        vec![false; 10],
        vec![true; 10],
        vec![false; 10],
        vec![true; 10],
    ]);

    let mut state = EvolveState::new(&genotype, population);
    let config = EvolveConfig::new();
    let mut reporter = EvolveReporterNoop::new();
    let mut rng = SmallRng::seed_from_u64(0);
    CrossoverMultiGene::new(4, true, true).call(
        &genotype,
        &mut state,
        &config,
        &mut reporter,
        &mut rng,
    );

    assert_eq!(
        inspect::population(&state.population),
        vec![
            vec![true, true, true, true, false, true, true, true, true, false],
            vec![false, false, false, false, true, false, false, false, false, true],
            vec![true, true, true, true, false, false, true, true, false, false],
            vec![false, false, false, false, true, true, false, false, true, true],
            vec![true; 10],
            vec![true; 10],
            vec![false; 10],
            vec![true; 10],
            vec![false; 10],
            vec![true; 10],
        ]
    )
}

#[test]
fn population_size_one() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(5)
        .build()
        .unwrap();

    let population = build::population(vec![vec![true, false, true, false, true]]);

    let mut state = EvolveState::new(&genotype, population);
    let config = EvolveConfig::new();
    let mut reporter = EvolveReporterNoop::new();
    let mut rng = SmallRng::seed_from_u64(0);
    CrossoverMultiGene::new(3, true, false).call(
        &genotype,
        &mut state,
        &config,
        &mut reporter,
        &mut rng,
    );

    assert_eq!(
        inspect::population(&state.population),
        vec![vec![true, false, true, false, true]]
    )
}