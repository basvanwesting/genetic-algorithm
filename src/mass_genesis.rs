//! MassGenesis is a config struct for the mechanism
use std::fmt;

#[derive(Clone, Debug)]
pub struct MassGenesis {
    pub uniformity_threshold: f32,
}

impl MassGenesis {
    pub fn new(uniformity_threshold: f32) -> Self {
        Self {
            uniformity_threshold,
        }
    }
}

impl fmt::Display for MassGenesis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "uniformity_threshold: {:3.3}", self.uniformity_threshold,)
    }
}
