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
    SingleGene,
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
            Mutates::SingleGene => MutateSingleGene(self.1).call(genotype, population, rng),
        }
    }
}

mod single_gene;
pub use self::single_gene::SingleGene as MutateSingleGene;
