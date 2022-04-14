mod support;

#[cfg(test)]
mod mutation_tests {
    use crate::support::*;
    use genetic_algorithm::context::Context;
    use genetic_algorithm::mutation;

    #[test]
    fn test_single_gene_binary() {
        let rng = SmallRng::seed_from_u64(0);
        let mut context = Context::new()
            .with_gene_size(3)
            .with_gene_values(vec![true, false])
            .with_mutation_probability(0.5)
            .with_rng(rng);

        let mut population = build::population(vec![
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, true],
        ]);

        mutation::single_gene(&mut context, &mut population);

        assert_eq!(
            inspect::population(&population),
            vec![
                vec![true, false, true],
                vec![true, true, true],
                vec![true, true, false],
                vec![true, true, true],
            ]
        );
    }

    #[test]
    fn test_single_gene_discrete() {
        let rng = SmallRng::seed_from_u64(0);
        let mut context = Context::new()
            .with_gene_size(3)
            .with_gene_values(vec![0, 1, 2, 3])
            .with_mutation_probability(0.5)
            .with_rng(rng);

        let mut population = build::population(vec![
            vec![0, 0, 0],
            vec![0, 0, 0],
            vec![0, 0, 0],
            vec![0, 0, 0],
        ]);

        mutation::single_gene(&mut context, &mut population);

        assert_eq!(
            inspect::population(&population),
            vec![vec![0, 2, 0], vec![0, 3, 0], vec![0, 0, 0], vec![0, 0, 0],]
        );
    }
}
