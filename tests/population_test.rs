mod support;

#[cfg(test)]
mod population_tests {
    use crate::support::*;
    use genetic_algorithm::fitness::placeholders::CountTrue;
    use genetic_algorithm::fitness::{Fitness, FitnessOrdering};
    use genetic_algorithm::genotype::BinaryGenotype;

    #[test]
    fn fitness_score_stddev() {
        let population = &mut build::population::<BinaryGenotype>(vec![
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
        assert_eq!(population.fitness_score_stddev(), 0.8660254);

        let population = &mut build::population::<BinaryGenotype>(vec![
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
        assert_eq!(population.fitness_score_stddev(), 0.3307189);
    }

    #[test]
    fn best_chromosome() {
        let population = &mut build::population_with_fitness_scores::<BinaryGenotype>(vec![
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
    fn fitness_score_prevalence() {
        let population = &mut build::population_with_fitness_scores::<BinaryGenotype>(vec![
            (vec![false, false, false], Some(0)),
            (vec![false, false, true], Some(2)),
            (vec![false, true, true], Some(2)),
            (vec![true, true, true], Some(3)),
            (vec![true, true, false], None),
        ]);

        assert_eq!(population.fitness_score_prevalence(Some(0)), 1);
        assert_eq!(population.fitness_score_prevalence(Some(2)), 2);
        assert_eq!(population.fitness_score_prevalence(None), 1);
    }

    #[test]
    fn fitness_score_uniformity() {
        let population = &mut build::population_with_fitness_scores::<BinaryGenotype>(vec![
            (vec![false, false, false], Some(0)),
            (vec![false, false, true], Some(2)),
            (vec![false, true, true], Some(2)),
            (vec![true, true, true], Some(3)),
            (vec![true, true, false], None),
        ]);

        assert_eq!(population.fitness_score_uniformity(), 0.5);
    }

    #[test]
    fn fitness_score_cardinality() {
        let population = &mut build::population_with_fitness_scores::<BinaryGenotype>(vec![
            (vec![false, false, false], Some(0)),
            (vec![false, false, true], Some(2)),
            (vec![false, true, true], Some(2)),
            (vec![true, true, true], Some(3)),
            (vec![true, true, false], None),
        ]);

        assert_eq!(population.fitness_score_cardinality(), 3);
    }

    #[test]
    fn trim() {
        let mut rng = SmallRng::seed_from_u64(0);
        let population = &mut build::population::<BinaryGenotype>(vec![
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
        let population = &mut build::population::<BinaryGenotype>(vec![
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
