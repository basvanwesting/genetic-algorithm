use chrono::{Datelike, NaiveDate, Weekday};
use genetic_algorithm::strategy::hill_climb::prelude::*;
use rand::prelude::*;
use rand::rngs::SmallRng;
use statrs::statistics::Statistics;
use std::collections::{HashMap, HashSet};

const NO_ASSIGN_PENALTY: isize = 1000;
const INVALID_ASSIGN_PENALTY: isize = 1000;
const STD_DEV_MULTIPLIER: f64 = 10.0;

#[derive(Debug, Eq, PartialEq, Hash)]
struct Adult {
    pub name: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub number_of_times: usize,
    pub monday: bool,
    pub tuesday: bool,
    pub wednesday: bool,
    pub thursday: bool,
    pub friday: bool,
}

impl Adult {
    pub fn new(
        name: String,
        start_date: NaiveDate,
        end_date: NaiveDate,
        number_of_times: usize,
    ) -> Adult {
        Self {
            name,
            start_date,
            end_date,
            number_of_times,
            monday: true,
            tuesday: true,
            wednesday: true,
            thursday: true,
            friday: true,
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
    type Genotype = UniqueGenotype;
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
    // INIT
    let default_start_date = NaiveDate::from_ymd(2022, 1, 1);
    let default_end_date = NaiveDate::from_ymd(2022, 12, 31);
    let adults: Vec<Adult> = vec![
        Adult::new("A".to_string(), default_start_date, default_end_date, 6),
        Adult::new("B".to_string(), default_start_date, default_end_date, 5),
        Adult::new("C".to_string(), default_start_date, default_end_date, 5),
        Adult::new("D".to_string(), default_start_date, default_end_date, 5),
        Adult::new("E".to_string(), default_start_date, default_end_date, 5),
    ];

    let periods = vec![
        (
            NaiveDate::from_ymd(2022, 1, 3),
            NaiveDate::from_ymd(2022, 1, 21),
        ),
        (
            NaiveDate::from_ymd(2022, 2, 3),
            NaiveDate::from_ymd(2022, 2, 21),
        ),
    ];

    let exceptions = vec![
        NaiveDate::from_ymd(2022, 1, 4),
        NaiveDate::from_ymd(2022, 2, 4),
    ];

    // SETUP
    let exceptions_set = exceptions
        .into_iter()
        .fold(HashSet::new(), |mut acc, date| {
            acc.insert(date);
            acc
        });

    let mut dates: Vec<NaiveDate> = vec![];
    periods.iter().for_each(|(start_date, end_date)| {
        let num_days = (*end_date - *start_date).num_days() as usize + 1;
        start_date
            .iter_days()
            .take(num_days)
            .for_each(|date| match date.weekday() {
                Weekday::Sat => (),
                Weekday::Sun => (),
                _ => {
                    if !exceptions_set.contains(&date) {
                        dates.push(date)
                    }
                }
            });
    });

    println!("number of dates to plan: {}", dates.len());

    // RUN

    let mut rng = SmallRng::from_entropy();
    let genotype = UniqueGenotype::builder()
        .with_allele_list(
            adults
                .iter()
                .enumerate()
                .flat_map(|(index, adult)| vec![index; adult.number_of_times])
                .collect(),
        )
        .build()
        .unwrap();

    println!("{}", genotype);

    let mut hill_climb = HillClimb::builder()
        .with_genotype(genotype)
        .with_variant(HillClimbVariant::Stochastic)
        .with_max_stale_generations(100000)
        .with_fitness(RecessFitness(&adults, &dates, false))
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .build()
        .unwrap();

    let now = std::time::Instant::now();
    hill_climb.call(&mut rng);
    let duration = now.elapsed();

    println!("{}", hill_climb);

    // REPORT

    if let Some(best_chromosome) = hill_climb.best_chromosome() {
        if let Some(fitness_score) = best_chromosome.fitness_score {
            println!("Solution with fitness score: {}", fitness_score);

            best_chromosome
                .genes
                .iter()
                .enumerate()
                .for_each(|(index, value)| {
                    let date = &dates[index];
                    let adult = &adults[*value];
                    println!("{}: {}", date, adult.name);
                });

            RecessFitness(&adults, &dates, true).calculate_for_chromosome(&best_chromosome);
        }
    } else {
        println!("Invalid solution with fitness score: None");
    }
    println!("{:?}", duration);
}
