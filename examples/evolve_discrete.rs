use genetic_algorithm::chromosome::Chromosome;
use genetic_algorithm::compete::CompeteTournament;
use genetic_algorithm::crossover::CrossoverAll;
use genetic_algorithm::evolve::Evolve;
use genetic_algorithm::fitness::Fitness;
use genetic_algorithm::gene::Gene;
use genetic_algorithm::genotype::DiscreteGenotype;
use genetic_algorithm::mutate::MutateOnce;
use rand::prelude::*;
use rand::rngs::SmallRng;

#[derive(Clone, Copy, Debug, Default)]
struct MyGene(u8, u8);
impl Gene for MyGene {}

#[derive(Clone, Debug)]
struct MyGeneFitness;
impl Fitness for MyGeneFitness {
    type Genotype = DiscreteGenotype<MyGene>;
    fn call_for_chromosome(&self, chromosome: &Chromosome<Self::Genotype>) -> isize {
        chromosome
            .genes
            .iter()
            .fold(0, |acc, c| acc + c.0 as isize + c.1 as isize) as isize
    }
}

fn main() {
    let rng = SmallRng::from_entropy();
    let genotype = DiscreteGenotype::<MyGene>::new()
        .with_gene_size(100)
        .with_gene_values(vec![MyGene(1, 2), MyGene(3, 4), MyGene(5, 6), MyGene(7, 8)])
        .build();

    println!("{}", genotype);

    let evolve = Evolve::new(genotype, rng)
        .with_population_size(100)
        .with_max_stale_generations(1000)
        .with_target_fitness_score(1500)
        .with_degeneration_range(0.001..0.995)
        .with_mutate(MutateOnce(0.2))
        .with_fitness(MyGeneFitness)
        .with_crossover(CrossoverAll(true))
        .with_compete(CompeteTournament(4))
        .call();

    println!("{}", evolve);
}
