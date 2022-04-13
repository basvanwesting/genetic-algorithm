mod support;

#[cfg(test)]
mod mutation_tests {
    use crate::support::builders::build_population_from_booleans;
    use genetic_algorithm::context::Context;
    use genetic_algorithm::mutation;

    #[test]
    fn test_single_gene_ensure_mutation() {
        let context = Context::new()
            .with_gene_size(3)
            .with_mutation_probability(1.0);

        let mut population = build_population_from_booleans(vec![
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, true],
        ]);

        mutation::single_gene(&context, &mut population);

        let number_of_true_values_post: usize = population
            .chromosomes
            .iter()
            .map(|c| c.genes.iter().filter(|&gene| gene.value).count())
            .sum();
        let number_of_false_values_post: usize = population
            .chromosomes
            .iter()
            .map(|c| c.genes.iter().filter(|&gene| !gene.value).count())
            .sum();

        assert_eq!(number_of_true_values_post, 8);
        assert_eq!(number_of_false_values_post, 4);
    }

    #[test]
    fn test_single_gene_ensure_no_mutation() {
        let context = Context::new()
            .with_gene_size(3)
            .with_mutation_probability(0.0);

        let mut population = build_population_from_booleans(vec![
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, true],
        ]);

        mutation::single_gene(&context, &mut population);

        let number_of_true_values_post: usize = population
            .chromosomes
            .iter()
            .map(|c| c.genes.iter().filter(|&gene| gene.value).count())
            .sum();
        let number_of_false_values_post: usize = population
            .chromosomes
            .iter()
            .map(|c| c.genes.iter().filter(|&gene| !gene.value).count())
            .sum();

        assert_eq!(number_of_true_values_post, 12);
        assert_eq!(number_of_false_values_post, 0);
    }
}
