mod support;

#[cfg(test)]
mod gene_tests {
    use crate::support::*;
    use genetic_algorithm::context::Context;
    use genetic_algorithm::gene::{BinaryGene, DiscreteGene, Gene};

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
}
