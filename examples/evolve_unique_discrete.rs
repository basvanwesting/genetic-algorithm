use distance::hamming;
use genetic_algorithm::chromosome::Chromosome;
use genetic_algorithm::compete::CompeteTournament;
use genetic_algorithm::crossover::CrossoverClone;
use genetic_algorithm::evolve::Evolve;
use genetic_algorithm::fitness::{Fitness, FitnessOrdering, FitnessValue};
use genetic_algorithm::gene::Gene;
use genetic_algorithm::genotype::{Genotype, UniqueDiscreteGenotype};
use genetic_algorithm::mutate::MutateOnce;
use itertools::Itertools;
use rand::prelude::*;
use rand::rngs::SmallRng;

#[derive(Clone, Debug, Default)]
struct MyGene(String);
impl Gene for MyGene {}
impl std::fmt::Display for MyGene {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug)]
struct MyGeneFitness;
impl Fitness for MyGeneFitness {
    type Genotype = UniqueDiscreteGenotype<MyGene>;
    fn call_for_chromosome(
        &mut self,
        chromosome: &Chromosome<Self::Genotype>,
    ) -> Option<FitnessValue> {
        let string = chromosome.genes.iter().join("");
        Some(hamming(&string, "genetic").unwrap() as FitnessValue)
    }
}

fn main() {
    let mut rng = SmallRng::from_entropy();
    let genotype = UniqueDiscreteGenotype::<MyGene>::builder()
        .with_gene_values(vec![
            MyGene("c".to_string()),
            MyGene("e".to_string()),
            MyGene("e".to_string()),
            MyGene("g".to_string()),
            MyGene("i".to_string()),
            MyGene("n".to_string()),
            MyGene("t".to_string()),
        ])
        .build()
        .unwrap();

    println!("{}", genotype);

    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_population_size(10)
        .with_max_stale_generations(100)
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_target_fitness_score(0)
        .with_mutate(MutateOnce(0.2))
        .with_fitness(MyGeneFitness)
        .with_crossover(CrossoverClone(true))
        .with_compete(CompeteTournament(4))
        .build()
        .unwrap()
        .call(&mut rng);

    println!("{}", evolve);
}
