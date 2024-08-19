mod support;

#[cfg(test)]
mod population_tests {
    use crate::support::*;
    use approx::assert_relative_eq;
    use genetic_algorithm::fitness::placeholders::CountTrue;
    use genetic_algorithm::fitness::{Fitness, FitnessOrdering};

    #[test]
    fn fitness_score_stddev() {
        let population = &mut build::population(vec![
            vec![false, true, true],
            vec![false, true, false],
            vec![false, false, true],
            vec![false, false, false],
            vec![true, true, true],
            vec![true, true, false],
            vec![true, false, true],
            vec![true, false, false],
        ]);

        assert_eq!(population.fitness_score_stddev(), 0.0);
        CountTrue.call_for_population(population, None);
        assert_relative_eq!(population.fitness_score_stddev(), 0.866, epsilon = 0.001);

        let population = &mut build::population(vec![
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, false],
        ]);

        assert_eq!(population.fitness_score_stddev(), 0.0);
        CountTrue.call_for_population(population, None);
        assert_relative_eq!(population.fitness_score_stddev(), 0.331, epsilon = 0.001);
    }

    #[test]
    fn best_chromosome() {
        let population = &mut build::population_with_fitness_scores(vec![
            (vec![false, false, false], Some(0)),
            (vec![false, false, true], Some(1)),
            (vec![false, true, true], Some(2)),
            (vec![true, true, true], Some(3)),
            (vec![true, true, false], None),
        ]);

        let best_chromosome = population.best_chromosome(FitnessOrdering::Maximize);
        assert_eq!(
            best_chromosome.map_or(Some(99), |c| c.fitness_score),
            Some(3)
        );
        let best_chromosome = population.best_chromosome(FitnessOrdering::Minimize);
        assert_eq!(
            best_chromosome.map_or(Some(99), |c| c.fitness_score),
            Some(0)
        );
    }

    #[test]
    fn fitness_score_cardinality() {
        let population = &mut build::population_with_fitness_scores(vec![
            (vec![false, false, false], Some(0)),
            (vec![false, false, true], Some(2)),
            (vec![false, true, true], Some(2)),
            (vec![true, true, true], Some(3)),
            (vec![true, true, false], None),
        ]);

        assert_eq!(population.fitness_score_cardinality(), 3 + 1);
    }

    #[test]
    fn trim() {
        let mut rng = SmallRng::seed_from_u64(0);
        let population = &mut build::population(vec![
            vec![false, true, true],
            vec![false, true, false],
            vec![false, false, true],
            vec![false, false, false],
            vec![true, true, true],
            vec![true, true, false],
            vec![true, false, true],
            vec![true, false, false],
        ]);

        population.trim(0.75, &mut rng);
        assert_eq!(
            inspect::population(population),
            vec![
                vec![true, true, true],
                vec![false, true, false],
                vec![false, true, true],
                vec![true, true, false],
                vec![false, false, true],
                vec![false, false, false],
            ]
        );
    }

    #[test]
    fn trim_never_less_than_two() {
        let mut rng = SmallRng::seed_from_u64(0);
        let population = &mut build::population(vec![
            vec![false, true, true],
            vec![false, true, false],
            vec![false, false, true],
            vec![false, false, false],
            vec![true, true, true],
            vec![true, true, false],
            vec![true, false, true],
            vec![true, false, false],
        ]);

        population.trim(0.01, &mut rng);
        assert_eq!(
            inspect::population(population),
            vec![vec![true, true, true], vec![false, true, false],]
        );
    }
}
