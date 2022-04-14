mod support;

#[cfg(test)]
mod fitness_tests {
    use crate::support::*;
    use genetic_algorithm::fitness;

    #[test]
    fn count_true_values() {
        let chromosome = builders::chromosome_binary(vec![true, true, true]);
        assert_eq!(fitness::count_true_values(&chromosome), 3);

        let chromosome = builders::chromosome_binary(vec![true, false, true]);
        assert_eq!(fitness::count_true_values(&chromosome), 2);

        let chromosome = builders::chromosome_binary(vec![true, false, false]);
        assert_eq!(fitness::count_true_values(&chromosome), 1);

        let chromosome = builders::chromosome_binary(vec![false, false, false]);
        assert_eq!(fitness::count_true_values(&chromosome), 0);
    }

    #[test]
    fn sum_discrete_values() {
        let chromosome = builders::chromosome_discrete(vec![1, 2, 3]);
        assert_eq!(fitness::sum_discrete_values(&chromosome), 6);

        let chromosome = builders::chromosome_discrete(vec![0, 0, 0]);
        assert_eq!(fitness::sum_discrete_values(&chromosome), 0);
    }

    #[test]
    fn sum_continuous_values() {
        let chromosome = builders::chromosome_continuous(vec![0.0, 0.0, 0.0]);
        assert_eq!(fitness::sum_continuous_values(&chromosome), 0);

        let chromosome = builders::chromosome_continuous(vec![0.1, 0.2, 0.3]);
        assert_eq!(fitness::sum_continuous_values(&chromosome), 0);

        let chromosome = builders::chromosome_continuous(vec![1.4, 2.4, 3.4]);
        assert_eq!(fitness::sum_continuous_values(&chromosome), 7);
    }
}
