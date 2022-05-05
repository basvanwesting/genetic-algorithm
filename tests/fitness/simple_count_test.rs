#[cfg(test)]
use crate::support::*;
use genetic_algorithm::fitness::{Fitness, FitnessSimpleCount};
use genetic_algorithm::genotype::BinaryGenotype;

#[test]
fn binary_genotype() {
    let chromosome = build::chromosome::<BinaryGenotype>(vec![true, true, true]);
    assert_eq!(FitnessSimpleCount.call_for_chromosome(&chromosome), 3);

    let chromosome = build::chromosome::<BinaryGenotype>(vec![true, false, true]);
    assert_eq!(FitnessSimpleCount.call_for_chromosome(&chromosome), 2);

    let chromosome = build::chromosome::<BinaryGenotype>(vec![true, false, false]);
    assert_eq!(FitnessSimpleCount.call_for_chromosome(&chromosome), 1);

    let chromosome = build::chromosome::<BinaryGenotype>(vec![false, false, false]);
    assert_eq!(FitnessSimpleCount.call_for_chromosome(&chromosome), 0);
}
