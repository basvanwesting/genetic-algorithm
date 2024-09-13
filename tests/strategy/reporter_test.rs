#[cfg(test)]
use genetic_algorithm::fitness::placeholders::{CountTrue, SumGenes};
use genetic_algorithm::strategy::permutate::prelude::*;
use genetic_algorithm::strategy::{StrategyConfig, StrategyReporter, StrategyState};

#[derive(Clone)]
pub struct GenericReporter(usize);
impl GenericReporter {
    pub fn new(period: usize) -> Self {
        Self(period)
    }
}
impl<G: Genotype, S: StrategyState<G>, C: StrategyConfig> StrategyReporter<G, S, C>
    for GenericReporter
{
    fn on_init(&mut self, genotype: &G, state: &S, config: &C) {
        println!("{}", self.0);
        println!("{}", genotype);
        println!("{}", state);
        println!("{}", config);
    }
}

#[derive(Clone)]
pub struct GenotypeReporter(usize);
impl GenotypeReporter {
    pub fn new(period: usize) -> Self {
        Self(period)
    }
}
impl<S: StrategyState<BinaryGenotype>, C: StrategyConfig> StrategyReporter<BinaryGenotype, S, C>
    for GenotypeReporter
{
    fn on_init(&mut self, genotype: &BinaryGenotype, state: &S, config: &C) {
        println!("{}", self.0);
        println!("{}", genotype);
        println!("{}", state);
        println!("{}", config);
    }
}

#[derive(Clone)]
pub struct SpecificReporter(usize);
impl SpecificReporter {
    pub fn new(period: usize) -> Self {
        Self(period)
    }
}
impl StrategyReporter<BinaryGenotype, PermutateState<BinaryGenotype>, PermutateConfig>
    for SpecificReporter
{
    fn on_init(
        &mut self,
        genotype: &BinaryGenotype,
        state: &PermutateState<BinaryGenotype>,
        config: &PermutateConfig,
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

    let permutate = Permutate::builder()
        .with_genotype(genotype)
        .with_fitness(CountTrue)
        // .with_reporter(PermutateReporterNoop::new())
        .call()
        .unwrap();

    println!("{:#?}", permutate.best_genes());
    assert_eq!(permutate.best_fitness_score(), Some(5));
    assert_eq!(
        permutate.best_genes().unwrap(),
        vec![true, true, true, true, true]
    );

    GenericReporter::new(0).on_init(&permutate.genotype, &permutate.state, &permutate.config);
    GenotypeReporter::new(1).on_init(&permutate.genotype, &permutate.state, &permutate.config);
    SpecificReporter::new(2).on_init(&permutate.genotype, &permutate.state, &permutate.config);

    // panic!()
}
