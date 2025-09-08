#[cfg(test)]
use crate::support::*;
use genetic_algorithm::centralized::chromosome::ChromosomeManager;
use genetic_algorithm::centralized::genotype::{Genotype, StaticBinaryGenotype};
use genetic_algorithm::centralized::mutate::{Mutate, MutateSingleGeneDynamic};
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
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, true],
        ],
    );

    let mut state = EvolveState::new(&genotype);
    state.population = population;
    state.population_cardinality = Some(5);
    let config = EvolveConfig::new();
    let mut reporter = StrategyReporterNoop::new();
    let mut rng = SmallRng::seed_from_u64(0);
    let mut mutate = MutateSingleGeneDynamic::new(0.1, 5);
    assert_eq!(mutate.mutation_probability, 0.0);
    state.population_cardinality = Some(2);
    mutate.call(&mut genotype, &mut state, &config, &mut reporter, &mut rng);
    assert_eq!(mutate.mutation_probability, 0.1);
    state.population_cardinality = Some(4);
    mutate.call(&mut genotype, &mut state, &config, &mut reporter, &mut rng);
    assert_eq!(mutate.mutation_probability, 0.2);
    state.population_cardinality = Some(5);
    mutate.call(&mut genotype, &mut state, &config, &mut reporter, &mut rng);
    assert_eq!(mutate.mutation_probability, 0.2);
    state.population_cardinality = Some(6);
    mutate.call(&mut genotype, &mut state, &config, &mut reporter, &mut rng);
    assert_eq!(mutate.mutation_probability, 0.1);
    state.population_cardinality = Some(6);
    mutate.call(&mut genotype, &mut state, &config, &mut reporter, &mut rng);
    assert_eq!(mutate.mutation_probability, 0.0);

    assert_eq!(
        static_inspect::population(&genotype, &state.population),
        vec![
            vec![false, true, true],
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, false],
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, true],
            vec![true, false, true],
            vec![true, true, true]
        ]
    );
}
