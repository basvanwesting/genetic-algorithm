mod support;

#[cfg(test)]
mod composite_genotype_tests {

    use genetic_algorithm::composite_genotype::{
        CompositeGenotypeAny, CompositeGenotypeEnum, GenotypeEnum,
    };
    use genetic_algorithm::genotype::{BinaryGenotype, ContinuousGenotype, DiscreteGenotype};

    #[test]
    fn test_composite_genotype_any() {
        let genotype_1 = BinaryGenotype::new().with_gene_size(10).build();
        let genotype_2 = ContinuousGenotype::new().with_gene_size(10).build();
        let genotype_3 = DiscreteGenotype::new()
            .with_gene_size(10)
            .with_gene_values(vec![3, 4, 5, 6])
            .build();

        let composite_genotype = CompositeGenotypeAny {
            genotypes: vec![
                Box::new(genotype_1),
                Box::new(genotype_2),
                Box::new(genotype_3),
            ],
        };

        assert_eq!(composite_genotype.gene_size(), 30);
        //assert!(false);
    }

    #[test]
    fn test_composite_genotype_enum() {
        let genotype_1 = BinaryGenotype::new().with_gene_size(10).build();
        let genotype_2 = ContinuousGenotype::new().with_gene_size(10).build();
        let genotype_3 = DiscreteGenotype::new()
            .with_gene_size(10)
            .with_gene_values(vec![3, 4, 5, 6])
            .build();

        let composite_genotype = CompositeGenotypeEnum {
            genotypes: vec![
                GenotypeEnum::BinaryGenotypeWrapper(genotype_1),
                GenotypeEnum::ContinuousGenotypeWrapper(genotype_2),
                GenotypeEnum::DiscreteGenotypeDiscreteGeneWrapper(genotype_3),
            ],
        };

        assert_eq!(composite_genotype.gene_size(), 30);
        //assert!(false);
    }
}
