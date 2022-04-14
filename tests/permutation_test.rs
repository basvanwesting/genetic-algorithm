#[cfg(test)]
mod permutation_tests {
    use genetic_algorithm::context::Context;
    use genetic_algorithm::fitness;
    use genetic_algorithm::permutate;

    #[test]
    fn test_call() {
        let context = Context::new()
            .with_gene_size(5)
            .with_gene_values(vec![true, false])
            .with_fitness_function(fitness::count_true_values);

        let best_chromosome = permutate::call(&context).unwrap();
        println!("{:#?}", best_chromosome);

        assert_eq!(best_chromosome.fitness, Some(5));
        //assert!(false);
    }
}
