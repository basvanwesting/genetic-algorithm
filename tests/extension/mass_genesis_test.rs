#[cfg(test)]
use crate::support::*;
use genetic_algorithm::extension::{Extension, ExtensionMassGenesis};
use genetic_algorithm::genotype::{BinaryGenotype, Genotype};
use genetic_algorithm::population::Population;
use genetic_algorithm::strategy::evolve::{EvolveConfig, EvolveReporterNoop, EvolveState};

#[test]
fn removes_randomly() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(3)
        .build()
        .unwrap();

    let mut population: Population<BinaryGenotype> = build::population_with_fitness_scores(vec![
        (vec![false, true, true], Some(1)),
        (vec![false, true, false], Some(2)),
        (vec![false, false, true], Some(3)),
        (vec![false, false, false], Some(4)),
        (vec![true, true, true], Some(5)),
        (vec![true, true, false], Some(3)),
        (vec![true, false, true], Some(2)),
        (vec![true, false, false], Some(1)),
    ]);
    population.chromosomes.reserve_exact(2);
    assert_eq!(population.fitness_score_cardinality(), 5);
    assert_eq!(population.chromosomes.capacity(), 10);

    let mut state = EvolveState::new(&genotype);
    state.population = population;
    let config = EvolveConfig::new();
    let mut reporter = EvolveReporterNoop::new();
    let mut rng = SmallRng::seed_from_u64(0);
    ExtensionMassGenesis::new(5).call(&genotype, &mut state, &config, &mut reporter, &mut rng);

    assert_eq!(
        inspect::population(&state.population),
        vec![vec![true, true, true], vec![true, true, true],]
    );
    assert_eq!(state.population.chromosomes.capacity(), 10);
}
