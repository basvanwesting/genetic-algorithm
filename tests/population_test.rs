#[cfg(test)]
mod population_tests {
    use genetic_algorithm::context::Context;
    use genetic_algorithm::population::Population;

    #[test]
    fn test_random_factory() {
        let population_size = 100;
        let gene_size = 16;
        let context = Context::new(gene_size, population_size, 4);
        let population = Population::random_factory(&context);
        println!("{:#?}", population);

        let number_of_true_values: usize = population
            .chromosomes
            .iter()
            .map(|c| c.genes.iter().filter(|&gene| *gene).count())
            .sum();
        let number_of_false_values: usize = population
            .chromosomes
            .iter()
            .map(|c| c.genes.iter().filter(|&gene| !*gene).count())
            .sum();

        assert_eq!(
            number_of_true_values + number_of_false_values,
            population_size * gene_size
        );
    }
}
