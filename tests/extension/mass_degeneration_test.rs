#[cfg(test)]
use crate::support::*;
use genetic_algorithm::extension::{Extension, ExtensionMassDegeneration};
use genetic_algorithm::genotype::{BinaryGenotype, Genotype};
use genetic_algorithm::strategy::evolve::{EvolveConfig, EvolveReporterNoop, EvolveState};

#[test]
fn removes_randomly() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(3)
        .build()
        .unwrap();

    let population = build::population(vec![
        vec![true, true, true],
        vec![true, true, true],
        vec![true, true, true],
        vec![true, true, true],
        vec![true, true, true],
        vec![true, true, true],
        vec![true, true, true],
        vec![true, true, true],
    ]);
    assert_eq!(population.fitness_score_cardinality(), 8);
    assert_eq!(population.chromosomes.capacity(), 8);

    let mut state = EvolveState::new(&genotype);
    state.population = population;
    let config = EvolveConfig::new();
    let mut reporter = EvolveReporterNoop::new();
    let mut rng = SmallRng::seed_from_u64(0);
    ExtensionMassDegeneration::new(8, 2).call(
        &genotype,
        &mut state,
        &config,
        &mut reporter,
        &mut rng,
    );

    assert_eq!(
        inspect::population(&state.population),
        vec![
            vec![true, true, true],
            vec![true, false, false],
            vec![true, true, true],
            vec![true, true, true],
            vec![true, false, false],
            vec![false, true, false],
            vec![false, false, true],
            vec![false, true, false],
        ]
    );
    assert_eq!(state.population.chromosomes.capacity(), 8);
}
