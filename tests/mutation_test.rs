mod support;

#[cfg(test)]
mod mutation_tests {
    use crate::support::*;
    use genetic_algorithm::context::Context;
    use genetic_algorithm::mutation;

    #[test]
    fn test_single_gene_ensure_mutation() {
        let rng = SmallRng::seed_from_u64(0);
        let mut context = Context::new()
            .with_gene_size(3)
            .with_gene_values(vec![true, false])
            .with_mutation_probability(1.0)
            .with_rng(rng);

        let mut population = builders::population_binary(vec![
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, true],
        ]);

        mutation::single_gene(&mut context, &mut population);

        assert_eq!(
            builders::inspect_population_binary(population),
            vec![
                vec![true, false, true],
                vec![true, false, true],
                vec![true, true, false],
                vec![true, false, true],
            ]
        );
    }

    #[test]
    fn test_single_gene_ensure_no_mutation() {
        let mut context = Context::new()
            .with_gene_size(3)
            .with_gene_values(vec![true, false])
            .with_mutation_probability(0.0);

        let mut population = builders::population_binary(vec![
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, true],
        ]);

        mutation::single_gene(&mut context, &mut population);

        assert_eq!(
            builders::inspect_population_binary(population),
            vec![
                vec![true, true, true],
                vec![true, true, true],
                vec![true, true, true],
                vec![true, true, true],
            ]
        );
    }
}
