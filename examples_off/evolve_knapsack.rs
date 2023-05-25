use genetic_algorithm::strategy::evolve::prelude::*;
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

#[derive(Clone, Debug)]
struct KnapsackFitness<'a> {
    pub items: &'a Vec<Item>,
    pub weight_limit: WeightLimit,
}
impl KnapsackFitness<'_> {
    const EXCESS_WEIGHT_PENALTY: FitnessValue = 1000;
}
impl Fitness for KnapsackFitness<'_> {
    type Genotype = BinaryGenotype;

    fn calculate_for_chromosome(
        &mut self,
        chromosome: &Chromosome<Self::Genotype>,
    ) -> Option<FitnessValue> {
        let item_indices: Vec<usize> = chromosome
            .genes
            .iter()
            .enumerate()
            .filter_map(|(i, v)| if *v { Some(i) } else { None })
            .collect();
        let weight: u16 = item_indices.iter().map(|i| self.items[*i].0).sum();
        let value: u16 = item_indices.iter().map(|i| self.items[*i].1).sum();

        // base score with total value
        let mut score = value as FitnessValue;

        // penalize score with excess weight, to nudge towards valid solutions
        if weight > self.weight_limit {
            score -= (weight - self.weight_limit) as FitnessValue * Self::EXCESS_WEIGHT_PENALTY;
        }

        Some(score)
    }
}

fn main() {
    env_logger::init();

    let items: Vec<Item> = vec![
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
    ];
    let weight_limit = 67;
    let fitness = KnapsackFitness {
        items: &items,
        weight_limit,
    };

    let mut rng = SmallRng::from_entropy();
    let genotype = BinaryGenotype::builder()
        .with_genes_size(items.len())
        .build()
        .unwrap();

    println!("{}", genotype);

    let mut evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(100)
        .with_max_stale_generations(100)
        .with_fitness(fitness)
        .with_mutate(MutateOnce::new(0.2))
        .with_crossover(CrossoverSinglePoint::new(true))
        .with_compete(CompeteTournament::new(4))
        .with_extension(ExtensionNoop::new())
        .build()
        .unwrap();

    let now = std::time::Instant::now();
    evolve.call(&mut rng);
    let duration = now.elapsed();

    println!("{}", evolve);

    if let Some(best_chromosome) = evolve.best_chromosome() {
        let selected_items = best_chromosome
            .genes
            .iter()
            .enumerate()
            .filter_map(|(i, v)| if *v { Some(&items[i]) } else { None });
        println!("selected items: {:?}", selected_items.collect::<Vec<_>>());
    }
    println!("{:?}", duration);
}
