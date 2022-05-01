pub type BinaryGene = bool;
pub type DiscreteGene = u32;
//pub type IndexGene = usize;
pub type ContinuousGene = f32;

pub trait Gene: Default + Copy + Clone + std::fmt::Debug {}

impl Gene for BinaryGene {}
impl Gene for DiscreteGene {}
impl Gene for ContinuousGene {}
//impl Gene for IndexGene {}
