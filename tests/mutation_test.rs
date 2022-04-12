#[cfg(test)]
mod mutation_tests {
    use genetic_algorithm::chromosome::Chromosome;
    use genetic_algorithm::context::Context;
    use genetic_algorithm::mutation;
    use genetic_algorithm::population::Population;

    #[test]
    fn test_single_gene_ensure_mutation() {
        let context = Context::new()
            .with_gene_size(3)
            .with_mutation_probability(1.0);

        let mut population = Population::new(vec![
            Chromosome::new(vec![true, true, true]),
            Chromosome::new(vec![true, true, true]),
            Chromosome::new(vec![true, true, true]),
            Chromosome::new(vec![true, true, true]),
        ]);

        mutation::single_gene(&context, &mut population);

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

    #[test]
    fn test_single_gene_ensure_no_mutation() {
        let context = Context::new()
            .with_gene_size(3)
            .with_mutation_probability(0.0);

        let mut population = Population::new(vec![
            Chromosome::new(vec![true, true, true]),
            Chromosome::new(vec![true, true, true]),
            Chromosome::new(vec![true, true, true]),
            Chromosome::new(vec![true, true, true]),
        ]);

        mutation::single_gene(&context, &mut population);

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

        assert_eq!(number_of_true_values_post, 12);
        assert_eq!(number_of_false_values_post, 0);
    }
}
