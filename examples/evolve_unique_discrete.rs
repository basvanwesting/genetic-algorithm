use distance::hamming;
use genetic_algorithm::chromosome::Chromosome;
use genetic_algorithm::compete::CompeteTournament;
use genetic_algorithm::crossover::CrossoverClone;
use genetic_algorithm::evolve::Evolve;
use genetic_algorithm::fitness::{Fitness, FitnessOrdering, FitnessValue};
use genetic_algorithm::genotype::{Genotype, UniqueDiscreteGenotype};
use genetic_algorithm::mutate::MutateOnce;
use itertools::Itertools;
use rand::prelude::*;
use rand::rngs::SmallRng;

#[derive(Clone, Debug)]
struct MyGeneFitness;
impl Fitness for MyGeneFitness {
    type Genotype = UniqueDiscreteGenotype<String>;
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
    let genotype = UniqueDiscreteGenotype::<String>::builder()
        .with_gene_values(vec![
            "c".to_string(),
            "e".to_string(),
            "e".to_string(),
            "g".to_string(),
            "i".to_string(),
            "n".to_string(),
            "t".to_string(),
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
