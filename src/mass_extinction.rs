//! MassExtinction is a config struct for the mechanism
use std::fmt;

#[derive(Clone, Debug)]
pub struct MassExtinction {
    pub uniformity_threshold: f32,
    pub survival_rate: f32,
}

impl MassExtinction {
    pub fn new(uniformity_threshold: f32, survival_rate: f32) -> Self {
        Self {
            uniformity_threshold,
            survival_rate,
        }
    }
}

impl fmt::Display for MassExtinction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "uniformity_threshold: {:3.3}, survival_rate: {:3.3}",
            self.uniformity_threshold, self.survival_rate
        )
    }
}
