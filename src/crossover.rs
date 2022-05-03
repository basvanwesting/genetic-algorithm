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
}

#[derive(Clone, Debug)]
pub enum Crossovers {
    Individual,
    All,
    Range,
    Cloning,
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
            Crossovers::Individual => {
                CrossoverIndividual(keep_parent).call(genotype, population, rng)
            }
            Crossovers::All => CrossoverAll(keep_parent).call(genotype, population, rng),
            Crossovers::Range => CrossoverRange(keep_parent).call(genotype, population, rng),
            Crossovers::Cloning => CrossoverCloning(keep_parent).call(genotype, population, rng),
        }
    }
}

mod individual;
pub use self::individual::Individual as CrossoverIndividual;

mod all;
pub use self::all::All as CrossoverAll;

mod range;
pub use self::range::Range as CrossoverRange;

mod cloning;
pub use self::cloning::Cloning as CrossoverCloning;
