use genetic_algorithm::strategy::hill_climb::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;

type Person = u8;
type TableSize = u8;
type HostsWithTableSizesPerRound = Vec<Vec<(Person, TableSize)>>;

#[derive(Clone, Debug)]
struct TableSeatingFitness(pub HostsWithTableSizesPerRound);
impl Fitness for TableSeatingFitness {
    type Genotype = MultiUniqueGenotype<Person>;
    fn calculate_for_chromosome(
        &mut self,
        chromosome: &FitnessChromosome<Self>,
        _genotype: &FitnessGenotype<Self>,
    ) -> Option<FitnessValue> {
        let hosts_with_table_sizes_per_round = &self.0;
        let mut people = chromosome.genes.clone();
        let mut tables: Vec<Vec<Person>> = vec![];

        for hosts_with_table_sizes in hosts_with_table_sizes_per_round {
            for (host, table_size) in hosts_with_table_sizes {
                let mut people_on_table: Vec<Person> =
                    people.drain(..(*table_size as usize - 1)).collect();
                people_on_table.push(*host);
                tables.push(people_on_table);
            }
        }

        let mut score = 0;
        let mut person_sets: HashMap<Person, HashSet<Person>> = HashMap::new();

        for table in tables {
            for person in &table {
                for other_person in &table {
                    if person != other_person {
                        let person_set = person_sets.entry(*person).or_default();
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
    env_logger::init();

    let people: Vec<Person> = (0..24).collect();
    let hosts_with_table_sizes_per_round: HostsWithTableSizesPerRound = vec![
        vec![(0, 5), (1, 5), (2, 5), (3, 5), (4, 4)],
        vec![(5, 4), (6, 4), (7, 4), (8, 4), (9, 4), (10, 4)],
        vec![(11, 4), (12, 4), (13, 4), (14, 4), (15, 4), (16, 4)],
        vec![(17, 5), (18, 5), (19, 5), (29, 5), (21, 4)],
    ];

    let number_of_rounds = hosts_with_table_sizes_per_round.len();
    let hosts_per_round: Vec<Vec<Person>> = hosts_with_table_sizes_per_round
        .iter()
        .map(|round| round.iter().map(|(h, _s)| *h).collect())
        .collect();
    let allele_lists: Vec<Vec<Person>> = (0..number_of_rounds)
        .map(|i| {
            people
                .iter()
                .filter(|person| !hosts_per_round[i].contains(person))
                .copied()
                .collect::<Vec<_>>()
        })
        .collect();

    let genotype = MultiUniqueGenotype::builder()
        .with_allele_lists(allele_lists)
        .build()
        .unwrap();

    println!("{}", genotype);

    let mut hill_climb = HillClimb::builder()
        .with_genotype(genotype)
        .with_variant(HillClimbVariant::Stochastic)
        .with_max_stale_generations(10000)
        .with_fitness(TableSeatingFitness(
            hosts_with_table_sizes_per_round.clone(),
        ))
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_target_fitness_score(0)
        .build()
        .unwrap();

    hill_climb.call();
    println!("{}", hill_climb);

    if let Some(fitness_score) = hill_climb.best_fitness_score() {
        if let Some(best_genes) = hill_climb.best_genes() {
            if fitness_score == 0 {
                println!("Valid solution");
            } else {
                println!("Wrong solution with fitness score: {}", fitness_score);
            }

            let mut person_counters: HashMap<u8, HashMap<u8, u8>> = HashMap::new();
            let mut people = best_genes.clone();

            for hosts_with_table_sizes in &hosts_with_table_sizes_per_round {
                println!("round:");
                for (host, table_size) in hosts_with_table_sizes {
                    let mut people_on_table: Vec<Person> = vec![*host];
                    people_on_table
                        .append(&mut people.drain(..(*table_size as usize - 1)).collect());
                    println!("  table: {:?}", people_on_table);

                    for person in &people_on_table {
                        for other_person in &people_on_table {
                            if person != other_person {
                                let person_counter = person_counters.entry(*person).or_default();

                                let count = person_counter.entry(*other_person).or_insert(0);
                                *count += 1
                            }
                        }
                    }
                }
            }

            for (person, person_counter) in person_counters {
                for (other_person, count) in person_counter {
                    if count > 1 {
                        println!(
                            "person {} and person {} meet {} number of times",
                            person, other_person, count
                        );
                    }
                }
            }
        }
    } else {
        println!("Invalid solution with fitness score: None");
    }
}
