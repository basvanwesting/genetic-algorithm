use genetic_algorithm::evolve::prelude::*;
use rand::prelude::*;
use rand::rngs::SmallRng;

#[derive(Clone, Debug)]
struct MyAllele(u8, u8);

#[derive(Clone, Debug)]
struct MyFitness;
impl Fitness for MyFitness {
    type Genotype = DiscreteGenotype<MyAllele>;
    fn calculate_for_chromosome(
        &mut self,
        chromosome: &Chromosome<Self::Genotype>,
    ) -> Option<FitnessValue> {
        Some(
            chromosome
                .genes
                .iter()
                .fold(0, |acc, c| acc + c.0 as FitnessValue + c.1 as FitnessValue)
                as FitnessValue,
        )
    }
}

fn main() {
    let mut rng = SmallRng::from_entropy();
    let genotype = DiscreteGenotype::<MyAllele>::builder()
        .with_genes_size(100)
        .with_allele_values(vec![
            MyAllele(1, 2),
            MyAllele(3, 4),
            MyAllele(5, 6),
            MyAllele(7, 8),
        ])
        .build()
        .unwrap();

    println!("{}", genotype);

    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_population_size(100)
        .with_max_stale_generations(1000)
        .with_target_fitness_score(1500)
        .with_degeneration_range(0.001..0.995)
        .with_mutate(MutateOnce(0.2))
        .with_fitness(MyFitness)
        .with_crossover(CrossoverAll(true))
        .with_compete(CompeteTournament(4))
        .call(&mut rng)
        .unwrap();

    println!("{}", evolve);
}
