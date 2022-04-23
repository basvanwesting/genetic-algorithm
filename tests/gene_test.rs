mod support;

#[cfg(test)]
mod gene_tests {
    use crate::support::*;
    use genetic_algorithm::genotype::Genotype;
    use genetic_algorithm::gene::{BinaryGene, ContinuousGene, DiscreteGene, Gene};

    #[test]
    fn test_random_binary_gene() {
        let genotype = Genotype::<BinaryGene>::new();

        let mut rng = SmallRng::seed_from_u64(0);
        assert_eq!(BinaryGene::random(&genotype, &mut rng), false);
        assert_eq!(BinaryGene::random(&genotype, &mut rng), false);
        assert_eq!(BinaryGene::random(&genotype, &mut rng), true);
    }

    #[test]
    fn test_random_discrete_gene() {
        let genotype = Genotype::new().with_gene_values(vec![3, 4, 5, 6]);

        let mut rng = SmallRng::seed_from_u64(0);
        assert_eq!(DiscreteGene::random(&genotype, &mut rng), 5);
        assert_eq!(DiscreteGene::random(&genotype, &mut rng), 6);
        assert_eq!(DiscreteGene::random(&genotype, &mut rng), 3);
    }

    #[test]
    fn test_random_continuous_gene() {
        let genotype = Genotype::<ContinuousGene>::new();

        let mut rng = SmallRng::seed_from_u64(0);
        assert_eq!(ContinuousGene::random(&genotype, &mut rng), 0.447325);
        assert_eq!(ContinuousGene::random(&genotype, &mut rng), 0.43914026);
        assert_eq!(ContinuousGene::random(&genotype, &mut rng), 0.9798802);
    }

    #[test]
    fn test_mutate_binary_gene() {
        let genotype = Genotype::new().with_gene_values(vec![true, false]);
        let mut rng = SmallRng::seed_from_u64(0);
        let mut gene: BinaryGene = true;
        assert_eq!(gene, true);
        gene.mutate(&genotype, &mut rng);
        assert_eq!(gene, false);
        gene.mutate(&genotype, &mut rng);
        assert_eq!(gene, true);
    }

    #[test]
    fn test_mutate_discrete_gene() {
        let genotype = Genotype::new().with_gene_values(vec![3, 4, 5, 6]);

        let mut rng = SmallRng::seed_from_u64(0);
        let mut gene: DiscreteGene = 3;
        assert_eq!(gene, 3);
        gene.mutate(&genotype, &mut rng);
        assert_eq!(gene, 5);
        gene.mutate(&genotype, &mut rng);
        assert_eq!(gene, 6);
    }

    #[test]
    fn test_mutate_continuous_gene() {
        let genotype = Genotype::<ContinuousGene>::new();

        let mut rng = SmallRng::seed_from_u64(0);
        let mut gene: ContinuousGene = 0.0;
        assert_eq!(gene, 0.0);
        gene.mutate(&genotype, &mut rng);
        assert_eq!(gene, 0.447325);
        gene.mutate(&genotype, &mut rng);
        assert_eq!(gene, 0.43914026);
    }
}
