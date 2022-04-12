#[cfg(test)]
mod crossover_tests {
    use genetic_algorithm::chromosome::Chromosome;
    use genetic_algorithm::context::Context;
    use genetic_algorithm::crossover;
    use genetic_algorithm::population::Population;

    #[test]
    fn test_individual_even() {
        let context = Context::new().with_gene_size(3).with_population_size(4);

        let population = Population::new(vec![
            Chromosome::new(vec![true, true, true]),
            Chromosome::new(vec![false, false, false]),
            Chromosome::new(vec![true, true, true]),
            Chromosome::new(vec![false, false, false]),
        ]);

        let child_population = crossover::individual(&context, &population);

        assert_eq!(child_population.chromosomes.len(), 4);
        println!("{:#?}", child_population);

        let number_of_true_values: usize = child_population
            .chromosomes
            .iter()
            .map(|c| c.genes.iter().filter(|&gene| *gene).count())
            .sum();

        assert_eq!(number_of_true_values, 6);
    }

    #[test]
    fn test_individual_odd() {
        let context = Context::new().with_gene_size(3).with_population_size(4);

        let population = Population::new(vec![
            Chromosome::new(vec![true, true, true]),
            Chromosome::new(vec![false, false, false]),
            Chromosome::new(vec![true, true, true]),
            Chromosome::new(vec![false, false, false]),
            Chromosome::new(vec![true, true, true]),
        ]);

        let child_population = crossover::individual(&context, &population);
        assert_eq!(child_population.chromosomes.len(), 4);
    }
}
