use crate::context::Context;
use rand::seq::SliceRandom;

pub type BinaryGene = bool;
pub type DiscreteGene = u8;

pub trait Gene: Copy + Clone {
    fn mutate(&mut self, _context: &mut Context<Self>) {}
}

impl Gene for BinaryGene {
    fn mutate(&mut self, _context: &mut Context<BinaryGene>) {
        *self = !*self;
    }
}

impl Gene for DiscreteGene {
    fn mutate(&mut self, context: &mut Context<DiscreteGene>) {
        *self = *context.gene_values.choose(&mut context.rng).unwrap();
    }
}
