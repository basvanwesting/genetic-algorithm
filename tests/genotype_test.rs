mod support;

#[cfg(test)]
mod genotype_tests {

    use crate::support::*;
    use genetic_algorithm::genotype::{
        BinaryGenotype, ContinuousGenotype, DiscreteGenotype, DiscreteUniqueGenotype, Genotype,
        PermutableGenotype, RangeGenotype, RangeUniqueGenotype,
    };

    #[test]
    fn test_binary_genotype() {
        let mut rng = SmallRng::seed_from_u64(0);
        let genotype = BinaryGenotype::new().with_gene_size(10).build();

        let mut chromosome = genotype.chromosome_factory(&mut rng);
        assert_eq!(
            inspect::chromosome(&chromosome),
            vec![true, true, false, true, false, false, false, true, true, false]
        );

        genotype.mutate_chromosome(&mut chromosome, &mut rng);
        assert_eq!(
            inspect::chromosome(&chromosome),
            vec![true, true, true, true, false, false, false, true, true, false]
        );

        assert_eq!(genotype.gene_values(), vec![true, false]);
    }

    #[test]
    fn test_discrete_genotype_integer() {
        let mut rng = SmallRng::seed_from_u64(0);
        let genotype = DiscreteGenotype::new()
            .with_gene_size(10)
            .with_gene_values(vec![3, 4, 5, 6])
            .build();

        let mut chromosome = genotype.chromosome_factory(&mut rng);
        assert_eq!(
            inspect::chromosome(&chromosome),
            vec![4, 4, 6, 4, 6, 6, 5, 4, 4, 6]
        );

        genotype.mutate_chromosome(&mut chromosome, &mut rng);
        genotype.mutate_chromosome(&mut chromosome, &mut rng);
        assert_eq!(
            inspect::chromosome(&chromosome),
            vec![4, 4, 6, 4, 6, 6, 3, 4, 4, 6]
        );

        assert_eq!(genotype.gene_values(), vec![3, 4, 5, 6]);
    }

    #[test]
    fn test_discrete_genotype_float() {
        let mut rng = SmallRng::seed_from_u64(0);
        let genotype = DiscreteGenotype::new()
            .with_gene_size(10)
            .with_gene_values(vec![0.3, 0.4, 0.5, 0.6])
            .build();

        let mut chromosome = genotype.chromosome_factory(&mut rng);
        assert_eq!(
            inspect::chromosome(&chromosome),
            vec![0.4, 0.4, 0.6, 0.4, 0.6, 0.6, 0.5, 0.4, 0.4, 0.6]
        );

        genotype.mutate_chromosome(&mut chromosome, &mut rng);
        genotype.mutate_chromosome(&mut chromosome, &mut rng);
        assert_eq!(
            inspect::chromosome(&chromosome),
            vec![0.4, 0.4, 0.6, 0.4, 0.6, 0.6, 0.3, 0.4, 0.4, 0.6]
        );

        assert_eq!(genotype.gene_values(), vec![0.3, 0.4, 0.5, 0.6]);
    }

    #[test]
    fn test_continuous_genotype() {
        let mut rng = SmallRng::seed_from_u64(0);
        let genotype = ContinuousGenotype::new().with_gene_size(10).build();

        let mut chromosome = genotype.chromosome_factory(&mut rng);
        assert_eq!(
            inspect::chromosome(&chromosome),
            vec![
                0.447325, 0.43914026, 0.9798802, 0.4621672, 0.897079, 0.9429498, 0.58814746,
                0.45637196, 0.39514416, 0.81885093
            ]
        );

        genotype.mutate_chromosome(&mut chromosome, &mut rng);
        assert_eq!(
            inspect::chromosome(&chromosome),
            vec![
                0.447325, 0.43914026, 0.9763819, 0.4621672, 0.897079, 0.9429498, 0.58814746,
                0.45637196, 0.39514416, 0.81885093
            ]
        );
    }

    #[test]
    fn test_discrete_unique_genotype() {
        let mut rng = SmallRng::seed_from_u64(0);
        let genotype = DiscreteUniqueGenotype::new()
            .with_gene_values(vec![2, 3, 4, 5, 6])
            .build();

        let mut chromosome = genotype.chromosome_factory(&mut rng);
        assert_eq!(inspect::chromosome(&chromosome), vec![5, 2, 3, 6, 4]);

        genotype.mutate_chromosome(&mut chromosome, &mut rng);
        assert_eq!(inspect::chromosome(&chromosome), vec![5, 2, 3, 4, 6]);

        assert_eq!(genotype.gene_values(), vec![2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_range_genotype_discrete() {
        let mut rng = SmallRng::seed_from_u64(0);
        let genotype = RangeGenotype::new()
            .with_gene_size(10)
            .with_gene_range(0..5)
            .build();

        let mut chromosome = genotype.chromosome_factory(&mut rng);
        assert_eq!(
            inspect::chromosome(&chromosome),
            vec![2, 2, 4, 2, 4, 4, 2, 2, 1, 4]
        );

        genotype.mutate_chromosome(&mut chromosome, &mut rng);
        genotype.mutate_chromosome(&mut chromosome, &mut rng);
        assert_eq!(
            inspect::chromosome(&chromosome),
            vec![2, 2, 4, 2, 4, 4, 0, 2, 1, 4]
        );

        assert_eq!(genotype.gene_values(), vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_range_genotype_continuous() {
        let mut rng = SmallRng::seed_from_u64(0);
        let genotype = RangeGenotype::new()
            .with_gene_size(10)
            .with_gene_range(0.0..5.0)
            .build();

        let mut chromosome = genotype.chromosome_factory(&mut rng);
        assert_eq!(
            inspect::chromosome(&chromosome),
            vec![
                2.236625, 2.1957011, 4.899401, 2.3108358, 4.485395, 4.714749, 2.940737, 2.2818594,
                1.9757205, 4.0942545
            ]
        );

        genotype.mutate_chromosome(&mut chromosome, &mut rng);
        assert_eq!(
            inspect::chromosome(&chromosome),
            vec![
                2.236625, 2.1957011, 4.8819094, 2.3108358, 4.485395, 4.714749, 2.940737, 2.2818594,
                1.9757205, 4.0942545
            ]
        );
    }

    #[test]
    fn test_range_unique_genotype() {
        let mut rng = SmallRng::seed_from_u64(0);
        let genotype = RangeUniqueGenotype::new().with_gene_range(2..7).build();

        let mut chromosome = genotype.chromosome_factory(&mut rng);
        assert_eq!(inspect::chromosome(&chromosome), vec![5, 2, 3, 6, 4]);

        genotype.mutate_chromosome(&mut chromosome, &mut rng);
        assert_eq!(inspect::chromosome(&chromosome), vec![5, 2, 3, 4, 6]);

        assert_eq!(genotype.gene_values(), vec![2, 3, 4, 5, 6]);
    }
}
