#[cfg(test)]
use crate::support::*;
use genetic_algorithm::crossover::{Crossover, CrossoverClone};
use genetic_algorithm::genotype::{BinaryGenotype, PermutableGenotype, UniqueIndexGenotype};

#[test]
fn population_odd() {
    let genotype = BinaryGenotype::new().with_gene_size(3).build();

    let population = build::population(vec![
        vec![true, true, true],
        vec![false, false, false],
        vec![true, true, true],
    ]);

    let mut rng = SmallRng::seed_from_u64(0);
    let population = CrossoverClone(true).call(&genotype, population, &mut rng);

    assert_eq!(
        inspect::population(&population),
        vec![
            vec![true, true, true],
            vec![false, false, false],
            vec![true, true, true],
            vec![true, true, true],
            vec![false, false, false],
            vec![true, true, true],
        ]
    )
}

#[test]
fn population_size_one() {
    let genotype = BinaryGenotype::new().with_gene_size(5).build();

    let population = build::population(vec![vec![true, false, true, false, true]]);

    let mut rng = SmallRng::seed_from_u64(0);
    let population = CrossoverClone(false).call(&genotype, population, &mut rng);

    assert_eq!(
        inspect::population(&population),
        vec![vec![true, false, true, false, true]]
    )
}

#[test]
fn genotype_is_unique_constraints() {
    let genotype = UniqueIndexGenotype::new().with_gene_value_size(5).build();
    let population = genotype.population_factory();
    let mut rng = SmallRng::seed_from_u64(0);

    let _population = CrossoverClone(false).call(&genotype, population.clone(), &mut rng);
}
