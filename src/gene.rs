use crate::context::Context;
use rand::seq::SliceRandom;

pub type BinaryGene = bool;
pub type DiscreteGene = u8;

pub trait Gene: Copy + Clone {
    fn mutate<T: Gene>(&mut self, _context: &Context<T>) {}
}

impl Gene for BinaryGene {
    fn mutate<BinaryGene: Gene>(&mut self, _context: &Context<BinaryGene>) {
        *self = !*self;
    }
}

impl Gene for DiscreteGene {
    fn mutate<DiscreteGene: Gene>(&mut self, context: &Context<DiscreteGene>) {
        *self = *context.gene_values.choose(&mut context.rng).unwrap();
    }
}
