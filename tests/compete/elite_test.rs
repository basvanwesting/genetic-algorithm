#[cfg(test)]
use crate::support::*;
use genetic_algorithm::compete::{Compete, CompeteElite};
use genetic_algorithm::fitness::{Fitness, FitnessOrdering, FitnessSimpleCount};
use genetic_algorithm::genotype::BinaryGenotype;

#[test]
fn maximize_population_surplus() {
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
    let population = CompeteElite.call(population, FitnessOrdering::Maximize, 4, &mut rng);

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
fn maximize_population_shortage() {
    let population = build::population::<BinaryGenotype>(vec![
        vec![false, false, false],
        vec![false, false, true],
        vec![false, true, false],
    ]);

    let mut rng = SmallRng::seed_from_u64(0);
    let population = FitnessSimpleCount.call_for_population(population);
    let population = CompeteElite.call(population, FitnessOrdering::Maximize, 4, &mut rng);

    assert_eq!(
        inspect::population(&population),
        vec![
            vec![false, false, false],
            vec![false, false, true],
            vec![false, true, false],
        ]
    );
}

#[test]
fn minimize_population_surplus() {
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
    let population = CompeteElite.call(population, FitnessOrdering::Minimize, 4, &mut rng);

    assert_eq!(
        inspect::population(&population),
        vec![
            vec![false, false, true],
            vec![false, true, false],
            vec![true, false, false],
            vec![false, false, false],
        ]
    );
}

#[test]
fn minimize_population_shortage() {
    let population = build::population::<BinaryGenotype>(vec![
        vec![false, false, false],
        vec![false, false, true],
        vec![false, true, false],
    ]);

    let mut rng = SmallRng::seed_from_u64(0);
    let population = FitnessSimpleCount.call_for_population(population);
    let population = CompeteElite.call(population, FitnessOrdering::Minimize, 4, &mut rng);

    assert_eq!(
        inspect::population(&population),
        vec![
            vec![false, false, true],
            vec![false, true, false],
            vec![false, false, false],
        ]
    );
}
