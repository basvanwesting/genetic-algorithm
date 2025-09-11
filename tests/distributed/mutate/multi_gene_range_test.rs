#[cfg(test)]
use crate::support::*;
use genetic_algorithm::distributed::genotype::{BinaryGenotype, Genotype, ListGenotype};
use genetic_algorithm::distributed::mutate::{Mutate, MutateMultiGeneRange};
use genetic_algorithm::distributed::population::Population;
use genetic_algorithm::distributed::strategy::evolve::{EvolveConfig, EvolveState};
use genetic_algorithm::distributed::strategy::StrategyReporterNoop;

#[test]
fn binary_genotype() {
    let mut genotype = BinaryGenotype::builder()
        .with_genes_size(3)
        .build()
        .unwrap();

    let population: Population<bool> = build::population(vec![
        vec![true, true, true],
        vec![true, true, true],
        vec![true, true, true],
        vec![true, true, true],
    ]);

    let mut state = EvolveState::new(&genotype);
    state.population = population;
    let config = EvolveConfig::new();
    let mut reporter = StrategyReporterNoop::new();
    let mut rng = SmallRng::seed_from_u64(0);
    MutateMultiGeneRange::new(1..=2, 0.5).call(
        &mut genotype,
        &mut state,
        &config,
        &mut reporter,
        &mut rng,
    );

    assert_eq!(
        inspect::population(&state.population),
        vec![
            vec![true, true, false],
            vec![true, false, false],
            vec![true, true, false],
            vec![false, false, true]
        ]
    );
}

#[test]
fn list_genotype() {
    let mut genotype = ListGenotype::builder()
        .with_genes_size(3)
        .with_allele_list(vec![0, 1, 2, 3])
        .build()
        .unwrap();

    let population: Population<u8> = build::population(vec![
        vec![0, 0, 0],
        vec![0, 0, 0],
        vec![0, 0, 0],
        vec![0, 0, 0],
    ]);

    let mut state = EvolveState::new(&genotype);
    state.population = population;
    let config = EvolveConfig::new();
    let mut reporter = StrategyReporterNoop::new();
    let mut rng = SmallRng::seed_from_u64(0);
    MutateMultiGeneRange::new(1..=3, 0.5).call(
        &mut genotype,
        &mut state,
        &config,
        &mut reporter,
        &mut rng,
    );

    assert_eq!(
        inspect::population(&state.population),
        vec![vec![0, 0, 3], vec![0, 0, 0], vec![0, 0, 2], vec![3, 0, 2]]
    );
}
