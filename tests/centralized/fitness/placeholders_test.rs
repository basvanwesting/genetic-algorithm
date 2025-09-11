#[cfg(test)]
use crate::support::*;
use genetic_algorithm::centralized::chromosome::ChromosomeManager;
use genetic_algorithm::centralized::fitness::placeholders::{
    CountStaticTrue, CountStaticTrueWithSleep, StaticCountdown, StaticCountdownNoisy, StaticZero,
    SumDynamicRange, SumStaticRange,
};
use genetic_algorithm::centralized::fitness::Fitness;
use genetic_algorithm::centralized::genotype::{
    DynamicRangeGenotype, Genotype, StaticBinaryGenotype, StaticRangeGenotype,
};

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

    // Test StaticZero with StaticRangeGenotype
    let mut zero = StaticZero::<StaticRangeGenotype<f32, 4, 5>>::new();
    assert_eq!(
        zero.calculate_for_population(&population, &genotype),
        vec![Some(0), Some(0), Some(0), Some(0), Some(0)]
    );

    // Test StaticCountdown with StaticRangeGenotype
    let mut countdown = StaticCountdown::<StaticRangeGenotype<f32, 4, 5>>::new(20);
    assert_eq!(
        countdown.calculate_for_population(&population, &genotype),
        vec![Some(19), Some(18), Some(17), Some(16), Some(15)]
    );

    // Test StaticCountdownNoisy with StaticRangeGenotype
    let mut countdown_noisy =
        StaticCountdownNoisy::<StaticRangeGenotype<f32, 4, 5>>::new(20, 3, 0..3);
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

    // Test StaticZero with StaticBinaryGenotype
    let mut zero = StaticZero::<StaticBinaryGenotype<4, 5>>::new();
    assert_eq!(
        zero.calculate_for_population(&population, &genotype),
        vec![Some(0), Some(0), Some(0), Some(0), Some(0)]
    );

    // Test StaticCountdown with StaticBinaryGenotype
    let mut countdown = StaticCountdown::<StaticBinaryGenotype<4, 5>>::new(20);
    assert_eq!(
        countdown.calculate_for_population(&population, &genotype),
        vec![Some(19), Some(18), Some(17), Some(16), Some(15)]
    );

    // Test StaticCountdownNoisy with StaticBinaryGenotype
    let mut countdown_noisy = StaticCountdownNoisy::<StaticBinaryGenotype<4, 5>>::new(20, 3, 0..3);
    let results = countdown_noisy.calculate_for_population(&population, &genotype);
    assert_eq!(results.len(), 5);
    assert!(results.iter().all(|r| r.is_some()));

    // Test CountStaticTrueWithSleep
    let mut sleep_fitness = CountStaticTrueWithSleep::<4, 5>::new(100, false);
    assert_eq!(
        sleep_fitness.calculate_for_population(&population, &genotype),
        vec![Some(4), Some(3), Some(2), Some(1), Some(0)]
    );
}
