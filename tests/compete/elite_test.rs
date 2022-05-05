#[cfg(test)]
use crate::support::*;
use genetic_algorithm::compete::{Compete, CompeteElite};
use genetic_algorithm::fitness::{Fitness, FitnessSimpleCount};
use genetic_algorithm::genotype::BinaryGenotype;

#[test]
fn population_surplus() {
    let population = build::population::<BinaryGenotype>(vec![
        vec![false, false, false],
        vec![false, false, true],
        vec![false, true, false],
        vec![false, true, true],
        vec![true, false, false],
        vec![true, false, true],
        vec![true, true, false],
        vec![true, true, true],
    ]);

    let mut rng = SmallRng::seed_from_u64(0);
    let population = FitnessSimpleCount.call_for_population(population);
    let population = CompeteElite.call(population, 4, &mut rng);

    assert_eq!(
        inspect::population(&population),
        vec![
            vec![false, true, true],
            vec![true, false, true],
            vec![true, true, false],
            vec![true, true, true],
        ]
    );
}

#[test]
fn population_shortage() {
    let population = build::population::<BinaryGenotype>(vec![
        vec![false, false, false],
        vec![false, false, true],
        vec![false, true, false],
    ]);

    let mut rng = SmallRng::seed_from_u64(0);
    let population = FitnessSimpleCount.call_for_population(population);
    let population = CompeteElite.call(population, 4, &mut rng);

    assert_eq!(
        inspect::population(&population),
        vec![
            vec![false, false, false],
            vec![false, false, true],
            vec![false, true, false],
        ]
    );
}
