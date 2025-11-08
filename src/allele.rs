//! The possible values for a single gene
use impl_trait_for_tuples::impl_for_tuples;
use rand::distributions::uniform::SampleUniform;
use std::hash::{Hash, Hasher};
use std::ops::{Add, AddAssign, Sub, SubAssign};

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
/// [crate::genotype::MultiRangeGenotype]
pub trait RangeAllele:
    Allele
    + Add<Output = Self>
    + Sub<Output = Self>
    + AddAssign
    + SubAssign
    // + Mul<Output = Self>
    // + Into<f64>
    + std::cmp::PartialOrd
    + Default
    + bytemuck::NoUninit
    + SampleUniform
{
    /// Used to build a start exclusive range, by adding the increment to the start
    fn smallest_increment() -> Self;

    /// Returns value 0 for iteration/counting
    fn zero() -> Self;

    /// Returns value 1 for iteration/counting
    fn one() -> Self;

    /// Floors to nearest integer (identity for integer types)
    fn floor(&self) -> Self;

    /// Needed as f32 and f64 don't implement saturating_sub and saturating_add
    fn clamped_add(current_value: Self, delta: Self, max_value: Self) -> Self;
    fn clamped_sub(current_value: Self, delta: Self, min_value: Self) -> Self;
    fn min(a: Self, b: Self) -> Self {
        if a < b { a } else { b }
    }
}

impl RangeAllele for f32 {
    fn smallest_increment() -> Self {
        f32::EPSILON
    }
    fn zero() -> Self {
        0.0
    }
    fn one() -> Self {
        1.0
    }
    fn floor(&self) -> Self {
        f32::floor(*self)
    }
    // ignore f32::MAX, not realistic for use case
    fn clamped_add(current_value: Self, delta: Self, max_value: Self) -> Self {
        let new_value = current_value + delta;
        if new_value > max_value {
            max_value
        } else {
            new_value
        }
    }
    // ignore f32::MIN, not realistic for use case
    fn clamped_sub(current_value: Self, delta: Self, min_value: Self) -> Self {
        let new_value = current_value - delta;
        if new_value < min_value {
            min_value
        } else {
            new_value
        }
    }
}
impl RangeAllele for f64 {
    fn smallest_increment() -> Self {
        f64::EPSILON
    }
    fn zero() -> Self {
        0.0
    }
    fn one() -> Self {
        1.0
    }
    fn floor(&self) -> Self {
        f64::floor(*self)
    }
    // ignore f64::MAX, not realistic for use case
    fn clamped_add(current_value: Self, delta: Self, max_value: Self) -> Self {
        let new_value = current_value + delta;
        if new_value > max_value {
            max_value
        } else {
            new_value
        }
    }
    // ignore f64::MIN, not realistic for use case
    fn clamped_sub(current_value: Self, delta: Self, min_value: Self) -> Self {
        let new_value = current_value - delta;
        if new_value < min_value {
            min_value
        } else {
            new_value
        }
    }
}
impl RangeAllele for i8 {
    fn smallest_increment() -> Self {
        1
    }
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
    fn floor(&self) -> Self {
        *self
    }
    fn clamped_add(current_value: Self, delta: Self, max_value: Self) -> Self {
        let new_value = current_value.saturating_add(delta);
        if new_value > max_value {
            max_value
        } else {
            new_value
        }
    }
    fn clamped_sub(current_value: Self, delta: Self, min_value: Self) -> Self {
        let new_value = current_value.saturating_sub(delta);
        if new_value < min_value {
            min_value
        } else {
            new_value
        }
    }
}
impl RangeAllele for i16 {
    fn smallest_increment() -> Self {
        1
    }
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
    fn floor(&self) -> Self {
        *self
    }
    fn clamped_add(current_value: Self, delta: Self, max_value: Self) -> Self {
        let new_value = current_value.saturating_add(delta);
        if new_value > max_value {
            max_value
        } else {
            new_value
        }
    }
    fn clamped_sub(current_value: Self, delta: Self, min_value: Self) -> Self {
        let new_value = current_value.saturating_sub(delta);
        if new_value < min_value {
            min_value
        } else {
            new_value
        }
    }
}
impl RangeAllele for i32 {
    fn smallest_increment() -> Self {
        1
    }
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
    fn floor(&self) -> Self {
        *self
    }
    fn clamped_add(current_value: Self, delta: Self, max_value: Self) -> Self {
        let new_value = current_value.saturating_add(delta);
        if new_value > max_value {
            max_value
        } else {
            new_value
        }
    }
    fn clamped_sub(current_value: Self, delta: Self, min_value: Self) -> Self {
        let new_value = current_value.saturating_sub(delta);
        if new_value < min_value {
            min_value
        } else {
            new_value
        }
    }
}
impl RangeAllele for u8 {
    fn smallest_increment() -> Self {
        1
    }
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
    fn floor(&self) -> Self {
        *self
    }
    fn clamped_add(current_value: Self, delta: Self, max_value: Self) -> Self {
        let new_value = current_value.saturating_add(delta);
        if new_value > max_value {
            max_value
        } else {
            new_value
        }
    }
    fn clamped_sub(current_value: Self, delta: Self, min_value: Self) -> Self {
        let new_value = current_value.saturating_sub(delta);
        if new_value < min_value {
            min_value
        } else {
            new_value
        }
    }
}
impl RangeAllele for u16 {
    fn smallest_increment() -> Self {
        1
    }
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
    fn floor(&self) -> Self {
        *self
    }
    fn clamped_add(current_value: Self, delta: Self, max_value: Self) -> Self {
        let new_value = current_value.saturating_add(delta);
        if new_value > max_value {
            max_value
        } else {
            new_value
        }
    }
    fn clamped_sub(current_value: Self, delta: Self, min_value: Self) -> Self {
        let new_value = current_value.saturating_sub(delta);
        if new_value < min_value {
            min_value
        } else {
            new_value
        }
    }
}
impl RangeAllele for u32 {
    fn smallest_increment() -> Self {
        1
    }
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
    fn floor(&self) -> Self {
        *self
    }
    fn clamped_add(current_value: Self, delta: Self, max_value: Self) -> Self {
        let new_value = current_value.saturating_add(delta);
        if new_value > max_value {
            max_value
        } else {
            new_value
        }
    }
    fn clamped_sub(current_value: Self, delta: Self, min_value: Self) -> Self {
        let new_value = current_value.saturating_sub(delta);
        if new_value < min_value {
            min_value
        } else {
            new_value
        }
    }
}
