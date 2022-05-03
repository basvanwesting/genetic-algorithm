mod support;

#[cfg(test)]
mod mutate_tests {
    use crate::support::*;
    use genetic_algorithm::genotype::{BinaryGenotype, IndexGenotype};
    use genetic_algorithm::mutate::{Mutate, MutateOnce};

    #[test]
    fn test_once_binary() {
        let genotype = BinaryGenotype::new().with_gene_size(3).build();

        let population = build::population(vec![
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, true],
        ]);

        let mut rng = SmallRng::seed_from_u64(0);
        let population = MutateOnce(0.5).call(&genotype, population, &mut rng);

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
    fn test_once_index() {
        let genotype = IndexGenotype::new()
            .with_gene_size(3)
            .with_gene_value_size(4)
            .build();

        let population = build::population(vec![
            vec![0, 0, 0],
            vec![0, 0, 0],
            vec![0, 0, 0],
            vec![0, 0, 0],
        ]);

        let mut rng = SmallRng::seed_from_u64(0);
        let population = MutateOnce(0.5).call(&genotype, population, &mut rng);

        assert_eq!(
            inspect::population(&population),
            vec![vec![0, 3, 0], vec![0, 0, 3], vec![0, 0, 0], vec![0, 3, 0],]
        );
    }
}
