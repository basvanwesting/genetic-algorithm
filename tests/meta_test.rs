mod support;

#[cfg(test)]
mod meta_tests {
    use genetic_algorithm::compete::{CompeteDispatch, Competes};
    use genetic_algorithm::crossover::{CrossoverDispatch, Crossovers};
    use genetic_algorithm::evolve_config::EvolveConfig;
    use genetic_algorithm::fitness::FitnessSimpleCount;
    use genetic_algorithm::genotype::BinaryGenotype;
    use genetic_algorithm::meta::{MetaConfig, MetaPermutate};
    use genetic_algorithm::mutate::{MutateDispatch, Mutates};

    #[test]
    fn general() {
        let rounds = 5;
        let population_sizes = vec![1, 2, 3, 4, 5];
        let max_stale_generations_options = vec![Some(10)];
        let target_fitness_score_options = vec![None];
        let degeneration_range_options = vec![None, Some(0.001..0.995)];
        let mutates = vec![
            MutateDispatch(Mutates::Once, 0.1),
            MutateDispatch(Mutates::Once, 0.2),
        ];
        let crossovers = vec![
            CrossoverDispatch(Crossovers::Single, true),
            CrossoverDispatch(Crossovers::Single, false),
            CrossoverDispatch(Crossovers::Range, true),
        ];
        let competes = vec![
            CompeteDispatch(Competes::Elite, 0),
            CompeteDispatch(Competes::Tournament, 4),
        ];
        let genotype = BinaryGenotype::new().with_gene_size(10).build();
        let fitness = FitnessSimpleCount;

        let evolve_config = EvolveConfig::new()
            .with_genotype(genotype)
            .with_fitness(fitness);

        let config = MetaConfig::new(
            evolve_config,
            rounds,
            population_sizes,
            max_stale_generations_options,
            target_fitness_score_options,
            degeneration_range_options,
            mutates,
            crossovers,
            competes,
        );

        let permutate = MetaPermutate::new(&config).call();

        println!();
        println!("{}", permutate);
    }
}
