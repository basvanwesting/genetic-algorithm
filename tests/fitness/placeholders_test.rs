#[cfg(test)]
use crate::support::*;
use genetic_algorithm::fitness::placeholders::{CountTrue, SumF32, SumIsize, SumUsize, Zero};
use genetic_algorithm::fitness::Fitness;

#[test]
fn zero() {
    let chromosome = build::chromosome(vec![true, true, true]);
    assert_eq!(Zero::new().calculate_for_chromosome(&chromosome), Some(0));

    let chromosome = build::chromosome(vec![0.1, 0.2, 0.3]);
    assert_eq!(Zero::new().calculate_for_chromosome(&chromosome), Some(0));

    let chromosome = build::chromosome(vec![0, 1, 2, 3]);
    assert_eq!(Zero::new().calculate_for_chromosome(&chromosome), Some(0));
}

#[test]
fn count_true() {
    let chromosome = build::chromosome(vec![true, true, true]);
    assert_eq!(CountTrue.calculate_for_chromosome(&chromosome), Some(3));

    let chromosome = build::chromosome(vec![true, false, true]);
    assert_eq!(CountTrue.calculate_for_chromosome(&chromosome), Some(2));

    let chromosome = build::chromosome(vec![true, false, false]);
    assert_eq!(CountTrue.calculate_for_chromosome(&chromosome), Some(1));

    let chromosome = build::chromosome(vec![false, false, false]);
    assert_eq!(CountTrue.calculate_for_chromosome(&chromosome), Some(0));
}

#[test]
fn sum_usize() {
    let chromosome = build::chromosome(vec![0, 1, 2, 3]);
    assert_eq!(SumUsize.calculate_for_chromosome(&chromosome), Some(6));

    let chromosome = build::chromosome(vec![0, 0, 0, 0]);
    assert_eq!(SumUsize.calculate_for_chromosome(&chromosome), Some(0));
}

#[test]
fn sum_isize() {
    let chromosome = build::chromosome(vec![-2, -1, 0, 1, 2, 3]);
    assert_eq!(SumIsize.calculate_for_chromosome(&chromosome), Some(3));

    let chromosome = build::chromosome(vec![0, 0, 0, 0]);
    assert_eq!(SumIsize.calculate_for_chromosome(&chromosome), Some(0));
}

#[test]
fn sum_f32() {
    let chromosome = build::chromosome(vec![0.0, 0.0, 0.0]);
    assert_eq!(SumF32(1e-3).calculate_for_chromosome(&chromosome), Some(0));

    let chromosome = build::chromosome(vec![0.1, 0.2, 0.3]);
    assert_eq!(
        SumF32(1e-3).calculate_for_chromosome(&chromosome),
        Some(600)
    );

    let chromosome = build::chromosome(vec![1.4, 2.4, 3.4]);
    assert_eq!(
        SumF32(1e-3).calculate_for_chromosome(&chromosome),
        Some(7200)
    );
}
