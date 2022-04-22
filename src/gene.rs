use crate::context::Context;
//use crate::global_rand;
//use rand::prelude::*;
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
        //global_rand::gen()
        rng.gen()
    }
    fn mutate<R: Rng>(&mut self, _context: &mut Context<BinaryGene>, _rng: &mut R) {
        *self = !*self;
    }
}

impl Gene for DiscreteGene {
    fn random<R: Rng>(context: &mut Context<DiscreteGene>, rng: &mut R) -> DiscreteGene {
        //*global_rand::choose(&context.gene_values)
        *context.gene_values.choose(rng).unwrap()
    }
    fn mutate<R: Rng>(&mut self, context: &mut Context<DiscreteGene>, rng: &mut R) {
        *self = DiscreteGene::random(context, rng);
    }
}

impl Gene for ContinuousGene {
    fn random<R: Rng>(_context: &mut Context<ContinuousGene>, rng: &mut R) -> ContinuousGene {
        //global_rand::gen()
        rng.gen()
    }
    fn mutate<R: Rng>(&mut self, context: &mut Context<ContinuousGene>, rng: &mut R) {
        *self = ContinuousGene::random(context, rng);
    }
}
