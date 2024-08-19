use genetic_algorithm::strategy::permutate::prelude::*;
use rand::prelude::*;
use rand::rngs::SmallRng;
use std::collections::{HashMap, HashSet};

type Row = usize;
type Column = usize;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WordPosition(pub Row, pub Column, pub Orientation);
impl Allele for WordPosition {}

#[derive(Clone, Debug)]
struct ScrabbleFitness {
    pub words: Vec<&'static str>,
    pub rows: usize,
    pub columns: usize,
    pub row_scores: Vec<isize>,
    pub column_scores: Vec<isize>,
    pub debug: bool,
    position_map: HashMap<(usize, usize), Vec<(usize, char)>>,
    related_word_ids: HashMap<usize, HashSet<usize>>,
    letter_board: Vec<Vec<char>>,
}
impl Fitness for ScrabbleFitness {
    type Allele = WordPosition;
    fn calculate_for_chromosome(
        &mut self,
        chromosome: &Chromosome<Self::Allele>,
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
                        self.letter_board[*row][*column] = '?';
                        score -= (self.rows * self.columns) as isize;
                        if self.debug {
                            println!(
                                "conflicting char: ({}, {}), at: ({}, {})",
                                *first_char, *second_char, *row, *column
                            );
                        }
                    }
                }
                rest => {
                    self.letter_board[*row][*column] = '?';
                    score -= (self.rows * self.columns * rest.len()) as isize;
                    if self.debug {
                        println!("conflicting multiple chars at: ({}, {})", *row, *column);
                    }
                }
            }
        }

        let mut touching_word_ids: HashSet<usize> = HashSet::with_capacity(self.words.len());
        let starting_set = self
            .related_word_ids
            .values()
            .max_by_key(|set| set.len())
            .unwrap();
        ScrabbleFitness::recursive_touching_sets(
            starting_set,
            &self.related_word_ids,
            &mut touching_word_ids,
        );

        self.words.iter().enumerate().for_each(|(index, word)| {
            if !touching_word_ids.contains(&index) {
                score -= (self.rows * self.columns * word.len()) as isize;
                if self.debug {
                    println!("word not touching main group: {}", word);
                }
            }
        });

        (0..self.rows).for_each(|row| {
            String::from_iter(self.letter_board[row].iter())
                .split_ascii_whitespace()
                .filter(|str| str.len() > 1)
                .for_each(|str| {
                    let known = self
                        .words
                        .iter()
                        .find(|word| word.eq_ignore_ascii_case(str));
                    if known.is_none() {
                        score -= (self.rows * self.columns * str.len()) as isize;
                        if self.debug {
                            println!("invalid horizontal string: {}", str);
                        }
                    }
                });
        });

        (0..self.columns).for_each(|column| {
            String::from_iter((0..self.rows).map(|row| self.letter_board[row][column]))
                .split_ascii_whitespace()
                .filter(|str| str.len() > 1)
                .for_each(|str| {
                    let known = self
                        .words
                        .iter()
                        .find(|word| word.eq_ignore_ascii_case(str));
                    if known.is_none() {
                        score -= (self.rows * self.columns * str.len()) as isize;
                        if self.debug {
                            println!("invalid vertical string: {}", str);
                        }
                    }
                });
        });

        Some(score)
    }
}

impl ScrabbleFitness {
    pub fn new(
        words: Vec<&'static str>,
        rows: usize,
        columns: usize,
        row_scores: Vec<isize>,
        column_scores: Vec<isize>,
        debug: bool,
    ) -> Self {
        let mut position_map = HashMap::with_capacity(rows * columns);
        for row in 0..rows {
            for column in 0..columns {
                position_map.insert((row, column), Vec::with_capacity(words.len()));
            }
        }

        let mut related_word_ids: HashMap<usize, HashSet<usize>> =
            HashMap::with_capacity(words.len());
        for index in 0..words.len() {
            related_word_ids.insert(index, HashSet::with_capacity(words.len()));
        }

        let letter_board: Vec<Vec<char>> = vec![vec![' '; columns]; rows];

        Self {
            words,
            rows,
            columns,
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

#[derive(Clone)]
pub struct CustomReporter(usize);
impl PermutateReporter for CustomReporter {
    type Allele = WordPosition;

    fn on_new_generation(
        &mut self,
        state: &PermutateState<Self::Allele>,
        _config: &PermutateConfig,
    ) {
        if state.current_generation() % self.0 == 0 {
            let width = state.total_population_size.to_string().len();
            println!(
                "progress: {:3.3}%, current_generation: {:>width$}, best_generation: {:>width$}",
                BigUint::from(state.current_generation() * 100) / &state.total_population_size,
                state.current_generation(),
                state.best_generation(),
            );
        }
    }

    fn on_new_best_chromosome(
        &mut self,
        state: &PermutateState<Self::Allele>,
        _config: &PermutateConfig,
    ) {
        println!(
            "new best - current_generation: {}, best_fitness_score: {:?}, best_genes: {:?}",
            state.current_generation(),
            state.best_fitness_score(),
            state.best_chromosome_as_ref().map(|c| &c.genes)
        );
    }
}

#[derive(Clone)]
pub struct CustomLogReporter(usize);
impl PermutateReporter for CustomLogReporter {
    type Allele = WordPosition;

    fn on_new_generation(
        &mut self,
        state: &PermutateState<Self::Allele>,
        _config: &PermutateConfig,
    ) {
        if state.current_generation() % self.0 == 0 {
            log::info!(
                "logger - current_generation: {}, best_fitness_score: {:?}",
                state.current_generation(),
                state.best_fitness_score(),
            );
        }
        log::debug!(
            "logger - current_generation: {}, best_generation: {}, best_fitness_score: {:?}",
            state.current_generation(),
            state.best_generation(),
            state.best_fitness_score(),
        );
        log::trace!(
            "logger - current_generation: {}, best_generation: {}, best_fitness_score: {:?}, genes: {:?}",
            state.current_generation(),
            state.best_generation(),
            state.best_fitness_score(),
            state
                .best_chromosome_as_ref()
                .map_or(vec![], |c| c.genes.clone()),
        );
    }
}

fn main() {
    env_logger::init();

    let rows = 5;
    let columns = 5;
    let row_scores: Vec<isize> = (0..rows)
        .rev()
        .zip(0..rows)
        .map(|(v1, v2)| (v1.min(v2) + 1) as isize)
        .collect();
    let column_scores: Vec<isize> = (0..columns)
        .rev()
        .zip(0..columns)
        .map(|(v1, v2)| (v1.min(v2) + 1) as isize)
        .collect();

    println!("{:?}", row_scores);
    println!("{:?}", column_scores);

    //let words: Vec<&'static str> = vec!["ada", "aad", "bas"];
    let words: Vec<&'static str> = vec!["bean", "glee", "edge", "light", "note"];
    let mut allele_lists: Vec<Vec<WordPosition>> = vec![vec![]; words.len()];
    words.iter().enumerate().for_each(|(index, word)| {
        for row in 0..rows {
            for column in 0..=(columns - word.len()) {
                allele_lists[index].push(WordPosition(row, column, Orientation::Horizontal));
            }
        }
        for row in 0..=(rows - word.len()) {
            for column in 0..columns {
                allele_lists[index].push(WordPosition(row, column, Orientation::Vertical));
            }
        }
    });

    let mut rng = SmallRng::from_entropy();
    let genotype = MultiListGenotype::builder()
        .with_allele_lists(allele_lists)
        .build()
        .unwrap();

    println!("{}", genotype);

    let mut permutate = Permutate::builder()
        .with_genotype(genotype)
        .with_fitness(ScrabbleFitness::new(
            words.clone(),
            rows,
            columns,
            row_scores.clone(),
            column_scores.clone(),
            false,
        ))
        .with_multithreading(true)
        // .with_reporter(PermutateReporterSimple::new(100_000))
        // .with_reporter(PermutateReporterLog::new())
        .with_reporter(CustomReporter(100_000))
        // .with_reporter(CustomLogReporter(100_000))
        .build()
        .unwrap();

    let now = std::time::Instant::now();

    if false {
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
            // debug info
            let mut fitness = ScrabbleFitness::new(
                words.clone(),
                rows,
                columns,
                row_scores.clone(),
                column_scores.clone(),
                true,
            );
            fitness.calculate_for_chromosome(&best_chromosome);
            fitness.letter_board.iter().for_each(|columns| {
                let string = String::from_iter(columns.iter());
                println!("{}", string.replace(" ", "."));
            });
        } else {
            println!("Invalid solution with fitness score: None");
        }
    }
}
