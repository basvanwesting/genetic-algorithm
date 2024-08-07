#[cfg(test)]
use crate::support::*;
use genetic_algorithm::genotype::{ContinuousGenotype, Genotype};
use genetic_algorithm::mutate::{Mutate, MutateSingleGeneDistance};
use genetic_algorithm::strategy::evolve::EvolveReporterNoop;

#[test]
fn continuous_genotype() {
    let genotype = ContinuousGenotype::builder()
        .with_genes_size(3)
        .with_allele_range(0.0..1.0)
        .build()
        .unwrap();

    let population = &mut build::population(vec![
        vec![0.5, 0.5, 0.5],
        vec![0.5, 0.5, 0.5],
        vec![0.5, 0.5, 0.5],
        vec![0.5, 0.5, 0.5],
    ]);

    let mut reporter = EvolveReporterNoop::default();
    let mut rng = SmallRng::seed_from_u64(0);
    MutateSingleGeneDistance::new(0.5, 0.0..0.1).call(
        &genotype,
        population,
        &mut reporter,
        &mut rng,
    );

    assert_eq!(
        inspect::population(population),
        vec![
            vec![0.5, 0.45608598, 0.5],
            vec![0.5, 0.5, 0.5],
            vec![0.5, 0.5, 0.5],
            vec![0.5, 0.5, 0.5],
        ]
    );

    MutateSingleGeneDistance::new(0.5, 0.0..0.1).call(
        &genotype,
        population,
        &mut reporter,
        &mut rng,
    );
    MutateSingleGeneDistance::new(0.5, 0.0..0.1).call(
        &genotype,
        population,
        &mut reporter,
        &mut rng,
    );
    MutateSingleGeneDistance::new(0.5, 0.0..0.1).call(
        &genotype,
        population,
        &mut reporter,
        &mut rng,
    );

    assert_eq!(
        inspect::population(population),
        vec![
            vec![0.46048558, 0.37730217, 0.58177435],
            vec![0.46539983, 0.4746074, 0.5],
            vec![0.5, 0.4715659, 0.5],
            vec![0.5921572, 0.4329331, 0.5],
        ]
    );
}
