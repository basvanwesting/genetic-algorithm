#[cfg(test)]
use crate::support::*;
use genetic_algorithm::genotype::{ContinuousGenotype, Genotype};
use genetic_algorithm::mutate::{Mutate, MutateSingleGeneNeighbour};
use genetic_algorithm::strategy::evolve::{EvolveConfig, EvolveReporterNoop, EvolveState};

#[test]
fn continuous_float_genotype() {
    let genotype = ContinuousGenotype::builder()
        .with_genes_size(3)
        .with_allele_range(0.0..1.0)
        .with_allele_neighbour_range(-0.1..0.1)
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
            vec![0.5, 0.59597605, 0.5],
            vec![0.5, 0.5, 0.58858997],
            vec![0.5, 0.5, 0.5],
            vec![0.5, 0.5637702, 0.5],
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
            vec![0.5, 0.59597605, 0.52890337],
            vec![0.5729175, 0.5864907, 0.5336115],
            vec![0.55756766, 0.45686823, 0.5942435],
            vec![0.5, 0.5637702, 0.48710027],
        ],
        0.001
    ));
}

#[test]
fn continuous_integer_genotype() {
    let genotype = ContinuousGenotype::builder()
        .with_genes_size(3)
        .with_allele_range(-10..10)
        .with_allele_neighbour_range(-1..2)
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
        vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, -1, 0], vec![0, 0, 0]],
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
        vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0],],
    );
}
