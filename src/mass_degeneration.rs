//! MassDegeneration is a config struct for the mechanism
use std::fmt;

#[derive(Clone, Debug)]
pub struct MassDegeneration {
    pub uniformity_threshold: f32,
    pub number_of_rounds: usize,
}

impl MassDegeneration {
    pub fn new(uniformity_threshold: f32, number_of_rounds: usize) -> Self {
        Self {
            uniformity_threshold,
            number_of_rounds,
        }
    }
}

impl fmt::Display for MassDegeneration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "uniformity_threshold: {:3.3}, number_of_rounds: {}",
            self.uniformity_threshold, self.number_of_rounds
        )
    }
}
