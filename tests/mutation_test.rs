#[cfg(test)]
mod mutation_tests {
    use genetic_algorithm::chromosome::Chromosome;
    use genetic_algorithm::context::Context;
    use genetic_algorithm::mutation::mutate_single_gene;
    use genetic_algorithm::population::Population;

    #[test]
    fn test_mutate_single_gene() {
        let context = Context::new(3, 4, 4);
        let mut population = Population::new(vec![
            Chromosome::new(vec![true, true, true]),
            Chromosome::new(vec![true, true, true]),
            Chromosome::new(vec![true, true, true]),
            Chromosome::new(vec![true, true, true]),
        ]);

        let number_of_true_values_pre: usize = population
            .chromosomes
            .iter()
            .map(|c| c.genes.iter().filter(|&gene| *gene).count())
            .sum();
        let number_of_false_values_pre: usize = population
            .chromosomes
            .iter()
            .map(|c| c.genes.iter().filter(|&gene| !*gene).count())
            .sum();

        assert_eq!(number_of_true_values_pre, 12);
        assert_eq!(number_of_false_values_pre, 0);

        mutate_single_gene(&context, &mut population);

        let number_of_true_values_post: usize = population
            .chromosomes
            .iter()
            .map(|c| c.genes.iter().filter(|&gene| *gene).count())
            .sum();
        let number_of_false_values_post: usize = population
            .chromosomes
            .iter()
            .map(|c| c.genes.iter().filter(|&gene| !*gene).count())
            .sum();

        assert_eq!(number_of_true_values_post, 8);
        assert_eq!(number_of_false_values_post, 4);
    }
}
