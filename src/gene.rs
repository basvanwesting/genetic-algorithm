pub type BinaryGene = bool;
pub type DiscreteGene = u8;
pub type ContinuousGene = f32;

pub trait Gene: Default + Copy + Clone + std::fmt::Debug {}

impl Gene for BinaryGene {}
impl Gene for DiscreteGene {}
impl Gene for ContinuousGene {}
