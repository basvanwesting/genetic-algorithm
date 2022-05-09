mod support;

#[cfg(test)]
mod evolve_tests {
    use crate::support::*;
    use genetic_algorithm::compete::CompeteTournament;
    use genetic_algorithm::crossover::CrossoverSingle;
    use genetic_algorithm::evolve::{Evolve, TryFromEvolveBuilderError};
    use genetic_algorithm::fitness::{
        FitnessOrdering, FitnessSimpleCount, FitnessSimpleSumContinuousGenotype,
        FitnessSimpleSumIndexGenotype, FitnessSimpleSumMultiIndexGenotype,
        FitnessSimpleSumUniqueIndexGenotype,
    };
    use genetic_algorithm::genotype::{
        BinaryGenotype, ContinuousGenotype, Genotype, IndexGenotype, MultiIndexGenotype,
        UniqueIndexGenotype,
    };
    use genetic_algorithm::mutate::MutateOnce;

    #[test]
    fn build_invalid_missing_ending_condition() {
        let genotype = BinaryGenotype::builder()
            .with_gene_size(10)
            .build()
            .unwrap();
        let evolve = Evolve::builder()
            .with_genotype(genotype)
            .with_population_size(100)
            .with_mutate(MutateOnce(0.1))
            .with_fitness(FitnessSimpleCount)
            .with_crossover(CrossoverSingle(true))
            .with_compete(CompeteTournament(4))
            .build();

        assert!(evolve.is_err());
        assert_eq!(
            evolve.err(),
            Some(TryFromEvolveBuilderError(
                "Evolve requires at least a max_stale_generations or target_fitness_score ending condition"
            ))
        );
    }

    #[test]
    fn build_invalid_incompatible_genotype_and_crossover() {
        let genotype = UniqueIndexGenotype::builder()
            .with_gene_value_size(10)
            .build()
            .unwrap();
        let evolve = Evolve::builder()
            .with_genotype(genotype)
            .with_population_size(100)
            .with_max_stale_generations(20)
            .with_mutate(MutateOnce(0.1))
            .with_fitness(FitnessSimpleSumUniqueIndexGenotype)
            .with_crossover(CrossoverSingle(true))
            .with_compete(CompeteTournament(4))
            .build();

        assert!(evolve.is_err());
        assert_eq!(
            evolve.err(),
            Some(TryFromEvolveBuilderError(
                "The provided Crossover strategy does not allow for the provided unique Genotype"
            ))
        );
    }

    #[test]
    fn call_binary_max_stale_generations_maximize() {
        let genotype = BinaryGenotype::builder()
            .with_gene_size(10)
            .build()
            .unwrap();
        let mut rng = SmallRng::seed_from_u64(0);
        let evolve = Evolve::builder()
            .with_genotype(genotype)
            .with_population_size(100)
            .with_max_stale_generations(20)
            .with_mutate(MutateOnce(0.1))
            .with_fitness(FitnessSimpleCount)
            .with_crossover(CrossoverSingle(true))
            .with_compete(CompeteTournament(4))
            .build()
            .unwrap()
            .call(&mut rng);

        let best_chromosome = evolve.best_chromosome.unwrap();
        println!("{:#?}", best_chromosome);

        assert_eq!(best_chromosome.fitness_score, Some(10));
        assert_eq!(
            inspect::chromosome(&best_chromosome),
            vec![true, true, true, true, true, true, true, true, true, true]
        );
    }

    #[test]
    fn call_binary_max_stale_generations_minimize() {
        let genotype = BinaryGenotype::builder()
            .with_gene_size(10)
            .build()
            .unwrap();
        let mut rng = SmallRng::seed_from_u64(0);
        let evolve = Evolve::builder()
            .with_genotype(genotype)
            .with_population_size(100)
            .with_fitness_ordering(FitnessOrdering::Minimize)
            .with_max_stale_generations(20)
            .with_mutate(MutateOnce(0.1))
            .with_fitness(FitnessSimpleCount)
            .with_crossover(CrossoverSingle(true))
            .with_compete(CompeteTournament(4))
            .build()
            .unwrap()
            .call(&mut rng);

        let best_chromosome = evolve.best_chromosome.unwrap();
        println!("{:#?}", best_chromosome);

        assert_eq!(best_chromosome.fitness_score, Some(0));
        assert_eq!(
            inspect::chromosome(&best_chromosome),
            vec![false, false, false, false, false, false, false, false, false, false]
        );
    }

    #[test]
    fn call_binary_target_fitness_score_maximize() {
        let genotype = BinaryGenotype::builder()
            .with_gene_size(10)
            .build()
            .unwrap();
        let mut rng = SmallRng::seed_from_u64(0);
        let evolve = Evolve::builder()
            .with_genotype(genotype)
            .with_population_size(100)
            .with_target_fitness_score(8)
            .with_mutate(MutateOnce(0.1))
            .with_fitness(FitnessSimpleCount)
            .with_crossover(CrossoverSingle(true))
            .with_compete(CompeteTournament(4))
            .build()
            .unwrap()
            .call(&mut rng);

        let best_chromosome = evolve.best_chromosome.unwrap();
        println!("{:#?}", best_chromosome);

        assert_eq!(best_chromosome.fitness_score, Some(9));
        assert_eq!(
            inspect::chromosome(&best_chromosome),
            vec![true, true, true, true, true, true, true, true, true, false]
        );
    }

    #[test]
    fn call_binary_target_fitness_score_minimize() {
        let genotype = BinaryGenotype::builder()
            .with_gene_size(10)
            .build()
            .unwrap();
        let mut rng = SmallRng::seed_from_u64(0);
        let evolve = Evolve::builder()
            .with_genotype(genotype)
            .with_population_size(100)
            .with_fitness_ordering(FitnessOrdering::Minimize)
            .with_target_fitness_score(0)
            .with_mutate(MutateOnce(0.1))
            .with_fitness(FitnessSimpleCount)
            .with_crossover(CrossoverSingle(true))
            .with_compete(CompeteTournament(4))
            .build()
            .unwrap()
            .call(&mut rng);

        let best_chromosome = evolve.best_chromosome.unwrap();
        println!("{:#?}", best_chromosome);

        assert_eq!(best_chromosome.fitness_score, Some(0));
        assert_eq!(
            inspect::chromosome(&best_chromosome),
            vec![false, false, false, false, false, false, false, false, false, false]
        );
    }

    #[test]
    fn call_binary_degeneration_range() {
        let genotype = BinaryGenotype::builder()
            .with_gene_size(10)
            .build()
            .unwrap();
        let mut rng = SmallRng::seed_from_u64(0);
        let evolve = Evolve::builder()
            .with_genotype(genotype)
            .with_population_size(100)
            .with_target_fitness_score(8)
            .with_degeneration_range(0.0001..1.0000)
            .with_mutate(MutateOnce(0.1))
            .with_fitness(FitnessSimpleCount)
            .with_crossover(CrossoverSingle(true))
            .with_compete(CompeteTournament(4))
            .build()
            .unwrap()
            .call(&mut rng);

        let best_chromosome = evolve.best_chromosome.unwrap();
        println!("{:#?}", best_chromosome);

        assert_eq!(best_chromosome.fitness_score, Some(9));
        assert_eq!(
            inspect::chromosome(&best_chromosome),
            vec![true, true, true, true, true, true, true, false, true, true]
        );
    }

    #[test]
    fn call_continuous() {
        let genotype = ContinuousGenotype::builder()
            .with_gene_size(10)
            .with_gene_range(0.0..1.0)
            .build()
            .unwrap();
        let mut rng = SmallRng::seed_from_u64(0);
        let evolve = Evolve::builder()
            .with_genotype(genotype)
            .with_population_size(100)
            .with_max_stale_generations(20)
            .with_mutate(MutateOnce(0.1))
            .with_fitness(FitnessSimpleSumContinuousGenotype)
            .with_crossover(CrossoverSingle(true))
            .with_compete(CompeteTournament(4))
            .build()
            .unwrap()
            .call(&mut rng);

        let best_chromosome = evolve.best_chromosome.unwrap();
        println!("{:#?}", best_chromosome);

        assert_eq!(best_chromosome.fitness_score, Some(9));
        assert_eq!(
            inspect::chromosome(&best_chromosome),
            vec![
                0.9651495, 0.9817951, 0.9798802, 0.82838106, 0.7647406, 0.93074965, 0.87062526,
                0.90698075, 0.9505005, 0.99518645
            ]
        );
    }

    #[test]
    fn call_index() {
        let genotype = IndexGenotype::builder()
            .with_gene_size(10)
            .with_gene_value_size(4)
            .build()
            .unwrap();

        let mut rng = SmallRng::seed_from_u64(0);
        let evolve = Evolve::builder()
            .with_genotype(genotype)
            .with_population_size(100)
            .with_max_stale_generations(20)
            .with_mutate(MutateOnce(0.1))
            .with_fitness(FitnessSimpleSumIndexGenotype)
            .with_crossover(CrossoverSingle(true))
            .with_compete(CompeteTournament(4))
            .build()
            .unwrap()
            .call(&mut rng);

        let best_chromosome = evolve.best_chromosome.unwrap();
        println!("{:#?}", best_chromosome);

        assert_eq!(best_chromosome.fitness_score, Some(30));
        assert_eq!(
            inspect::chromosome(&best_chromosome),
            vec![3, 3, 3, 3, 3, 3, 3, 3, 3, 3]
        );
    }

    #[test]
    fn call_multi_index() {
        let genotype = MultiIndexGenotype::builder()
            .with_gene_value_sizes(vec![5, 2, 1, 4])
            .build()
            .unwrap();
        let mut rng = SmallRng::seed_from_u64(0);
        let evolve = Evolve::builder()
            .with_genotype(genotype)
            .with_population_size(100)
            .with_max_stale_generations(20)
            .with_mutate(MutateOnce(0.1))
            .with_fitness(FitnessSimpleSumMultiIndexGenotype)
            .with_crossover(CrossoverSingle(true))
            .with_compete(CompeteTournament(4))
            .build()
            .unwrap()
            .call(&mut rng);

        let best_chromosome = evolve.best_chromosome.unwrap();
        println!("{:#?}", best_chromosome);

        assert_eq!(best_chromosome.fitness_score, Some(8));
        assert_eq!(inspect::chromosome(&best_chromosome), vec![4, 1, 0, 3]);
    }

    #[test]
    fn population_factory_binary() {
        let genotype = BinaryGenotype::builder().with_gene_size(4).build().unwrap();
        let mut rng = SmallRng::seed_from_u64(0);
        let mut evolve = Evolve::builder()
            .with_genotype(genotype)
            .with_population_size(8)
            .with_max_stale_generations(20)
            .with_mutate(MutateOnce(0.1))
            .with_fitness(FitnessSimpleCount)
            .with_crossover(CrossoverSingle(true))
            .with_compete(CompeteTournament(4))
            .build()
            .unwrap();

        let population = evolve.population_factory(&mut rng);
        println!("{:#?}", population);

        assert_eq!(
            inspect::population(&population),
            vec![
                vec![true, true, false, true],
                vec![false, false, false, true],
                vec![true, false, true, false],
                vec![false, true, false, true],
                vec![true, true, false, false],
                vec![false, true, true, false],
                vec![true, false, false, true],
                vec![false, true, false, true],
            ]
        )
    }
}
