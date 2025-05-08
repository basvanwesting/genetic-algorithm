#[cfg(test)]
use crate::support::*;
use genetic_algorithm::chromosome::ChromosomeManager;
use genetic_algorithm::extension::{Extension, ExtensionMassGenesis};
use genetic_algorithm::genotype::{BinaryGenotype, Genotype, StaticMatrixGenotype};
use genetic_algorithm::population::Population;
use genetic_algorithm::strategy::evolve::{EvolveConfig, EvolveState};
use genetic_algorithm::strategy::StrategyReporterNoop;
use rand::prelude::*;

#[test]
fn removes_lesser() {
    let mut genotype = BinaryGenotype::builder()
        .with_genes_size(3)
        .build()
        .unwrap();

    let mut population: Population<BinaryChromosome> = build::population_with_fitness_scores(vec![
        (vec![false, true, true], Some(1)),
        (vec![false, true, false], Some(2)),
        (vec![false, false, true], Some(3)),
        (vec![false, false, false], Some(4)),
        (vec![true, true, true], Some(5)),
        (vec![true, true, false], Some(3)),
        (vec![true, false, true], Some(2)),
        (vec![true, false, false], Some(1)),
    ]);
    population.chromosomes.reserve_exact(2);
    assert_eq!(population.chromosomes.capacity(), 10);

    let mut state = EvolveState::new(&genotype);
    state.population = population;
    state.population_cardinality = Some(5);
    let config = EvolveConfig::new();
    let mut reporter = StrategyReporterNoop::new();
    let mut rng = SmallRng::seed_from_u64(0);
    ExtensionMassGenesis::new(5).call(&mut genotype, &mut state, &config, &mut reporter, &mut rng);

    assert_eq!(
        inspect::population_with_fitness_scores(&state.population),
        vec![
            (vec![true, true, true], Some(5)),
            (vec![false, false, false], Some(4)),
        ]
    );
    assert_eq!(state.population.chromosomes.capacity(), 10);
}

#[test]
fn removes_lesser_no_fitness() {
    let mut genotype = BinaryGenotype::builder()
        .with_genes_size(3)
        .build()
        .unwrap();

    let mut population: Population<BinaryChromosome> = build::population_with_fitness_scores(vec![
        (vec![false, true, true], None),
        (vec![false, true, false], None),
        (vec![false, false, true], None),
        (vec![false, false, false], None),
        (vec![true, true, true], None),
        (vec![true, true, false], None),
        (vec![true, false, true], None),
        (vec![true, false, false], None),
    ]);
    population.chromosomes.reserve_exact(2);
    assert_eq!(population.chromosomes.capacity(), 10);

    let mut state = EvolveState::new(&genotype);
    state.population = population;
    state.population_cardinality = Some(5);
    let config = EvolveConfig::new();
    let mut reporter = StrategyReporterNoop::new();
    let mut rng = SmallRng::seed_from_u64(0);
    ExtensionMassGenesis::new(5).call(&mut genotype, &mut state, &config, &mut reporter, &mut rng);

    assert_eq!(
        inspect::population_with_fitness_scores(&state.population),
        vec![
            (vec![false, true, true], None),
            (vec![false, true, false], None)
        ]
    );
    assert_eq!(state.population.chromosomes.capacity(), 10);
}

#[test]
fn removes_lesser_matrix() {
    let rng = &mut SmallRng::seed_from_u64(1);
    let mut genotype = StaticMatrixGenotype::<u8, 3, 8>::builder()
        .with_genes_size(3)
        .with_allele_range(0..=10)
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let mut chromosomes = (0..8)
        .map(|_| genotype.chromosome_constructor_random(rng))
        .collect::<Vec<_>>();
    chromosomes
        .iter_mut()
        .for_each(|c| c.fitness_score = Some(rng.gen_range(0..10)));
    let mut population = Population::new(chromosomes);

    population.chromosomes.reserve_exact(2);
    assert_eq!(population.chromosomes.capacity(), 10);

    let mut state = EvolveState::new(&genotype);
    state.population = population;
    state.population_cardinality = Some(6);
    let config = EvolveConfig::new();
    let mut reporter = StrategyReporterNoop::new();
    ExtensionMassGenesis::new(7).call(&mut genotype, &mut state, &config, &mut reporter, rng);

    assert_eq!(
        state
            .population
            .chromosomes
            .iter()
            .map(|c| genotype.genes_slice(c).to_vec())
            .collect::<Vec<_>>(),
        vec![vec![7, 6, 9], vec![3, 5, 10]]
    );
    assert_eq!(state.population.chromosomes.capacity(), 10);
}
