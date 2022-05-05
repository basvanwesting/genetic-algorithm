#[cfg(test)]
use crate::support::*;
use genetic_algorithm::fitness::{
    Fitness, FitnessSimpleSumContinuousGenotype, FitnessSimpleSumIndexGenotype,
};
use genetic_algorithm::genotype::{ContinuousGenotype, IndexGenotype};

#[test]
fn test_index() {
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
fn test_continuous() {
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
