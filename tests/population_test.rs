#[cfg(test)]
mod population_tests {
    use genetic_algorithm::context::Context;
    use genetic_algorithm::population::Population;

    #[test]
    fn test_random_factory() {
        let context = Context::new().with_gene_size(16).with_population_size(100);

        let population = Population::random_factory(&context);
        println!("{:#?}", population);

        let number_of_true_values: usize = population
            .chromosomes
            .iter()
            .map(|c| c.genes.iter().filter(|&gene| gene.value).count())
            .sum();
        let number_of_false_values: usize = population
            .chromosomes
            .iter()
            .map(|c| c.genes.iter().filter(|&gene| !gene.value).count())
            .sum();

        assert_eq!(number_of_true_values + number_of_false_values, 16 * 100);
    }
}
