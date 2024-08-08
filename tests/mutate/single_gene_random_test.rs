#[cfg(test)]
use crate::support::*;
use genetic_algorithm::genotype::{BinaryGenotype, DiscreteGenotype, Genotype};
use genetic_algorithm::mutate::{Mutate, MutateSingleGeneRandom};
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
    MutateSingleGeneRandom::new(0.5).call(&genotype, &mut state, &config, &mut reporter, &mut rng);

    assert_eq!(
        inspect::population(&state.population),
        vec![
            vec![true, false, true],
            vec![true, true, true],
            vec![true, true, false],
            vec![true, true, true],
        ]
    );
}

#[test]
fn discrete_genotype() {
    let genotype = DiscreteGenotype::builder()
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
    MutateSingleGeneRandom::new(0.5).call(&genotype, &mut state, &config, &mut reporter, &mut rng);

    assert_eq!(
        inspect::population(&state.population),
        vec![vec![0, 3, 0], vec![0, 0, 3], vec![0, 0, 0], vec![0, 3, 0],]
    );
}
