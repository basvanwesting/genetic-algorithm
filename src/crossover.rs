mod all;
mod clone;
mod range;
mod single;

pub use self::all::All as CrossoverAll;
pub use self::clone::Clone as CrossoverClone;
pub use self::range::Range as CrossoverRange;
pub use self::single::Single as CrossoverSingle;

use crate::genotype::Genotype;
use crate::population::Population;
use rand::Rng;

pub trait Crossover: Clone + std::fmt::Debug {
    fn call<T: Genotype, R: Rng>(
        &self,
        genotype: &T,
        population: Population<T>,
        rng: &mut R,
    ) -> Population<T>;

    fn allow_unique_genotype(&self) -> bool {
        false
    }
}

#[derive(Clone, Debug)]
pub enum Crossovers {
    Single,
    All,
    Range,
    Clone,
}
pub type KeepParent = bool;

#[derive(Clone, Debug)]
pub struct CrossoverDispatch(pub Crossovers, pub KeepParent);
impl Crossover for CrossoverDispatch {
    fn call<T: Genotype, R: Rng>(
        &self,
        genotype: &T,
        population: Population<T>,
        rng: &mut R,
    ) -> Population<T> {
        let keep_parent = self.1;
        match self.0 {
            Crossovers::Single => CrossoverSingle(keep_parent).call(genotype, population, rng),
            Crossovers::All => CrossoverAll(keep_parent).call(genotype, population, rng),
            Crossovers::Range => CrossoverRange(keep_parent).call(genotype, population, rng),
            Crossovers::Clone => CrossoverClone(keep_parent).call(genotype, population, rng),
        }
    }
}
