mod support;

#[cfg(test)]
mod population_tests {
    use crate::support::*;
    use genetic_algorithm::context::Context;
    use genetic_algorithm::fitness;

    #[test]
    fn test_uniformity() {
        let rng = SmallRng::seed_from_u64(0);
        let context = Context::new()
            .with_gene_size(3)
            .with_gene_values(vec![true, false])
            .with_population_size(8)
            .with_fitness_function(fitness::count_true_values)
            .with_rng(rng);

        let mut population = build::population(vec![
            vec![true, true, true],
            vec![true, true, false],
            vec![true, false, true],
            vec![true, false, false],
        ]);

        let best_chromosome = population.best_chromosome().unwrap();
        assert_eq!(population.uniformity(&context, best_chromosome), 0.0);

        population.calculate_fitness(&context);
        population.sort();

        let best_chromosome = population.best_chromosome().unwrap();
        assert_eq!(population.uniformity(&context, best_chromosome), 0.125);
    }

    #[test]
    fn test_mass_extinction() {
        let rng = SmallRng::seed_from_u64(0);
        let context = Context::new()
            .with_gene_size(3)
            .with_gene_values(vec![true, false])
            .with_population_size(8)
            .with_fitness_function(fitness::count_true_values)
            .with_rng(rng);

        let mut population = build::population(vec![
            vec![false, true, true],
            vec![false, true, false],
            vec![false, false, true],
            vec![false, false, false],
            vec![true, true, true],
            vec![true, true, false],
            vec![true, false, true],
            vec![true, false, false],
        ]);

        population.calculate_fitness(&context);
        population.sort();
        population.mass_extinction(2);

        assert_eq!(
            inspect::population(&population),
            vec![vec![true, false, true], vec![true, true, true]]
        )
    }
}
