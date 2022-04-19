mod support;

#[cfg(test)]
mod crossover_tests {
    use crate::support::*;
    use genetic_algorithm::context::Context;
    use genetic_algorithm::crossover;
    use genetic_algorithm::crossover::Crossover;

    #[test]
    fn test_individual_even() {
        let rng = SmallRng::seed_from_u64(0);
        let mut context = Context::new()
            .with_gene_size(5)
            .with_gene_values(vec![true, false])
            .with_population_size(4)
            .with_rng(rng);

        let population = build::population(vec![
            vec![true, true, true, true, true],
            vec![false, false, false, false, false],
            vec![true, true, true, true, true],
            vec![false, false, false, false, false],
        ]);

        let population = crossover::Individual(false).call(&mut context, population);

        assert_eq!(
            inspect::population(&population),
            vec![
                vec![true, true, false, true, true],
                vec![false, false, true, false, false],
                vec![true, true, false, true, true],
                vec![false, false, true, false, false],
            ]
        )
    }

    #[test]
    fn test_individual_odd() {
        let rng = SmallRng::seed_from_u64(0);
        let mut context = Context::new()
            .with_gene_size(5)
            .with_gene_values(vec![true, false])
            .with_population_size(4)
            .with_rng(rng);

        let population = build::population(vec![
            vec![true, true, true, true, true],
            vec![false, false, false, false, false],
            vec![true, true, true, true, true],
            vec![false, false, false, false, false],
            vec![true, true, true, true, true],
        ]);

        let population = crossover::Individual(false).call(&mut context, population);

        assert_eq!(
            inspect::population(&population),
            vec![
                vec![true, true, false, true, true],
                vec![false, false, true, false, false],
                vec![true, true, false, true, true],
                vec![false, false, true, false, false],
            ]
        )
    }

    #[test]
    fn test_all_even() {
        let rng = SmallRng::seed_from_u64(0);
        let mut context = Context::new()
            .with_gene_size(6)
            .with_gene_values(vec![true, false])
            .with_population_size(4)
            .with_rng(rng);

        let population = build::population(vec![
            vec![true, true, true, true, true],
            vec![false, false, false, false, false],
            vec![true, true, true, true, true],
            vec![false, false, false, false, false],
        ]);

        let population = crossover::All(false).call(&mut context, population);

        assert_eq!(
            inspect::population(&population),
            vec![
                vec![false, false, true, false, true],
                vec![true, true, false, true, false],
                vec![true, false, false, true, false],
                vec![false, true, true, false, true],
            ]
        )
    }

    #[test]
    fn test_all_odd() {
        let rng = SmallRng::seed_from_u64(0);
        let mut context = Context::new()
            .with_gene_size(3)
            .with_gene_values(vec![true, false])
            .with_population_size(4)
            .with_rng(rng);

        let population = build::population(vec![
            vec![true, true, true, true, true],
            vec![false, false, false, false, false],
            vec![true, true, true, true, true],
            vec![false, false, false, false, false],
            vec![true, true, true, true, true],
        ]);

        let population = crossover::All(false).call(&mut context, population);

        assert_eq!(
            inspect::population(&population),
            vec![
                vec![false, false, true, true, true],
                vec![true, true, false, false, false],
                vec![false, true, true, true, true],
                vec![true, false, false, false, false],
            ]
        )
    }

    #[test]
    fn test_all_even_keep_parent() {
        let rng = SmallRng::seed_from_u64(0);
        let mut context = Context::new()
            .with_gene_size(6)
            .with_gene_values(vec![true, false])
            .with_population_size(4)
            .with_rng(rng);

        let population = build::population(vec![
            vec![true, true, true, true, true],
            vec![false, false, false, false, false],
            vec![true, true, true, true, true],
            vec![false, false, false, false, false],
        ]);

        let population = crossover::All(true).call(&mut context, population);

        assert_eq!(
            inspect::population(&population),
            vec![
                vec![false, false, true, false, true],
                vec![true, true, false, true, false],
                vec![true, false, false, true, false],
                vec![false, true, true, false, true],
                vec![true, true, true, true, true],
                vec![false, false, false, false, false],
                vec![true, true, true, true, true],
                vec![false, false, false, false, false],
            ]
        )
    }

    #[test]
    fn test_range_even() {
        let rng = SmallRng::seed_from_u64(0);
        let mut context = Context::new()
            .with_gene_size(6)
            .with_gene_values(vec![true, false])
            .with_population_size(4)
            .with_rng(rng);

        let population = build::population(vec![
            vec![true, true, true, true, true],
            vec![false, false, false, false, false],
            vec![true, true, true, true, true],
            vec![false, false, false, false, false],
        ]);

        let population = crossover::Range(false).call(&mut context, population);

        assert_eq!(
            inspect::population(&population),
            vec![
                vec![true, true, false, false, false],
                vec![false, false, true, true, true],
                vec![true, true, false, false, false],
                vec![false, false, true, true, true],
            ]
        )
    }
}
