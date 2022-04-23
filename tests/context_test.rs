mod support;

#[cfg(test)]
mod context_tests {
    use crate::support::*;
    use genetic_algorithm::context::Context;

    #[test]
    fn test_random_chromosome_factory() {
        let context = Context::new()
            .with_gene_size(10)
            .with_gene_values(vec![true, false])
            .with_population_size(100);

        let mut rng = SmallRng::seed_from_u64(0);
        let chromosome = context.random_chromosome_factory(&mut rng);

        assert_eq!(
            inspect::chromosome(&chromosome),
            vec![false, false, true, false, true, true, true, false, false, true]
        );
    }
}
