mod support;

#[cfg(test)]
mod permutate_tests {
    use crate::support::*;
    use genetic_algorithm::fitness;
    use genetic_algorithm::genotype::{
        BinaryGenotype, ContinuousGenotype, DiscreteRandomGenotype,
    };
    use genetic_algorithm::permutate::Permutate;

    #[test]
    fn test_call_binary() {
        let genotype = BinaryGenotype::new().with_gene_size(5);

        let permutate = Permutate::new(genotype)
            .with_fitness(fitness::SimpleSum)
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
    fn test_call_discrete() {
        let genotype = DiscreteRandomGenotype::new()
            .with_gene_size(5)
            .with_gene_values(vec![0, 1, 2, 3]);

        let permutate = Permutate::new(genotype)
            .with_fitness(fitness::SimpleSum)
            .call();

        let best_chromosome = permutate.best_chromosome.unwrap();
        println!("{:#?}", best_chromosome);

        assert_eq!(best_chromosome.fitness_score, Some(15));
        assert_eq!(inspect::chromosome(&best_chromosome), vec![3, 3, 3, 3, 3]);
    }

    #[test]
    fn test_call_continuous() {
        let genotype = ContinuousGenotype::new().with_gene_size(5);

        let permutate = Permutate::new(genotype)
            .with_fitness(fitness::SimpleSum)
            .call();

        let best_chromosome = permutate.best_chromosome;
        println!("{:#?}", best_chromosome);

        assert_eq!(best_chromosome, None);
    }

    #[test]
    fn test_population_factory_1() {
        let genotype = BinaryGenotype::new().with_gene_size(1);

        let permutate = Permutate::new(genotype).with_fitness(fitness::SimpleSum);
        let population = permutate.population_factory();
        println!("{:#?}", population);

        assert_eq!(
            inspect::population(&population),
            vec![vec![true], vec![false],]
        )
    }

    #[test]
    fn test_population_factory_2() {
        let genotype = BinaryGenotype::new().with_gene_size(2);

        let permutate = Permutate::new(genotype).with_fitness(fitness::SimpleSum);
        let population = permutate.population_factory();
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
        let genotype = BinaryGenotype::new().with_gene_size(3);

        let permutate = Permutate::new(genotype).with_fitness(fitness::SimpleSum);
        let population = permutate.population_factory();
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
}
