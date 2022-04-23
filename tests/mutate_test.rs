mod support;

#[cfg(test)]
mod mutate_tests {
    use crate::support::*;
    use genetic_algorithm::genotype::{BinaryRandomGenotype, DiscreteRandomGenotype};
    use genetic_algorithm::mutate;
    use genetic_algorithm::mutate::Mutate;

    #[test]
    fn test_single_gene_binary() {
        let genotype = BinaryRandomGenotype::new().with_gene_size(3);

        let population = build::population(vec![
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, true],
        ]);

        let mut rng = SmallRng::seed_from_u64(0);
        let population = mutate::SingleGene(0.5).call(&genotype, population, &mut rng);

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
    fn test_single_gene_discrete() {
        let genotype = DiscreteRandomGenotype::new()
            .with_gene_size(3)
            .with_gene_values(vec![0, 1, 2, 3]);

        let population = build::population(vec![
            vec![0, 0, 0],
            vec![0, 0, 0],
            vec![0, 0, 0],
            vec![0, 0, 0],
        ]);

        let mut rng = SmallRng::seed_from_u64(0);
        let population = mutate::SingleGene(0.5).call(&genotype, population, &mut rng);

        assert_eq!(
            inspect::population(&population),
            vec![vec![0, 2, 0], vec![0, 3, 0], vec![1, 0, 0], vec![0, 0, 0],]
        );
    }

    //#[test]
    //fn test_multiple_gene_discrete() {
    //let genotype = DiscreteRandomGenotype::new()
    //.with_gene_size(3)
    //.with_gene_values(vec![0, 1, 2, 3]);

    //let population = build::population(vec![
    //vec![0, 0, 0],
    //vec![0, 0, 0],
    //vec![0, 0, 0],
    //vec![0, 0, 0],
    //]);

    //let mut rng = SmallRng::seed_from_u64(0);
    //let population = mutate::MultipleGene(0.5).call(&genotype, population, &mut rng);

    //assert_eq!(
    //inspect::population(&population),
    //vec![vec![2, 3, 0], vec![0, 1, 0], vec![0, 0, 1], vec![0, 0, 0],]
    //);
    //}

    //#[test]
    //fn test_swap_single_gene_discrete() {
    //let genotype = DiscreteRandomGenotype::new()
    //.with_gene_size(5)
    //.with_gene_values(vec![1, 2, 3, 4, 5]);

    //let population = build::population(vec![
    //vec![1, 2, 3, 4, 5],
    //vec![1, 2, 3, 4, 5],
    //vec![1, 2, 3, 4, 5],
    //vec![1, 2, 3, 4, 5],
    //]);

    //let mut rng = SmallRng::seed_from_u64(0);
    //let population = mutate::SwapSingleGene(0.5).call(&genotype, population, &mut rng);

    //assert_eq!(
    //inspect::population(&population),
    //vec![
    //vec![1, 2, 5, 4, 3],
    //vec![1, 2, 3, 4, 5],
    //vec![1, 2, 3, 4, 5],
    //vec![1, 5, 3, 4, 2],
    //]
    //);
    //}
}
