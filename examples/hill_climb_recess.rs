use chrono::{Datelike, NaiveDate, Weekday};
use genetic_algorithm::strategy::hill_climb::prelude::*;
use itertools::Itertools;
use statrs::statistics::Statistics;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

const INVALID_ASSIGN_PENALTY: FitnessValue = 1_000_000;
const MIN_ALLOWED_INTERVAL: i64 = 21;

#[derive(Debug)]
struct Adult {
    pub name: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub allowed_weekdays: Vec<Weekday>,
    pub weight_to_assign: f64,
    pub number_of_assigns: usize,
    pub number_of_assigns_modifier: isize,
}

impl Adult {
    pub fn new(
        name: String,
        start_date: NaiveDate,
        end_date: NaiveDate,
        allowed_weekdays: Vec<Weekday>,
        number_of_assigns_modifier: isize,
    ) -> Adult {
        Self {
            name,
            start_date,
            end_date,
            allowed_weekdays,
            number_of_assigns_modifier,
            weight_to_assign: 0.0,
            number_of_assigns: 0,
        }
    }
    pub fn allow_weekday(&self, weekday: Weekday) -> bool {
        self.allowed_weekdays.contains(&weekday)
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
    type Genotype = UniqueGenotype<usize>;
    fn calculate_for_chromosome(
        &mut self,
        chromosome: &LegacyChromosome<Self::Genotype>,
        _genotype: &Self::Genotype,
    ) -> Option<FitnessValue> {
        let adults = self.0;
        let dates = self.1;
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
                    score -= INVALID_ASSIGN_PENALTY;
                }
                if adult.start_date > *date || adult.end_date < *date {
                    score -= INVALID_ASSIGN_PENALTY;
                }
                assigns
                    .entry(adult)
                    .and_modify(|dates| dates.push(date))
                    .or_insert(vec![date]);
            });

        let mut min_interval: i64 = 999_999;
        adults.iter().for_each(|adult| {
            if let Some(dates) = assigns.get(adult) {
                dates.windows(2).for_each(|pair| {
                    let interval = (*pair[1] - *pair[0]).num_days();
                    if interval < MIN_ALLOWED_INTERVAL {
                        score -= INVALID_ASSIGN_PENALTY;
                    }
                    if min_interval > interval {
                        min_interval = interval;
                    }
                });
            }
        });

        score += min_interval as FitnessValue;
        Some(score as FitnessValue)
    }
}

fn main() {
    env_logger::init();

    // INPUT
    let default_start_date = NaiveDate::from_ymd_opt(2022, 1, 1).unwrap();
    let default_end_date = NaiveDate::from_ymd_opt(2022, 12, 31).unwrap();
    let default_allowed_weekdays = vec![Weekday::Mon, Weekday::Tue, Weekday::Thu];
    let alt_start_date = NaiveDate::from_ymd_opt(2022, 6, 1).unwrap();
    let alt_allowed_weekdays = vec![Weekday::Mon, Weekday::Tue, Weekday::Thu];
    let mut adults: Vec<Adult> = vec![
        Adult::new(
            "A".to_string(),
            alt_start_date,
            default_end_date,
            alt_allowed_weekdays.clone(),
            2,
        ),
        Adult::new(
            "B".to_string(),
            alt_start_date,
            default_end_date,
            alt_allowed_weekdays.clone(),
            -1,
        ),
        Adult::new(
            "C".to_string(),
            alt_start_date,
            default_end_date,
            alt_allowed_weekdays.clone(),
            0,
        ),
        Adult::new(
            "D".to_string(),
            alt_start_date,
            default_end_date,
            alt_allowed_weekdays.clone(),
            0,
        ),
        Adult::new(
            "E".to_string(),
            alt_start_date,
            default_end_date,
            default_allowed_weekdays.clone(),
            0,
        ),
        Adult::new(
            "F".to_string(),
            alt_start_date,
            default_end_date,
            default_allowed_weekdays.clone(),
            0,
        ),
        Adult::new(
            "G".to_string(),
            alt_start_date,
            default_end_date,
            default_allowed_weekdays.clone(),
            0,
        ),
        Adult::new(
            "H".to_string(),
            alt_start_date,
            default_end_date,
            default_allowed_weekdays.clone(),
            0,
        ),
        Adult::new(
            "I".to_string(),
            alt_start_date,
            default_end_date,
            default_allowed_weekdays.clone(),
            0,
        ),
        Adult::new(
            "J".to_string(),
            default_start_date,
            default_end_date,
            default_allowed_weekdays.clone(),
            0,
        ),
        Adult::new(
            "K".to_string(),
            default_start_date,
            default_end_date,
            default_allowed_weekdays.clone(),
            0,
        ),
        Adult::new(
            "L".to_string(),
            default_start_date,
            default_end_date,
            default_allowed_weekdays.clone(),
            0,
        ),
        Adult::new(
            "M".to_string(),
            default_start_date,
            default_end_date,
            default_allowed_weekdays.clone(),
            0,
        ),
        Adult::new(
            "N".to_string(),
            default_start_date,
            default_end_date,
            default_allowed_weekdays.clone(),
            0,
        ),
        Adult::new(
            "O".to_string(),
            default_start_date,
            default_end_date,
            default_allowed_weekdays.clone(),
            0,
        ),
        Adult::new(
            "P".to_string(),
            default_start_date,
            default_end_date,
            default_allowed_weekdays.clone(),
            0,
        ),
        Adult::new(
            "Q".to_string(),
            default_start_date,
            default_end_date,
            default_allowed_weekdays.clone(),
            0,
        ),
        Adult::new(
            "R".to_string(),
            default_start_date,
            default_end_date,
            default_allowed_weekdays.clone(),
            0,
        ),
        Adult::new(
            "S".to_string(),
            default_start_date,
            default_end_date,
            default_allowed_weekdays.clone(),
            0,
        ),
        Adult::new(
            "T".to_string(),
            default_start_date,
            default_end_date,
            default_allowed_weekdays.clone(),
            0,
        ),
        Adult::new(
            "U".to_string(),
            default_start_date,
            default_end_date,
            default_allowed_weekdays.clone(),
            0,
        ),
        Adult::new(
            "V".to_string(),
            default_start_date,
            default_end_date,
            default_allowed_weekdays.clone(),
            0,
        ),
        Adult::new(
            "W".to_string(),
            default_start_date,
            default_end_date,
            default_allowed_weekdays.clone(),
            0,
        ),
        Adult::new(
            "X".to_string(),
            default_start_date,
            default_end_date,
            default_allowed_weekdays.clone(),
            0,
        ),
    ];

    let periods = vec![
        (
            NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2022, 3, 1).unwrap(),
        ),
        (
            NaiveDate::from_ymd_opt(2022, 4, 1).unwrap(),
            NaiveDate::from_ymd_opt(2022, 6, 1).unwrap(),
        ),
        (
            NaiveDate::from_ymd_opt(2022, 7, 1).unwrap(),
            NaiveDate::from_ymd_opt(2022, 9, 1).unwrap(),
        ),
        (
            NaiveDate::from_ymd_opt(2022, 10, 1).unwrap(),
            NaiveDate::from_ymd_opt(2022, 12, 1).unwrap(),
        ),
    ];

    let exceptions = vec![
        NaiveDate::from_ymd_opt(2022, 1, 4).unwrap(),
        NaiveDate::from_ymd_opt(2022, 2, 4).unwrap(),
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
                                adult.weight_to_assign += 1.0
                            }
                        });
                    }
                }
                _ => (),
            });
    });
    let mut number_of_dates_to_assign = dates.len();

    let total_weight_to_assign: f64 = adults.iter().map(|adult| adult.weight_to_assign).sum();
    adults.iter_mut().for_each(|adult| {
        adult.weight_to_assign /= total_weight_to_assign; // normalize weight_to_assign
        adult.weight_to_assign *= dates.len() as f64; // weight_to_assign in terms of fractional dates
        adult.weight_to_assign += adult.number_of_assigns_modifier as f64;
    });

    // assign modulo of fractional dates
    adults.iter_mut().for_each(|adult| {
        let assign_dates = std::cmp::min(
            number_of_dates_to_assign,
            adult.weight_to_assign.floor() as usize,
        );
        adult.number_of_assigns += assign_dates;
        adult.weight_to_assign -= assign_dates as f64;
        number_of_dates_to_assign -= assign_dates;
    });

    // assign remainder of fractional dates
    adults
        .iter_mut()
        .sorted_by(|b, a| a.weight_to_assign.partial_cmp(&b.weight_to_assign).unwrap())
        .take(number_of_dates_to_assign)
        .for_each(|adult| {
            adult.weight_to_assign -= 1.0;
            adult.number_of_assigns += 1;
        });

    // RUN

    let genotype = UniqueGenotype::builder()
        .with_allele_list(
            adults
                .iter()
                .enumerate()
                .flat_map(|(index, adult)| vec![index; adult.number_of_assigns])
                .collect(),
        )
        .build()
        .unwrap();

    println!("{}", genotype);

    let hill_climb_builder = HillClimb::builder()
        .with_genotype(genotype)
        //.with_variant(HillClimbVariant::Stochastic)
        //.with_max_stale_generations(100)
        .with_variant(HillClimbVariant::SteepestAscent)
        .with_max_stale_generations(1)
        .with_par_fitness(true)
        .with_fitness(RecessFitness(&adults, &dates))
        .with_fitness_ordering(FitnessOrdering::Maximize);

    let now = std::time::Instant::now();
    let hill_climb = hill_climb_builder.call_repeatedly(1).unwrap();
    let duration = now.elapsed();

    println!("{}", hill_climb);

    // REPORT

    if let Some(best_chromosome) = hill_climb.best_chromosome() {
        if let Some(fitness_score) = best_chromosome.fitness_score {
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

                    if adult.start_date > *date || adult.end_date < *date {
                        println!("{}: {} INVALID out of start/end range", date, adult.name);
                    } else if !adult.allow_weekday(date.weekday()) {
                        println!("{}: {} INVALID on denied weekday", date, adult.name);
                    } else {
                        println!("{}: {}", date, adult.name);
                    }
                });

            adults.iter().for_each(|adult| {
                if let Some(dates) = assigns.get(adult) {
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
                    println!("{}", dates.iter().map(|date| format!("{} {}", date.weekday(), date)).join(", "));
                    println!(
                        "number_of_assigns: {}, interval in days, min: {}, max: {}, mean: {}, std_dev: {}",
                        count, min, max, mean, std_dev
                    );
                } else {
                    println!("{:?}", adult);
                    println!("no assigns");
                }
            });

            let count = dates.len();
            let max = Statistics::max(&all_intervals);
            let min = Statistics::min(&all_intervals);
            let mean = Statistics::mean(&all_intervals);
            let std_dev = Statistics::population_std_dev(&all_intervals);

            println!("=== OVERALL ===");
            println!(
                "number_of_assigns: {}, interval in days, min: {}, max: {}, mean: {}, std_dev: {}",
                count, min, max, mean, std_dev
            );

            if fitness_score >= INVALID_ASSIGN_PENALTY as isize {
                println!("Invalid solution with fitness score: {}", fitness_score);
            } else {
                println!("Valid solution with fitness score: {}", fitness_score);
            }
        }
    } else {
        println!("Invalid solution with fitness score: None");
    }
    println!("{:?}", duration);
}
