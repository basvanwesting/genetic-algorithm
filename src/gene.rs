pub type BinaryGene = bool;
pub type DiscreteGene = u8;

pub trait Gene: Copy + Clone {
    fn mutate(&mut self) {}
}

impl Gene for BinaryGene {
    fn mutate(&mut self) {
        if *self {
            *self = false
        } else {
            *self = true
        };
    }
}

impl Gene for DiscreteGene {
    fn mutate(&mut self) {
        *self += 1;
    }
}
