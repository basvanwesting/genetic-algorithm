use chrono::{Datelike, NaiveDate, Weekday};
use genetic_algorithm::strategy::hill_climb::prelude::*;
use rand::prelude::*;
use rand::rngs::SmallRng;
use statrs::statistics::Statistics;
use std::collections::HashMap;

const NO_ASSIGN_PENALTY: isize = 1000;
const INVALID_ASSIGN_PENALTY: isize = 1000;
const STD_DEV_MULTIPLIER: f64 = 10.0;

#[derive(Debug, Eq, PartialEq, Hash)]
struct Adult {
    pub name: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub monday: bool,
    pub tuesday: bool,
    pub wednesday: bool,
    pub thursday: bool,
    pub friday: bool,
    pub times: usize,
}

impl Adult {
    pub fn new(name: String, start_date: NaiveDate, end_date: NaiveDate) -> Adult {
        Self {
            name,
            start_date,
            end_date,
            monday: true,
            tuesday: true,
            wednesday: true,
            thursday: true,
            friday: true,
            times: 0,
        }
    }
    pub fn allow_weekday(&self, weekday: Weekday) -> bool {
        match weekday {
            Weekday::Mon => self.monday,
            Weekday::Tue => self.tuesday,
            Weekday::Wed => self.wednesday,
            Weekday::Thu => self.thursday,
            Weekday::Fri => self.friday,
            _ => false,
        }
    }
}

#[derive(Clone, Debug)]
struct RecessFitness<'a>(pub &'a Vec<Adult>, pub &'a Vec<NaiveDate>, pub bool);
impl<'a> Fitness for RecessFitness<'a> {
    type Genotype = DiscreteGenotype;
    fn calculate_for_chromosome(
        &mut self,
        chromosome: &Chromosome<Self::Genotype>,
    ) -> Option<FitnessValue> {
        let adults = self.0;
        let dates = self.1;
        let debug = self.2;
        let mut score = 0;

        let mut assigns: HashMap<&Adult, Vec<&NaiveDate>> = HashMap::new();
        chromosome
            .genes
            .iter()
            .enumerate()
            .for_each(|(index, value)| {
                let date = &dates[index];
                let adult = &adults[*value];
                if !adult.allow_weekday(date.weekday()) {
                    score += INVALID_ASSIGN_PENALTY;
                }
                if adult.start_date > *date {
                    score += INVALID_ASSIGN_PENALTY;
                }
                if adult.end_date < *date {
                    score += INVALID_ASSIGN_PENALTY;
                }
                assigns
                    .entry(adult)
                    .and_modify(|dates| dates.push(date))
                    .or_insert(vec![date]);
            });

        let mut intervals: Vec<f64> = vec![];
        adults.iter().for_each(|adult| match assigns.get(adult) {
            Some(dates) => {
                dates.windows(2).for_each(|pair| {
                    let duration = *pair[1] - *pair[0];
                    intervals.push(duration.num_days() as f64);
                });
            }
            None => {
                score += NO_ASSIGN_PENALTY;
            }
        });

        let min = Statistics::min(&intervals);
        score -= min as isize; // maximize min
        let max = Statistics::max(&intervals);
        score += max as isize; // minimize max
        let mean = Statistics::mean(&intervals);
        score -= mean as isize; // maximize mean
        let std_dev = Statistics::std_dev(&intervals);
        score += (STD_DEV_MULTIPLIER * std_dev) as isize; // minimize std_dev

        if debug {
            println!(
                "interval in days, min: {}, max: {}, mean: {}, std_dev: {}",
                min, max, mean, std_dev
            );
        }
        Some(score)
    }
}

fn main() {
    let default_start_date = NaiveDate::from_ymd(2022, 1, 1);
    let default_end_date = NaiveDate::from_ymd(2022, 12, 31);
    let adults: Vec<Adult> = vec![
        Adult::new("A".to_string(), default_start_date, default_end_date),
        Adult::new("B".to_string(), default_start_date, default_end_date),
        Adult::new("C".to_string(), default_start_date, default_end_date),
        Adult::new("D".to_string(), default_start_date, default_end_date),
        Adult::new("E".to_string(), default_start_date, default_end_date),
    ];
    let dates: Vec<NaiveDate> = vec![
        NaiveDate::from_ymd(2022, 1, 3),
        NaiveDate::from_ymd(2022, 1, 4),
        NaiveDate::from_ymd(2022, 1, 5),
        NaiveDate::from_ymd(2022, 1, 6),
        NaiveDate::from_ymd(2022, 1, 7),
        NaiveDate::from_ymd(2022, 1, 10),
        NaiveDate::from_ymd(2022, 1, 11),
        NaiveDate::from_ymd(2022, 1, 12),
        NaiveDate::from_ymd(2022, 1, 13),
        NaiveDate::from_ymd(2022, 1, 14),
        NaiveDate::from_ymd(2022, 1, 17),
        NaiveDate::from_ymd(2022, 1, 18),
        NaiveDate::from_ymd(2022, 1, 19),
        NaiveDate::from_ymd(2022, 1, 20),
        NaiveDate::from_ymd(2022, 1, 21),
    ];

    let mut rng = SmallRng::from_entropy();
    let genotype = DiscreteGenotype::builder()
        .with_genes_size(dates.len())
        .with_allele_list((0..adults.len()).collect())
        .build()
        .unwrap();

    println!("{}", genotype);

    let mut hill_climb = HillClimb::builder()
        .with_genotype(genotype)
        .with_variant(HillClimbVariant::Stochastic)
        .with_max_stale_generations(10000)
        .with_fitness(RecessFitness(&adults, &dates, false))
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .build()
        .unwrap();

    let now = std::time::Instant::now();
    hill_climb.call(&mut rng);
    let duration = now.elapsed();

    println!("{}", hill_climb);

    if let Some(best_chromosome) = hill_climb.best_chromosome() {
        if let Some(fitness_score) = best_chromosome.fitness_score {
            println!("Solution with fitness score: {}", fitness_score);

            let mut adult_counts = HashMap::new();
            best_chromosome
                .genes
                .iter()
                .enumerate()
                .for_each(|(index, value)| {
                    let date = &dates[index];
                    let adult = &adults[*value];
                    println!("{}: {}", date, adult.name);

                    adult_counts
                        .entry(adult)
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                });

            adult_counts.iter().for_each(|(adult, count)| {
                println!("{}: {}", adult.name, count);
            });

            RecessFitness(&adults, &dates, true).calculate_for_chromosome(&best_chromosome);
        }
    } else {
        println!("Invalid solution with fitness score: None");
    }
    println!("{:?}", duration);
}
