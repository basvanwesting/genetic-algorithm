#[cfg(test)]
use crate::support::*;
use genetic_algorithm::fitness::placeholders::{
    CountTrue, CountTrueWithSleep, Countdown, CountdownNoisy, SumGenes, Zero,
};
use genetic_algorithm::fitness::Fitness;

#[test]
fn zero() {
    let chromosome: Chromosome<BinaryGenotype> = build::chromosome(vec![true, true, true]);
    assert_eq!(Zero::new().calculate_for_chromosome(&chromosome), Some(0));

    let chromosome: Chromosome<RangeGenotype<f32>> = build::chromosome(vec![0.1, 0.2, 0.3]);
    assert_eq!(Zero::new().calculate_for_chromosome(&chromosome), Some(0));

    let chromosome: Chromosome<ListGenotype<u8>> = build::chromosome(vec![0, 1, 2, 3]);
    assert_eq!(Zero::new().calculate_for_chromosome(&chromosome), Some(0));
}

#[test]
fn count_true() {
    let chromosome: Chromosome<BinaryGenotype> = build::chromosome(vec![true, true, true]);
    assert_eq!(CountTrue.calculate_for_chromosome(&chromosome), Some(3));

    let chromosome: Chromosome<BinaryGenotype> = build::chromosome(vec![true, false, true]);
    assert_eq!(CountTrue.calculate_for_chromosome(&chromosome), Some(2));

    let chromosome: Chromosome<BinaryGenotype> = build::chromosome(vec![true, false, false]);
    assert_eq!(CountTrue.calculate_for_chromosome(&chromosome), Some(1));

    let chromosome: Chromosome<BinaryGenotype> = build::chromosome(vec![false, false, false]);
    assert_eq!(CountTrue.calculate_for_chromosome(&chromosome), Some(0));
}

#[test]
fn sum_alleles_usize() {
    let chromosome: Chromosome<ListGenotype<u8>> = build::chromosome(vec![0, 1, 2, 3]);
    assert_eq!(
        SumGenes::new().calculate_for_chromosome(&chromosome),
        Some(6)
    );

    let chromosome: Chromosome<ListGenotype<u8>> = build::chromosome(vec![0, 0, 0, 0]);
    assert_eq!(
        SumGenes::new().calculate_for_chromosome(&chromosome),
        Some(0)
    );
}

#[test]
fn sum_alleles_isize() {
    let chromosome: Chromosome<ListGenotype<i8>> = build::chromosome(vec![-2, -1, 0, 1, 2, 3]);
    assert_eq!(
        SumGenes::new().calculate_for_chromosome(&chromosome),
        Some(3)
    );

    let chromosome: Chromosome<ListGenotype<i8>> = build::chromosome(vec![0, 0, 0, 0]);
    assert_eq!(
        SumGenes::new().calculate_for_chromosome(&chromosome),
        Some(0)
    );

    let chromosome: Chromosome<ListGenotype<i8>> = build::chromosome(vec![-2, -1, 0, -1, -2, -3]);
    assert_eq!(
        SumGenes::new().calculate_for_chromosome(&chromosome),
        Some(-9)
    );
}

#[test]
fn sum_alleles_f32() {
    let chromosome: Chromosome<RangeGenotype<f32>> =
        build::chromosome(vec![0.0_f32, 0.0_f32, 0.0_f32]);
    assert_eq!(
        SumGenes::new().calculate_for_chromosome(&chromosome),
        Some(0)
    );

    let chromosome: Chromosome<RangeGenotype<f32>> =
        build::chromosome(vec![0.1_f32, 0.2_f32, 0.3_f32]);
    assert_eq!(
        SumGenes::new().calculate_for_chromosome(&chromosome),
        Some(0)
    );

    let chromosome: Chromosome<RangeGenotype<f32>> =
        build::chromosome(vec![1.4_f32, 2.4_f32, 3.4_f32]);
    assert_eq!(
        SumGenes::new().calculate_for_chromosome(&chromosome),
        Some(7)
    );
}

#[test]
fn sum_alleles_with_precision_f32() {
    let chromosome: Chromosome<RangeGenotype<f32>> =
        build::chromosome(vec![0.0_f32, 0.0_f32, 0.0_f32]);
    assert_eq!(
        SumGenes::new_with_precision(1e-3).calculate_for_chromosome(&chromosome),
        Some(0)
    );

    let chromosome: Chromosome<RangeGenotype<f32>> =
        build::chromosome(vec![0.1_f32, 0.2_f32, 0.3_f32]);
    assert_eq!(
        SumGenes::new_with_precision(1e-3).calculate_for_chromosome(&chromosome),
        Some(600)
    );

    let chromosome: Chromosome<RangeGenotype<f32>> =
        build::chromosome(vec![1.4_f32, 2.4_f32, 3.4_f32]);
    assert_eq!(
        SumGenes::new_with_precision(1e-3).calculate_for_chromosome(&chromosome),
        Some(7200)
    );
}

#[test]
fn sum_alleles_with_precision_f64() {
    let chromosome: Chromosome<RangeGenotype<f64>> =
        build::chromosome(vec![0.0_f64, 0.0_f64, 0.0_f64]);
    assert_eq!(
        SumGenes::new_with_precision(1e-3).calculate_for_chromosome(&chromosome),
        Some(0)
    );

    let chromosome: Chromosome<RangeGenotype<f64>> =
        build::chromosome(vec![0.1_f64, 0.2_f64, 0.3_f64]);
    assert_eq!(
        SumGenes::new_with_precision(1e-3).calculate_for_chromosome(&chromosome),
        Some(600)
    );

    let chromosome: Chromosome<RangeGenotype<f64>> =
        build::chromosome(vec![1.4_f64, 2.4_f64, 3.4_f64]);
    assert_eq!(
        SumGenes::new_with_precision(1e-3).calculate_for_chromosome(&chromosome),
        Some(7199)
    );
}

#[test]
fn count_true_with_sleep() {
    let chromosome: Chromosome<BinaryGenotype> = build::chromosome(vec![true, false, true]);
    assert_eq!(
        CountTrueWithSleep::new(1000, false).calculate_for_chromosome(&chromosome),
        Some(2)
    );
}

#[test]
fn countdown() {
    let mut fitness = Countdown::new(5);
    let chromosome: Chromosome<RangeGenotype<f32>> =
        build::chromosome(vec![0.1_f32, 0.2_f32, 0.3_f32]);
    let fitness_scores: Vec<Option<isize>> = (0..6)
        .map(|_| fitness.calculate_for_chromosome(&chromosome))
        .collect();
    assert_eq!(
        fitness_scores,
        vec![Some(4), Some(3), Some(2), Some(1), Some(0), Some(0)]
    )
}

#[test]
fn countdown_with_noise() {
    let mut fitness = CountdownNoisy::new(20, 5, 0..2);
    let chromosome: Chromosome<RangeGenotype<f32>> =
        build::chromosome(vec![0.1_f32, 0.2_f32, 0.3_f32]);
    let fitness_scores: Vec<Option<isize>> = (0..22)
        .map(|_| fitness.calculate_for_chromosome(&chromosome))
        .collect();
    assert_eq!(
        fitness_scores,
        vec![
            Some(20),
            Some(20),
            Some(21),
            Some(20),
            Some(21),
            Some(16),
            Some(16),
            Some(15),
            Some(15),
            Some(16),
            Some(10),
            Some(11),
            Some(11),
            Some(10),
            Some(11),
            Some(5),
            Some(5),
            Some(5),
            Some(6),
            Some(6),
            Some(0),
            Some(0)
        ]
    )
}
