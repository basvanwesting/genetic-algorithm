mod support;

#[cfg(test)]
mod population_tests {
    use crate::support::*;
    use genetic_algorithm::fitness::{Fitness, SimpleSumBinaryGenotype};
    use genetic_algorithm::genotype::BinaryGenotype;

    #[test]
    fn test_fitness_score_stddev() {
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
        let population = SimpleSumBinaryGenotype.call_for_population(population);
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
        let population = SimpleSumBinaryGenotype.call_for_population(population);
        assert_eq!(population.fitness_score_stddev(), 0.3307189);
    }
}
