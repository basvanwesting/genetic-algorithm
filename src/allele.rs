//! The possible values for a single gene
use impl_trait_for_tuples::impl_for_tuples;
use std::ops::Add;

/// Standard Allele, suitable for [crate::genotype::Genotype]. Implemented for a set of primitives by default
#[impl_for_tuples(0, 12)]
pub trait Allele: Clone + Copy + Send + Sync + std::fmt::Debug {}

impl Allele for bool {}
impl Allele for char {}
impl Allele for f32 {}
impl Allele for f64 {}
impl Allele for i128 {}
impl Allele for i16 {}
impl Allele for i32 {}
impl Allele for i64 {}
impl Allele for i8 {}
impl Allele for isize {}
impl Allele for u128 {}
impl Allele for u16 {}
impl Allele for u32 {}
impl Allele for u64 {}
impl Allele for u8 {}
impl Allele for usize {}

/// Special Allele subtrait, used for [crate::genotype::RangeGenotype],
/// [crate::genotype::MultiRangeGenotype], [crate::genotype::DynamicRangeGenotype] and
/// [crate::genotype::StaticRangeGenotype]
pub trait RangeAllele:
    Allele + Add<Output = Self> + std::cmp::PartialOrd + Default + bytemuck::NoUninit
{
    /// used to build a start exclusive range, by adding the increment to the start
    fn smallest_increment() -> Self;
}

impl RangeAllele for f32 {
    fn smallest_increment() -> Self {
        f32::EPSILON
    }
}
impl RangeAllele for f64 {
    fn smallest_increment() -> Self {
        f64::EPSILON
    }
}
impl RangeAllele for i128 {
    fn smallest_increment() -> Self {
        1
    }
}
impl RangeAllele for i16 {
    fn smallest_increment() -> Self {
        1
    }
}
impl RangeAllele for i32 {
    fn smallest_increment() -> Self {
        1
    }
}
impl RangeAllele for i64 {
    fn smallest_increment() -> Self {
        1
    }
}
impl RangeAllele for i8 {
    fn smallest_increment() -> Self {
        1
    }
}
impl RangeAllele for isize {
    fn smallest_increment() -> Self {
        1
    }
}
impl RangeAllele for u128 {
    fn smallest_increment() -> Self {
        1
    }
}
impl RangeAllele for u16 {
    fn smallest_increment() -> Self {
        1
    }
}
impl RangeAllele for u32 {
    fn smallest_increment() -> Self {
        1
    }
}
impl RangeAllele for u64 {
    fn smallest_increment() -> Self {
        1
    }
}
impl RangeAllele for u8 {
    fn smallest_increment() -> Self {
        1
    }
}
impl RangeAllele for usize {
    fn smallest_increment() -> Self {
        1
    }
}
