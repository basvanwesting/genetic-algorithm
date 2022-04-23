mod support;

#[cfg(test)]
mod genotype_tests {
    use crate::support::*;
    use genetic_algorithm::genotype::Genotype;

    #[test]
    fn test_random_chromosome_factory() {
        let genotype = Genotype::new()
            .with_gene_size(10)
            .with_gene_values(vec![true, false]);

        let mut rng = SmallRng::seed_from_u64(0);
        let chromosome = genotype.random_chromosome_factory(&mut rng);

        assert_eq!(
            inspect::chromosome(&chromosome),
            vec![false, false, true, false, true, true, true, false, false, true]
        );
    }
}
