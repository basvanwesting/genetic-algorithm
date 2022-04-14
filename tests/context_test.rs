mod support;

#[cfg(test)]
mod context_tests {
    use crate::support::*;
    use genetic_algorithm::context::Context;
    use genetic_algorithm::gene::BinaryGene;

    #[test]
    fn test_random_chromosome_factory() {
        let rng = SmallRng::seed_from_u64(0);
        let mut context = Context::<BinaryGene>::new()
            .with_gene_size(10)
            .with_population_size(100)
            .with_tournament_size(4)
            .with_rng(rng);

        let chromosome = context.random_chromosome_factory();

        assert_eq!(
            builders::booleans_from_chromosome(chromosome),
            vec![false, true, false, true, true, true, false, false, true, true]
        );
    }

    #[test]
    fn test_permutation_population_factory_1() {
        let context = Context::<BinaryGene>::new().with_gene_size(1);

        let population = context.permutation_population_factory();
        println!("{:#?}", population);

        assert_eq!(
            builders::booleans_from_population(population),
            vec![vec![true], vec![false],]
        )
    }

    #[test]
    fn test_permutation_population_factory_2() {
        let context = Context::<BinaryGene>::new().with_gene_size(2);

        let population = context.permutation_population_factory();
        println!("{:#?}", population);

        assert_eq!(
            builders::booleans_from_population(population),
            vec![
                vec![true, true],
                vec![true, false],
                vec![false, true],
                vec![false, false],
            ]
        )
    }

    #[test]
    fn test_permutation_population_factory_3() {
        let context = Context::<BinaryGene>::new().with_gene_size(3);

        let population = context.permutation_population_factory();
        println!("{:#?}", population);

        assert_eq!(
            builders::booleans_from_population(population),
            vec![
                vec![true, true, true],
                vec![true, true, false],
                vec![true, false, true],
                vec![true, false, false],
                vec![false, true, true],
                vec![false, true, false],
                vec![false, false, true],
                vec![false, false, false],
            ]
        )
    }
}
