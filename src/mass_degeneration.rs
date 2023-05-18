//! MassDegeneration is a config struct for the mechanism
use std::fmt;

#[derive(Clone, Debug)]
pub struct MassDegeneration {
    pub min_fitness_score_stddev: f32,
    pub max_fitness_score_stddev: f32,
}

impl MassDegeneration {
    pub fn new(min_fitness_score_stddev: f32, max_fitness_score_stddev: f32) -> Self {
        Self {
            min_fitness_score_stddev,
            max_fitness_score_stddev,
        }
    }
}

impl fmt::Display for MassDegeneration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "fitness_score_stddev, min: {:3.3}, max: {:3.3}",
            self.min_fitness_score_stddev, self.max_fitness_score_stddev
        )
    }
}
