#[cfg(test)]
mod fitness_tests {
    use genetic_algorithm::chromosome::Chromosome;
    use genetic_algorithm::fitness;

    #[test]
    fn test_simple_sum() {
        let chromosome = Chromosome::new(vec![true, true, true]);
        assert_eq!(fitness::simple_sum(&chromosome), 3);

        let chromosome = Chromosome::new(vec![true, false, true]);
        assert_eq!(fitness::simple_sum(&chromosome), 2);

        let chromosome = Chromosome::new(vec![true, false, false]);
        assert_eq!(fitness::simple_sum(&chromosome), 1);

        let chromosome = Chromosome::new(vec![false, false, false]);
        assert_eq!(fitness::simple_sum(&chromosome), 0);
    }
}
