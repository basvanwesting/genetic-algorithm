use genetic_algorithm::strategy::evolve::prelude::*;
use rand::prelude::*;
use rand::rngs::SmallRng;

const ROWS: usize = 8;
const COLUMNS: usize = 8;

type Row = usize;
type Column = usize;

#[derive(Clone, Debug, Eq, PartialEq)]
enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct NamePosition(pub Row, pub Column, pub Orientation);

#[derive(Clone, Debug)]
struct NamesFitness(pub Vec<&'static str>, pub bool);
impl Fitness for NamesFitness {
    type Genotype = MultiDiscreteGenotype<NamePosition>;
    fn calculate_for_chromosome(
        &mut self,
        chromosome: &Chromosome<Self::Genotype>,
    ) -> Option<FitnessValue> {
        let names = &self.0;
        let debug = self.1;

        let mut board: [[char; COLUMNS]; ROWS] = [[' '; COLUMNS]; ROWS];
        let mut score = 0;

        chromosome
            .genes
            .iter()
            .enumerate()
            .for_each(|(index, value)| {
                let name = names[index];
                match *value {
                    NamePosition(row, column, Orientation::Horizontal) => {
                        name.chars().enumerate().for_each(|(char_index, char)| {
                            if column + char_index < COLUMNS {
                                let current_char = board[row][column + char_index];
                                if current_char == ' ' {
                                    board[row][column + char_index] = char;
                                } else if current_char == char {
                                    score -= 10;
                                } else {
                                    score += 100;
                                }
                            } else {
                                score += 100;
                            }
                        })
                    }
                    NamePosition(row, column, Orientation::Vertical) => {
                        name.chars().enumerate().for_each(|(char_index, char)| {
                            if row + char_index < ROWS {
                                let current_char = board[row + char_index][column];
                                if current_char == ' ' {
                                    board[row + char_index][column] = char;
                                } else if current_char == char {
                                    score -= 10;
                                } else {
                                    score += 100;
                                }
                            } else {
                                score += 100;
                            }
                        })
                    }
                }
            });

        for row in 0..ROWS {
            let string = String::from_iter(board[row]);
            string
                .split_ascii_whitespace()
                .filter(|str| str.len() > 1)
                .for_each(|str| {
                    let known = names.iter().find(|name| name.eq_ignore_ascii_case(str));
                    if known.is_none() {
                        if debug {
                            println!("invalid horizontal string: {}", str);
                        }
                        score += 10;
                    }
                });
        }

        for column in 0..COLUMNS {
            let string = String::from_iter((0..ROWS).map(move |row| board[row][column]));
            string
                .split_ascii_whitespace()
                .filter(|str| str.len() > 1)
                .for_each(|str| {
                    let known = names.iter().find(|name| name.eq_ignore_ascii_case(str));
                    if known.is_none() {
                        if debug {
                            println!("invalid vertical string: {}", str);
                        }
                        score += 10;
                    }
                });
        }

        Some(score)
    }
}

fn main() {
    let names: Vec<&'static str> = vec!["damian", "jerald", "ava", "amir", "lenard"];
    let mut allele_lists: Vec<Vec<NamePosition>> = vec![vec![]; names.len()];
    names.iter().enumerate().for_each(|(index, name)| {
        for row in 0..=(ROWS - name.len()) {
            for column in 0..=(COLUMNS - name.len()) {
                allele_lists[index].push(NamePosition(row, column, Orientation::Horizontal));
                allele_lists[index].push(NamePosition(row, column, Orientation::Vertical));
            }
        }
    });

    let mut rng = SmallRng::from_entropy();
    let genotype = MultiDiscreteGenotype::builder()
        .with_allele_lists(allele_lists)
        .build()
        .unwrap();

    println!("{}", genotype);

    let mut evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_population_size(100)
        .with_max_stale_generations(10000)
        .with_fitness(NamesFitness(names.clone(), false))
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_mutate(MutateOnce(0.2))
        .with_crossover(CrossoverUniform(true))
        .with_compete(CompeteTournament(4))
        .build()
        .unwrap();

    let now = std::time::Instant::now();
    evolve.call(&mut rng);
    let duration = now.elapsed();

    println!("{}", evolve);

    if let Some(best_chromosome) = evolve.best_chromosome() {
        if let Some(_fitness_score) = best_chromosome.fitness_score {
            let mut board: [[char; COLUMNS]; ROWS] = [['.'; COLUMNS]; ROWS];

            // debug info
            NamesFitness(names.clone(), true).calculate_for_chromosome(&best_chromosome);

            best_chromosome
                .genes
                .iter()
                .enumerate()
                .for_each(|(index, value)| {
                    let name = names[index];
                    match *value {
                        NamePosition(row, column, Orientation::Horizontal) => {
                            name.chars().enumerate().for_each(|(char_index, char)| {
                                board[row][column + char_index] = char;
                            })
                        }
                        NamePosition(row, column, Orientation::Vertical) => {
                            name.chars().enumerate().for_each(|(char_index, char)| {
                                board[row + char_index][column] = char;
                            })
                        }
                    }
                });

            board.iter().for_each(|columns| {
                let string = String::from_iter(columns.iter());
                println!("{}", string);
            });
        } else {
            println!("Invalid solution with fitness score: None");
        }
    }
    println!("{:?}", duration);
}
