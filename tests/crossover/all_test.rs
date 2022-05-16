#[cfg(test)]
use crate::support::*;
use genetic_algorithm::crossover::{Crossover, CrossoverAll};
use genetic_algorithm::genotype::{BinaryGenotype, Genotype};

#[test]
fn population_even() {
    let genotype = BinaryGenotype::builder().with_gene_size(6).build().unwrap();

    let population = &mut build::population(vec![
        vec![true, true, true, true, true],
        vec![false, false, false, false, false],
        vec![true, true, true, true, true],
        vec![false, false, false, false, false],
    ]);

    let mut rng = SmallRng::seed_from_u64(0);
    CrossoverAll(false).call(&genotype, population, &mut rng);

    assert_eq!(
        inspect::population(population),
        vec![
            vec![false, false, true, false, true],
            vec![true, true, false, true, false],
            vec![true, false, false, true, false],
            vec![false, true, true, false, true],
        ]
    )
}

#[test]
fn population_odd() {
    let genotype = BinaryGenotype::builder().with_gene_size(3).build().unwrap();

    let population = &mut build::population(vec![
        vec![true, true, true, true, true],
        vec![false, false, false, false, false],
        vec![true, true, true, true, true],
        vec![false, false, false, false, false],
        vec![true, true, true, true, true],
    ]);

    let mut rng = SmallRng::seed_from_u64(0);
    CrossoverAll(false).call(&genotype, population, &mut rng);

    assert_eq!(
        inspect::population(population),
        vec![
            vec![false, false, true, true, true],
            vec![true, true, false, false, false],
            vec![false, true, true, true, true],
            vec![true, false, false, false, false],
            vec![true, true, true, true, true],
        ]
    )
}

#[test]
fn population_even_keep_parent() {
    let genotype = BinaryGenotype::builder().with_gene_size(6).build().unwrap();

    let population = &mut build::population(vec![
        vec![true, true, true, true, true],
        vec![false, false, false, false, false],
        vec![true, true, true, true, true],
        vec![false, false, false, false, false],
    ]);

    let mut rng = SmallRng::seed_from_u64(0);
    CrossoverAll(true).call(&genotype, population, &mut rng);

    assert_eq!(
        inspect::population(population),
        vec![
            vec![true, true, true, true, true],
            vec![false, false, false, false, false],
            vec![true, true, true, true, true],
            vec![false, false, false, false, false],
            vec![false, false, true, false, true],
            vec![true, true, false, true, false],
            vec![true, false, false, true, false],
            vec![false, true, true, false, true],
        ]
    )
}

#[test]
fn allow_unique_genotype() {
    assert_eq!(CrossoverAll(false).allow_unique_genotype(), false);
    assert_eq!(CrossoverAll(true).allow_unique_genotype(), false);
}
