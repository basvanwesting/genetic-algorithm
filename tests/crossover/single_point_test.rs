#[cfg(test)]
use crate::support::*;
use genetic_algorithm::crossover::{Crossover, CrossoverSinglePoint};
use genetic_algorithm::genotype::{BinaryGenotype, Genotype};

#[test]
fn population_even() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(6)
        .build()
        .unwrap();

    let population = &mut build::population(vec![
        vec![true, true, true, true, true],
        vec![false, false, false, false, false],
        vec![true, true, true, true, true],
        vec![false, false, false, false, false],
    ]);

    let mut rng = SmallRng::seed_from_u64(0);
    CrossoverSinglePoint(false).call(&genotype, population, &mut rng);

    assert_eq!(
        inspect::population(population),
        vec![
            vec![true, true, false, false, false],
            vec![false, false, true, true, true],
            vec![true, true, false, false, false],
            vec![false, false, true, true, true],
        ]
    )
}

#[test]
fn population_even_keep_parents() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(6)
        .build()
        .unwrap();

    let population = &mut build::population(vec![
        vec![true, true, true, true, true],
        vec![false, false, false, false, false],
        vec![true, true, true, true, true],
        vec![false, false, false, false, false],
    ]);

    let mut rng = SmallRng::seed_from_u64(0);
    CrossoverSinglePoint(true).call(&genotype, population, &mut rng);

    assert_eq!(
        inspect::population(population),
        vec![
            vec![true, true, true, true, true],
            vec![false, false, false, false, false],
            vec![true, true, true, true, true],
            vec![false, false, false, false, false],
            vec![true, true, false, false, false],
            vec![false, false, true, true, true],
            vec![true, true, false, false, false],
            vec![false, false, true, true, true],
        ]
    )
}

#[test]
fn allow_unique_genotype() {
    assert_eq!(CrossoverSinglePoint(false).allow_unique_genotype(), false);
    assert_eq!(CrossoverSinglePoint(true).allow_unique_genotype(), false);
}
