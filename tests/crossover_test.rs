mod support;

#[cfg(test)]
mod crossover_tests {
    use crate::support::*;
    use genetic_algorithm::context::Context;
    use genetic_algorithm::crossover;

    #[test]
    fn test_individual_even() {
        let rng = SmallRng::seed_from_u64(0);
        let mut context = Context::new()
            .with_gene_size(3)
            .with_gene_values(vec![true, false])
            .with_population_size(4)
            .with_rng(rng);

        let population = builders::population_from_booleans(vec![
            vec![true, true, true],
            vec![false, false, false],
            vec![true, true, true],
            vec![false, false, false],
        ]);

        let child_population = crossover::individual(&mut context, &population);

        assert_eq!(
            builders::booleans_from_population(child_population),
            vec![
                vec![true, false, true],
                vec![false, true, false],
                vec![true, false, true],
                vec![false, true, false],
            ]
        )
    }

    #[test]
    fn test_individual_odd() {
        let rng = SmallRng::seed_from_u64(0);
        let mut context = Context::new()
            .with_gene_size(3)
            .with_gene_values(vec![true, false])
            .with_population_size(4)
            .with_rng(rng);

        let population = builders::population_from_booleans(vec![
            vec![true, true, true],
            vec![false, false, false],
            vec![true, true, true],
            vec![false, false, false],
            vec![true, true, true],
        ]);

        let child_population = crossover::individual(&mut context, &population);

        assert_eq!(
            builders::booleans_from_population(child_population),
            vec![
                vec![true, false, true],
                vec![false, true, false],
                vec![true, false, true],
                vec![false, true, false],
            ]
        )
    }
}
