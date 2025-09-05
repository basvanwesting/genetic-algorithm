#[cfg(test)]
use crate::support::*;
use genetic_algorithm::distributed::chromosome::ChromosomeManager;
use genetic_algorithm::distributed::extension::{Extension, ExtensionMassGenesis};
use genetic_algorithm::distributed::genotype::{BinaryGenotype, Genotype, StaticMatrixGenotype};
use genetic_algorithm::distributed::population::Population;
use genetic_algorithm::distributed::strategy::evolve::{EvolveConfig, EvolveState};
use genetic_algorithm::distributed::strategy::StrategyReporterNoop;
use rand::prelude::*;

#[test]
fn removes_lesser() {
    let mut genotype = BinaryGenotype::builder()
        .with_genes_size(3)
        .with_genes_hashing(true)
        .build()
        .unwrap();

    let mut population: Population<BinaryChromosome> = build::population_with_fitness_scores(vec![
        (vec![true, true, true], Some(0)),
        (vec![true, true, false], Some(1)),
        (vec![true, false, false], Some(2)),
        (vec![true, true, true], Some(0)),
        (vec![true, true, false], Some(1)),
        (vec![true, false, false], Some(2)),
        (vec![true, true, true], Some(0)),
        (vec![true, true, false], Some(1)),
    ]);
    population.chromosomes.reserve_exact(2);
    assert_eq!(population.chromosomes.capacity(), 10);

    population.chromosomes.iter_mut().for_each(|chromosome| {
        let genes_hash = genotype.calculate_genes_hash(chromosome);
        chromosome.set_genes_hash(genes_hash);
    });

    let mut state = EvolveState::new(&genotype);
    assert_eq!(population.genes_cardinality(), Some(3));
    state.population_cardinality = population.genes_cardinality();
    state.population = population;

    let config = EvolveConfig::new();
    let mut reporter = StrategyReporterNoop::new();
    let mut rng = SmallRng::seed_from_u64(0);
    ExtensionMassGenesis::new(3).call(&mut genotype, &mut state, &config, &mut reporter, &mut rng);

    assert_eq!(
        inspect::population_with_fitness_scores(&state.population),
        vec![
            (vec![true, false, false], Some(2)),
            (vec![true, true, false], Some(1)),
        ]
    );
    assert_eq!(state.population.chromosomes.capacity(), 10);
}

#[test]
fn removes_lesser_no_fitness() {
    let mut genotype = BinaryGenotype::builder()
        .with_genes_size(3)
        .with_genes_hashing(true)
        .build()
        .unwrap();

    let mut population: Population<BinaryChromosome> = build::population_with_fitness_scores(vec![
        (vec![true, true, true], None),
        (vec![true, true, false], None),
        (vec![true, false, false], None),
        (vec![true, true, true], None),
        (vec![true, true, false], None),
        (vec![true, false, false], None),
        (vec![true, true, true], None),
        (vec![true, true, false], None),
    ]);
    population.chromosomes.reserve_exact(2);
    assert_eq!(population.chromosomes.capacity(), 10);

    population.chromosomes.iter_mut().for_each(|chromosome| {
        let genes_hash = genotype.calculate_genes_hash(chromosome);
        chromosome.set_genes_hash(genes_hash);
    });

    let mut state = EvolveState::new(&genotype);
    assert_eq!(population.genes_cardinality(), Some(3));
    state.population_cardinality = population.genes_cardinality();
    state.population = population;

    let config = EvolveConfig::new();
    let mut reporter = StrategyReporterNoop::new();
    let mut rng = SmallRng::seed_from_u64(0);
    ExtensionMassGenesis::new(3).call(&mut genotype, &mut state, &config, &mut reporter, &mut rng);

    assert_eq!(
        inspect::population_with_fitness_scores(&state.population),
        vec![
            (vec![true, true, true], None),
            (vec![true, true, false], None)
        ]
    );
    assert_eq!(state.population.chromosomes.capacity(), 10);
}

#[test]
fn removes_lesser_matrix() {
    let rng = &mut SmallRng::seed_from_u64(1);
    let mut genotype = StaticMatrixGenotype::<u8, 3, 8>::builder()
        .with_genes_size(3)
        .with_genes_hashing(true)
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

    population.chromosomes.iter_mut().for_each(|chromosome| {
        let genes_hash = genotype.calculate_genes_hash(chromosome);
        chromosome.set_genes_hash(genes_hash);
    });

    let mut state = EvolveState::new(&genotype);
    assert_eq!(population.genes_cardinality(), Some(8));
    state.population_cardinality = population.genes_cardinality();
    state.population = population;

    let config = EvolveConfig::new();
    let mut reporter = StrategyReporterNoop::new();
    ExtensionMassGenesis::new(10).call(&mut genotype, &mut state, &config, &mut reporter, rng);

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
