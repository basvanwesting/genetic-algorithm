#[cfg(test)]
mod population_tests {
    use crate::support::*;
    use approx::assert_relative_eq;
    use genetic_algorithm::centralized::chromosome::ChromosomeManager;
    use genetic_algorithm::centralized::fitness::placeholders::CountStaticTrue;
    use genetic_algorithm::centralized::fitness::{Fitness, FitnessOrdering};
    use genetic_algorithm::centralized::genotype::{Genotype, StaticBinaryGenotype};

    #[test]
    fn fitness_score_stddev() {
        let mut genotype = StaticBinaryGenotype::<3, 20>::builder()
            .with_genes_size(3)
            .build()
            .unwrap();
        genotype.chromosomes_setup();

        let population = &mut static_build::population(
            &mut genotype,
            vec![
                vec![false, true, true],
                vec![false, true, false],
                vec![false, false, true],
                vec![false, false, false],
                vec![true, true, true],
                vec![true, true, false],
                vec![true, false, true],
                vec![true, false, false],
            ],
        );

        assert_eq!(population.fitness_score_stddev(), 0.0);
        CountStaticTrue::<3, 20>::new().call_for_population(population, &genotype, None, None);
        assert_relative_eq!(population.fitness_score_stddev(), 0.866, epsilon = 0.001);

        let population = &mut static_build::population(
            &mut genotype,
            vec![
                vec![true, true, true],
                vec![true, true, true],
                vec![true, true, true],
                vec![true, true, true],
                vec![true, true, true],
                vec![true, true, true],
                vec![true, true, true],
                vec![true, true, false],
            ],
        );

        assert_eq!(population.fitness_score_stddev(), 0.0);
        CountStaticTrue::<3, 20>::new().call_for_population(population, &genotype, None, None);
        assert_relative_eq!(population.fitness_score_stddev(), 0.331, epsilon = 0.001);
    }

    #[test]
    fn best_chromosome() {
        let mut genotype = StaticBinaryGenotype::<3, 20>::builder()
            .with_genes_size(3)
            .build()
            .unwrap();
        genotype.chromosomes_setup();

        let population = static_build::population_with_fitness_scores(
            &mut genotype,
            vec![
                (vec![false, true, true], Some(2)),
                (vec![false, false, false], Some(0)),
                (vec![true, true, true], Some(3)),
                (vec![false, false, true], Some(1)),
                (vec![true, true, false], None),
            ],
        );

        let best_chromosome = population.best_chromosome(FitnessOrdering::Maximize);
        assert_eq!(
            best_chromosome.map_or(Some(99), |c| c.fitness_score()),
            Some(3)
        );
        let best_chromosome = population.best_chromosome(FitnessOrdering::Minimize);
        assert_eq!(
            best_chromosome.map_or(Some(99), |c| c.fitness_score()),
            Some(0)
        );
    }

    #[test]
    fn best_chromosome_index() {
        let mut genotype = StaticBinaryGenotype::<3, 20>::builder()
            .with_genes_size(3)
            .build()
            .unwrap();
        genotype.chromosomes_setup();

        let population = static_build::population_with_fitness_scores(
            &mut genotype,
            vec![
                (vec![false, true, true], Some(2)),
                (vec![false, false, false], Some(0)),
                (vec![true, true, true], Some(3)),
                (vec![false, false, true], Some(1)),
                (vec![true, true, false], None),
            ],
        );

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
    fn best_chromosome_indices_no_fitness() {
        let mut genotype = StaticBinaryGenotype::<3, 20>::builder()
            .with_genes_size(3)
            .build()
            .unwrap();
        genotype.chromosomes_setup();

        let population = static_build::population_with_fitness_scores(
            &mut genotype,
            vec![
                (vec![false, true, true], None),
                (vec![false, false, false], None),
                (vec![true, true, true], None),
                (vec![false, false, true], None),
                (vec![true, true, false], None),
            ],
        );

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
    fn chromosome_indices_all_variants_with_fitness_with_genes_hash() {
        let mut genotype = StaticBinaryGenotype::<3, 20>::builder()
            .with_genes_size(3)
            .with_genes_hashing(true)
            .build()
            .unwrap();
        genotype.chromosomes_setup();

        let mut population = static_build::population_with_fitness_scores(
            &mut genotype,
            vec![
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
            ],
        );

        population.chromosomes.iter_mut().for_each(|chromosome| {
            let genes_hash = genotype.calculate_genes_hash(chromosome);
            chromosome.set_genes_hash(genes_hash);
        });

        // uniqueness
        assert_eq!(population.unique_chromosome_indices(), vec![0, 2, 3, 6, 8]);

        // top N
        assert_eq!(
            population.best_unique_chromosome_indices(2, FitnessOrdering::Maximize),
            vec![0, 3]
        );
        assert_eq!(
            population.best_chromosome_indices(2, FitnessOrdering::Maximize),
            vec![3, 5] // same genes hash
        );
        assert_eq!(
            population.best_chromosome_indices(3, FitnessOrdering::Maximize),
            vec![0, 3, 5] // same genes hash
        );

        // top 1
        assert_eq!(
            population.best_unique_chromosome_indices(1, FitnessOrdering::Maximize),
            vec![3]
        );
        assert_eq!(
            population.best_chromosome_indices(1, FitnessOrdering::Maximize),
            vec![3]
        );

        // top 0
        assert_eq!(
            population.best_unique_chromosome_indices(0, FitnessOrdering::Maximize),
            vec![]
        );
        assert_eq!(
            population.best_chromosome_indices(0, FitnessOrdering::Maximize),
            vec![]
        );

        // top all
        assert_eq!(
            population.best_unique_chromosome_indices(10, FitnessOrdering::Maximize),
            vec![0, 2, 3, 6]
        );
        assert_eq!(
            population.best_chromosome_indices(10, FitnessOrdering::Maximize),
            vec![0, 1, 3, 4, 5, 6, 7] // one less
        );

        // top N
        assert_eq!(
            population.best_unique_chromosome_indices(2, FitnessOrdering::Minimize),
            vec![2, 6]
        );
        assert_eq!(
            population.best_chromosome_indices(2, FitnessOrdering::Minimize),
            vec![2, 4] // same genes hash
        );
        assert_eq!(
            population.best_chromosome_indices(3, FitnessOrdering::Minimize),
            vec![2, 4, 6] // same genes hash
        );

        // top 1
        assert_eq!(
            population.best_unique_chromosome_indices(1, FitnessOrdering::Minimize),
            vec![2]
        );
        assert_eq!(
            population.best_chromosome_indices(1, FitnessOrdering::Minimize),
            vec![2]
        );

        // top 0
        assert_eq!(
            population.best_unique_chromosome_indices(0, FitnessOrdering::Minimize),
            vec![]
        );
        assert_eq!(
            population.best_chromosome_indices(0, FitnessOrdering::Minimize),
            vec![]
        );

        // top all
        assert_eq!(
            population.best_unique_chromosome_indices(10, FitnessOrdering::Minimize),
            vec![0, 2, 3, 6]
        );
        assert_eq!(
            population.best_chromosome_indices(10, FitnessOrdering::Minimize),
            vec![0, 1, 2, 4, 5, 6, 7] // one less
        );
    }

    #[test]
    fn chromosome_indices_all_variants_without_fitness_with_genes_hash() {
        let mut genotype = StaticBinaryGenotype::<3, 20>::builder()
            .with_genes_size(3)
            .with_genes_hashing(true)
            .build()
            .unwrap();
        genotype.chromosomes_setup();

        let mut population = static_build::population_with_fitness_scores(
            &mut genotype,
            vec![
                (vec![false, true, true], None),
                (vec![false, true, true], None),
                (vec![false, false, false], None),
                (vec![true, true, true], None),
                (vec![false, false, false], None),
                (vec![true, true, true], None),
                (vec![false, false, true], None),
                (vec![false, false, true], None),
                (vec![true, true, false], None),
                (vec![true, true, false], None),
            ],
        );

        population.chromosomes.iter_mut().for_each(|chromosome| {
            let genes_hash = genotype.calculate_genes_hash(chromosome);
            chromosome.set_genes_hash(genes_hash);
        });

        // uniqueness
        assert_eq!(population.unique_chromosome_indices(), vec![0, 2, 3, 6, 8]);

        // all empty
        assert_eq!(
            population.best_unique_chromosome_indices(2, FitnessOrdering::Maximize),
            vec![]
        );
        assert_eq!(
            population.best_chromosome_indices(2, FitnessOrdering::Maximize),
            vec![]
        );
    }

    #[test]
    fn chromosome_indices_all_variants_with_fitness_without_genes_hash() {
        let mut genotype = StaticBinaryGenotype::<3, 20>::builder()
            .with_genes_size(3)
            .build()
            .unwrap();
        genotype.chromosomes_setup();

        let population = static_build::population_with_fitness_scores(
            &mut genotype,
            vec![
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
            ],
        );

        // uniqueness
        assert_eq!(population.unique_chromosome_indices(), vec![]);

        // top N
        assert_eq!(
            population.best_unique_chromosome_indices(2, FitnessOrdering::Maximize),
            vec![]
        );
        assert_eq!(
            population.best_chromosome_indices(2, FitnessOrdering::Maximize),
            vec![3, 5] // same genes hash
        );
        assert_eq!(
            population.best_chromosome_indices(3, FitnessOrdering::Maximize),
            vec![0, 3, 5] // same genes hash
        );

        // top 1
        assert_eq!(
            population.best_unique_chromosome_indices(1, FitnessOrdering::Maximize),
            vec![]
        );
        assert_eq!(
            population.best_chromosome_indices(1, FitnessOrdering::Maximize),
            vec![3]
        );

        // top 0
        assert_eq!(
            population.best_unique_chromosome_indices(0, FitnessOrdering::Maximize),
            vec![]
        );
        assert_eq!(
            population.best_chromosome_indices(0, FitnessOrdering::Maximize),
            vec![]
        );

        // top all
        assert_eq!(
            population.best_unique_chromosome_indices(10, FitnessOrdering::Maximize),
            vec![]
        );
        assert_eq!(
            population.best_chromosome_indices(10, FitnessOrdering::Maximize),
            vec![0, 1, 3, 4, 5, 6, 7] // one less
        );

        // top N
        assert_eq!(
            population.best_unique_chromosome_indices(2, FitnessOrdering::Minimize),
            vec![]
        );
        assert_eq!(
            population.best_chromosome_indices(2, FitnessOrdering::Minimize),
            vec![2, 4] // same genes hash
        );
        assert_eq!(
            population.best_chromosome_indices(3, FitnessOrdering::Minimize),
            vec![2, 4, 6] // same genes hash
        );

        // top 1
        assert_eq!(
            population.best_unique_chromosome_indices(1, FitnessOrdering::Minimize),
            vec![]
        );
        assert_eq!(
            population.best_chromosome_indices(1, FitnessOrdering::Minimize),
            vec![2]
        );

        // top 0
        assert_eq!(
            population.best_unique_chromosome_indices(0, FitnessOrdering::Minimize),
            vec![]
        );
        assert_eq!(
            population.best_chromosome_indices(0, FitnessOrdering::Minimize),
            vec![]
        );

        // top all
        assert_eq!(
            population.best_unique_chromosome_indices(10, FitnessOrdering::Minimize),
            vec![]
        );
        assert_eq!(
            population.best_chromosome_indices(10, FitnessOrdering::Minimize),
            vec![0, 1, 2, 4, 5, 6, 7] // one less
        );
    }

    #[test]
    fn fitness_score_cardinality() {
        let mut genotype = StaticBinaryGenotype::<3, 20>::builder()
            .with_genes_size(3)
            .build()
            .unwrap();
        genotype.chromosomes_setup();

        let population = static_build::population(
            &mut genotype,
            vec![
                vec![false, false, false],
                vec![false, false, true],
                vec![false, true, true],
                vec![true, true, true],
                vec![true, true, false],
            ],
        );
        assert_eq!(population.fitness_score_cardinality(), None);

        let mut genotype = StaticBinaryGenotype::<3, 20>::builder()
            .with_genes_size(3)
            .build()
            .unwrap();
        genotype.chromosomes_setup();

        let population = static_build::population_with_fitness_scores(
            &mut genotype,
            vec![
                (vec![false, false, false], Some(0)),
                (vec![false, false, true], Some(2)),
                (vec![false, true, true], Some(2)),
                (vec![true, true, true], Some(3)),
                (vec![true, true, false], None),
            ],
        );

        assert_eq!(population.fitness_score_cardinality(), Some(3));
    }

    #[test]
    fn genes_cardinality() {
        let mut genotype = StaticBinaryGenotype::<3, 20>::builder()
            .with_genes_size(3)
            .with_genes_hashing(true)
            .build()
            .unwrap();
        genotype.chromosomes_setup();

        let population = static_build::population(
            &mut genotype,
            vec![
                vec![false, false, false],
                vec![false, false, true],
                vec![false, true, true],
                vec![true, true, true],
                vec![true, true, false],
                vec![false, false, false],
                vec![false, false, true],
                vec![false, true, true],
            ],
        );

        // With StaticBinary and genes_hashing enabled, genes_hash is automatically set
        assert_eq!(population.genes_cardinality(), Some(5));
    }

    #[test]
    fn parents_and_offspring_size() {
        let mut genotype = StaticBinaryGenotype::<3, 20>::builder()
            .with_genes_size(3)
            .build()
            .unwrap();
        genotype.chromosomes_setup();

        let population = static_build::population_with_age(
            &mut genotype,
            vec![
                (vec![false, false, false], 1),
                (vec![false, false, true], 1),
                (vec![false, true, true], 0),
                (vec![true, true, true], 1),
                (vec![true, true, false], 1),
                (vec![false, false, false], 0),
                (vec![false, false, true], 1),
                (vec![false, true, true], 0),
            ],
        );

        assert_eq!(population.parents_and_offspring_size(), (5, 3));
    }
}
