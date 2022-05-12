use genetic_algorithm::evolve::prelude::*;
use rand::prelude::*;
use rand::rngs::SmallRng;

// see https://en.wikipedia.org/wiki/Knapsack_problem
// example data 10 items with (weight, value):
//   (23, 505), (26, 352), (20, 458), (18, 220), (32, 354), (27, 414), (29, 498), (26, 545), (30, 473), (27, 543),
// Optimal value is 1270 with items: (18, 220), (23, 505) and (26, 545)

type WeightLimit = u16;
type Weight = u16;
type Value = u16;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Item(pub Weight, pub Value);
impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "weight: {}, value: {}", self.0, self.1)
    }
}

#[derive(Clone, Debug)]
struct KnapsackFitness(pub WeightLimit);
impl KnapsackFitness {
    const EXCESS_WEIGHT_PENALTY: FitnessValue = 1000;
}
impl Fitness for KnapsackFitness {
    type Genotype = SetGenotype<Item>;

    fn calculate_for_chromosome(
        &mut self,
        chromosome: &Chromosome<Self::Genotype>,
    ) -> Option<FitnessValue> {
        let weight: u16 = chromosome.genes.iter().map(|c| c.0).sum();
        let value: u16 = chromosome.genes.iter().map(|c| c.1).sum();

        // base score with total value
        let mut score = value as FitnessValue;

        // penalize score with excess weight, to nudge towards valid solutions
        if weight > self.0 {
            score -= (weight - self.0) as FitnessValue * Self::EXCESS_WEIGHT_PENALTY;
        }

        Some(score)
    }
}

fn main() {
    let mut rng = SmallRng::from_entropy();
    let genotype = SetGenotype::<Item>::builder()
        .with_gene_size(10)
        .with_gene_values(vec![
            Item(23, 505),
            Item(26, 352),
            Item(20, 458),
            Item(18, 220),
            Item(32, 354),
            Item(27, 414),
            Item(29, 498),
            Item(26, 545),
            Item(30, 473),
            Item(27, 543),
        ])
        .build()
        .unwrap();

    println!("{}", genotype);

    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_population_size(100)
        .with_max_stale_generations(100)
        .with_mutate(MutateOnce(0.3))
        .with_fitness(KnapsackFitness(67)) // set the weight limit
        .with_crossover(CrossoverClone(true))
        .with_compete(CompeteTournament(4))
        .build()
        .unwrap()
        .call(&mut rng);

    println!("{}", evolve);
}
