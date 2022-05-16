mod support;

#[cfg(test)]
mod permutate_tests {
    use crate::support::*;
    use genetic_algorithm::fitness::placeholders::{
        CountTrue, SumDiscreteGenotype, SumMultiDiscreteGenotype,
    };
    use genetic_algorithm::fitness::FitnessOrdering;
    use genetic_algorithm::genotype::{
        BinaryGenotype, DiscreteGenotype, Genotype, MultiDiscreteGenotype,
    };
    use genetic_algorithm::permutate::Permutate;

    //#[test]
    //build_invalid cannot be tested because invalid doesn't even have a type

    #[test]
    fn call_binary_maximize() {
        let genotype = BinaryGenotype::builder().with_gene_size(5).build().unwrap();

        let permutate = Permutate::builder()
            .with_genotype(genotype)
            .with_fitness(CountTrue)
            .call()
            .unwrap();

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
            .with_fitness(CountTrue)
            .call()
            .unwrap();

        let best_chromosome = permutate.best_chromosome.unwrap();
        println!("{:#?}", best_chromosome);

        assert_eq!(best_chromosome.fitness_score, Some(0));
        assert_eq!(
            inspect::chromosome(&best_chromosome),
            vec![false, false, false, false, false]
        );
    }

    #[test]
    fn call_discrete() {
        let genotype = DiscreteGenotype::builder()
            .with_gene_size(5)
            .with_allele_values((0..10).collect())
            .build()
            .unwrap();

        let permutate = Permutate::builder()
            .with_genotype(genotype)
            .with_fitness(SumDiscreteGenotype)
            .call()
            .unwrap();

        let best_chromosome = permutate.best_chromosome.unwrap();
        println!("{:#?}", best_chromosome);

        assert_eq!(best_chromosome.fitness_score, Some(45));
        assert_eq!(inspect::chromosome(&best_chromosome), vec![9, 9, 9, 9, 9]);
    }

    #[test]
    fn call_multi_discrete() {
        let genotype = MultiDiscreteGenotype::builder()
            .with_allele_multi_values(vec![
                vec![0, 1, 2, 3, 4],
                vec![0, 1],
                vec![0],
                vec![0, 1, 2, 3],
            ])
            .build()
            .unwrap();

        let permutate = Permutate::builder()
            .with_genotype(genotype)
            .with_fitness(SumMultiDiscreteGenotype)
            .call()
            .unwrap();

        let best_chromosome = permutate.best_chromosome.unwrap();
        println!("{:#?}", best_chromosome);

        assert_eq!(best_chromosome.fitness_score, Some(8));
        assert_eq!(inspect::chromosome(&best_chromosome), vec![4, 1, 0, 3]);
    }
}
