#[cfg(test)]
use crate::support::*;
use genetic_algorithm::compete::{Compete, CompeteElite};
use genetic_algorithm::fitness::placeholders::CountTrue;
use genetic_algorithm::fitness::{Fitness, FitnessOrdering};
use genetic_algorithm::genotype::BinaryGenotype;

#[test]
fn maximize_population_surplus() {
    let population = &mut build::population::<BinaryGenotype>(vec![
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
    CountTrue.call_for_population(population, 1);
    CompeteElite.call(population, FitnessOrdering::Maximize, 4, &mut rng);

    assert_eq!(
        inspect::population(population),
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
    let population = &mut build::population::<BinaryGenotype>(vec![
        vec![false, false, false],
        vec![false, false, true],
        vec![false, true, false],
    ]);

    let mut rng = SmallRng::seed_from_u64(0);
    CountTrue.call_for_population(population, 1);
    CompeteElite.call(population, FitnessOrdering::Maximize, 4, &mut rng);

    assert_eq!(
        inspect::population(population),
        vec![
            vec![false, false, false],
            vec![false, false, true],
            vec![false, true, false],
        ]
    );
}

#[test]
fn minimize_population_surplus() {
    let population = &mut build::population::<BinaryGenotype>(vec![
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
    CountTrue.call_for_population(population, 1);
    CompeteElite.call(population, FitnessOrdering::Minimize, 4, &mut rng);

    assert_eq!(
        inspect::population(population),
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
    let population = &mut build::population::<BinaryGenotype>(vec![
        vec![false, false, false],
        vec![false, false, true],
        vec![false, true, false],
    ]);

    let mut rng = SmallRng::seed_from_u64(0);
    CountTrue.call_for_population(population, 1);
    CompeteElite.call(population, FitnessOrdering::Minimize, 4, &mut rng);

    assert_eq!(
        inspect::population(population),
        vec![
            vec![false, false, true],
            vec![false, true, false],
            vec![false, false, false],
        ]
    );
}

#[test]
fn fitness_ordering_with_none_fitness() {
    let population = &mut build::population_with_fitness_scores::<BinaryGenotype>(vec![
        (vec![false, false, false], Some(0)),
        (vec![false, false, true], Some(1)),
        (vec![false, true, true], Some(2)),
        (vec![true, true, true], Some(3)),
        (vec![true, true, false], None),
    ]);

    let mut rng = SmallRng::seed_from_u64(0);
    CompeteElite.call(population, FitnessOrdering::Maximize, 5, &mut rng);
    assert_eq!(
        inspect::population_with_fitness_scores(population),
        vec![
            (vec![true, true, false], None),
            (vec![false, false, false], Some(0)),
            (vec![false, false, true], Some(1)),
            (vec![false, true, true], Some(2)),
            (vec![true, true, true], Some(3)),
        ]
    );

    CompeteElite.call(population, FitnessOrdering::Minimize, 5, &mut rng);
    assert_eq!(
        inspect::population_with_fitness_scores(population),
        vec![
            (vec![true, true, false], None),
            (vec![true, true, true], Some(3)),
            (vec![false, true, true], Some(2)),
            (vec![false, false, true], Some(1)),
            (vec![false, false, false], Some(0)),
        ]
    );
}
