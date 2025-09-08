#[cfg(test)]
use crate::support::*;
use genetic_algorithm::centralized::chromosome::ChromosomeManager;
use genetic_algorithm::centralized::genotype::{Genotype, StaticBinaryGenotype};
use genetic_algorithm::centralized::mutate::{Mutate, MutateMultiGeneDynamic};
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
    let mut mutate = MutateMultiGeneDynamic::new(2, 0.2, 5);
    assert_eq!(mutate.mutation_probability, 0.0);
    state.population_cardinality = Some(2);
    mutate.call(&mut genotype, &mut state, &config, &mut reporter, &mut rng);
    assert_eq!(mutate.mutation_probability, 0.2);
    state.population_cardinality = Some(4);
    mutate.call(&mut genotype, &mut state, &config, &mut reporter, &mut rng);
    assert_eq!(mutate.mutation_probability, 0.4);
    state.population_cardinality = Some(5);
    mutate.call(&mut genotype, &mut state, &config, &mut reporter, &mut rng);
    assert_eq!(mutate.mutation_probability, 0.4);
    state.population_cardinality = Some(6);
    mutate.call(&mut genotype, &mut state, &config, &mut reporter, &mut rng);
    assert_eq!(mutate.mutation_probability, 0.2);
    state.population_cardinality = Some(6);
    mutate.call(&mut genotype, &mut state, &config, &mut reporter, &mut rng);
    assert_eq!(mutate.mutation_probability, 0.0);

    assert_eq!(
        static_inspect::population(&genotype, &state.population),
        vec![
            vec![true, true, true],
            vec![true, true, true],
            vec![false, true, false],
            vec![false, true, false],
            vec![true, false, false],
            vec![false, true, false],
            vec![false, true, false],
            vec![false, false, true],
            vec![true, false, false],
            vec![true, false, false],
        ]
    );
}
