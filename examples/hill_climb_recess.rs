use chrono::{Datelike, NaiveDate, Weekday};
use genetic_algorithm::strategy::hill_climb::prelude::*;
use itertools::Itertools;
use rand::prelude::*;
use rand::rngs::SmallRng;
use statrs::statistics::Statistics;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

const INVALID_ASSIGN_PENALTY: f64 = 100_000.0;
const MEAN_MULTIPLIER: f64 = 100.0;
const STD_DEV_MULTIPLIER: f64 = 1000.0;

#[derive(Debug)]
struct Adult {
    pub name: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub weight: f64,
    pub number_of_times: usize,
    pub monday: bool,
    pub tuesday: bool,
    pub wednesday: bool,
    pub thursday: bool,
    pub friday: bool,
}

impl Adult {
    pub fn new(name: String, start_date: NaiveDate, end_date: NaiveDate) -> Adult {
        Self {
            name,
            start_date,
            end_date,
            weight: 0.0,
            number_of_times: 0,
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
impl PartialEq for Adult {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl Eq for Adult {}
impl Hash for Adult {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

#[derive(Clone, Debug)]
struct RecessFitness<'a>(pub &'a Vec<Adult>, pub &'a Vec<NaiveDate>);
impl<'a> Fitness for RecessFitness<'a> {
    type Genotype = UniqueGenotype;
    fn calculate_for_chromosome(
        &mut self,
        chromosome: &Chromosome<Self::Genotype>,
    ) -> Option<FitnessValue> {
        let adults = self.0;
        let dates = self.1;
        let mut score = 0.0;

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
        adults.iter().for_each(|adult| {
            assigns.get(adult).unwrap().windows(2).for_each(|pair| {
                let duration = *pair[1] - *pair[0];
                intervals.push(duration.num_days() as f64);
            });
        });

        let mean = Statistics::mean(&intervals);
        score -= MEAN_MULTIPLIER * mean; // maximize mean
        let std_dev = Statistics::population_std_dev(&intervals);
        score += STD_DEV_MULTIPLIER * std_dev; // minimize std_dev

        Some(score as isize)
    }
}

fn main() {
    // INPUT
    let default_start_date = NaiveDate::from_ymd(2022, 1, 1);
    let alt_start_date = NaiveDate::from_ymd(2022, 6, 1);
    let default_end_date = NaiveDate::from_ymd(2022, 12, 31);
    let mut adults: Vec<Adult> = vec![
        Adult::new("A".to_string(), alt_start_date, default_end_date),
        Adult::new("B".to_string(), alt_start_date, default_end_date),
        Adult::new("C".to_string(), alt_start_date, default_end_date),
        Adult::new("D".to_string(), alt_start_date, default_end_date),
        Adult::new("E".to_string(), alt_start_date, default_end_date),
        Adult::new("F".to_string(), alt_start_date, default_end_date),
        Adult::new("G".to_string(), alt_start_date, default_end_date),
        Adult::new("H".to_string(), alt_start_date, default_end_date),
        Adult::new("I".to_string(), alt_start_date, default_end_date),
        Adult::new("J".to_string(), default_start_date, default_end_date),
        Adult::new("K".to_string(), default_start_date, default_end_date),
        Adult::new("L".to_string(), default_start_date, default_end_date),
        Adult::new("M".to_string(), default_start_date, default_end_date),
        Adult::new("N".to_string(), default_start_date, default_end_date),
        Adult::new("O".to_string(), default_start_date, default_end_date),
        Adult::new("P".to_string(), default_start_date, default_end_date),
        Adult::new("Q".to_string(), default_start_date, default_end_date),
        Adult::new("R".to_string(), default_start_date, default_end_date),
        Adult::new("S".to_string(), default_start_date, default_end_date),
        Adult::new("T".to_string(), default_start_date, default_end_date),
        Adult::new("U".to_string(), default_start_date, default_end_date),
        Adult::new("V".to_string(), default_start_date, default_end_date),
        Adult::new("W".to_string(), default_start_date, default_end_date),
        Adult::new("X".to_string(), default_start_date, default_end_date),
    ];

    let periods = vec![
        (
            NaiveDate::from_ymd(2022, 1, 1),
            NaiveDate::from_ymd(2022, 3, 1),
        ),
        (
            NaiveDate::from_ymd(2022, 4, 1),
            NaiveDate::from_ymd(2022, 6, 1),
        ),
        (
            NaiveDate::from_ymd(2022, 7, 1),
            NaiveDate::from_ymd(2022, 9, 1),
        ),
        (
            NaiveDate::from_ymd(2022, 10, 1),
            NaiveDate::from_ymd(2022, 12, 1),
        ),
    ];

    let exceptions = vec![
        NaiveDate::from_ymd(2022, 1, 4),
        NaiveDate::from_ymd(2022, 2, 4),
    ];

    // SETUP

    let mut dates: Vec<NaiveDate> = vec![];
    periods.iter().for_each(|(start_date, end_date)| {
        let num_days = (*end_date - *start_date).num_days() as usize + 1;
        start_date
            .iter_days()
            .take(num_days)
            .for_each(|date| match date.weekday() {
                Weekday::Mon | Weekday::Tue | Weekday::Thu => {
                    if !exceptions.contains(&date) {
                        dates.push(date);
                        adults.iter_mut().for_each(|adult| {
                            if adult.start_date <= date && adult.end_date >= date {
                                adult.weight += 1.0
                            }
                        });
                    }
                }
                _ => (),
            });
    });
    let mut number_of_dates_to_assign = dates.len();

    let total_weight: f64 = adults.iter().map(|adult| adult.weight).sum();
    adults.iter_mut().for_each(|adult| {
        adult.weight /= total_weight; // normalize weight
        adult.weight *= dates.len() as f64; // weight in terms of fractional dates

        // assign modulo of fractional dates
        let assign_dates = adult.weight.floor();
        adult.number_of_times += assign_dates as usize;
        adult.weight -= assign_dates;
        number_of_dates_to_assign -= assign_dates as usize;
    });

    // assign remainder of fractional dates
    adults
        .iter_mut()
        .sorted_by(|b, a| a.weight.partial_cmp(&b.weight).unwrap())
        .take(number_of_dates_to_assign)
        .for_each(|adult| {
            adult.weight -= 1.0;
            adult.number_of_times += 1;
        });

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
        .with_fitness(RecessFitness(&adults, &dates))
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

            let mut assigns: HashMap<&Adult, Vec<&NaiveDate>> = HashMap::new();
            let mut all_intervals: Vec<f64> = vec![];
            best_chromosome
                .genes
                .iter()
                .enumerate()
                .for_each(|(index, value)| {
                    let date = &dates[index];
                    let adult = &adults[*value];
                    assigns
                        .entry(adult)
                        .and_modify(|dates| dates.push(date))
                        .or_insert(vec![date]);

                    println!("{}: {}", date, adult.name);
                });

            adults.iter().for_each(|adult| {
                let dates = assigns.get(adult).unwrap();

                let mut intervals: Vec<f64> = vec![];
                dates.windows(2).for_each(|pair| {
                    let interval = (*pair[1] - *pair[0]).num_days() as f64;
                    intervals.push(interval);
                    all_intervals.push(interval);
                });

                let count = dates.len();
                let max = Statistics::max(&intervals);
                let min = Statistics::min(&intervals);
                let mean = Statistics::mean(&intervals);
                let std_dev = Statistics::population_std_dev(&intervals);

                println!("{:?}", adult);
                println!(
                    "number_of_times: {}, interval in days, min: {}, max: {}, mean: {}, std_dev: {}",
                    count, min, max, mean, std_dev
                );
            });

            let count = dates.len();
            let max = Statistics::max(&all_intervals);
            let min = Statistics::min(&all_intervals);
            let mean = Statistics::mean(&all_intervals);
            let std_dev = Statistics::population_std_dev(&all_intervals);

            println!("=== OVERALL ===");
            println!(
                "number_of_times: {}, interval in days, min: {}, max: {}, mean: {}, std_dev: {}",
                count, min, max, mean, std_dev
            );
        }
    } else {
        println!("Invalid solution with fitness score: None");
    }
    println!("{:?}", duration);
}
