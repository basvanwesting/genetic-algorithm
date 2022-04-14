use crate::context::Context;

pub type BinaryGene = bool;
pub type DiscreteGene = u8;

pub trait Gene: Copy + Clone {
    fn mutate<T: Gene>(&mut self, _context: &Context<T>) {}
}

impl Gene for BinaryGene {
    fn mutate<T: Gene>(&mut self, _context: &Context<T>) {
        if *self {
            *self = false
        } else {
            *self = true
        };
    }
}

impl Gene for DiscreteGene {
    fn mutate<T: Gene>(&mut self, _context: &Context<T>) {
        *self += 1;
    }
}
