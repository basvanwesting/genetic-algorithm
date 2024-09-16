#[cfg(test)]
use genetic_algorithm::fitness::placeholders::{CountTrue, SumGenes};
use genetic_algorithm::strategy::evolve::prelude::*;
use genetic_algorithm::strategy::hill_climb::prelude::*;
use genetic_algorithm::strategy::StrategyBuilder;
use genetic_algorithm::strategy::{
    StrategyConfig, StrategyReporter, StrategyState, StrategyVariant,
};

#[derive(Clone)]
pub struct GenericReporterBinary;
impl StrategyReporter for GenericReporterBinary {
    type Genotype = BinaryGenotype;

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

#[derive(Clone)]
pub struct GenericReporterRange;
impl StrategyReporter for GenericReporterRange {
    type Genotype = RangeGenotype;

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

    let pick = "evolve";

    let builder = StrategyBuilder::new()
        .with_genotype(genotype)
        .with_variant(StrategyVariant::HillClimb(HillClimbVariant::Stochastic))
        .with_reporter(GenericReporterBinary)
        .with_target_population_size(100)
        .with_target_fitness_score(5)
        .with_fitness(CountTrue)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_crossover(CrossoverSingleGene::new())
        .with_select(SelectTournament::new(4, 0.9))
        .with_rng_seed_from_u64(0);

    let result = match pick {
        "permutate" => {
            let permutate = builder.to_permutate_builder().call().unwrap();
            permutate.best_genes_and_fitness_score()
        }
        "evolve" => {
            let evolve = builder.to_evolve_builder().call().unwrap();
            evolve.best_genes_and_fitness_score()
        }
        "hill_climb" => {
            let hill_climb = builder.to_hill_climb_builder().call().unwrap();
            hill_climb.best_genes_and_fitness_score()
        }
        &_ => todo!(),
    };

    if let Some((_best_genes, fitness_score)) = result {
        assert_eq!(fitness_score, 5);
    } else {
        panic!("no result")
    }

    // panic!()
}

#[test]
fn generic_strategy_range() {
    let genotype = RangeGenotype::builder()
        .with_genes_size(5)
        .with_allele_range(0.0..=1.0)
        .build()
        .unwrap();

    let pick = "evolve";

    let builder = StrategyBuilder::new()
        .with_genotype(genotype)
        .with_variant(StrategyVariant::HillClimb(HillClimbVariant::Stochastic))
        .with_reporter(GenericReporterRange)
        .with_target_population_size(100)
        .with_target_fitness_score(4500)
        .with_fitness(SumGenes::new_with_precision(1e-3))
        .with_mutate(MutateSingleGene::new(0.1))
        .with_crossover(CrossoverSingleGene::new())
        .with_select(SelectTournament::new(4, 0.9))
        .with_rng_seed_from_u64(0);

    let result = match pick {
        // "permutate" => {
        //     let permutate = builder.to_permutate_builder().call().unwrap();
        //     permutate.best_genes_and_fitness_score()
        // }
        "evolve" => {
            let evolve = builder.to_evolve_builder().call().unwrap();
            evolve.best_genes_and_fitness_score()
        }
        "hill_climb" => {
            let hill_climb = builder.to_hill_climb_builder().call().unwrap();
            hill_climb.best_genes_and_fitness_score()
        }
        &_ => todo!(),
    };

    if let Some((_best_genes, fitness_score)) = result {
        assert_eq!(fitness_score, 4511);
    } else {
        panic!("no result")
    }

    // panic!()
}
