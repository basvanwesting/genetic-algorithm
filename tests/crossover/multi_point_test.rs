#[cfg(test)]
use crate::support::*;
use genetic_algorithm::crossover::{Crossover, CrossoverMultiPoint};
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
    let mut rng = SmallRng::seed_from_u64(1);
    CrossoverMultiPoint::new(3, false).call(
        &genotype,
        &mut state,
        &config,
        &mut reporter,
        &mut rng,
    );

    assert_eq!(
        inspect::population(&state.population),
        vec![
            vec![false, true, true, true, true, true, true, false, false, false],
            vec![true, false, false, false, false, false, false, true, true, true],
            vec![true, true, true, false, true, true, true, true, true, false],
            vec![false, false, false, true, false, false, false, false, false, true]
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
    let mut rng = SmallRng::seed_from_u64(1);
    CrossoverMultiPoint::new(3, true).call(&genotype, &mut state, &config, &mut reporter, &mut rng);

    assert_eq!(
        inspect::population(&state.population),
        vec![
            vec![false, true, true, true, true, true, true, false, false, false],
            vec![true, false, false, false, false, false, false, true, true, true],
            vec![true, true, true, false, true, true, true, true, true, false],
            vec![false, false, false, true, false, false, false, false, false, true],
            vec![true; 10],
            vec![false; 10],
            vec![true; 10],
            vec![false; 10],
        ]
    )
}
