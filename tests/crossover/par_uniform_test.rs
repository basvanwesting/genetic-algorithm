#[cfg(test)]
use crate::support::*;
use genetic_algorithm::crossover::{Crossover, CrossoverParUniform};
use genetic_algorithm::genotype::{BinaryGenotype, Genotype};
use genetic_algorithm::strategy::evolve::{EvolveConfig, EvolveReporterNoop, EvolveState};

#[test]
fn population_even() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(5)
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
    CrossoverParUniform::new(false).call(&genotype, &mut state, &config, &mut reporter, &mut rng);

    // cannot assert result as parallel execution in combination with randomness is not determinstic. Just assert it doens't panic
    assert_eq!(state.population.size(), 4);
}

#[test]
fn population_odd() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(5)
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
    CrossoverParUniform::new(false).call(&genotype, &mut state, &config, &mut reporter, &mut rng);

    // cannot assert result as parallel execution in combination with randomness is not determinstic. Just assert it doens't panic
    assert_eq!(state.population.size(), 5);
}

#[test]
fn population_even_keep_parent() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(5)
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
    CrossoverParUniform::new(true).call(&genotype, &mut state, &config, &mut reporter, &mut rng);

    // cannot assert result as parallel execution in combination with randomness is not determinstic. Just assert it doens't panic
    assert_eq!(state.population.size(), 8);
}

#[test]
fn population_odd_keep_parent() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(5)
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
    CrossoverParUniform::new(true).call(&genotype, &mut state, &config, &mut reporter, &mut rng);

    // cannot assert result as parallel execution in combination with randomness is not determinstic. Just assert it doens't panic
    assert_eq!(state.population.size(), 10);
}
