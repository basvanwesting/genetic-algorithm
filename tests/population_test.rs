mod support;

#[cfg(test)]
mod population_tests {
    use crate::support::*;
    use genetic_algorithm::context::Context;
    use genetic_algorithm::population::Population;

    #[test]
    fn test_random_factory() {
        let context = Context::new().with_gene_size(16).with_population_size(100);

        let population = Population::random_factory(&context);
        println!("{:#?}", population);

        assert_eq!(
            helpers::number_of_true_values_in_population(&population)
                + helpers::number_of_false_values_in_population(&population),
            16 * 100
        );
    }
}
