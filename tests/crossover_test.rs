mod support;

#[cfg(test)]
mod crossover_tests {
    use crate::support::*;
    use genetic_algorithm::crossover::{
        Crossover, CrossoverAll, CrossoverClone, CrossoverSingle, CrossoverRange,
    };
    use genetic_algorithm::genotype::BinaryGenotype;

    #[test]
    fn test_single_even() {
        let genotype = BinaryGenotype::new().with_gene_size(5).build();

        let population = build::population(vec![
            vec![true, true, true, true, true],
            vec![false, false, false, false, false],
            vec![true, true, true, true, true],
            vec![false, false, false, false, false],
        ]);

        let mut rng = SmallRng::seed_from_u64(0);
        let population = CrossoverSingle(false).call(&genotype, population, &mut rng);

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
    fn test_single_odd() {
        let genotype = BinaryGenotype::new().with_gene_size(5).build();

        let population = build::population(vec![
            vec![true, true, true, true, true],
            vec![false, false, false, false, false],
            vec![true, true, true, true, true],
            vec![false, false, false, false, false],
            vec![true, true, true, true, true],
        ]);

        let mut rng = SmallRng::seed_from_u64(0);
        let population = CrossoverSingle(false).call(&genotype, population, &mut rng);

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
        let genotype = BinaryGenotype::new().with_gene_size(6).build();

        let population = build::population(vec![
            vec![true, true, true, true, true],
            vec![false, false, false, false, false],
            vec![true, true, true, true, true],
            vec![false, false, false, false, false],
        ]);

        let mut rng = SmallRng::seed_from_u64(0);
        let population = CrossoverAll(false).call(&genotype, population, &mut rng);

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
        let genotype = BinaryGenotype::new().with_gene_size(3).build();

        let population = build::population(vec![
            vec![true, true, true, true, true],
            vec![false, false, false, false, false],
            vec![true, true, true, true, true],
            vec![false, false, false, false, false],
            vec![true, true, true, true, true],
        ]);

        let mut rng = SmallRng::seed_from_u64(0);
        let population = CrossoverAll(false).call(&genotype, population, &mut rng);

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
        let genotype = BinaryGenotype::new().with_gene_size(6).build();

        let population = build::population(vec![
            vec![true, true, true, true, true],
            vec![false, false, false, false, false],
            vec![true, true, true, true, true],
            vec![false, false, false, false, false],
        ]);

        let mut rng = SmallRng::seed_from_u64(0);
        let population = CrossoverAll(true).call(&genotype, population, &mut rng);

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
        let genotype = BinaryGenotype::new().with_gene_size(6).build();

        let population = build::population(vec![
            vec![true, true, true, true, true],
            vec![false, false, false, false, false],
            vec![true, true, true, true, true],
            vec![false, false, false, false, false],
        ]);

        let mut rng = SmallRng::seed_from_u64(0);
        let population = CrossoverRange(false).call(&genotype, population, &mut rng);

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

    #[test]
    fn test_clone_odd() {
        let genotype = BinaryGenotype::new().with_gene_size(3).build();

        let population = build::population(vec![
            vec![true, true, true],
            vec![false, false, false],
            vec![true, true, true],
        ]);

        let mut rng = SmallRng::seed_from_u64(0);
        let population = CrossoverClone(true).call(&genotype, population, &mut rng);

        assert_eq!(
            inspect::population(&population),
            vec![
                vec![true, true, true],
                vec![false, false, false],
                vec![true, true, true],
                vec![true, true, true],
                vec![false, false, false],
                vec![true, true, true],
            ]
        )
    }
}
