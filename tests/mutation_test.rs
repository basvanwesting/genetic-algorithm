mod support;

#[cfg(test)]
mod mutation_tests {
    use crate::support::*;
    use genetic_algorithm::context::Context;
    use genetic_algorithm::mutation;

    #[test]
    fn test_single_gene_ensure_mutation() {
        let context = Context::new()
            .with_gene_size(3)
            .with_mutation_probability(1.0);

        let mut population = builders::population_from_booleans(vec![
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, true],
        ]);

        mutation::single_gene(&context, &mut population);

        assert_eq!(helpers::number_of_true_values_in_population(&population), 8);
        assert_eq!(
            helpers::number_of_false_values_in_population(&population),
            4
        );
    }

    #[test]
    fn test_single_gene_ensure_no_mutation() {
        let context = Context::new()
            .with_gene_size(3)
            .with_mutation_probability(0.0);

        let mut population = builders::population_from_booleans(vec![
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, true],
        ]);

        mutation::single_gene(&context, &mut population);

        assert_eq!(
            helpers::number_of_true_values_in_population(&population),
            12
        );
        assert_eq!(
            helpers::number_of_false_values_in_population(&population),
            0
        );
    }
}
