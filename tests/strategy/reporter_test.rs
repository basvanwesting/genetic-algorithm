#[cfg(test)]
use genetic_algorithm::fitness::placeholders::CountTrue;
use genetic_algorithm::strategy::evolve::prelude::*;
use genetic_algorithm::strategy::hill_climb::prelude::*;
use genetic_algorithm::strategy::StrategyBuilder;
use genetic_algorithm::strategy::{StrategyConfig, StrategyReporter, StrategyState};

#[derive(Clone)]
pub struct GenericReporter(usize);
impl GenericReporter {
    pub fn new(period: usize) -> Self {
        Self(period)
    }
}
impl StrategyReporter for GenericReporter {
    type Genotype = BinaryGenotype;

    fn on_init<S: StrategyState<Self::Genotype>, C: StrategyConfig>(
        &mut self,
        genotype: &Self::Genotype,
        state: &S,
        config: &C,
    ) {
        println!("{}", self.0);
        println!("{}", genotype);
        println!("{}", state);
        println!("{}", config);
    }
}

#[test]
fn test_reporters() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(5)
        .build()
        .unwrap();

    let pick = "evolve";

    let builder = StrategyBuilder::new()
        .with_genotype(genotype)
        .with_variant(HillClimbVariant::Stochastic)
        .with_reporter(GenericReporter::new(0))
        .with_target_population_size(100)
        .with_target_fitness_score(5)
        .with_fitness(CountTrue)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_crossover(CrossoverSingleGene::new())
        .with_select(SelectTournament::new(4, 0.9));

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
