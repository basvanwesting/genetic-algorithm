#[cfg(test)]
use crate::support::*;
use genetic_algorithm::compete::{Compete, CompeteTournament};
use genetic_algorithm::fitness::{Fitness, FitnessSimpleCount};
use genetic_algorithm::genotype::BinaryGenotype;

#[test]
fn test_surplus() {
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
    let population = CompeteTournament(4).call(population, 4, &mut rng);

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
fn test_shortage() {
    let population = build::population::<BinaryGenotype>(vec![
        vec![false, false, false],
        vec![false, false, true],
    ]);

    let mut rng = SmallRng::seed_from_u64(0);
    let population = FitnessSimpleCount.call_for_population(population);
    let population = CompeteTournament(4).call(population, 4, &mut rng);

    assert_eq!(
        inspect::population(&population),
        vec![vec![false, false, true], vec![false, false, false],]
    );
}
