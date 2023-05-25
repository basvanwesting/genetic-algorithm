use super::Crossover;
use crate::genotype::Genotype;
use crate::population::Population;
use rand::Rng;

/// Children are clones of the parents, effectively doubling the population if you keep the parents.
/// Acts as no-op if the parents are not kept.
///
/// Allowed for unique genotypes.
#[derive(Clone, Debug)]
pub struct Clone {
    pub keep_parent: bool,
}
impl Crossover for Clone {
    fn call<T: Genotype, R: Rng>(
        &mut self,
        _genotype: &T,
        population: &mut Population<T>,
        _rng: &mut R,
    ) {
        if self.keep_parent {
            let mut clones = population.clone();
            clones.reset_age();
            population.merge(&mut clones);
        }
    }

    fn require_crossover_indexes(&self) -> bool {
        false
    }
    fn require_crossover_points(&self) -> bool {
        false
    }
}

impl Clone {
    pub fn new(keep_parent: bool) -> Self {
        Self { keep_parent }
    }
}
