#[cfg(test)]
use crate::support::*;
use genetic_algorithm::fitness::placeholders::{
    CountTrue, SumContinuousGenotype, SumDiscreteGenotype, Zero,
};
use genetic_algorithm::fitness::Fitness;
use genetic_algorithm::genotype::{BinaryGenotype, ContinuousGenotype, DiscreteGenotype};

#[test]
fn zero() {
    let chromosome = build::chromosome::<BinaryGenotype>(vec![true, true, true]);
    assert_eq!(Zero::new().calculate_for_chromosome(&chromosome), Some(0));

    let chromosome = build::chromosome::<ContinuousGenotype>(vec![0.1, 0.2, 0.3]);
    assert_eq!(Zero::new().calculate_for_chromosome(&chromosome), Some(0));

    let chromosome = build::chromosome::<DiscreteGenotype>(vec![0, 1, 2, 3]);
    assert_eq!(Zero::new().calculate_for_chromosome(&chromosome), Some(0));
}

#[test]
fn count_true() {
    let chromosome = build::chromosome::<BinaryGenotype>(vec![true, true, true]);
    assert_eq!(CountTrue.calculate_for_chromosome(&chromosome), Some(3));

    let chromosome = build::chromosome::<BinaryGenotype>(vec![true, false, true]);
    assert_eq!(CountTrue.calculate_for_chromosome(&chromosome), Some(2));

    let chromosome = build::chromosome::<BinaryGenotype>(vec![true, false, false]);
    assert_eq!(CountTrue.calculate_for_chromosome(&chromosome), Some(1));

    let chromosome = build::chromosome::<BinaryGenotype>(vec![false, false, false]);
    assert_eq!(CountTrue.calculate_for_chromosome(&chromosome), Some(0));
}

#[test]
fn sum_discrete_genotype() {
    let chromosome = build::chromosome::<DiscreteGenotype>(vec![0, 1, 2, 3]);
    assert_eq!(
        SumDiscreteGenotype.calculate_for_chromosome(&chromosome),
        Some(6)
    );

    let chromosome = build::chromosome::<DiscreteGenotype>(vec![0, 0, 0, 0]);
    assert_eq!(
        SumDiscreteGenotype.calculate_for_chromosome(&chromosome),
        Some(0)
    );
}

#[test]
fn sum_continuous_genotype() {
    let chromosome = build::chromosome::<ContinuousGenotype>(vec![0.0, 0.0, 0.0]);
    assert_eq!(
        SumContinuousGenotype.calculate_for_chromosome(&chromosome),
        Some(0)
    );

    let chromosome = build::chromosome::<ContinuousGenotype>(vec![0.1, 0.2, 0.3]);
    assert_eq!(
        SumContinuousGenotype.calculate_for_chromosome(&chromosome),
        Some(0)
    );

    let chromosome = build::chromosome::<ContinuousGenotype>(vec![1.4, 2.4, 3.4]);
    assert_eq!(
        SumContinuousGenotype.calculate_for_chromosome(&chromosome),
        Some(7)
    );
}
