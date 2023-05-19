mod support;

#[cfg(test)]
mod meta_tests {
    use genetic_algorithm::compete::{CompeteDispatch, Competes};
    use genetic_algorithm::crossover::{CrossoverDispatch, Crossovers};
    use genetic_algorithm::extension::{
        ExtensionMassDegeneration, ExtensionMassExtinction, ExtensionMassGenesis,
        ExtensionMassInvasion, ExtensionNoop,
    };
    use genetic_algorithm::fitness::placeholders::CountTrue;
    use genetic_algorithm::genotype::{BinaryGenotype, Genotype};
    use genetic_algorithm::meta::{MetaConfig, MetaPermutate};
    use genetic_algorithm::mutate::{MutateDynamicOnce, MutateDynamicRounds, MutateOnce};
    use genetic_algorithm::strategy::evolve::EvolveBuilder;
    use genetic_algorithm::strategy::Strategy;

    #[test]
    fn general() {
        let rounds = 5;
        let population_sizes = vec![1, 2, 3, 4, 5];
        let max_stale_generations_options = vec![Some(10)];
        let mutates = vec![
            MutateOnce::new_dispatch(0.2),
            MutateDynamicOnce::new_dispatch(0.2, 0.25),
            MutateDynamicRounds::new_dispatch(0.1, 0.25),
        ];
        let crossovers = vec![
            CrossoverDispatch(Crossovers::SingleGene, true),
            CrossoverDispatch(Crossovers::SingleGene, false),
            CrossoverDispatch(Crossovers::SinglePoint, true),
        ];
        let competes = vec![
            CompeteDispatch(Competes::Elite, 0),
            CompeteDispatch(Competes::Tournament, 4),
        ];
        let extensions = vec![
            ExtensionNoop::new_dispatch(),
            ExtensionMassDegeneration::new_dispatch(0.9, 10),
            ExtensionMassExtinction::new_dispatch(0.9, 0.1),
            ExtensionMassGenesis::new_dispatch(0.9),
            ExtensionMassInvasion::new_dispatch(0.9, 0.1),
        ];
        let genotype = BinaryGenotype::builder()
            .with_genes_size(10)
            .build()
            .unwrap();
        let fitness = CountTrue;

        let evolve_builder = EvolveBuilder::new()
            .with_genotype(genotype)
            .with_fitness(fitness);
        let evolve_fitness_to_micro_second_factor = 1_000_000;

        let config = MetaConfig::builder()
            .with_evolve_builder(evolve_builder)
            .with_evolve_fitness_to_micro_second_factor(evolve_fitness_to_micro_second_factor)
            .with_rounds(rounds)
            .with_population_sizes(population_sizes)
            .with_max_stale_generations_options(max_stale_generations_options)
            .with_mutates(mutates)
            .with_crossovers(crossovers)
            .with_competes(competes)
            .with_extensions(extensions)
            .build()
            .unwrap();

        let permutate = MetaPermutate::new(&config).call();

        println!();
        println!("{}", permutate);

        assert!(permutate.inner_permutate.is_some());
        assert!(permutate
            .inner_permutate
            .unwrap()
            .best_chromosome()
            .is_some());
    }
}
