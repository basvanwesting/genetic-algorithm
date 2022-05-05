#[cfg(test)]
use crate::support::*;
use genetic_algorithm::crossover::{Crossover, CrossoverSingle};
use genetic_algorithm::genotype::{BinaryGenotype, PermutableGenotype, UniqueIndexGenotype};

#[test]
fn test_even() {
    let genotype = BinaryGenotype::new().with_gene_size(5).build();

    let population = build::population(vec![
        vec![true, true, true, true, true],
        vec![false, false, false, false, false],
        vec![true, true, true, true, true],
        vec![false, false, false, false, false],
    ]);

    let mut rng = SmallRng::seed_from_u64(0);
    let population = CrossoverSingle(false).call(&genotype, population, &mut rng);

    assert_eq!(
        inspect::population(&population),
        vec![
            vec![true, true, false, true, true],
            vec![false, false, true, false, false],
            vec![true, true, false, true, true],
            vec![false, false, true, false, false],
        ]
    )
}

#[test]
fn test_odd() {
    let genotype = BinaryGenotype::new().with_gene_size(5).build();

    let population = build::population(vec![
        vec![true, true, true, true, true],
        vec![false, false, false, false, false],
        vec![true, true, true, true, true],
        vec![false, false, false, false, false],
        vec![true, true, true, true, true],
    ]);

    let mut rng = SmallRng::seed_from_u64(0);
    let population = CrossoverSingle(false).call(&genotype, population, &mut rng);

    assert_eq!(
        inspect::population(&population),
        vec![
            vec![true, true, false, true, true],
            vec![false, false, true, false, false],
            vec![true, true, false, true, true],
            vec![false, false, true, false, false],
        ]
    )
}

#[test]
fn test_population_size_1() {
    let genotype = BinaryGenotype::new().with_gene_size(5).build();

    let population = build::population(vec![vec![true, false, true, false, true]]);

    let mut rng = SmallRng::seed_from_u64(0);
    let population = CrossoverSingle(false).call(&genotype, population, &mut rng);

    assert_eq!(
        inspect::population(&population),
        vec![vec![true, false, true, false, true]]
    )
}

#[test]
#[should_panic(expected = "Cannot use Crossover::Single for unique genotype")]
fn test_is_unique_constraints() {
    let genotype = UniqueIndexGenotype::new().with_gene_value_size(5).build();
    let population = genotype.population_factory();
    let mut rng = SmallRng::seed_from_u64(0);

    let _population = CrossoverSingle(false).call(&genotype, population.clone(), &mut rng);
}
