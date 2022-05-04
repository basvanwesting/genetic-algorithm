pub type IndexGene = usize;
pub type BinaryGene = bool;
pub type ContinuousGene = f32;

pub trait Gene: Default + Clone + std::fmt::Debug {}

impl Gene for IndexGene {}
impl Gene for BinaryGene {}
impl Gene for ContinuousGene {}
