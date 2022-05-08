mod support;

#[cfg(test)]
mod permutate_tests {
    use crate::support::*;
    use genetic_algorithm::fitness::{
        FitnessOrdering, FitnessSimpleCount, FitnessSimpleSumIndexGenotype,
        FitnessSimpleSumMultiIndexGenotype,
    };
    use genetic_algorithm::genotype::{
        BinaryGenotype, Genotype, IndexGenotype, MultiIndexGenotype,
    };
    use genetic_algorithm::permutate::Permutate;

    //#[test]
    //build_invalid cannot be tested because invalid doesn't even have a type

    #[test]
    fn call_binary_maximize() {
        let genotype = BinaryGenotype::builder().with_gene_size(5).build().unwrap();

        let permutate = Permutate::builder()
            .with_genotype(genotype)
            .with_fitness(FitnessSimpleCount)
            .build()
            .unwrap()
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
    fn call_binary_minimize() {
        let genotype = BinaryGenotype::builder().with_gene_size(5).build().unwrap();

        let permutate = Permutate::builder()
            .with_genotype(genotype)
            .with_fitness_ordering(FitnessOrdering::Minimize)
            .with_fitness(FitnessSimpleCount)
            .build()
            .unwrap()
            .call();

        let best_chromosome = permutate.best_chromosome.unwrap();
        println!("{:#?}", best_chromosome);

        assert_eq!(best_chromosome.fitness_score, Some(0));
        assert_eq!(
            inspect::chromosome(&best_chromosome),
            vec![false, false, false, false, false]
        );
    }

    #[test]
    fn call_index() {
        let genotype = IndexGenotype::builder()
            .with_gene_size(5)
            .with_gene_value_size(10)
            .build()
            .unwrap();

        let permutate = Permutate::builder()
            .with_genotype(genotype)
            .with_fitness(FitnessSimpleSumIndexGenotype)
            .build()
            .unwrap()
            .call();

        let best_chromosome = permutate.best_chromosome.unwrap();
        println!("{:#?}", best_chromosome);

        assert_eq!(best_chromosome.fitness_score, Some(45));
        assert_eq!(inspect::chromosome(&best_chromosome), vec![9, 9, 9, 9, 9]);
    }

    #[test]
    fn call_multi_index() {
        let genotype = MultiIndexGenotype::builder()
            .with_gene_value_sizes(vec![5, 2, 1, 4])
            .build()
            .unwrap();

        let permutate = Permutate::builder()
            .with_genotype(genotype)
            .with_fitness(FitnessSimpleSumMultiIndexGenotype)
            .build()
            .unwrap()
            .call();

        let best_chromosome = permutate.best_chromosome.unwrap();
        println!("{:#?}", best_chromosome);

        assert_eq!(best_chromosome.fitness_score, Some(8));
        assert_eq!(inspect::chromosome(&best_chromosome), vec![4, 1, 0, 3]);
    }
}
