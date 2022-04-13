mod support;

#[cfg(test)]
mod context_tests {
    use crate::support::*;
    use genetic_algorithm::context::Context;

    #[test]
    fn test_random_chromosome_factory() {
        let context = Context::new()
            .with_gene_size(10)
            .with_population_size(100)
            .with_tournament_size(4);

        let chromosome = context.random_chromosome_factory();
        println!("{:#?}", chromosome);
        assert_eq!(chromosome.genes.len(), 10);
    }

    #[test]
    fn test_permutation_population_factory_1() {
        let context = Context::new().with_gene_size(1);

        let population = context.permutation_population_factory();
        println!("{:#?}", population);

        let data = builders::booleans_from_population(population);
        assert_eq!(data, vec![vec![true], vec![false],])
    }

    #[test]
    fn test_permutation_population_factory_2() {
        let context = Context::new().with_gene_size(2);

        let population = context.permutation_population_factory();
        println!("{:#?}", population);

        let data = builders::booleans_from_population(population);
        assert_eq!(
            data,
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
        let context = Context::new().with_gene_size(3);

        let population = context.permutation_population_factory();
        println!("{:#?}", population);

        let data = builders::booleans_from_population(population);
        assert_eq!(
            data,
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
