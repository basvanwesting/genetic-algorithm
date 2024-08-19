#[cfg(test)]
use crate::support::*;
use genetic_algorithm::genotype::{RangeGenotype, Genotype};
use genetic_algorithm::mutate::{Mutate, MutateSingleGeneNeighbour};
use genetic_algorithm::strategy::evolve::{EvolveConfig, EvolveReporterNoop, EvolveState};

#[test]
fn range_float_genotype() {
    let genotype = RangeGenotype::builder()
        .with_genes_size(3)
        .with_allele_range(0.0..=1.0)
        .with_allele_neighbour_range(-0.1..=0.1)
        .build()
        .unwrap();

    let population = build::population(vec![
        vec![0.5, 0.5, 0.5],
        vec![0.5, 0.5, 0.5],
        vec![0.5, 0.5, 0.5],
        vec![0.5, 0.5, 0.5],
    ]);

    let mut state = EvolveState::new(population);
    let config = EvolveConfig::new();
    let mut reporter = EvolveReporterNoop::new();
    let mut rng = SmallRng::seed_from_u64(0);
    MutateSingleGeneNeighbour::new(0.5).call(
        &genotype,
        &mut state,
        &config,
        &mut reporter,
        &mut rng,
    );

    assert!(relative_population_eq(
        inspect::population(&state.population),
        vec![
            vec![0.5, 0.595, 0.5],
            vec![0.5, 0.5, 0.588],
            vec![0.5, 0.5, 0.5],
            vec![0.5, 0.563, 0.5],
        ],
        0.001,
    ));

    MutateSingleGeneNeighbour::new(0.5).call(
        &genotype,
        &mut state,
        &config,
        &mut reporter,
        &mut rng,
    );
    MutateSingleGeneNeighbour::new(0.5).call(
        &genotype,
        &mut state,
        &config,
        &mut reporter,
        &mut rng,
    );
    MutateSingleGeneNeighbour::new(0.5).call(
        &genotype,
        &mut state,
        &config,
        &mut reporter,
        &mut rng,
    );

    assert!(relative_population_eq(
        inspect::population(&state.population),
        vec![
            vec![0.5, 0.595, 0.528],
            vec![0.572, 0.586, 0.533],
            vec![0.557, 0.456, 0.594],
            vec![0.5, 0.563, 0.487],
        ],
        0.001
    ));
}

#[test]
fn range_integer_genotype() {
    let genotype = RangeGenotype::builder()
        .with_genes_size(3)
        .with_allele_range(-9..=9)
        .with_allele_neighbour_range(-1..=1)
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
    MutateSingleGeneNeighbour::new(0.5).call(
        &genotype,
        &mut state,
        &config,
        &mut reporter,
        &mut rng,
    );

    assert_eq!(
        inspect::population(&state.population),
        vec![vec![0, 1, 0], vec![0, 0, 1], vec![0, 0, 0], vec![0, 1, 0]],
    );

    MutateSingleGeneNeighbour::new(0.5).call(
        &genotype,
        &mut state,
        &config,
        &mut reporter,
        &mut rng,
    );
    MutateSingleGeneNeighbour::new(0.5).call(
        &genotype,
        &mut state,
        &config,
        &mut reporter,
        &mut rng,
    );
    MutateSingleGeneNeighbour::new(0.5).call(
        &genotype,
        &mut state,
        &config,
        &mut reporter,
        &mut rng,
    );

    assert_eq!(
        inspect::population(&state.population),
        vec![vec![0, 1, 0], vec![1, 1, 0], vec![1, -1, 1], vec![0, 1, 0]]
    );
}
