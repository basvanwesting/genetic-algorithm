mod support;

#[cfg(test)]
mod fitness_tests {
    use crate::support::*;
    use genetic_algorithm::fitness;

    #[test]
    fn test_simple_sum() {
        let chromosome = builders::chromosome_from_booleans(vec![true, true, true]);
        assert_eq!(fitness::simple_sum(&chromosome), 3);

        let chromosome = builders::chromosome_from_booleans(vec![true, false, true]);
        assert_eq!(fitness::simple_sum(&chromosome), 2);

        let chromosome = builders::chromosome_from_booleans(vec![true, false, false]);
        assert_eq!(fitness::simple_sum(&chromosome), 1);

        let chromosome = builders::chromosome_from_booleans(vec![false, false, false]);
        assert_eq!(fitness::simple_sum(&chromosome), 0);
    }
}
