mod support;

#[cfg(test)]
mod population_tests {
    use crate::support::*;
    use genetic_algorithm::chromosome::Chromosome;
    use genetic_algorithm::fitness::{Fitness, FitnessOrdering, FitnessSimpleCount};
    use genetic_algorithm::genotype::BinaryGenotype;
    use genetic_algorithm::population::Population;

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
        let population = FitnessSimpleCount.call_for_population(population);
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
        let population = FitnessSimpleCount.call_for_population(population);
        assert_eq!(population.fitness_score_stddev(), 0.3307189);
    }

    #[test]
    fn best_chromosome() {
        let population = Population::new(vec![
            Chromosome::<BinaryGenotype> {
                genes: vec![false, false, false],
                fitness_score: Some(0),
            },
            Chromosome::<BinaryGenotype> {
                genes: vec![false, false, true],
                fitness_score: Some(1),
            },
            Chromosome::<BinaryGenotype> {
                genes: vec![false, true, true],
                fitness_score: Some(2),
            },
            Chromosome::<BinaryGenotype> {
                genes: vec![true, true, true],
                fitness_score: Some(3),
            },
            Chromosome::<BinaryGenotype> {
                genes: vec![true, true, false],
                fitness_score: None,
            },
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
    fn sort() {
        let mut population = Population::new(vec![
            Chromosome::<BinaryGenotype> {
                genes: vec![false, false, false],
                fitness_score: Some(0),
            },
            Chromosome::<BinaryGenotype> {
                genes: vec![false, false, true],
                fitness_score: Some(1),
            },
            Chromosome::<BinaryGenotype> {
                genes: vec![false, true, true],
                fitness_score: Some(2),
            },
            Chromosome::<BinaryGenotype> {
                genes: vec![true, true, true],
                fitness_score: Some(3),
            },
            Chromosome::<BinaryGenotype> {
                genes: vec![true, true, false],
                fitness_score: None,
            },
        ]);

        population.sort(FitnessOrdering::Maximize);
        assert_eq!(
            inspect::population(&population),
            vec![
                vec![true, true, false],
                vec![false, false, false],
                vec![false, false, true],
                vec![false, true, true],
                vec![true, true, true],
            ]
        );

        population.sort(FitnessOrdering::Minimize);
        assert_eq!(
            inspect::population(&population),
            vec![
                vec![true, true, false],
                vec![true, true, true],
                vec![false, true, true],
                vec![false, false, true],
                vec![false, false, false],
            ]
        );
    }
}
