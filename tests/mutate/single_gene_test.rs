#[cfg(test)]
use crate::support::*;
use genetic_algorithm::genotype::{
    BinaryGenotype, Genotype, ListGenotype, MutationType, RangeGenotype,
};
use genetic_algorithm::mutate::{Mutate, MutateSingleGene};
use genetic_algorithm::population::Population;
use genetic_algorithm::strategy::evolve::{EvolveConfig, EvolveState};
use genetic_algorithm::strategy::StrategyReporterNoop;

#[test]
fn binary_genotype() {
    let genotype = BinaryGenotype::builder()
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
    MutateSingleGene::new(0.5).call(&genotype, &mut state, &config, &mut reporter, &mut rng);

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

    let mut state = EvolveState::new(&genotype);
    state.population = population;
    let config = EvolveConfig::new();
    let mut reporter = StrategyReporterNoop::new();
    let mut rng = SmallRng::seed_from_u64(0);
    MutateSingleGene::new(0.5).call(&genotype, &mut state, &config, &mut reporter, &mut rng);

    assert_eq!(
        inspect::population(&state.population),
        vec![vec![0, 3, 0], vec![0, 0, 3], vec![0, 0, 0], vec![0, 3, 0],]
    );
}

#[test]
fn range_float_genotype() {
    let genotype = RangeGenotype::builder()
        .with_genes_size(3)
        .with_allele_range(0.0..=1.0)
        .with_mutation_type(MutationType::Range(0.1))
        .build()
        .unwrap();

    let population = build::population(vec![
        vec![0.5, 0.5, 0.5],
        vec![0.5, 0.5, 0.5],
        vec![0.5, 0.5, 0.5],
        vec![0.5, 0.5, 0.5],
    ]);

    let mut state = EvolveState::new(&genotype);
    state.population = population;
    let config = EvolveConfig::new();
    let mut reporter = StrategyReporterNoop::new();
    let mut rng = SmallRng::seed_from_u64(0);
    MutateSingleGene::new(0.75).call(&genotype, &mut state, &config, &mut reporter, &mut rng);

    assert!(relative_population_eq(
        inspect::population(&state.population),
        vec![
            vec![0.5, 0.402, 0.5],
            vec![0.5, 0.5, 0.5],
            vec![0.5, 0.5, 0.5],
            vec![0.5, 0.539, 0.5],
        ],
        0.001,
    ));

    MutateSingleGene::new(0.5).call(&genotype, &mut state, &config, &mut reporter, &mut rng);
    MutateSingleGene::new(0.5).call(&genotype, &mut state, &config, &mut reporter, &mut rng);
    MutateSingleGene::new(0.5).call(&genotype, &mut state, &config, &mut reporter, &mut rng);

    assert!(relative_population_eq(
        inspect::population(&state.population),
        vec![
            vec![0.5, 0.402, 0.479],
            vec![0.413, 0.406, 0.5],
            vec![0.460, 0.5, 0.5],
            vec![0.5, 0.539, 0.421],
        ],
        0.001
    ));
}

#[test]
fn range_integer_genotype() {
    let genotype = RangeGenotype::builder()
        .with_genes_size(3)
        .with_allele_range(0..=9)
        .with_mutation_type(MutationType::Range(1))
        .build()
        .unwrap();

    let population = build::population(vec![
        vec![5, 5, 5],
        vec![5, 5, 5],
        vec![5, 5, 5],
        vec![5, 5, 5],
    ]);

    let mut state = EvolveState::new(&genotype);
    state.population = population;
    let config = EvolveConfig::new();
    let mut reporter = StrategyReporterNoop::new();
    let mut rng = SmallRng::seed_from_u64(0);
    MutateSingleGene::new(0.75).call(&genotype, &mut state, &config, &mut reporter, &mut rng);

    assert_eq!(
        inspect::population(&state.population),
        vec![vec![5, 4, 5], vec![5, 5, 5], vec![5, 5, 5], vec![5, 6, 5]],
    );

    MutateSingleGene::new(1.0).call(&genotype, &mut state, &config, &mut reporter, &mut rng);
    MutateSingleGene::new(1.0).call(&genotype, &mut state, &config, &mut reporter, &mut rng);

    assert_eq!(
        inspect::population(&state.population),
        vec![vec![5, 4, 5], vec![4, 5, 4], vec![6, 5, 6], vec![6, 6, 4]]
    );
}
