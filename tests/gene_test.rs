#[cfg(test)]
mod mutation_tests {
    use genetic_algorithm::gene::Gene;

    #[test]
    fn test_mutate() {
        let mut gene = Gene::new(true);
        assert_eq!(gene.value, true);
        gene.mutate();
        assert_eq!(gene.value, false);
        gene.mutate();
        assert_eq!(gene.value, true);
    }
}
