mod support;

#[cfg(test)]
mod population_tests {
    use crate::support::*;
    use genetic_algorithm::context::Context;
    use genetic_algorithm::gene::BinaryGene;

    #[test]
    fn test_random_factory() {
        let rng = SmallRng::seed_from_u64(0);
        let mut context = Context::<BinaryGene>::new()
            .with_gene_size(4)
            .with_population_size(8)
            .with_rng(rng);

        let population = context.random_population_factory();
        println!("{:#?}", population);

        assert_eq!(
            builders::booleans_from_population(population),
            vec![
                vec![false, true, false, true],
                vec![true, true, false, false],
                vec![true, true, true, false],
                vec![true, false, true, false],
                vec![false, true, true, true],
                vec![true, false, false, true],
                vec![true, true, false, false],
                vec![false, true, true, false],
            ]
        )
    }
}
