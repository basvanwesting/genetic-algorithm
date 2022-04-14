mod support;

#[cfg(test)]
mod competition_tests {
    use crate::support::*;
    use genetic_algorithm::competition;
    use genetic_algorithm::context::Context;
    use genetic_algorithm::gene::BinaryGene;

    #[test]
    fn test_tournament() {
        let context = Context::<BinaryGene>::new()
            .with_gene_size(3)
            .with_population_size(4)
            .with_tournament_size(4);

        let mut population = builders::population_from_booleans(vec![
            vec![false, false, false],
            vec![false, false, true],
            vec![false, true, false],
            vec![false, true, true],
            vec![true, false, false],
            vec![true, false, true],
            vec![true, true, false],
            vec![true, true, true],
        ]);

        population.calculate_fitness(&context);
        let new_population = competition::tournament(&context, population);

        assert_eq!(new_population.chromosomes.len(), 4);

        // safe enough value, although not by definition true
        assert!(helpers::number_of_true_values_in_population(&new_population) >= 8);
        println!("{:#?}", new_population);
    }

    #[test]
    fn test_tournament_shortage() {
        let context = Context::<BinaryGene>::new()
            .with_gene_size(3)
            .with_population_size(4)
            .with_tournament_size(4);

        let mut population = builders::population_from_booleans(vec![
            vec![false, false, false],
            vec![false, false, true],
        ]);

        population.calculate_fitness(&context);
        let new_population = competition::tournament(&context, population);

        assert_eq!(new_population.chromosomes.len(), 2);
    }
}
