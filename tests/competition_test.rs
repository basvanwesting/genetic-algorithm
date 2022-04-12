#[cfg(test)]
mod competition_tests {
    use genetic_algorithm::chromosome::Chromosome;
    use genetic_algorithm::competition::tournament;
    use genetic_algorithm::context::Context;
    use genetic_algorithm::population::Population;

    #[test]
    fn test_tournament() {
        let context = Context::new(3, 4, 4);
        let population = Population::new(vec![
            Chromosome::new(vec![false, false, false]),
            Chromosome::new(vec![false, false, true]),
            Chromosome::new(vec![false, true, false]),
            Chromosome::new(vec![false, true, true]),
            Chromosome::new(vec![true, false, false]),
            Chromosome::new(vec![true, false, true]),
            Chromosome::new(vec![true, true, false]),
            Chromosome::new(vec![true, true, true]),
        ]);

        let new_population = tournament(&context, population);

        assert_eq!(new_population.chromosomes.len(), 4);

        let number_of_true_values: usize = new_population
            .chromosomes
            .iter()
            .map(|c| c.genes.iter().filter(|&gene| *gene).count())
            .sum();

        // safe enough value, although not by definition true
        assert!(number_of_true_values >= 8);
        println!("{:#?}", new_population);
    }
}
