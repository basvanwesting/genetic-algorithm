#[cfg(test)]
use crate::support::*;
use genetic_algorithm::compete::{Compete, CompeteTournament};
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
    let population = CompeteTournament(4).call(population, FitnessOrdering::Maximize, 4, &mut rng);

    assert_eq!(
        inspect::population(&population),
        vec![
            vec![true, true, true],
            vec![true, true, false],
            vec![false, true, true],
            vec![true, false, false],
        ]
    );
}

#[test]
fn maximize_population_shortage() {
    let population = build::population::<BinaryGenotype>(vec![
        vec![false, false, false],
        vec![false, false, true],
    ]);

    let mut rng = SmallRng::seed_from_u64(0);
    let population = FitnessSimpleCount.call_for_population(population);
    let population = CompeteTournament(4).call(population, FitnessOrdering::Maximize, 4, &mut rng);

    assert_eq!(
        inspect::population(&population),
        vec![vec![false, false, true], vec![false, false, false],]
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
    let population = CompeteTournament(4).call(population, FitnessOrdering::Minimize, 4, &mut rng);

    assert_eq!(
        inspect::population(&population),
        vec![
            vec![false, false, false],
            vec![false, true, false],
            vec![false, false, true],
            vec![true, false, false],
        ]
    );
}

#[test]
fn minimize_population_shortage() {
    let population = build::population::<BinaryGenotype>(vec![
        vec![false, false, false],
        vec![false, false, true],
    ]);

    let mut rng = SmallRng::seed_from_u64(0);
    let population = FitnessSimpleCount.call_for_population(population);
    let population = CompeteTournament(4).call(population, FitnessOrdering::Minimize, 4, &mut rng);

    assert_eq!(
        inspect::population(&population),
        vec![vec![false, false, false], vec![false, false, true],]
    );
}

#[test]
fn minimize_population_surplus_with_none_fitness() {
    let population = build::population_with_fitness_scores::<BinaryGenotype>(vec![
        (vec![false, false, false], Some(0)),
        (vec![false, false, true], None),
        (vec![false, true, false], Some(1)),
        (vec![false, true, true], Some(2)),
        (vec![true, false, false], Some(1)),
        (vec![true, false, true], Some(2)),
        (vec![true, true, false], Some(2)),
        (vec![true, true, true], Some(3)),
    ]);

    let mut rng = SmallRng::seed_from_u64(0);
    let population = CompeteTournament(4).call(population, FitnessOrdering::Minimize, 4, &mut rng);

    assert_eq!(
        inspect::population_with_fitness_scores(&population),
        vec![
            (vec![false, false, false], Some(0)),
            (vec![false, true, false], Some(1)),
            (vec![true, false, false], Some(1)),
            (vec![true, false, true], Some(2)),
        ]
    );
}

#[test]
fn minimize_population_shortage_with_none_fitness() {
    let population = build::population_with_fitness_scores::<BinaryGenotype>(vec![
        (vec![false, false, false], None),
        (vec![false, false, true], Some(1)),
        (vec![false, true, true], Some(2)),
    ]);

    let mut rng = SmallRng::seed_from_u64(0);
    let population = CompeteTournament(4).call(population, FitnessOrdering::Minimize, 4, &mut rng);

    assert_eq!(
        inspect::population_with_fitness_scores(&population),
        vec![
            (vec![false, false, true], Some(1)),
            (vec![false, true, true], Some(2)),
            (vec![false, false, false], None),
        ]
    );
}
