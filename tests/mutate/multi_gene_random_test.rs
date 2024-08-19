#[cfg(test)]
use crate::support::*;
use genetic_algorithm::genotype::{BinaryGenotype, ListGenotype, Genotype};
use genetic_algorithm::mutate::{Mutate, MutateMultiGeneRandom};
use genetic_algorithm::strategy::evolve::{EvolveConfig, EvolveReporterNoop, EvolveState};

#[test]
fn binary_genotype() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(3)
        .build()
        .unwrap();

    let population = build::population(vec![
        vec![true, true, true],
        vec![true, true, true],
        vec![true, true, true],
        vec![true, true, true],
    ]);

    let mut state = EvolveState::new(population);
    let config = EvolveConfig::new();
    let mut reporter = EvolveReporterNoop::new();
    let mut rng = SmallRng::seed_from_u64(0);
    MutateMultiGeneRandom::new(2, 0.5).call(
        &genotype,
        &mut state,
        &config,
        &mut reporter,
        &mut rng,
    );

    assert_eq!(
        inspect::population(&state.population),
        vec![
            vec![true, false, false],
            vec![true, true, true],
            vec![true, true, true],
            vec![true, false, false]
        ]
    );
}

#[test]
fn list_genotype() {
    let genotype = ListGenotype::builder()
        .with_genes_size(3)
        .with_allele_list(vec![0, 1, 2, 3])
        .build()
        .unwrap();

    let population = build::population(vec![
        vec![0, 0, 0],
        vec![0, 0, 0],
        vec![0, 0, 0],
        vec![0, 0, 0],
    ]);

    let mut state = EvolveState::new(population);
    let config = EvolveConfig::new();
    let mut reporter = EvolveReporterNoop::new();
    let mut rng = SmallRng::seed_from_u64(0);
    MutateMultiGeneRandom::new(2, 0.5).call(
        &genotype,
        &mut state,
        &config,
        &mut reporter,
        &mut rng,
    );

    assert_eq!(
        inspect::population(&state.population),
        vec![vec![0, 3, 0], vec![0, 0, 0], vec![0, 0, 0], vec![3, 3, 0]]
    );
}
