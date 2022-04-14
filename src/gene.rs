use crate::context::Context;
use rand::prelude::*;
use rand::seq::SliceRandom;

pub type BinaryGene = bool;
pub type DiscreteGene = u8;
pub type ContinuousGene = f32;

pub trait Gene: Copy + Clone {
    fn random(_context: &mut Context<Self>) -> Self;
    fn mutate(&mut self, _context: &mut Context<Self>);
}

impl Gene for BinaryGene {
    fn random(context: &mut Context<BinaryGene>) -> BinaryGene {
        context.rng.gen()
    }
    fn mutate(&mut self, _context: &mut Context<BinaryGene>) {
        *self = !*self;
    }
}

impl Gene for DiscreteGene {
    fn random(context: &mut Context<DiscreteGene>) -> DiscreteGene {
        *context.gene_values.choose(&mut context.rng).unwrap()
    }
    fn mutate(&mut self, context: &mut Context<DiscreteGene>) {
        *self = DiscreteGene::random(context);
    }
}

impl Gene for ContinuousGene {
    fn random(context: &mut Context<ContinuousGene>) -> ContinuousGene {
        context.rng.gen()
    }
    fn mutate(&mut self, context: &mut Context<ContinuousGene>) {
        *self = ContinuousGene::random(context);
    }
}
