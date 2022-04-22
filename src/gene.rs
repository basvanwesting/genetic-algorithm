use crate::context::Context;
use crate::global_rand;
use rand::prelude::*;
use rand::seq::SliceRandom;

pub type BinaryGene = bool;
pub type DiscreteGene = u8;
pub type ContinuousGene = f32;

pub trait Gene: Copy + Clone + std::fmt::Display + std::fmt::Debug {
    fn random(_context: &mut Context<Self>) -> Self;
    fn mutate(&mut self, _context: &mut Context<Self>);
}

impl Gene for BinaryGene {
    fn random(_context: &mut Context<BinaryGene>) -> BinaryGene {
        global_rand::gen()
        //context.rng.gen()
    }
    fn mutate(&mut self, _context: &mut Context<BinaryGene>) {
        *self = !*self;
    }
}

impl Gene for DiscreteGene {
    fn random(context: &mut Context<DiscreteGene>) -> DiscreteGene {
        *global_rand::choose(&context.gene_values)
        //*context.gene_values.choose(&mut context.rng).unwrap()
    }
    fn mutate(&mut self, context: &mut Context<DiscreteGene>) {
        *self = DiscreteGene::random(context);
    }
}

impl Gene for ContinuousGene {
    fn random(_context: &mut Context<ContinuousGene>) -> ContinuousGene {
        global_rand::gen()
        //context.rng.gen()
    }
    fn mutate(&mut self, context: &mut Context<ContinuousGene>) {
        *self = ContinuousGene::random(context);
    }
}
