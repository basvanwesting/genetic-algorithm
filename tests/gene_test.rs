#[cfg(test)]
mod gene_tests {
    use genetic_algorithm::context::Context;
    use genetic_algorithm::gene::{BinaryGene, DiscreteGene, Gene};

    #[test]
    fn test_mutate_binary_gene() {
        let context = Context::<BinaryGene>::new();
        let mut gene: BinaryGene = true;
        assert_eq!(gene, true);
        gene.mutate(&context);
        assert_eq!(gene, false);
        gene.mutate(&context);
        assert_eq!(gene, true);
    }

    #[test]
    fn test_mutate_discrete_gene() {
        let context = Context::<DiscreteGene>::new();
        let mut gene: DiscreteGene = 0;
        assert_eq!(gene, 0);
        gene.mutate(&context);
        assert_eq!(gene, 1);
        gene.mutate(&context);
        assert_eq!(gene, 2);
    }
}
