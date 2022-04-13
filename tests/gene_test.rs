#[cfg(test)]
mod gene_tests {
    use genetic_algorithm::gene::Gene;

    #[test]
    fn test_mutate() {
        let mut gene = Gene(true);
        assert_eq!(gene.0, true);
        gene.mutate();
        assert_eq!(gene.0, false);
        gene.mutate();
        assert_eq!(gene.0, true);
    }
}
