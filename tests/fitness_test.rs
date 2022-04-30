mod support;

#[cfg(test)]
mod fitness_tests {
    use crate::support::*;
    use genetic_algorithm::fitness::{
        Fitness, SimpleSumBinaryGenotype, SimpleSumContinuousGenotype,
        SimpleSumDiscreteGenotypeContinuousGene, SimpleSumDiscreteGenotypeDiscreteGene,
    };
    use genetic_algorithm::gene::{ContinuousGene, DiscreteGene};
    use genetic_algorithm::genotype::{BinaryGenotype, ContinuousGenotype, DiscreteGenotype};

    #[test]
    fn test_simple_sum_binary() {
        let chromosome = build::chromosome::<BinaryGenotype>(vec![true, true, true]);
        assert_eq!(SimpleSumBinaryGenotype.call_for_chromosome(&chromosome), 3);

        let chromosome = build::chromosome::<BinaryGenotype>(vec![true, false, true]);
        assert_eq!(SimpleSumBinaryGenotype.call_for_chromosome(&chromosome), 2);

        let chromosome = build::chromosome::<BinaryGenotype>(vec![true, false, false]);
        assert_eq!(SimpleSumBinaryGenotype.call_for_chromosome(&chromosome), 1);

        let chromosome = build::chromosome::<BinaryGenotype>(vec![false, false, false]);
        assert_eq!(SimpleSumBinaryGenotype.call_for_chromosome(&chromosome), 0);
    }

    #[test]
    fn test_simple_sum_discrete() {
        let chromosome = build::chromosome::<DiscreteGenotype<DiscreteGene>>(vec![1, 2, 3]);
        assert_eq!(
            SimpleSumDiscreteGenotypeDiscreteGene.call_for_chromosome(&chromosome),
            6
        );

        let chromosome = build::chromosome::<DiscreteGenotype<DiscreteGene>>(vec![0, 0, 0]);
        assert_eq!(
            SimpleSumDiscreteGenotypeDiscreteGene.call_for_chromosome(&chromosome),
            0
        );
    }

    #[test]
    fn test_simple_sum_continuous() {
        let chromosome = build::chromosome::<ContinuousGenotype>(vec![0.0, 0.0, 0.0]);
        assert_eq!(
            SimpleSumContinuousGenotype.call_for_chromosome(&chromosome),
            0
        );

        let chromosome = build::chromosome::<ContinuousGenotype>(vec![0.1, 0.2, 0.3]);
        assert_eq!(
            SimpleSumContinuousGenotype.call_for_chromosome(&chromosome),
            0
        );

        let chromosome = build::chromosome::<ContinuousGenotype>(vec![1.4, 2.4, 3.4]);
        assert_eq!(
            SimpleSumContinuousGenotype.call_for_chromosome(&chromosome),
            7
        );

        let chromosome = build::chromosome::<DiscreteGenotype<ContinuousGene>>(vec![0.0, 0.0, 0.0]);
        assert_eq!(
            SimpleSumDiscreteGenotypeContinuousGene.call_for_chromosome(&chromosome),
            0
        );

        let chromosome = build::chromosome::<DiscreteGenotype<ContinuousGene>>(vec![0.1, 0.2, 0.3]);
        assert_eq!(
            SimpleSumDiscreteGenotypeContinuousGene.call_for_chromosome(&chromosome),
            0
        );

        let chromosome = build::chromosome::<DiscreteGenotype<ContinuousGene>>(vec![1.4, 2.4, 3.4]);
        assert_eq!(
            SimpleSumDiscreteGenotypeContinuousGene.call_for_chromosome(&chromosome),
            7
        );
    }
}
