mod support;

#[cfg(test)]
mod population_tests {
    use crate::support::*;
    use genetic_algorithm::fitness::{Fitness, FitnessOrdering, FitnessCountTrue};
    use genetic_algorithm::genotype::BinaryGenotype;

    #[test]
    fn fitness_score_stddev() {
        let population = build::population::<BinaryGenotype>(vec![
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
        let population = FitnessCountTrue.call_for_population(population);
        assert_eq!(population.fitness_score_stddev(), 0.8660254);

        let population = build::population::<BinaryGenotype>(vec![
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
        let population = FitnessCountTrue.call_for_population(population);
        assert_eq!(population.fitness_score_stddev(), 0.3307189);
    }

    #[test]
    fn best_chromosome() {
        let population = build::population_with_fitness_scores::<BinaryGenotype>(vec![
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
}
