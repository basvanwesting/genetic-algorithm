mod support;

#[cfg(test)]
mod permutate_tests {
    use crate::support::*;
    use genetic_algorithm::fitness;
    use genetic_algorithm::genotype::{
        BinaryGenotype, IndexGenotype, MultiIndexGenotype, PermutableGenotype,
    };
    use genetic_algorithm::permutate::Permutate;

    #[test]
    fn test_call_binary() {
        let genotype = BinaryGenotype::new().with_gene_size(5).build();

        let permutate = Permutate::new(genotype)
            .with_fitness(fitness::SimpleSumBinaryGenotype)
            .call();

        let best_chromosome = permutate.best_chromosome.unwrap();
        println!("{:#?}", best_chromosome);

        assert_eq!(best_chromosome.fitness_score, Some(5));
        assert_eq!(
            inspect::chromosome(&best_chromosome),
            vec![true, true, true, true, true]
        );
    }

    #[test]
    fn test_call_index() {
        let genotype = IndexGenotype::new()
            .with_gene_size(5)
            .with_gene_value_size(10)
            .build();

        let permutate = Permutate::new(genotype)
            .with_fitness(fitness::SimpleSumIndexGenotype)
            .call();

        let best_chromosome = permutate.best_chromosome.unwrap();
        println!("{:#?}", best_chromosome);

        assert_eq!(best_chromosome.fitness_score, Some(45));
        assert_eq!(inspect::chromosome(&best_chromosome), vec![9, 9, 9, 9, 9]);
    }

    #[test]
    fn test_call_multi_index() {
        let genotype = MultiIndexGenotype::new()
            .with_gene_value_sizes(vec![5, 2, 1, 4])
            .build();

        let permutate = Permutate::new(genotype)
            .with_fitness(fitness::SimpleSumMultiIndexGenotype)
            .call();

        let best_chromosome = permutate.best_chromosome.unwrap();
        println!("{:#?}", best_chromosome);

        assert_eq!(best_chromosome.fitness_score, Some(8));
        assert_eq!(inspect::chromosome(&best_chromosome), vec![4, 1, 0, 3]);
    }

    #[test]
    fn test_population_factory_1() {
        let genotype = BinaryGenotype::new().with_gene_size(1).build();
        let population = genotype.population_factory();
        println!("{:#?}", population);

        assert_eq!(
            inspect::population(&population),
            vec![vec![true], vec![false],]
        )
    }

    #[test]
    fn test_population_factory_2() {
        let genotype = BinaryGenotype::new().with_gene_size(2).build();
        let population = genotype.population_factory();
        println!("{:#?}", population);

        assert_eq!(
            inspect::population(&population),
            vec![
                vec![true, true],
                vec![true, false],
                vec![false, true],
                vec![false, false],
            ]
        )
    }

    #[test]
    fn test_population_factory_3() {
        let genotype = BinaryGenotype::new().with_gene_size(3).build();
        let population = genotype.population_factory();
        println!("{:#?}", population);

        assert_eq!(
            inspect::population(&population),
            vec![
                vec![true, true, true],
                vec![true, true, false],
                vec![true, false, true],
                vec![true, false, false],
                vec![false, true, true],
                vec![false, true, false],
                vec![false, false, true],
                vec![false, false, false],
            ]
        )
    }

    #[test]
    fn test_population_factory_multi_1() {
        let genotype = MultiIndexGenotype::new()
            .with_gene_value_sizes(vec![1])
            .build();

        assert_eq!(
            inspect::population(&genotype.population_factory()),
            vec![vec![0]]
        );
    }

    #[test]
    fn test_population_factory_multi_4() {
        let genotype = MultiIndexGenotype::new()
            .with_gene_value_sizes(vec![1, 2, 3, 4])
            .build();

        assert_eq!(
            inspect::population(&genotype.population_factory()),
            vec![
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 1],
                vec![0, 0, 0, 2],
                vec![0, 0, 0, 3],
                vec![0, 0, 1, 0],
                vec![0, 0, 1, 1],
                vec![0, 0, 1, 2],
                vec![0, 0, 1, 3],
                vec![0, 0, 2, 0],
                vec![0, 0, 2, 1],
                vec![0, 0, 2, 2],
                vec![0, 0, 2, 3],
                vec![0, 1, 0, 0],
                vec![0, 1, 0, 1],
                vec![0, 1, 0, 2],
                vec![0, 1, 0, 3],
                vec![0, 1, 1, 0],
                vec![0, 1, 1, 1],
                vec![0, 1, 1, 2],
                vec![0, 1, 1, 3],
                vec![0, 1, 2, 0],
                vec![0, 1, 2, 1],
                vec![0, 1, 2, 2],
                vec![0, 1, 2, 3],
            ]
        );
    }
}
