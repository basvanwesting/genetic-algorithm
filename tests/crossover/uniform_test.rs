#[cfg(test)]
use crate::support::*;
use genetic_algorithm::crossover::{Crossover, CrossoverUniform};
use genetic_algorithm::genotype::{BinaryGenotype, Genotype};
use genetic_algorithm::strategy::evolve::{EvolveConfig, EvolveReporterNoop, EvolveState};

#[test]
fn population_even() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(6)
        .build()
        .unwrap();

    let population = build::population(vec![
        vec![true, true, true, true, true],
        vec![false, false, false, false, false],
        vec![true, true, true, true, true],
        vec![false, false, false, false, false],
    ]);

    let mut state = EvolveState::new(&genotype, population);
    let config = EvolveConfig::new();
    let mut reporter = EvolveReporterNoop::new();
    let mut rng = SmallRng::seed_from_u64(0);
    CrossoverUniform::new(false).call(
        &genotype,
        &mut state,
        &config,
        &mut reporter,
        &mut rng,
        None,
    );

    assert_eq!(
        inspect::population(&state.population),
        vec![
            vec![false, false, true, false, true],
            vec![true, true, false, true, false],
            vec![true, false, false, true, false],
            vec![false, true, true, false, true],
        ]
    )
}

#[test]
fn population_odd() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(3)
        .build()
        .unwrap();

    let population = build::population(vec![
        vec![true, true, true, true, true],
        vec![false, false, false, false, false],
        vec![true, true, true, true, true],
        vec![false, false, false, false, false],
        vec![true, true, true, true, true],
    ]);

    let mut state = EvolveState::new(&genotype, population);
    let config = EvolveConfig::new();
    let mut reporter = EvolveReporterNoop::new();
    let mut rng = SmallRng::seed_from_u64(0);
    CrossoverUniform::new(false).call(
        &genotype,
        &mut state,
        &config,
        &mut reporter,
        &mut rng,
        None,
    );

    assert_eq!(
        inspect::population(&state.population),
        vec![
            vec![false, false, true, true, true],
            vec![true, true, false, false, false],
            vec![false, true, true, true, true],
            vec![true, false, false, false, false],
            vec![true, true, true, true, true],
        ]
    )
}

#[test]
fn population_even_keep_parent() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(6)
        .build()
        .unwrap();

    let population = build::population(vec![
        vec![true, true, true, true, true],
        vec![false, false, false, false, false],
        vec![true, true, true, true, true],
        vec![false, false, false, false, false],
    ]);

    let mut state = EvolveState::new(&genotype, population);
    let config = EvolveConfig::new();
    let mut reporter = EvolveReporterNoop::new();
    let mut rng = SmallRng::seed_from_u64(0);
    CrossoverUniform::new(true).call(
        &genotype,
        &mut state,
        &config,
        &mut reporter,
        &mut rng,
        None,
    );

    assert_eq!(
        inspect::population(&state.population),
        vec![
            vec![true, true, true, true, true],
            vec![false, false, false, false, false],
            vec![true, true, true, true, true],
            vec![false, false, false, false, false],
            vec![false, false, true, false, true],
            vec![true, true, false, true, false],
            vec![true, false, false, true, false],
            vec![false, true, true, false, true],
        ]
    )
}
