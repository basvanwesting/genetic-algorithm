mod support;

#[cfg(test)]
mod fitness_tests {
    use crate::support::*;
    use genetic_algorithm::fitness::{Fitness, FitnessSimpleSum};

    #[test]
    fn test_simple_sum_binary() {
        let chromosome = build::chromosome(vec![true, true, true]);
        assert_eq!(FitnessSimpleSum.call_for_chromosome(&chromosome), 3);

        let chromosome = build::chromosome(vec![true, false, true]);
        assert_eq!(FitnessSimpleSum.call_for_chromosome(&chromosome), 2);

        let chromosome = build::chromosome(vec![true, false, false]);
        assert_eq!(FitnessSimpleSum.call_for_chromosome(&chromosome), 1);

        let chromosome = build::chromosome(vec![false, false, false]);
        assert_eq!(FitnessSimpleSum.call_for_chromosome(&chromosome), 0);
    }

    #[test]
    fn test_simple_sum_discrete() {
        let chromosome = build::chromosome(vec![1, 2, 3]);
        assert_eq!(FitnessSimpleSum.call_for_chromosome(&chromosome), 6);

        let chromosome = build::chromosome(vec![0, 0, 0]);
        assert_eq!(FitnessSimpleSum.call_for_chromosome(&chromosome), 0);
    }

    #[test]
    fn test_simple_sum_continuous() {
        let chromosome = build::chromosome(vec![0.0, 0.0, 0.0]);
        assert_eq!(FitnessSimpleSum.call_for_chromosome(&chromosome), 0);

        let chromosome = build::chromosome(vec![0.1, 0.2, 0.3]);
        assert_eq!(FitnessSimpleSum.call_for_chromosome(&chromosome), 0);

        let chromosome = build::chromosome(vec![1.4, 2.4, 3.4]);
        assert_eq!(FitnessSimpleSum.call_for_chromosome(&chromosome), 7);
    }
}
