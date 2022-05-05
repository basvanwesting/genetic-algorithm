mod support;

#[cfg(test)]
mod permutate_tests {
    use crate::support::*;
    use genetic_algorithm::fitness::{
        FitnessSimpleCount, FitnessSimpleSumIndexGenotype, FitnessSimpleSumMultiIndexGenotype,
    };
    use genetic_algorithm::genotype::{BinaryGenotype, IndexGenotype, MultiIndexGenotype};
    use genetic_algorithm::permutate::Permutate;

    #[test]
    fn call_binary() {
        let genotype = BinaryGenotype::new().with_gene_size(5).build();

        let permutate = Permutate::new(genotype)
            .with_fitness(FitnessSimpleCount)
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
    fn call_index() {
        let genotype = IndexGenotype::new()
            .with_gene_size(5)
            .with_gene_value_size(10)
            .build();

        let permutate = Permutate::new(genotype)
            .with_fitness(FitnessSimpleSumIndexGenotype)
            .call();

        let best_chromosome = permutate.best_chromosome.unwrap();
        println!("{:#?}", best_chromosome);

        assert_eq!(best_chromosome.fitness_score, Some(45));
        assert_eq!(inspect::chromosome(&best_chromosome), vec![9, 9, 9, 9, 9]);
    }

    #[test]
    fn call_multi_index() {
        let genotype = MultiIndexGenotype::new()
            .with_gene_value_sizes(vec![5, 2, 1, 4])
            .build();

        let permutate = Permutate::new(genotype)
            .with_fitness(FitnessSimpleSumMultiIndexGenotype)
            .call();

        let best_chromosome = permutate.best_chromosome.unwrap();
        println!("{:#?}", best_chromosome);

        assert_eq!(best_chromosome.fitness_score, Some(8));
        assert_eq!(inspect::chromosome(&best_chromosome), vec![4, 1, 0, 3]);
    }
}
