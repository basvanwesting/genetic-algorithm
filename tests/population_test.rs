mod support;

#[cfg(test)]
mod population_tests {
    use crate::support::*;
    use genetic_algorithm::fitness;
    use genetic_algorithm::fitness::Fitness;

    #[test]
    fn test_fitness_score_stddev() {
        let mut population = build::population(vec![
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
        fitness::SimpleSum.call_for_population(&mut population);
        assert_eq!(population.fitness_score_stddev(), 0.8660254);

        let mut population = build::population(vec![
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
        fitness::SimpleSum.call_for_population(&mut population);
        assert_eq!(population.fitness_score_stddev(), 0.3307189);
    }
}
