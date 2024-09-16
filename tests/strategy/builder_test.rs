#[cfg(test)]
use genetic_algorithm::fitness::placeholders::{CountOnes, CountTrue, SumGenes};
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
fn generic_strategy_binary() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(5)
        .build()
        .unwrap();

    let builder = StrategyBuilder::new()
        .with_genotype(genotype)
        .with_reporter(GenericReporter::new())
        .with_target_population_size(100)
        .with_target_fitness_score(5)
        .with_fitness(CountTrue)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_crossover(CrossoverSingleGene::new())
        .with_select(SelectTournament::new(4, 0.9))
        .with_rng_seed_from_u64(0);

    let variant = StrategyVariant::Evolve(EvolveVariant::Standard);
    // let variant = StrategyVariant::HillClimb(HillClimbVariant::Stochastic);
    // let variant = StrategyVariant::HillClimb(HillClimbVariant::SteepestAscent);
    // let variant = StrategyVariant::Permutate(PermutateVariant::Standard);

    let result = match variant {
        StrategyVariant::Permutate(_) => {
            let permutate = builder.to_permutate_builder().call().unwrap();
            permutate.best_genes_and_fitness_score()
        }
        StrategyVariant::Evolve(_) => {
            let evolve = builder.to_evolve_builder().call().unwrap();
            evolve.best_genes_and_fitness_score()
        }
        StrategyVariant::HillClimb(hill_climb_variant) => {
            let hill_climb = builder
                .to_hill_climb_builder()
                .with_variant(hill_climb_variant)
                .call()
                .unwrap();
            hill_climb.best_genes_and_fitness_score()
        }
    };

    assert!(result.is_some());
    let (_best_genes, fitness_score) = result.unwrap();
    assert_eq!(fitness_score, 5);
}

#[test]
fn generic_strategy_range() {
    let genotype = RangeGenotype::builder()
        .with_genes_size(5)
        .with_allele_range(0.0..=1.0)
        .with_allele_mutation_range(0.1..=0.1)
        .build()
        .unwrap();

    let builder = StrategyBuilder::new()
        .with_genotype(genotype)
        .with_reporter(GenericReporter::new())
        .with_target_population_size(100)
        .with_target_fitness_score(4500)
        .with_fitness(SumGenes::new_with_precision(1e-3))
        .with_mutate(MutateSingleGene::new(0.1))
        .with_crossover(CrossoverSingleGene::new())
        .with_select(SelectTournament::new(4, 0.9))
        .with_rng_seed_from_u64(0);

    // let variant = StrategyVariant::Evolve(EvolveVariant::Standard);
    // let variant = StrategyVariant::HillClimb(HillClimbVariant::Stochastic);
    let variant = StrategyVariant::HillClimb(HillClimbVariant::SteepestAscent);
    // let variant = StrategyVariant::Permutate(PermutateVariant::Standard);

    let result = match variant {
        StrategyVariant::Permutate(_) => {
            todo!();
            // TODO: genotype is not permutable, so arm is invalid, is this ok?
            // let permutate = builder.to_permutate_builder().call().unwrap();
            // permutate.best_genes_and_fitness_score()
        }
        StrategyVariant::Evolve(_) => {
            let evolve = builder.to_evolve_builder().call().unwrap();
            evolve.best_genes_and_fitness_score()
        }
        StrategyVariant::HillClimb(hill_climb_variant) => {
            let hill_climb = builder
                .to_hill_climb_builder()
                .with_variant(hill_climb_variant)
                .call()
                .unwrap();
            hill_climb.best_genes_and_fitness_score()
        }
    };

    assert!(result.is_some());
    let (_best_genes, fitness_score) = result.unwrap();
    assert_eq!(fitness_score, 4567);
}

#[test]
fn generic_strategy_bit() {
    let genotype = BitGenotype::builder().with_genes_size(5).build().unwrap();

    let builder = StrategyBuilder::new()
        .with_genotype(genotype)
        .with_reporter(GenericReporter::new())
        .with_target_population_size(100)
        .with_target_fitness_score(5)
        .with_fitness(CountOnes)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_crossover(CrossoverSingleGene::new())
        .with_select(SelectTournament::new(4, 0.9))
        .with_rng_seed_from_u64(0);

    // let variant = StrategyVariant::Evolve(EvolveVariant::Standard);
    // let variant = StrategyVariant::HillClimb(HillClimbVariant::Stochastic);
    // let variant = StrategyVariant::HillClimb(HillClimbVariant::SteepestAscent);
    let variant = StrategyVariant::Permutate(PermutateVariant::Standard);

    let result = match variant {
        StrategyVariant::Permutate(_) => {
            let permutate = builder.to_permutate_builder().call().unwrap();
            permutate.best_genes_and_fitness_score()
        }
        StrategyVariant::Evolve(_) => {
            let evolve = builder.to_evolve_builder().call().unwrap();
            evolve.best_genes_and_fitness_score()
        }
        StrategyVariant::HillClimb(hill_climb_variant) => {
            let hill_climb = builder
                .to_hill_climb_builder()
                .with_variant(hill_climb_variant)
                .call()
                .unwrap();
            hill_climb.best_genes_and_fitness_score()
        }
    };

    assert!(result.is_some());
    let (_best_genes, fitness_score) = result.unwrap();
    assert_eq!(fitness_score, 5);
}
