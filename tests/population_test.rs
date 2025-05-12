mod support;

#[cfg(test)]
mod population_tests {
    use crate::support::*;
    use approx::assert_relative_eq;
    use genetic_algorithm::fitness::placeholders::CountTrue;
    use genetic_algorithm::fitness::{Fitness, FitnessOrdering};

    #[test]
    fn fitness_score_stddev() {
        let genotype = BinaryGenotype::builder()
            .with_genes_size(3)
            .build()
            .unwrap();

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
        CountTrue.call_for_population(population, &genotype, None, None);
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
        CountTrue.call_for_population(population, &genotype, None, None);
        assert_relative_eq!(population.fitness_score_stddev(), 0.331, epsilon = 0.001);
    }

    #[test]
    fn best_chromosome() {
        let population: Population<BinaryChromosome> = build::population_with_fitness_scores(vec![
            (vec![false, true, true], Some(2)),
            (vec![false, false, false], Some(0)),
            (vec![true, true, true], Some(3)),
            (vec![false, false, true], Some(1)),
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
    fn best_chromosome_index() {
        let population: Population<BinaryChromosome> = build::population_with_fitness_scores(vec![
            (vec![false, true, true], Some(2)),
            (vec![false, false, false], Some(0)),
            (vec![true, true, true], Some(3)),
            (vec![false, false, true], Some(1)),
            (vec![true, true, false], None),
        ]);

        assert_eq!(
            population.best_chromosome_index(FitnessOrdering::Maximize),
            Some(2)
        );
        assert_eq!(
            population.best_chromosome_index(FitnessOrdering::Minimize),
            Some(1)
        );
    }

    #[test]
    fn best_chromosome_indices() {
        let population: Population<BinaryChromosome> = build::population_with_fitness_scores(vec![
            (vec![false, true, true], Some(2)),
            (vec![false, false, false], Some(0)),
            (vec![true, true, true], Some(3)),
            (vec![false, false, true], Some(1)),
            (vec![true, true, false], None),
        ]);

        assert_eq!(
            population.best_chromosome_indices(2, FitnessOrdering::Maximize),
            vec![0, 2]
        );
        assert_eq!(
            population.best_chromosome_indices(1, FitnessOrdering::Maximize),
            vec![2]
        );
        assert_eq!(
            population.best_chromosome_indices(0, FitnessOrdering::Maximize),
            vec![]
        );
        assert_eq!(
            population.best_chromosome_indices(10, FitnessOrdering::Maximize),
            vec![0, 2, 3]
        );

        assert_eq!(
            population.best_chromosome_indices(2, FitnessOrdering::Minimize),
            vec![1, 3]
        );
        assert_eq!(
            population.best_chromosome_indices(1, FitnessOrdering::Minimize),
            vec![1]
        );
        assert_eq!(
            population.best_chromosome_indices(0, FitnessOrdering::Minimize),
            vec![]
        );
        assert_eq!(
            population.best_chromosome_indices(10, FitnessOrdering::Minimize),
            vec![0, 1, 3]
        );
    }

    #[test]
    fn best_chromosome_indices_no_fitness() {
        let population: Population<BinaryChromosome> = build::population_with_fitness_scores(vec![
            (vec![false, true, true], None),
            (vec![false, false, false], None),
            (vec![true, true, true], None),
            (vec![false, false, true], None),
            (vec![true, true, false], None),
        ]);

        assert_eq!(
            population.best_chromosome_indices(2, FitnessOrdering::Maximize),
            vec![]
        );
        assert_eq!(
            population.best_chromosome_indices(1, FitnessOrdering::Maximize),
            vec![]
        );
        assert_eq!(
            population.best_chromosome_indices(0, FitnessOrdering::Maximize),
            vec![]
        );
        assert_eq!(
            population.best_chromosome_indices(10, FitnessOrdering::Maximize),
            vec![]
        );

        assert_eq!(
            population.best_chromosome_indices(2, FitnessOrdering::Minimize),
            vec![]
        );
        assert_eq!(
            population.best_chromosome_indices(1, FitnessOrdering::Minimize),
            vec![]
        );
        assert_eq!(
            population.best_chromosome_indices(0, FitnessOrdering::Minimize),
            vec![]
        );
        assert_eq!(
            population.best_chromosome_indices(10, FitnessOrdering::Minimize),
            vec![]
        );
    }

    #[test]
    fn unique_chromosome_indices() {
        let genotype = BinaryGenotype::builder()
            .with_genes_size(3)
            .with_genes_hashing(true)
            .build()
            .unwrap();

        let mut population: Population<BinaryChromosome> =
            build::population_with_fitness_scores(vec![
                (vec![false, true, true], Some(2)),
                (vec![false, true, true], Some(2)),
                (vec![false, false, false], Some(0)),
                (vec![true, true, true], Some(3)),
                (vec![false, false, false], Some(0)),
                (vec![true, true, true], Some(3)),
                (vec![false, false, true], Some(1)),
                (vec![false, false, true], Some(1)),
                (vec![true, true, false], None),
                (vec![true, true, false], None),
            ]);

        population.chromosomes.iter_mut().for_each(|chromosome| {
            let genes_hash = genotype.calculate_genes_hash(chromosome);
            chromosome.set_genes_hash(genes_hash);
        });

        assert_eq!(population.unique_chromosome_indices(), vec![0, 2, 3, 6, 8]);
    }

    #[test]
    fn best_unique_chromosome_indices() {
        let genotype = BinaryGenotype::builder()
            .with_genes_size(3)
            .with_genes_hashing(true)
            .build()
            .unwrap();

        let mut population: Population<BinaryChromosome> =
            build::population_with_fitness_scores(vec![
                (vec![false, true, true], Some(2)),
                (vec![false, true, true], Some(2)),
                (vec![false, false, false], Some(0)),
                (vec![true, true, true], Some(3)),
                (vec![false, false, false], Some(0)),
                (vec![true, true, true], Some(3)),
                (vec![false, false, true], Some(1)),
                (vec![false, false, true], Some(1)),
                (vec![true, true, false], None),
                (vec![true, true, false], None),
            ]);

        population.chromosomes.iter_mut().for_each(|chromosome| {
            let genes_hash = genotype.calculate_genes_hash(chromosome);
            chromosome.set_genes_hash(genes_hash);
        });

        assert_eq!(
            population.best_unique_chromosome_indices(2, FitnessOrdering::Maximize),
            vec![0, 3]
        );
        assert_eq!(
            population.best_unique_chromosome_indices(1, FitnessOrdering::Maximize),
            vec![3]
        );
        assert_eq!(
            population.best_unique_chromosome_indices(0, FitnessOrdering::Maximize),
            vec![]
        );
        assert_eq!(
            population.best_unique_chromosome_indices(10, FitnessOrdering::Maximize),
            vec![0, 2, 3, 6]
        );

        assert_eq!(
            population.best_unique_chromosome_indices(2, FitnessOrdering::Minimize),
            vec![2, 6]
        );
        assert_eq!(
            population.best_unique_chromosome_indices(1, FitnessOrdering::Minimize),
            vec![2]
        );
        assert_eq!(
            population.best_unique_chromosome_indices(0, FitnessOrdering::Minimize),
            vec![]
        );
        assert_eq!(
            population.best_unique_chromosome_indices(10, FitnessOrdering::Minimize),
            vec![0, 2, 3, 6]
        );
    }

    #[test]
    fn fitness_score_cardinality() {
        let population: Population<BinaryChromosome> = build::population(vec![
            vec![false, false, false],
            vec![false, false, true],
            vec![false, true, true],
            vec![true, true, true],
            vec![true, true, false],
        ]);
        assert_eq!(population.fitness_score_cardinality(), None);

        let population: Population<BinaryChromosome> = build::population_with_fitness_scores(vec![
            (vec![false, false, false], Some(0)),
            (vec![false, false, true], Some(2)),
            (vec![false, true, true], Some(2)),
            (vec![true, true, true], Some(3)),
            (vec![true, true, false], None),
        ]);

        assert_eq!(population.fitness_score_cardinality(), Some(3));
    }

    #[test]
    fn genes_cardinality() {
        let genotype = BinaryGenotype::builder()
            .with_genes_size(3)
            .with_genes_hashing(true)
            .build()
            .unwrap();

        let mut population: Population<BinaryChromosome> = build::population(vec![
            vec![false, false, false],
            vec![false, false, true],
            vec![false, true, true],
            vec![true, true, true],
            vec![true, true, false],
            vec![false, false, false],
            vec![false, false, true],
            vec![false, true, true],
        ]);

        assert_eq!(population.genes_cardinality(), None);

        population.chromosomes.iter_mut().for_each(|chromosome| {
            let genes_hash = genotype.calculate_genes_hash(chromosome);
            chromosome.reset_state(genes_hash);
        });

        assert_eq!(population.genes_cardinality(), Some(5));
    }
}
