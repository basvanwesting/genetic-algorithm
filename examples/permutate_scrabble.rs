use genetic_algorithm::strategy::permutate::prelude::*;
use rand::prelude::*;
use rand::rngs::SmallRng;
use std::collections::{HashMap, HashSet};

const ROWS: usize = 5;
const COLUMNS: usize = 5;

type Row = usize;
type Column = usize;

#[derive(Clone, Debug, Eq, PartialEq)]
enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct WordPosition(pub Row, pub Column, pub Orientation);

#[derive(Clone, Debug)]
struct ScrabbleFitness {
    pub words: Vec<&'static str>,
    pub row_scores: Vec<isize>,
    pub column_scores: Vec<isize>,
    pub debug: bool,
    position_map: HashMap<(usize, usize), Vec<(usize, char)>>,
    related_word_ids: HashMap<usize, HashSet<usize>>,
    letter_board: [[char; COLUMNS]; ROWS],
}
impl Fitness for ScrabbleFitness {
    type Genotype = MultiDiscreteGenotype<WordPosition>;
    fn calculate_for_chromosome(
        &mut self,
        chromosome: &Chromosome<Self::Genotype>,
    ) -> Option<FitnessValue> {
        let mut score: isize = 0;

        self.position_map
            .iter_mut()
            .for_each(|(_, vec)| vec.clear());
        self.related_word_ids
            .iter_mut()
            .for_each(|(_, set)| set.clear());

        chromosome
            .genes
            .iter()
            .enumerate()
            .for_each(|(index, value)| {
                let word = self.words[index];
                match *value {
                    WordPosition(row, column, Orientation::Horizontal) => {
                        word.chars().enumerate().for_each(|(char_index, char)| {
                            self.position_map
                                .get_mut(&(row, column + char_index))
                                .unwrap()
                                .push((index, char));
                        })
                    }
                    WordPosition(row, column, Orientation::Vertical) => {
                        word.chars().enumerate().for_each(|(char_index, char)| {
                            self.position_map
                                .get_mut(&(row + char_index, column))
                                .unwrap()
                                .push((index, char));
                        })
                    }
                }
            });

        // position score
        for ((row, column), position_data) in &self.position_map {
            match position_data.as_slice() {
                [] => {
                    self.letter_board[*row][*column] = ' ';
                }
                [(_index, char)] => {
                    self.letter_board[*row][*column] = *char;
                    score += self.row_scores[*row] + self.column_scores[*column];
                }
                [(first_index, first_char), (second_index, second_char)] => {
                    if *first_char == *second_char {
                        self.letter_board[*row][*column] = *first_char;
                        score += 3 * self.row_scores[*row] + 3 * self.column_scores[*column];
                        self.related_word_ids
                            .get_mut(first_index)
                            .unwrap()
                            .insert(*second_index);
                        self.related_word_ids
                            .get_mut(second_index)
                            .unwrap()
                            .insert(*first_index);
                    } else {
                        score -= (ROWS * COLUMNS) as isize;
                    }
                }
                [(_first_index, first_char), _, rest @ ..] => {
                    self.letter_board[*row][*column] = *first_char;
                    score -= (ROWS * COLUMNS * rest.len()) as isize
                }
            }
        }

        let mut touching_word_ids: HashSet<usize> = HashSet::with_capacity(self.words.len());
        let starting_set = self.related_word_ids.values().nth(0).unwrap();
        ScrabbleFitness::recursive_touching_sets(
            starting_set,
            &self.related_word_ids,
            &mut touching_word_ids,
        );

        self.words.iter().enumerate().for_each(|(index, word)| {
            if !touching_word_ids.contains(&index) {
                score -= (ROWS * COLUMNS * word.len()) as isize
            }
        });

        for row in 0..ROWS {
            String::from_iter(self.letter_board[row])
                .split_ascii_whitespace()
                .filter(|str| str.len() > 1)
                .for_each(|str| {
                    let known = self
                        .words
                        .iter()
                        .find(|word| word.eq_ignore_ascii_case(str));
                    if known.is_none() {
                        if self.debug {
                            println!("invalid horizontal string: {}", str);
                        }
                        score -= (ROWS * COLUMNS * str.len()) as isize;
                    }
                });
        }

        for column in 0..COLUMNS {
            String::from_iter((0..ROWS).map(|row| self.letter_board[row][column]))
                .split_ascii_whitespace()
                .filter(|str| str.len() > 1)
                .for_each(|str| {
                    let known = self
                        .words
                        .iter()
                        .find(|word| word.eq_ignore_ascii_case(str));
                    if known.is_none() {
                        if self.debug {
                            println!("invalid vertical string: {}", str);
                        }
                        score -= (ROWS * COLUMNS * str.len()) as isize;
                    }
                });
        }

        Some(score)
    }
}

impl ScrabbleFitness {
    pub fn new(
        words: Vec<&'static str>,
        row_scores: Vec<isize>,
        column_scores: Vec<isize>,
        debug: bool,
    ) -> Self {
        let mut position_map = HashMap::with_capacity(ROWS * COLUMNS);
        for row in 0..ROWS {
            for column in 0..COLUMNS {
                position_map.insert((row, column), Vec::with_capacity(words.len()));
            }
        }

        let mut related_word_ids: HashMap<usize, HashSet<usize>> =
            HashMap::with_capacity(words.len());
        for index in 0..words.len() {
            related_word_ids.insert(index, HashSet::with_capacity(words.len()));
        }

        let letter_board: [[char; COLUMNS]; ROWS] = [[' '; COLUMNS]; ROWS];

        Self {
            words,
            row_scores,
            column_scores,
            debug,
            position_map,
            related_word_ids,
            letter_board,
        }
    }
    pub fn recursive_touching_sets(
        set: &HashSet<usize>,
        data: &HashMap<usize, HashSet<usize>>,
        acc: &mut HashSet<usize>,
    ) {
        set.iter().for_each(|index| {
            if acc.insert(*index) {
                if let Some(next_set) = data.get(index) {
                    ScrabbleFitness::recursive_touching_sets(next_set, data, acc);
                }
            }
        })
    }
}

fn main() {
    let row_scores: Vec<isize> = (0..ROWS)
        .rev()
        .zip(0..ROWS)
        .map(|(v1, v2)| (v1.min(v2) + 1) as isize)
        .collect();
    let column_scores: Vec<isize> = (0..COLUMNS)
        .rev()
        .zip(0..COLUMNS)
        .map(|(v1, v2)| (v1.min(v2) + 1) as isize)
        .collect();

    println!("{:?}", row_scores);
    println!("{:?}", column_scores);

    let words: Vec<&'static str> = vec!["ada", "aad", "bas"];
    let mut allele_lists: Vec<Vec<WordPosition>> = vec![vec![]; words.len()];
    words.iter().enumerate().for_each(|(index, word)| {
        for row in 0..ROWS {
            for column in 0..=(COLUMNS - word.len()) {
                allele_lists[index].push(WordPosition(row, column, Orientation::Horizontal));
            }
        }
        for row in 0..=(ROWS - word.len()) {
            for column in 0..COLUMNS {
                allele_lists[index].push(WordPosition(row, column, Orientation::Vertical));
            }
        }
    });

    let mut rng = SmallRng::from_entropy();
    let genotype = MultiDiscreteGenotype::builder()
        .with_allele_lists(allele_lists)
        .build()
        .unwrap();

    println!("{}", genotype);

    let mut permutate = Permutate::builder()
        .with_genotype(genotype)
        .with_fitness(ScrabbleFitness::new(
            words.clone(),
            row_scores.clone(),
            column_scores.clone(),
            false,
        ))
        .build()
        .unwrap();

    let now = std::time::Instant::now();

    if true {
        let guard = pprof::ProfilerGuardBuilder::default()
            .frequency(1000)
            .blocklist(&["libc", "libgcc", "pthread", "vdso"])
            .build()
            .unwrap();

        permutate.call(&mut rng);

        if let Ok(report) = guard.report().build() {
            let file = std::fs::File::create("flamegraph_scrabble.svg").unwrap();
            report.flamegraph(file).unwrap();
        };
    } else {
        permutate.call(&mut rng);
    }

    let duration = now.elapsed();
    println!("{:?}", duration);

    println!("{}", permutate);

    if let Some(best_chromosome) = permutate.best_chromosome() {
        if let Some(_fitness_score) = best_chromosome.fitness_score {
            let mut letter_board: [[char; COLUMNS]; ROWS] = [['.'; COLUMNS]; ROWS];

            // debug info
            ScrabbleFitness::new(
                words.clone(),
                row_scores.clone(),
                column_scores.clone(),
                true,
            )
            .calculate_for_chromosome(&best_chromosome);

            best_chromosome
                .genes
                .iter()
                .enumerate()
                .for_each(|(index, value)| {
                    let word = words[index];
                    match *value {
                        WordPosition(row, column, Orientation::Horizontal) => {
                            word.chars().enumerate().for_each(|(char_index, char)| {
                                letter_board[row][column + char_index] = char;
                            })
                        }
                        WordPosition(row, column, Orientation::Vertical) => {
                            word.chars().enumerate().for_each(|(char_index, char)| {
                                letter_board[row + char_index][column] = char;
                            })
                        }
                    }
                });

            letter_board.iter().for_each(|columns| {
                let string = String::from_iter(columns.iter());
                println!("{}", string);
            });
        } else {
            println!("Invalid solution with fitness score: None");
        }
    }
}
