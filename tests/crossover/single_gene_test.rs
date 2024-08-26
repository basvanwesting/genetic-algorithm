#[cfg(test)]
use crate::support::*;
use genetic_algorithm::crossover::{Crossover, CrossoverSingleGene};
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
    CrossoverSingleGene::new(false).call(&genotype, &mut state, &config, &mut reporter, &mut rng);

    assert_eq!(
        inspect::population(&state.population),
        vec![
            vec![true, true, false, true, true],
            vec![false, false, true, false, false],
            vec![true, true, false, true, true],
            vec![false, false, true, false, false],
        ]
    )
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
    CrossoverSingleGene::new(false).call(&genotype, &mut state, &config, &mut reporter, &mut rng);

    assert_eq!(
        inspect::population(&state.population),
        vec![
            vec![true, true, false, true, true],
            vec![false, false, true, false, false],
            vec![true, true, false, true, true],
            vec![false, false, true, false, false],
            vec![true, true, true, true, true],
        ]
    )
}

#[test]
fn population_even_keep_parents() {
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
    CrossoverSingleGene::new(true).call(&genotype, &mut state, &config, &mut reporter, &mut rng);

    assert_eq!(
        inspect::population(&state.population),
        vec![
            vec![true, true, false, true, true],
            vec![false, false, true, false, false],
            vec![true, true, false, true, true],
            vec![false, false, true, false, false],
            vec![true, true, true, true, true],
            vec![false, false, false, false, false],
            vec![true, true, true, true, true],
            vec![false, false, false, false, false],
        ]
    )
}

#[test]
fn population_odd_keep_parents() {
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
    CrossoverSingleGene::new(true).call(&genotype, &mut state, &config, &mut reporter, &mut rng);

    assert_eq!(
        inspect::population(&state.population),
        vec![
            vec![true, true, false, true, true],
            vec![false, false, true, false, false],
            vec![true, true, false, true, true],
            vec![false, false, true, false, false],
            vec![true, true, true, true, true],
            vec![true, true, true, true, true],
            vec![false, false, false, false, false],
            vec![true, true, true, true, true],
            vec![false, false, false, false, false],
            vec![true, true, true, true, true],
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
    CrossoverSingleGene::new(false).call(&genotype, &mut state, &config, &mut reporter, &mut rng);

    assert_eq!(
        inspect::population(&state.population),
        vec![vec![true, false, true, false, true]]
    )
}
