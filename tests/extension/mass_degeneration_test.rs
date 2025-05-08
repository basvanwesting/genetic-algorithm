#[cfg(test)]
use crate::support::*;
use genetic_algorithm::extension::{Extension, ExtensionMassDegeneration};
use genetic_algorithm::genotype::{BinaryGenotype, Genotype};
use genetic_algorithm::population::Population;
use genetic_algorithm::strategy::evolve::{EvolveConfig, EvolveState};
use genetic_algorithm::strategy::StrategyReporterNoop;

#[test]
fn degenerates_randomly() {
    let mut genotype = BinaryGenotype::builder()
        .with_genes_size(3)
        .build()
        .unwrap();

    let mut population: Population<BinaryChromosome> = build::population_with_fitness_scores(vec![
        (vec![true, true, true], Some(2)),
        (vec![true, true, true], Some(1)),
        (vec![true, true, true], Some(4)),
        (vec![true, true, true], Some(5)),
        (vec![true, true, true], Some(7)),
        (vec![true, true, true], Some(3)),
        (vec![true, true, true], Some(8)),
        (vec![true, true, true], Some(6)),
    ]);
    population.chromosomes.reserve_exact(2);
    assert_eq!(population.chromosomes.capacity(), 10);

    let mut state = EvolveState::new(&genotype);
    state.population = population;
    state.population_cardinality = Some(6);
    let config = EvolveConfig::new();
    let mut reporter = StrategyReporterNoop::new();
    let mut rng = SmallRng::seed_from_u64(0);
    ExtensionMassDegeneration::new(7, 2, 0.25).call(
        &mut genotype,
        &mut state,
        &config,
        &mut reporter,
        &mut rng,
    );

    assert_eq!(
        inspect::population_with_fitness_scores(&state.population),
        vec![
            // elite
            (vec![true, true, true], Some(7)),
            (vec![true, true, true], Some(8)),
            // others
            (vec![true, true, true], None),
            (vec![true, true, true], None),
            (vec![true, false, false], None),
            (vec![false, true, false], None),
            (vec![true, false, false], None),
            (vec![true, true, true], None),
        ]
    );
    assert_eq!(state.population.chromosomes.capacity(), 10);
}

#[test]
fn degenerates_randomly_no_elite() {
    let mut genotype = BinaryGenotype::builder()
        .with_genes_size(3)
        .build()
        .unwrap();

    let mut population: Population<BinaryChromosome> = build::population_with_fitness_scores(vec![
        (vec![true, true, true], Some(2)),
        (vec![true, true, true], Some(1)),
        (vec![true, true, true], Some(4)),
        (vec![true, true, true], Some(5)),
        (vec![true, true, true], Some(7)),
        (vec![true, true, true], Some(3)),
        (vec![true, true, true], Some(8)),
        (vec![true, true, true], Some(6)),
    ]);
    population.chromosomes.reserve_exact(2);
    assert_eq!(population.chromosomes.capacity(), 10);

    let mut state = EvolveState::new(&genotype);
    state.population = population;
    state.population_cardinality = Some(6);
    let config = EvolveConfig::new();
    let mut reporter = StrategyReporterNoop::new();
    let mut rng = SmallRng::seed_from_u64(0);
    ExtensionMassDegeneration::new(7, 2, 0.0).call(
        &mut genotype,
        &mut state,
        &config,
        &mut reporter,
        &mut rng,
    );

    assert_eq!(
        inspect::population_with_fitness_scores(&state.population),
        vec![
            (vec![true, true, true], None),
            (vec![true, false, false], None),
            (vec![true, true, true], None),
            (vec![true, true, true], None),
            (vec![true, false, false], None),
            (vec![false, true, false], None),
            (vec![false, false, true], None),
            (vec![false, true, false], None)
        ]
    );
    assert_eq!(state.population.chromosomes.capacity(), 10);
}
