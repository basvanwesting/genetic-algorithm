mod support;

#[cfg(test)]
mod context_tests {
    use crate::support::*;
    use genetic_algorithm::context::Context;

    #[test]
    fn test_random_chromosome_factory() {
        let mut context = Context::new()
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

    #[test]
    fn test_random_population_factory() {
        let mut context = Context::new()
            .with_gene_size(4)
            .with_gene_values(vec![true, false])
            .with_population_size(8);

        let mut rng = SmallRng::seed_from_u64(0);
        let population = context.random_population_factory(&mut rng);
        println!("{:#?}", population);

        assert_eq!(
            inspect::population(&population),
            vec![
                vec![false, false, true, false],
                vec![true, true, true, false],
                vec![false, true, false, true],
                vec![true, false, true, false],
                vec![false, false, true, true],
                vec![true, false, false, true],
                vec![false, true, true, false],
                vec![true, false, true, false],
            ]
        )
    }

    #[test]
    fn test_permutation_population_factory_1() {
        let context = Context::new()
            .with_gene_size(1)
            .with_gene_values(vec![true, false]);

        let population = context.permutation_population_factory();
        println!("{:#?}", population);

        assert_eq!(
            inspect::population(&population),
            vec![vec![true], vec![false],]
        )
    }

    #[test]
    fn test_permutation_population_factory_2() {
        let context = Context::new()
            .with_gene_size(2)
            .with_gene_values(vec![true, false]);

        let population = context.permutation_population_factory();
        println!("{:#?}", population);

        assert_eq!(
            inspect::population(&population),
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
        let context = Context::new()
            .with_gene_size(3)
            .with_gene_values(vec![true, false]);

        let population = context.permutation_population_factory();
        println!("{:#?}", population);

        assert_eq!(
            inspect::population(&population),
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
