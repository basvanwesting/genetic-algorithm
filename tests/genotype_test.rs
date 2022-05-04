mod support;

#[cfg(test)]
mod genotype_tests {

    use crate::support::*;
    use genetic_algorithm::genotype::{
        BinaryGenotype, ContinuousGenotype, DiscreteGenotype, Genotype, IndexGenotype,
        MultiIndexGenotype, PermutableGenotype, UniqueDiscreteGenotype, UniqueIndexGenotype,
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
    fn test_index_genotype() {
        let mut rng = SmallRng::seed_from_u64(0);
        let genotype = IndexGenotype::new()
            .with_gene_size(10)
            .with_gene_value_size(5)
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
    fn test_unique_index_genotype() {
        let mut rng = SmallRng::seed_from_u64(0);
        let genotype = UniqueIndexGenotype::new().with_gene_value_size(5).build();

        let mut chromosome = genotype.chromosome_factory(&mut rng);
        assert_eq!(inspect::chromosome(&chromosome), vec![3, 0, 1, 4, 2]);

        genotype.mutate_chromosome(&mut chromosome, &mut rng);
        assert_eq!(inspect::chromosome(&chromosome), vec![3, 0, 1, 2, 4]);

        assert_eq!(genotype.gene_values(), vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_multi_index_genotype() {
        let mut rng = SmallRng::seed_from_u64(0);
        let genotype = MultiIndexGenotype::new()
            .with_gene_value_sizes(vec![5, 2, 3, 4])
            .build();

        let mut chromosome = genotype.chromosome_factory(&mut rng);
        assert_eq!(inspect::chromosome(&chromosome), vec![2, 0, 2, 1]);

        genotype.mutate_chromosome(&mut chromosome, &mut rng);
        //genotype.mutate_chromosome(&mut chromosome, &mut rng);
        assert_eq!(inspect::chromosome(&chromosome), vec![2, 0, 2, 3]);
    }

    #[test]
    fn test_discrete_genotype() {
        let mut rng = SmallRng::seed_from_u64(0);
        let genotype = DiscreteGenotype::new()
            .with_gene_size(5)
            .with_gene_values(vec![5, 2, 3, 4])
            .build();

        let mut chromosome = genotype.chromosome_factory(&mut rng);
        assert_eq!(inspect::chromosome(&chromosome), vec![2, 2, 4, 2, 4]);

        genotype.mutate_chromosome(&mut chromosome, &mut rng);
        //genotype.mutate_chromosome(&mut chromosome, &mut rng);
        assert_eq!(inspect::chromosome(&chromosome), vec![2, 2, 4, 2, 3]);
    }

    #[test]
    fn test_unique_discrete_genotype() {
        let mut rng = SmallRng::seed_from_u64(0);
        let genotype = UniqueDiscreteGenotype::new()
            .with_gene_values(vec![5, 2, 3, 4])
            .build();

        let mut chromosome = genotype.chromosome_factory(&mut rng);
        assert_eq!(inspect::chromosome(&chromosome), vec![4, 5, 2, 3]);

        genotype.mutate_chromosome(&mut chromosome, &mut rng);
        assert_eq!(inspect::chromosome(&chromosome), vec![4, 5, 3, 2]);

        assert_eq!(genotype.gene_values(), vec![5, 2, 3, 4]);
    }
}
