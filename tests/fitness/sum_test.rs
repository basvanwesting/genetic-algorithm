#[cfg(test)]
use crate::support::*;
use genetic_algorithm::fitness::{
    Fitness, FitnessSumContinuousGenotype, FitnessSumDiscreteGenotype,
};
use genetic_algorithm::genotype::{ContinuousGenotype, DiscreteGenotype};

#[test]
fn discrete_genotype() {
    let chromosome = build::chromosome::<DiscreteGenotype<usize>>(vec![0, 1, 2, 3]);
    assert_eq!(
        FitnessSumDiscreteGenotype.call_for_chromosome(&chromosome),
        Some(6)
    );

    let chromosome = build::chromosome::<DiscreteGenotype<usize>>(vec![0, 0, 0, 0]);
    assert_eq!(
        FitnessSumDiscreteGenotype.call_for_chromosome(&chromosome),
        Some(0)
    );
}

#[test]
fn continuous_genotype() {
    let chromosome = build::chromosome::<ContinuousGenotype>(vec![0.0, 0.0, 0.0]);
    assert_eq!(
        FitnessSumContinuousGenotype.call_for_chromosome(&chromosome),
        Some(0)
    );

    let chromosome = build::chromosome::<ContinuousGenotype>(vec![0.1, 0.2, 0.3]);
    assert_eq!(
        FitnessSumContinuousGenotype.call_for_chromosome(&chromosome),
        Some(0)
    );

    let chromosome = build::chromosome::<ContinuousGenotype>(vec![1.4, 2.4, 3.4]);
    assert_eq!(
        FitnessSumContinuousGenotype.call_for_chromosome(&chromosome),
        Some(7)
    );
}
