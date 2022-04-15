mod support;

#[cfg(test)]
mod mutate_tests {
    use crate::support::*;
    use genetic_algorithm::context::Context;
    use genetic_algorithm::mutate;
    use genetic_algorithm::mutate::Mutate;

    #[test]
    fn test_mutate_single_gene_binary() {
        let rng = SmallRng::seed_from_u64(0);
        let mut context = Context::new()
            .with_gene_size(3)
            .with_gene_values(vec![true, false])
            .with_rng(rng);

        let mut population = build::population(vec![
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, true],
        ]);

        mutate::SingleGene(0.5).call(&mut context, &mut population);

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
    fn test_mutate_single_gene_discrete() {
        let rng = SmallRng::seed_from_u64(0);
        let mut context = Context::new()
            .with_gene_size(3)
            .with_gene_values(vec![0, 1, 2, 3])
            .with_rng(rng);

        let mut population = build::population(vec![
            vec![0, 0, 0],
            vec![0, 0, 0],
            vec![0, 0, 0],
            vec![0, 0, 0],
        ]);

        mutate::SingleGene(0.5).call(&mut context, &mut population);

        assert_eq!(
            inspect::population(&population),
            vec![vec![0, 2, 0], vec![0, 3, 0], vec![0, 0, 0], vec![0, 0, 0],]
        );
    }

    #[test]
    fn test_mutate_multiple_gene_discrete() {
        let rng = SmallRng::seed_from_u64(0);
        let mut context = Context::new()
            .with_gene_size(3)
            .with_gene_values(vec![0, 1, 2, 3])
            .with_rng(rng);

        let mut population = build::population(vec![
            vec![0, 0, 0],
            vec![0, 0, 0],
            vec![0, 0, 0],
            vec![0, 0, 0],
        ]);

        mutate::MultipleGene(0.5).call(&mut context, &mut population);

        assert_eq!(
            inspect::population(&population),
            vec![vec![2, 3, 0], vec![0, 1, 0], vec![0, 0, 1], vec![0, 0, 0],]
        );
    }
}
