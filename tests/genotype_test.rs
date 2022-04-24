mod support;

#[cfg(test)]
mod genotype_tests {

    use crate::support::*;
    use genetic_algorithm::genotype::{
        BinaryGenotype, ContinuousRandomGenotype, DiscreteRandomGenotype,
        DiscreteUniqueGenotype, Genotype,
    };

    #[test]
    fn test_binary_genotype() {
        let mut rng = SmallRng::seed_from_u64(0);
        let genotype = BinaryGenotype::new().with_gene_size(10);

        let mut chromosome = genotype.chromosome_factory(&mut rng);
        assert_eq!(
            inspect::chromosome(&chromosome),
            vec![false, false, true, false, true, true, true, false, false, true]
        );

        genotype.mutate_chromosome(&mut chromosome, &mut rng);
        assert_eq!(
            inspect::chromosome(&chromosome),
            vec![false, false, false, false, true, true, true, false, false, true]
        );
    }

    #[test]
    fn test_discrete_random_genotype() {
        let mut rng = SmallRng::seed_from_u64(0);
        let genotype = DiscreteRandomGenotype::new()
            .with_gene_size(10)
            .with_gene_values(vec![3, 4, 5, 6]);

        let mut chromosome = genotype.chromosome_factory(&mut rng);
        assert_eq!(
            inspect::chromosome(&chromosome),
            vec![5, 6, 3, 4, 6, 5, 4, 6, 3, 6]
        );

        genotype.mutate_chromosome(&mut chromosome, &mut rng);
        genotype.mutate_chromosome(&mut chromosome, &mut rng);
        assert_eq!(
            inspect::chromosome(&chromosome),
            vec![4, 6, 3, 4, 6, 5, 4, 6, 3, 6]
        );
    }

    #[test]
    fn test_continuous_random_genotype() {
        let mut rng = SmallRng::seed_from_u64(0);
        let genotype = ContinuousRandomGenotype::new().with_gene_size(10);

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
        let genotype = DiscreteUniqueGenotype::new().with_gene_values(vec![2, 3, 4, 5, 6]);

        let mut chromosome = genotype.chromosome_factory(&mut rng);
        assert_eq!(inspect::chromosome(&chromosome), vec![5, 2, 3, 6, 4]);

        genotype.mutate_chromosome(&mut chromosome, &mut rng);
        assert_eq!(inspect::chromosome(&chromosome), vec![6, 2, 3, 5, 4]);
    }
}
