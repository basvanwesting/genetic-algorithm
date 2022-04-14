mod support;

#[cfg(test)]
mod gene_tests {
    use crate::support::*;
    use genetic_algorithm::context::Context;
    use genetic_algorithm::gene::{BinaryGene, ContinuousGene, DiscreteGene, Gene};

    #[test]
    fn test_random_binary_gene() {
        let rng = SmallRng::seed_from_u64(0);
        let mut context = Context::<BinaryGene>::new().with_rng(rng);

        assert_eq!(BinaryGene::random(&mut context), false);
        assert_eq!(BinaryGene::random(&mut context), false);
        assert_eq!(BinaryGene::random(&mut context), true);
    }

    #[test]
    fn test_random_discrete_gene() {
        let rng = SmallRng::seed_from_u64(0);
        let mut context = Context::new()
            .with_gene_values(vec![3, 4, 5, 6])
            .with_rng(rng);

        assert_eq!(DiscreteGene::random(&mut context), 5);
        assert_eq!(DiscreteGene::random(&mut context), 6);
        assert_eq!(DiscreteGene::random(&mut context), 3);
    }

    #[test]
    fn test_random_continuous_gene() {
        let rng = SmallRng::seed_from_u64(0);
        let mut context = Context::<ContinuousGene>::new().with_rng(rng);

        assert_eq!(ContinuousGene::random(&mut context), 0.447325);
        assert_eq!(ContinuousGene::random(&mut context), 0.43914026);
        assert_eq!(ContinuousGene::random(&mut context), 0.9798802);
    }

    #[test]
    fn test_mutate_binary_gene() {
        let mut context = Context::new().with_gene_values(vec![true, false]);
        let mut gene: BinaryGene = true;
        assert_eq!(gene, true);
        gene.mutate(&mut context);
        assert_eq!(gene, false);
        gene.mutate(&mut context);
        assert_eq!(gene, true);
    }

    #[test]
    fn test_mutate_discrete_gene() {
        let rng = SmallRng::seed_from_u64(0);
        let mut context = Context::new()
            .with_gene_values(vec![3, 4, 5, 6])
            .with_rng(rng);

        let mut gene: DiscreteGene = 3;
        assert_eq!(gene, 3);
        gene.mutate(&mut context);
        assert_eq!(gene, 5);
        gene.mutate(&mut context);
        assert_eq!(gene, 6);
    }

    #[test]
    fn test_mutate_continuous_gene() {
        let rng = SmallRng::seed_from_u64(0);
        let mut context = Context::<ContinuousGene>::new().with_rng(rng);

        let mut gene: ContinuousGene = 0.0;
        assert_eq!(gene, 0.0);
        gene.mutate(&mut context);
        assert_eq!(gene, 0.447325);
        gene.mutate(&mut context);
        assert_eq!(gene, 0.43914026);
    }
}
