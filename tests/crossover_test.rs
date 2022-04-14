mod support;

#[cfg(test)]
mod crossover_tests {
    use crate::support::*;
    use genetic_algorithm::context::Context;
    use genetic_algorithm::crossover;
    use genetic_algorithm::gene::BinaryGene;

    #[test]
    fn test_individual_even() {
        let context = Context::<BinaryGene>::new()
            .with_gene_size(3)
            .with_population_size(4);

        let population = builders::population_from_booleans(vec![
            vec![true, true, true],
            vec![false, false, false],
            vec![true, true, true],
            vec![false, false, false],
        ]);

        let child_population = crossover::individual(&context, &population);

        assert_eq!(child_population.chromosomes.len(), 4);
        println!("{:#?}", child_population);

        assert_eq!(
            helpers::number_of_true_values_in_population(&child_population),
            6
        );
    }

    #[test]
    fn test_individual_odd() {
        let context = Context::<BinaryGene>::new()
            .with_gene_size(3)
            .with_population_size(4);

        let population = builders::population_from_booleans(vec![
            vec![true, true, true],
            vec![false, false, false],
            vec![true, true, true],
            vec![false, false, false],
            vec![true, true, true],
        ]);

        let child_population = crossover::individual(&context, &population);
        assert_eq!(child_population.chromosomes.len(), 4);
    }
}
