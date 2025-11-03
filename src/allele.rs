//! The possible values for a single gene
use impl_trait_for_tuples::impl_for_tuples;
use std::hash::{Hash, Hasher};
use std::ops::{Add, Sub};

/// Standard Allele, suitable for [crate::genotype::Genotype]. Implemented for a set of primitives by default
pub trait Allele: Clone + Copy + Send + Sync + std::fmt::Debug {
    /// Hash a slice of alleles. This method allows type-specific hashing behavior.
    /// For most types, this uses the standard Hash trait.
    /// For float types (f32, f64), this hashes the bytes for deterministic results.
    fn hash_slice(slice: &[Self], hasher: &mut impl Hasher)
    where
        Self: Sized;
}

/// Macro for implementing Allele with default hash_slice
/// Use this for any type that implements Hash and needs the standard hashing behavior
#[macro_export]
macro_rules! impl_allele{
    ($($t:ty),*) => {
        $(
            impl $crate::allele::Allele for $t {
                fn hash_slice(slice: &[Self], hasher: &mut impl ::std::hash::Hasher) {
                    ::std::hash::Hash::hash(slice, hasher);
                }
            }
        )*
    }
}

impl_allele!(bool, char, i128, i16, i32, i64, i8, isize, u128, u16, u32, u64, u8, usize);
impl Allele for f32 {
    fn hash_slice(slice: &[Self], hasher: &mut impl Hasher) {
        let bytes: &[u8] = bytemuck::cast_slice(slice);
        bytes.hash(hasher);
    }
}
impl Allele for f64 {
    fn hash_slice(slice: &[Self], hasher: &mut impl Hasher) {
        let bytes: &[u8] = bytemuck::cast_slice(slice);
        bytes.hash(hasher);
    }
}

// Tuple implementations using impl_for_tuples macro
#[impl_for_tuples(0, 12)]
impl Allele for Tuple {
    for_tuples!( where #( Tuple: Allele + Hash ),* );

    fn hash_slice(slice: &[Self], hasher: &mut impl Hasher) {
        slice.hash(hasher);
    }
}

/// Special Allele subtrait, used for [crate::genotype::RangeGenotype],
/// [crate::genotype::MultiRangeGenotype], [crate::genotype::DynamicRangeGenotype] and
/// [crate::genotype::StaticRangeGenotype]
pub trait RangeAllele:
    Allele
    + Add<Output = Self>
    + Sub<Output = Self>
    // + Mul<Output = Self>
    + Into<f64>
    + std::cmp::PartialOrd
    + Default
    + bytemuck::NoUninit
{
    /// used to build a start exclusive range, by adding the increment to the start
    fn smallest_increment() -> Self;

    /// Returns value 1 for iteration/counting
    fn one() -> Self;

    /// Floors to nearest integer (identity for integer types)
    fn floor(&self) -> Self;

    /// Scale by fraction (always between 0.0.and 1.0)
    fn scale_by_fraction(&self, fraction: f64) -> Self;
}

impl RangeAllele for f32 {
    fn smallest_increment() -> Self {
        f32::EPSILON
    }
    fn one() -> Self {
        1.0
    }
    fn floor(&self) -> Self {
        f32::floor(*self)
    }
    fn scale_by_fraction(&self, fraction: f64) -> Self {
        self * fraction as f32
    }
}
impl RangeAllele for f64 {
    fn smallest_increment() -> Self {
        f64::EPSILON
    }
    fn one() -> Self {
        1.0
    }
    fn floor(&self) -> Self {
        f64::floor(*self)
    }
    fn scale_by_fraction(&self, fraction: f64) -> Self {
        self * fraction
    }
}
impl RangeAllele for i8 {
    fn smallest_increment() -> Self {
        1
    }
    fn one() -> Self {
        1
    }
    fn floor(&self) -> Self {
        *self
    }
    fn scale_by_fraction(&self, fraction: f64) -> Self {
        (*self as f64 * fraction).round() as i8
    }
}
impl RangeAllele for i16 {
    fn smallest_increment() -> Self {
        1
    }
    fn one() -> Self {
        1
    }
    fn floor(&self) -> Self {
        *self
    }
    fn scale_by_fraction(&self, fraction: f64) -> Self {
        (*self as f64 * fraction).round() as i16
    }
}
impl RangeAllele for i32 {
    fn smallest_increment() -> Self {
        1
    }
    fn one() -> Self {
        1
    }
    fn floor(&self) -> Self {
        *self
    }
    fn scale_by_fraction(&self, fraction: f64) -> Self {
        (*self as f64 * fraction).round() as i32
    }
}
impl RangeAllele for u8 {
    fn smallest_increment() -> Self {
        1
    }
    fn one() -> Self {
        1
    }
    fn floor(&self) -> Self {
        *self
    }
    fn scale_by_fraction(&self, fraction: f64) -> Self {
        (*self as f64 * fraction).round() as u8
    }
}
impl RangeAllele for u16 {
    fn smallest_increment() -> Self {
        1
    }
    fn one() -> Self {
        1
    }
    fn floor(&self) -> Self {
        *self
    }
    fn scale_by_fraction(&self, fraction: f64) -> Self {
        (*self as f64 * fraction).round() as u16
    }
}
impl RangeAllele for u32 {
    fn smallest_increment() -> Self {
        1
    }
    fn one() -> Self {
        1
    }
    fn floor(&self) -> Self {
        *self
    }
    fn scale_by_fraction(&self, fraction: f64) -> Self {
        (*self as f64 * fraction).round() as u32
    }
}
