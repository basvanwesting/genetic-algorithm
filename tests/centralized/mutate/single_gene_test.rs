#[cfg(test)]
use crate::support::*;
use genetic_algorithm::centralized::chromosome::ChromosomeManager;
use genetic_algorithm::centralized::genotype::{
    Genotype, StaticBinaryGenotype, StaticRangeGenotype,
};
use genetic_algorithm::centralized::mutate::{Mutate, MutateSingleGene};
use genetic_algorithm::centralized::strategy::evolve::{EvolveConfig, EvolveState};
use genetic_algorithm::centralized::strategy::StrategyReporterNoop;

#[test]
fn binary_genotype() {
    let mut genotype = StaticBinaryGenotype::<3, 10>::builder()
        .with_genes_size(3)
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let population = static_build::population(
        &mut genotype,
        vec![
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, true],
        ],
    );

    let mut state = EvolveState::new(&genotype);
    state.population = population;
    let config = EvolveConfig::new();
    let mut reporter = StrategyReporterNoop::new();
    let mut rng = SmallRng::seed_from_u64(0);
    MutateSingleGene::new(0.5).call(&mut genotype, &mut state, &config, &mut reporter, &mut rng);

    assert_eq!(
        static_inspect::population(&genotype, &state.population),
        vec![
            vec![true, false, true],
            vec![true, true, true],
            vec![true, true, false],
            vec![true, true, true],
        ]
    );
}

#[test]
fn range_float_genotype_unscaled() {
    let mut genotype = StaticRangeGenotype::<f64, 3, 10>::builder()
        .with_genes_size(3)
        .with_allele_range(0.0..=1.0)
        .with_allele_mutation_range(-0.1..=0.1)
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let population = static_build::population(
        &mut genotype,
        vec![
            vec![0.5, 0.5, 0.5],
            vec![0.5, 0.5, 0.5],
            vec![0.5, 0.5, 0.5],
            vec![0.5, 0.5, 0.5],
        ],
    );

    let mut state = EvolveState::new(&genotype);
    state.population = population;
    let config = EvolveConfig::new();
    let mut reporter = StrategyReporterNoop::new();
    let mut rng = SmallRng::seed_from_u64(0);
    MutateSingleGene::new(0.5).call(&mut genotype, &mut state, &config, &mut reporter, &mut rng);

    assert!(relative_population_eq(
        static_inspect::population(&genotype, &state.population),
        vec![
            vec![0.5, 0.595, 0.5],
            vec![0.5, 0.5, 0.588],
            vec![0.5, 0.5, 0.5],
            vec![0.5, 0.563, 0.5],
        ],
        0.001,
    ));

    MutateSingleGene::new(0.5).call(&mut genotype, &mut state, &config, &mut reporter, &mut rng);
    MutateSingleGene::new(0.5).call(&mut genotype, &mut state, &config, &mut reporter, &mut rng);
    MutateSingleGene::new(0.5).call(&mut genotype, &mut state, &config, &mut reporter, &mut rng);

    assert!(relative_population_eq(
        static_inspect::population(&genotype, &state.population),
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
fn range_float_genotype_scaled() {
    let mut genotype = StaticRangeGenotype::<f64, 3, 10>::builder()
        .with_genes_size(3)
        .with_allele_range(0.0..=1.0)
        .with_allele_mutation_scaled_range(vec![-0.1..=0.1, -0.01..=0.01, -0.001..=0.001])
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let population = static_build::population(
        &mut genotype,
        vec![
            vec![0.5, 0.5, 0.5],
            vec![0.5, 0.5, 0.5],
            vec![0.5, 0.5, 0.5],
            vec![0.5, 0.5, 0.5],
        ],
    );

    let mut state = EvolveState::new(&genotype);
    state.population = population;
    let config = EvolveConfig::new();
    let mut reporter = StrategyReporterNoop::new();
    let mut rng = SmallRng::seed_from_u64(0);
    MutateSingleGene::new(0.5).call(&mut genotype, &mut state, &config, &mut reporter, &mut rng);
    assert!(relative_population_eq(
        static_inspect::population(&genotype, &state.population),
        vec![
            vec![0.5, 0.4, 0.5],
            vec![0.5, 0.5, 0.4],
            vec![0.5, 0.5, 0.5],
            vec![0.5, 0.4, 0.5],
        ],
        0.001,
    ));

    MutateSingleGene::new(0.5).call(&mut genotype, &mut state, &config, &mut reporter, &mut rng);
    assert!(relative_population_eq(
        static_inspect::population(&genotype, &state.population),
        vec![
            vec![0.5, 0.4, 0.4],
            vec![0.5, 0.5, 0.5],
            vec![0.4, 0.5, 0.5],
            vec![0.5, 0.4, 0.5],
        ],
        0.001,
    ));

    state.current_scale_index = Some(1);
    MutateSingleGene::new(0.5).call(&mut genotype, &mut state, &config, &mut reporter, &mut rng);
    assert!(relative_population_eq(
        static_inspect::population(&genotype, &state.population),
        vec![
            vec![0.5, 0.4, 0.4],
            vec![0.49, 0.5, 0.5],
            vec![0.4, 0.5, 0.49],
            vec![0.5, 0.4, 0.51],
        ],
        0.001
    ));
}

#[test]
fn range_integer_genotype() {
    let mut genotype = StaticRangeGenotype::<i32, 3, 10>::builder()
        .with_genes_size(3)
        .with_allele_range(-9..=9)
        .with_allele_mutation_range(-1..=1)
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let population = static_build::population(
        &mut genotype,
        vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]],
    );

    let mut state = EvolveState::new(&genotype);
    state.population = population;
    let config = EvolveConfig::new();
    let mut reporter = StrategyReporterNoop::new();
    let mut rng = SmallRng::seed_from_u64(0);
    MutateSingleGene::new(0.5).call(&mut genotype, &mut state, &config, &mut reporter, &mut rng);

    assert_eq!(
        static_inspect::population(&genotype, &state.population),
        vec![vec![0, 1, 0], vec![0, 0, 1], vec![0, 0, 0], vec![0, 1, 0]],
    );

    MutateSingleGene::new(0.5).call(&mut genotype, &mut state, &config, &mut reporter, &mut rng);
    MutateSingleGene::new(0.5).call(&mut genotype, &mut state, &config, &mut reporter, &mut rng);
    MutateSingleGene::new(0.5).call(&mut genotype, &mut state, &config, &mut reporter, &mut rng);

    assert_eq!(
        static_inspect::population(&genotype, &state.population),
        vec![vec![0, 1, 0], vec![1, 1, 0], vec![1, -1, 1], vec![0, 1, 0]]
    );
}
