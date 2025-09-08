#[cfg(test)]
use crate::support::*;
use genetic_algorithm::centralized::chromosome::{
    BinaryChromosome, ChromosomeManager, ListChromosome, RangeChromosome,
};
use genetic_algorithm::centralized::fitness::placeholders::{
    CountStaticTrue, CountStaticTrueWithSleep, CountTrue, CountTrueWithSleep, Countdown, 
    CountdownNoisy, StaticCountdown, StaticCountdownNoisy, SumDynamicRange, SumGenes, 
    SumStaticRange, Zero,
};
use genetic_algorithm::centralized::fitness::Fitness;
use genetic_algorithm::centralized::genotype::{
    DynamicRangeGenotype, Genotype, StaticBinaryGenotype, StaticRangeGenotype,
};

#[test]
fn binary_genotype() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(3)
        .build()
        .unwrap();
    let chromosome: BinaryChromosome = build::chromosome(vec![true, true, true]);
    assert_eq!(
        Zero::new().calculate_for_chromosome(&chromosome, &genotype),
        Some(0)
    );

    let chromosome: BinaryChromosome = build::chromosome(vec![true, true, true]);
    assert_eq!(
        CountTrue.calculate_for_chromosome(&chromosome, &genotype),
        Some(3)
    );

    let chromosome: BinaryChromosome = build::chromosome(vec![true, false, true]);
    assert_eq!(
        CountTrue.calculate_for_chromosome(&chromosome, &genotype),
        Some(2)
    );

    let chromosome: BinaryChromosome = build::chromosome(vec![true, false, false]);
    assert_eq!(
        CountTrue.calculate_for_chromosome(&chromosome, &genotype),
        Some(1)
    );

    let chromosome: BinaryChromosome = build::chromosome(vec![false, false, false]);
    assert_eq!(
        CountTrue.calculate_for_chromosome(&chromosome, &genotype),
        Some(0)
    );

    let chromosome: BinaryChromosome = build::chromosome(vec![true, false, true]);
    assert_eq!(
        CountTrueWithSleep::new(1000, false).calculate_for_chromosome(&chromosome, &genotype),
        Some(2)
    );
}

#[test]
fn list_genotype_u8() {
    let genotype = ListGenotype::builder()
        .with_genes_size(3)
        .with_allele_list((0..10).collect())
        .build()
        .unwrap();

    let chromosome: ListChromosome<u8> = build::chromosome(vec![0, 1, 2, 3]);
    assert_eq!(
        Zero::new().calculate_for_chromosome(&chromosome, &genotype),
        Some(0)
    );

    let chromosome: ListChromosome<u8> = build::chromosome(vec![0, 1, 2, 3]);
    assert_eq!(
        SumGenes::new().calculate_for_chromosome(&chromosome, &genotype),
        Some(6)
    );

    let chromosome: ListChromosome<u8> = build::chromosome(vec![0, 0, 0, 0]);
    assert_eq!(
        SumGenes::new().calculate_for_chromosome(&chromosome, &genotype),
        Some(0)
    );
}

#[test]
fn list_genotype_i8() {
    let genotype = ListGenotype::builder()
        .with_genes_size(3)
        .with_allele_list((-10..10).collect())
        .build()
        .unwrap();

    let chromosome: ListChromosome<i8> = build::chromosome(vec![-2, -1, 0, 1, 2, 3]);
    assert_eq!(
        SumGenes::new().calculate_for_chromosome(&chromosome, &genotype),
        Some(3)
    );

    let chromosome: ListChromosome<i8> = build::chromosome(vec![0, 0, 0, 0]);
    assert_eq!(
        SumGenes::new().calculate_for_chromosome(&chromosome, &genotype),
        Some(0)
    );

    let chromosome: ListChromosome<i8> = build::chromosome(vec![-2, -1, 0, -1, -2, -3]);
    assert_eq!(
        SumGenes::new().calculate_for_chromosome(&chromosome, &genotype),
        Some(-9)
    );
}

#[test]
fn range_genotype_f32() {
    let genotype = RangeGenotype::builder()
        .with_genes_size(3)
        .with_allele_range(0.0_f32..=1.0_f32)
        .build()
        .unwrap();

    let chromosome: RangeChromosome<f32> = build::chromosome(vec![0.1, 0.2, 0.3]);
    assert_eq!(
        Zero::new().calculate_for_chromosome(&chromosome, &genotype),
        Some(0)
    );

    let chromosome: RangeChromosome<f32> = build::chromosome(vec![0.0_f32, 0.0_f32, 0.0_f32]);
    assert_eq!(
        SumGenes::new().calculate_for_chromosome(&chromosome, &genotype),
        Some(0)
    );

    let chromosome: RangeChromosome<f32> = build::chromosome(vec![0.1_f32, 0.2_f32, 0.3_f32]);
    assert_eq!(
        SumGenes::new().calculate_for_chromosome(&chromosome, &genotype),
        Some(0)
    );

    let chromosome: RangeChromosome<f32> = build::chromosome(vec![1.4_f32, 2.4_f32, 3.4_f32]);
    assert_eq!(
        SumGenes::new().calculate_for_chromosome(&chromosome, &genotype),
        Some(7)
    );

    let chromosome: RangeChromosome<f32> = build::chromosome(vec![0.0_f32, 0.0_f32, 0.0_f32]);
    assert_eq!(
        SumGenes::new_with_precision(1e-3).calculate_for_chromosome(&chromosome, &genotype),
        Some(0)
    );

    let chromosome: RangeChromosome<f32> = build::chromosome(vec![0.1_f32, 0.2_f32, 0.3_f32]);
    assert_eq!(
        SumGenes::new_with_precision(1e-3).calculate_for_chromosome(&chromosome, &genotype),
        Some(600)
    );

    let chromosome: RangeChromosome<f32> = build::chromosome(vec![1.4_f32, 2.4_f32, 3.4_f32]);
    assert_eq!(
        SumGenes::new_with_precision(1e-3).calculate_for_chromosome(&chromosome, &genotype),
        Some(7200)
    );

    let mut fitness = Countdown::new(5);
    let chromosome: RangeChromosome<f32> = build::chromosome(vec![0.1_f32, 0.2_f32, 0.3_f32]);
    let fitness_scores: Vec<Option<isize>> = (0..6)
        .map(|_| fitness.calculate_for_chromosome(&chromosome, &genotype))
        .collect();
    assert_eq!(
        fitness_scores,
        vec![Some(4), Some(3), Some(2), Some(1), Some(0), Some(0)]
    );

    let mut fitness = CountdownNoisy::new(20, 5, 0..2);
    let chromosome: RangeChromosome<f32> = build::chromosome(vec![0.1_f32, 0.2_f32, 0.3_f32]);
    let fitness_scores: Vec<Option<isize>> = (0..22)
        .map(|_| fitness.calculate_for_chromosome(&chromosome, &genotype))
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

#[test]
fn range_genotype_f64() {
    let genotype = RangeGenotype::builder()
        .with_genes_size(3)
        .with_allele_range(0.0_f64..=1.0_f64)
        .build()
        .unwrap();

    let chromosome: RangeChromosome<f64> = build::chromosome(vec![0.0_f64, 0.0_f64, 0.0_f64]);
    assert_eq!(
        SumGenes::new_with_precision(1e-3).calculate_for_chromosome(&chromosome, &genotype),
        Some(0)
    );

    let chromosome: RangeChromosome<f64> = build::chromosome(vec![0.1_f64, 0.2_f64, 0.3_f64]);
    assert_eq!(
        SumGenes::new_with_precision(1e-3).calculate_for_chromosome(&chromosome, &genotype),
        Some(600)
    );

    let chromosome: RangeChromosome<f64> = build::chromosome(vec![1.4_f64, 2.4_f64, 3.4_f64]);
    assert_eq!(
        SumGenes::new_with_precision(1e-3).calculate_for_chromosome(&chromosome, &genotype),
        Some(7199)
    );
}

#[test]
fn dynamic_range_genotype_f32() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = DynamicRangeGenotype::builder()
        .with_genes_size(4)
        .with_allele_range(0.0_f32..=1.0_f32)
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let population = Population::new(
        [
            genotype.chromosome_constructor_random(&mut rng),
            genotype.chromosome_constructor_random(&mut rng),
            genotype.chromosome_constructor_random(&mut rng),
            genotype.chromosome_constructor_random(&mut rng),
            genotype.chromosome_constructor_random(&mut rng),
        ]
        .to_vec(),
    );

    assert_eq!(
        SumDynamicRange::new_with_precision(1e-3).calculate_for_population(&population, &genotype),
        vec![Some(2328), Some(2884), Some(2431), Some(1845), Some(2041)]
    );
}

#[test]
fn static_range_genotype_f32() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = StaticRangeGenotype::<f32, 4, 5>::builder()
        .with_genes_size(4)
        .with_allele_range(0.0_f32..=1.0_f32)
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let population = Population::new(
        [
            genotype.chromosome_constructor_random(&mut rng),
            genotype.chromosome_constructor_random(&mut rng),
            genotype.chromosome_constructor_random(&mut rng),
            genotype.chromosome_constructor_random(&mut rng),
            genotype.chromosome_constructor_random(&mut rng),
        ]
        .to_vec(),
    );

    assert_eq!(
        SumStaticRange::new_with_precision(1e-3).calculate_for_population(&population, &genotype),
        vec![Some(2328), Some(2884), Some(2431), Some(1845), Some(2041)]
    );

    // Test StaticCountdown with StaticRangeGenotype
    let mut countdown = StaticCountdown::<StaticRangeGenotype<f32, 4, 5>>::new(20);
    assert_eq!(
        countdown.calculate_for_population(&population, &genotype),
        vec![Some(19), Some(18), Some(17), Some(16), Some(15)]
    );

    // Test StaticCountdownNoisy with StaticRangeGenotype
    let mut countdown_noisy = StaticCountdownNoisy::<StaticRangeGenotype<f32, 4, 5>>::new(20, 3, 0..3);
    let results = countdown_noisy.calculate_for_population(&population, &genotype);
    assert_eq!(results.len(), 5);
    assert!(results.iter().all(|r| r.is_some()));
}

#[test]
fn static_binary_genotype() {
    let mut genotype = StaticBinaryGenotype::<4, 5>::builder()
        .with_genes_size(4)
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    // Create a population with known true/false values
    let population = static_build::population(
        &mut genotype,
        vec![
            vec![true, true, true, true],     // 4 true values
            vec![true, true, true, false],    // 3 true values
            vec![true, true, false, false],   // 2 true values
            vec![true, false, false, false],  // 1 true value
            vec![false, false, false, false], // 0 true values
        ],
    );

    assert_eq!(
        CountStaticTrue::<4, 5>::new().calculate_for_population(&population, &genotype),
        vec![Some(4), Some(3), Some(2), Some(1), Some(0)]
    );

    // Test CountStaticTrueWithSleep
    assert_eq!(
        CountStaticTrueWithSleep::<4, 5>::new(100, false).calculate_for_population(&population, &genotype),
        vec![Some(4), Some(3), Some(2), Some(1), Some(0)]
    );

    // Test StaticCountdown
    let mut countdown = StaticCountdown::<StaticBinaryGenotype<4, 5>>::new(10);
    assert_eq!(
        countdown.calculate_for_population(&population, &genotype),
        vec![Some(9), Some(8), Some(7), Some(6), Some(5)]
    );

    // Test StaticCountdownNoisy
    let mut countdown_noisy = StaticCountdownNoisy::<StaticBinaryGenotype<4, 5>>::new(10, 2, 0..2);
    let results = countdown_noisy.calculate_for_population(&population, &genotype);
    assert_eq!(results.len(), 5);
    // Values should be around multiples of step size with noise
    assert!(results.iter().all(|r| r.is_some()));
}
