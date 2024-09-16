#[cfg(test)]
use genetic_algorithm::fitness::placeholders::CountTrue;
use genetic_algorithm::strategy::evolve::prelude::*;
use genetic_algorithm::strategy::hill_climb::prelude::*;
use genetic_algorithm::strategy::permutate::prelude::*;
use genetic_algorithm::strategy::StrategyBuilder;
use genetic_algorithm::strategy::{
    StrategyConfig, StrategyReporter, StrategyState, StrategyVariant,
};
use std::marker::PhantomData;

#[derive(Clone)]
pub struct GenericReporter<G: Genotype>(pub PhantomData<G>);
impl<G: Genotype> Default for GenericReporter<G> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
impl<G: Genotype> GenericReporter<G> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<G: Genotype> StrategyReporter for GenericReporter<G> {
    type Genotype = G;

    fn on_init<S: StrategyState<Self::Genotype>, C: StrategyConfig>(
        &mut self,
        genotype: &Self::Genotype,
        state: &S,
        config: &C,
    ) {
        println!("{}", genotype);
        println!("{}", state);
        println!("{}", config);
    }
}

#[test]
fn build_invalid_missing_variant() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(5)
        .build()
        .unwrap();

    let builder = StrategyBuilder::new()
        .with_genotype(genotype)
        // .with_variant(StrategyVariant::Evolve(EvolveVariant::Standard))
        .with_reporter(GenericReporter::new())
        .with_target_population_size(100)
        .with_target_fitness_score(5)
        .with_fitness(CountTrue)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_crossover(CrossoverSingleGene::new())
        .with_select(SelectTournament::new(4, 0.9))
        .with_rng_seed_from_u64(0);

    let strategy = builder.build();
    assert!(strategy.is_err());
    assert_eq!(
        strategy.err(),
        Some(TryFromStrategyBuilderError("StrategyVariant is required"))
    );
}

#[test]
fn generic_strategy_evolve() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(5)
        .build()
        .unwrap();

    let builder = StrategyBuilder::new()
        .with_genotype(genotype)
        .with_variant(StrategyVariant::Evolve(EvolveVariant::Standard))
        .with_reporter(GenericReporter::new())
        .with_target_population_size(100)
        .with_target_fitness_score(5)
        .with_fitness(CountTrue)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_crossover(CrossoverSingleGene::new())
        .with_select(SelectTournament::new(4, 0.9))
        .with_rng_seed_from_u64(0);

    let mut strategy = builder.build().unwrap();
    strategy.call();
    let result = strategy.best_genes_and_fitness_score();

    assert!(result.is_some());
    let (_best_genes, fitness_score) = result.unwrap();
    assert_eq!(fitness_score, 5);
}

#[test]
fn generic_strategy_permutate() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(5)
        .build()
        .unwrap();

    let builder = StrategyBuilder::new()
        .with_genotype(genotype)
        .with_variant(StrategyVariant::Permutate(PermutateVariant::Standard))
        .with_reporter(GenericReporter::new())
        .with_target_population_size(100)
        .with_target_fitness_score(5)
        .with_fitness(CountTrue)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_crossover(CrossoverSingleGene::new())
        .with_select(SelectTournament::new(4, 0.9))
        .with_rng_seed_from_u64(0);

    let mut strategy = builder.build().unwrap();
    strategy.call();
    let result = strategy.best_genes_and_fitness_score();

    assert!(result.is_some());
    let (_best_genes, fitness_score) = result.unwrap();
    assert_eq!(fitness_score, 5);
}

#[test]
fn generic_strategy_hill_climb_steepest_ascent() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(5)
        .build()
        .unwrap();

    let builder = StrategyBuilder::new()
        .with_genotype(genotype)
        .with_variant(StrategyVariant::HillClimb(HillClimbVariant::SteepestAscent))
        .with_reporter(GenericReporter::new())
        .with_target_population_size(100)
        .with_target_fitness_score(5)
        .with_fitness(CountTrue)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_crossover(CrossoverSingleGene::new())
        .with_select(SelectTournament::new(4, 0.9))
        .with_rng_seed_from_u64(0);

    let mut strategy = builder.build().unwrap();
    strategy.call();
    let result = strategy.best_genes_and_fitness_score();

    assert!(result.is_some());
    let (_best_genes, fitness_score) = result.unwrap();
    assert_eq!(fitness_score, 5);
}
