#[cfg(test)]
mod context_tests {
    use genetic_algorithm::context::Context;

    #[test]
    fn test_random_chromosome_factory() {
        let context = Context::new(10, 100);
        let chromosome = context.random_chromosome_factory();
        println!("{:#?}", chromosome);
        assert_eq!(chromosome.genes.len(), 10);
    }
}
