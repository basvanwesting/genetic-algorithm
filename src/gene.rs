use crate::context::Context;
use rand::seq::SliceRandom;
use rand::Rng;

pub type BinaryGene = bool;
pub type DiscreteGene = u8;
pub type ContinuousGene = f32;

pub trait Gene: Copy + Clone + std::fmt::Display + std::fmt::Debug {
    fn random<R: Rng>(_context: &mut Context<Self>, rng: &mut R) -> Self;
    fn mutate<R: Rng>(&mut self, _context: &mut Context<Self>, rng: &mut R);
}

impl Gene for BinaryGene {
    fn random<R: Rng>(_context: &mut Context<BinaryGene>, rng: &mut R) -> BinaryGene {
        rng.gen()
    }
    fn mutate<R: Rng>(&mut self, _context: &mut Context<BinaryGene>, _rng: &mut R) {
        *self = !*self;
    }
}

impl Gene for DiscreteGene {
    fn random<R: Rng>(context: &mut Context<DiscreteGene>, rng: &mut R) -> DiscreteGene {
        *context.gene_values.choose(rng).unwrap()
    }
    fn mutate<R: Rng>(&mut self, context: &mut Context<DiscreteGene>, rng: &mut R) {
        *self = DiscreteGene::random(context, rng);
    }
}

impl Gene for ContinuousGene {
    fn random<R: Rng>(_context: &mut Context<ContinuousGene>, rng: &mut R) -> ContinuousGene {
        rng.gen()
    }
    fn mutate<R: Rng>(&mut self, context: &mut Context<ContinuousGene>, rng: &mut R) {
        *self = ContinuousGene::random(context, rng);
    }
}
