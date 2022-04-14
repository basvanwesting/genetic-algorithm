mod support;

#[cfg(test)]
mod permutation_tests {
    use crate::support::*;
    use genetic_algorithm::context::Context;
    use genetic_algorithm::fitness;
    use genetic_algorithm::permutate;

    #[test]
    fn test_call_binary() {
        let context = Context::new()
            .with_gene_size(5)
            .with_gene_values(vec![true, false])
            .with_fitness_function(fitness::count_true_values);

        let best_chromosome = permutate::call(&context).unwrap();
        println!("{:#?}", best_chromosome);

        assert_eq!(best_chromosome.fitness, Some(5));
        assert_eq!(
            inspect::chromosome(&best_chromosome),
            vec![true, true, true, true, true]
        );
    }

    #[test]
    fn test_call_discrete() {
        let context = Context::new()
            .with_gene_size(5)
            .with_gene_values(vec![0, 1, 2, 3])
            .with_fitness_function(fitness::sum_discrete_values);

        let best_chromosome = permutate::call(&context).unwrap();
        println!("{:#?}", best_chromosome);

        assert_eq!(best_chromosome.fitness, Some(15));
        assert_eq!(inspect::chromosome(&best_chromosome), vec![3, 3, 3, 3, 3]);
    }

    #[test]
    fn test_call_continuous() {
        let context = Context::new()
            .with_gene_size(5)
            .with_fitness_function(fitness::sum_continuous_values);

        let best_chromosome = permutate::call(&context);
        println!("{:#?}", best_chromosome);

        assert_eq!(best_chromosome, None);
    }
}
