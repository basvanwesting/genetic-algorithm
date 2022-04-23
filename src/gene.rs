use crate::genotype::Genotype;
use rand::seq::SliceRandom;
use rand::Rng;

pub type BinaryGene = bool;
pub type DiscreteGene = u8;
pub type ContinuousGene = f32;

pub trait Gene: Copy + Clone + std::fmt::Display + std::fmt::Debug {
    fn random<R: Rng>(_genotype: &Genotype<Self>, rng: &mut R) -> Self;
    fn mutate<R: Rng>(&mut self, _genotype: &Genotype<Self>, rng: &mut R);
}

impl Gene for BinaryGene {
    fn random<R: Rng>(_genotype: &Genotype<BinaryGene>, rng: &mut R) -> BinaryGene {
        rng.gen()
    }
    fn mutate<R: Rng>(&mut self, _genotype: &Genotype<BinaryGene>, _rng: &mut R) {
        *self = !*self;
    }
}

impl Gene for DiscreteGene {
    fn random<R: Rng>(genotype: &Genotype<DiscreteGene>, rng: &mut R) -> DiscreteGene {
        *genotype.gene_values.choose(rng).unwrap()
    }
    fn mutate<R: Rng>(&mut self, genotype: &Genotype<DiscreteGene>, rng: &mut R) {
        *self = DiscreteGene::random(genotype, rng);
    }
}

impl Gene for ContinuousGene {
    fn random<R: Rng>(_genotype: &Genotype<ContinuousGene>, rng: &mut R) -> ContinuousGene {
        rng.gen()
    }
    fn mutate<R: Rng>(&mut self, genotype: &Genotype<ContinuousGene>, rng: &mut R) {
        *self = ContinuousGene::random(genotype, rng);
    }
}
