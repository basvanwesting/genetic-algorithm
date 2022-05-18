use genetic_algorithm::strategy::hill_climb::prelude::*;
use rand::prelude::*;
use rand::rngs::SmallRng;
use std::collections::HashMap;
use std::collections::HashSet;

// see https://en.wikipedia.org/wiki/Eight_queens_puzzle
#[derive(Clone, Debug)]
struct TableSettingFitness(pub u8, pub Vec<usize>);
impl Fitness for TableSettingFitness {
    type Genotype = MultiUniqueGenotype<u8>;
    fn calculate_for_chromosome(
        &mut self,
        chromosome: &Chromosome<Self::Genotype>,
    ) -> Option<FitnessValue> {
        let number_of_rounds = &self.0;
        let table_sizes = &self.1;
        let mut people = chromosome.genes.clone();
        let mut tables: Vec<Vec<u8>> = vec![];

        for _round in 0..*number_of_rounds {
            for table_size in table_sizes {
                tables.push(people.drain(..table_size).collect());
            }
        }

        let mut score = 0;
        let mut person_sets: HashMap<u8, HashSet<u8>> = HashMap::new();

        for table in tables {
            for person in &table {
                for other_person in &table {
                    if person != other_person {
                        let person_set = person_sets.entry(*person).or_insert(HashSet::new());
                        if person_set.insert(*other_person) {
                            // new insert
                        } else {
                            // existing insert
                            score += 1;
                        }
                    }
                }
            }
        }

        Some(score)
    }
}

fn main() {
    let number_of_people: u8 = 12;
    let table_sizes: Vec<usize> = vec![3, 3, 3, 3];
    let number_of_rounds: u8 = 4;

    let mut rng = SmallRng::from_entropy();
    let genotype = MultiUniqueGenotype::builder()
        .with_allele_multi_values(
            (0..number_of_rounds)
                .map(|_| (0..number_of_people).collect())
                .collect(),
        )
        .build()
        .unwrap();

    println!("{}", genotype);

    let mut hill_climb = HillClimb::builder()
        .with_genotype(genotype)
        .with_max_stale_generations(100000)
        .with_fitness(TableSettingFitness(number_of_rounds, table_sizes.clone()))
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_target_fitness_score(0)
        .build()
        .unwrap();

    let now = std::time::Instant::now();
    hill_climb.call(&mut rng);
    let duration = now.elapsed();

    println!("{}", hill_climb);

    if let Some(best_chromosome) = hill_climb.best_chromosome() {
        if let Some(fitness_score) = best_chromosome.fitness_score {
            if fitness_score == 0 {
                println!("Valid solution");
            } else {
                println!("Wrong solution with fitness score: {}", fitness_score);
            }

            let mut person_counters: HashMap<u8, HashMap<u8, u8>> = HashMap::new();
            let mut people = best_chromosome.genes.clone();
            for round in 0..number_of_rounds {
                println!("round: {}", round);
                for table_size in &table_sizes {
                    let people_on_table = people.drain(..table_size).collect::<Vec<u8>>();
                    println!("  table: {:?}", people_on_table);

                    for person in &people_on_table {
                        for other_person in &people_on_table {
                            if person != other_person {
                                let person_counter =
                                    person_counters.entry(*person).or_insert(HashMap::new());

                                let count = person_counter.entry(*other_person).or_insert(0);
                                *count += 1
                            }
                        }
                    }
                }
            }
            println!("person_counters: {:?}", person_counters);
        }
    } else {
        println!("Invalid solution with fitness score: None");
    }
    println!("{:?}", duration);
}
