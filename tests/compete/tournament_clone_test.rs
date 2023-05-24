#[cfg(test)]
use crate::support::*;
use genetic_algorithm::compete::{Compete, CompeteTournamentClone};
use genetic_algorithm::fitness::placeholders::CountTrue;
use genetic_algorithm::fitness::{Fitness, FitnessOrdering};
use genetic_algorithm::genotype::BinaryGenotype;
use genetic_algorithm::strategy::evolve::EvolveConfig;

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
    CountTrue.call_for_population(population, None);
    let evolve_config = EvolveConfig {
        fitness_ordering: FitnessOrdering::Maximize,
        target_population_size: 4,
        ..Default::default()
    };
    CompeteTournamentClone::new(4).call(population, &evolve_config, &mut rng);

    assert_eq!(
        inspect::population(population),
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
    let population = &mut build::population::<BinaryGenotype>(vec![
        vec![false, false, false],
        vec![false, false, true],
    ]);

    let mut rng = SmallRng::seed_from_u64(0);
    CountTrue.call_for_population(population, None);
    let evolve_config = EvolveConfig {
        fitness_ordering: FitnessOrdering::Maximize,
        target_population_size: 4,
        ..Default::default()
    };
    CompeteTournamentClone::new(4).call(population, &evolve_config, &mut rng);

    assert_eq!(
        inspect::population(population),
        vec![vec![false, false, true], vec![false, false, false],]
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
    CountTrue.call_for_population(population, None);
    let evolve_config = EvolveConfig {
        fitness_ordering: FitnessOrdering::Minimize,
        target_population_size: 4,
        ..Default::default()
    };
    CompeteTournamentClone::new(4).call(population, &evolve_config, &mut rng);

    assert_eq!(
        inspect::population(population),
        vec![
            vec![false, false, false],
            vec![false, false, true],
            vec![true, false, false],
            vec![true, true, false],
        ]
    );
}

#[test]
fn minimize_population_shortage() {
    let population = &mut build::population::<BinaryGenotype>(vec![
        vec![false, false, false],
        vec![false, false, true],
    ]);

    let mut rng = SmallRng::seed_from_u64(0);
    CountTrue.call_for_population(population, None);
    let evolve_config = EvolveConfig {
        fitness_ordering: FitnessOrdering::Minimize,
        target_population_size: 4,
        ..Default::default()
    };
    CompeteTournamentClone::new(4).call(population, &evolve_config, &mut rng);

    assert_eq!(
        inspect::population(population),
        vec![vec![false, false, false], vec![false, false, true],]
    );
}

#[test]
fn minimize_population_surplus_with_none_fitness() {
    let population = &mut build::population_with_fitness_scores::<BinaryGenotype>(vec![
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
    let evolve_config = EvolveConfig {
        fitness_ordering: FitnessOrdering::Minimize,
        target_population_size: 4,
        ..Default::default()
    };
    CompeteTournamentClone::new(4).call(population, &evolve_config, &mut rng);

    assert_eq!(
        inspect::population_with_fitness_scores(population),
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
    let population = &mut build::population_with_fitness_scores::<BinaryGenotype>(vec![
        (vec![false, false, false], None),
        (vec![false, false, true], Some(1)),
        (vec![false, true, true], Some(2)),
    ]);

    let mut rng = SmallRng::seed_from_u64(0);
    let evolve_config = EvolveConfig {
        fitness_ordering: FitnessOrdering::Minimize,
        target_population_size: 4,
        ..Default::default()
    };
    CompeteTournamentClone::new(4).call(population, &evolve_config, &mut rng);

    assert_eq!(
        inspect::population_with_fitness_scores(population),
        vec![
            (vec![false, false, true], Some(1)),
            (vec![false, true, true], Some(2)),
            (vec![false, false, false], None),
        ]
    );
}
