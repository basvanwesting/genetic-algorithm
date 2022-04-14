#[cfg(test)]
mod evolve_tests {
    use genetic_algorithm::context::Context;
    use genetic_algorithm::evolve;
    use genetic_algorithm::fitness;

    #[test]
    fn test_call() {
        let mut context = Context::new()
            .with_gene_size(10)
            .with_gene_values(vec![true, false])
            .with_population_size(100)
            .with_fitness_function(fitness::count_true_values)
            .with_tournament_size(4);

        let best_chromosome = evolve::call(&mut context).unwrap();
        println!("{:#?}", best_chromosome);

        assert_eq!(best_chromosome.fitness, Some(10));
        //assert!(false);
    }
}
