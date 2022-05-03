mod support;

#[cfg(test)]
mod fitness_tests {
    use crate::support::*;
    use genetic_algorithm::fitness::{
        Fitness, FitnessSimpleSumBinaryGenotype, FitnessSimpleSumContinuousGenotype,
        FitnessSimpleSumIndexGenotype,
    };
    use genetic_algorithm::genotype::{BinaryGenotype, ContinuousGenotype, IndexGenotype};

    #[test]
    fn test_simple_sum_binary() {
        let chromosome = build::chromosome::<BinaryGenotype>(vec![true, true, true]);
        assert_eq!(
            FitnessSimpleSumBinaryGenotype.call_for_chromosome(&chromosome),
            3
        );

        let chromosome = build::chromosome::<BinaryGenotype>(vec![true, false, true]);
        assert_eq!(
            FitnessSimpleSumBinaryGenotype.call_for_chromosome(&chromosome),
            2
        );

        let chromosome = build::chromosome::<BinaryGenotype>(vec![true, false, false]);
        assert_eq!(
            FitnessSimpleSumBinaryGenotype.call_for_chromosome(&chromosome),
            1
        );

        let chromosome = build::chromosome::<BinaryGenotype>(vec![false, false, false]);
        assert_eq!(
            FitnessSimpleSumBinaryGenotype.call_for_chromosome(&chromosome),
            0
        );
    }

    #[test]
    fn test_simple_sum_index() {
        let chromosome = build::chromosome::<IndexGenotype>(vec![0, 1, 2, 3]);
        assert_eq!(
            FitnessSimpleSumIndexGenotype.call_for_chromosome(&chromosome),
            6
        );

        let chromosome = build::chromosome::<IndexGenotype>(vec![0, 0, 0, 0]);
        assert_eq!(
            FitnessSimpleSumIndexGenotype.call_for_chromosome(&chromosome),
            0
        );
    }

    #[test]
    fn test_simple_sum_continuous() {
        let chromosome = build::chromosome::<ContinuousGenotype>(vec![0.0, 0.0, 0.0]);
        assert_eq!(
            FitnessSimpleSumContinuousGenotype.call_for_chromosome(&chromosome),
            0
        );

        let chromosome = build::chromosome::<ContinuousGenotype>(vec![0.1, 0.2, 0.3]);
        assert_eq!(
            FitnessSimpleSumContinuousGenotype.call_for_chromosome(&chromosome),
            0
        );

        let chromosome = build::chromosome::<ContinuousGenotype>(vec![1.4, 2.4, 3.4]);
        assert_eq!(
            FitnessSimpleSumContinuousGenotype.call_for_chromosome(&chromosome),
            7
        );
    }
}
