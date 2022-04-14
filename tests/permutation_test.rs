#[cfg(test)]
mod permutation_tests {
    use genetic_algorithm::context::Context;
    use genetic_algorithm::gene::BinaryGene;
    use genetic_algorithm::permutate;

    #[test]
    fn test_call() {
        let context = Context::<BinaryGene>::new().with_gene_size(5);

        let best_chromosome = permutate::call(&context).unwrap();
        println!("{:#?}", best_chromosome);

        assert_eq!(best_chromosome.fitness, Some(5));
        //assert!(false);
    }
}
