use crate::genotype::Genotype;
use crate::population::Population;
use rand::Rng;

pub trait Mutate: Clone + std::fmt::Debug {
    fn call<T: Genotype, R: Rng>(
        &self,
        genotype: &T,
        population: Population<T>,
        rng: &mut R,
    ) -> Population<T>;
}

#[derive(Clone, Debug)]
pub enum Mutates {
    Once,
}
pub type MutationProbability = f32;

#[derive(Clone, Debug)]
pub struct MutateDispatch(pub Mutates, pub MutationProbability);
impl Mutate for MutateDispatch {
    fn call<T: Genotype, R: Rng>(
        &self,
        genotype: &T,
        population: Population<T>,
        rng: &mut R,
    ) -> Population<T> {
        match self.0 {
            Mutates::Once => MutateOnce(self.1).call(genotype, population, rng),
        }
    }
}

mod once;
pub use self::once::Once as MutateOnce;
