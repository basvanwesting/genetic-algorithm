use super::{Crossover, KeepParent};
use crate::genotype::Genotype;
use crate::population::Population;
use rand::Rng;

/// Children are clones of the parents, effectively doubling the population if you keep the parents.
/// Acts as no-op if the parents are not kept.
///
/// Allowed for unique genotypes.
#[derive(Clone, Debug)]
pub struct Clone(pub KeepParent);
impl Crossover for Clone {
    fn call<T: Genotype, R: Rng>(
        &self,
        _genotype: &T,
        population: &mut Population<T>,
        _rng: &mut R,
    ) {
        if self.0 {
            let mut clones = population.clone();
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
