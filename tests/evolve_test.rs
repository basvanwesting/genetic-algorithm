mod support;

#[cfg(test)]
mod evolve_tests {
    use crate::support::*;
    use genetic_algorithm::context::Context;
    use genetic_algorithm::evolve::Evolve;
    use genetic_algorithm::fitness;
    use genetic_algorithm::gene::ContinuousGene;
    use genetic_algorithm::mutate;

    #[test]
    fn test_call_binary() {
        let rng = SmallRng::seed_from_u64(0);
        let context = Context::new()
            .with_gene_size(10)
            .with_gene_values(vec![true, false])
            .with_population_size(100)
            .with_tournament_size(4)
            .with_rng(rng);

        let evolve = Evolve::new(context, mutate::SingleGene, fitness::SimpleSum).call();
        let best_chromosome = evolve.best_chromosome.unwrap();
        println!("{:#?}", best_chromosome);

        assert_eq!(best_chromosome.fitness, Some(10));
        assert_eq!(
            inspect::chromosome(&best_chromosome),
            vec![true, true, true, true, true, true, true, true, true, true]
        );
    }

    #[test]
    fn test_call_discrete() {
        let rng = SmallRng::seed_from_u64(0);
        let context = Context::new()
            .with_gene_size(10)
            .with_gene_values(vec![0, 1, 2, 3])
            .with_population_size(100)
            .with_tournament_size(4)
            .with_rng(rng);

        let evolve = Evolve::new(context, mutate::SingleGene, fitness::SimpleSum).call();
        let best_chromosome = evolve.best_chromosome.unwrap();
        println!("{:#?}", best_chromosome);

        assert_eq!(best_chromosome.fitness, Some(30));
        assert_eq!(
            inspect::chromosome(&best_chromosome),
            vec![3, 3, 3, 3, 3, 3, 3, 3, 3, 3]
        );
    }

    #[test]
    fn test_call_continuous() {
        let rng = SmallRng::seed_from_u64(0);
        let context = Context::<ContinuousGene>::new()
            .with_gene_size(10)
            .with_population_size(100)
            .with_tournament_size(4)
            .with_rng(rng);

        let evolve = Evolve::new(context, mutate::SingleGene, fitness::SimpleSum).call();
        let best_chromosome = evolve.best_chromosome.unwrap();
        println!("{:#?}", best_chromosome);

        assert_eq!(best_chromosome.fitness, Some(9));
        assert_eq!(
            inspect::chromosome(&best_chromosome),
            vec![
                0.9989109, 0.97775495, 0.8780951, 0.8283811, 0.9203737, 0.9307497, 0.9379812,
                0.96504027, 0.6531458, 0.9249813
            ]
        );
    }
}
