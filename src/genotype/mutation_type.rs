pub use crate::allele::Allele;
use std::ops::RangeInclusive;

/// Controls mutation behavior for numeric genotypes (Range and MultiRange).
///
/// Determines how genes are modified during mutation operations, from completely
/// random replacement to fine-grained local adjustments. Different mutation types
/// enable different search strategies and optimization characteristics.
///
/// # Variants
///
/// ## `Random` (default)
/// Replaces the current gene value with a completely new random value sampled
/// uniformly from the full allele range. Provides maximum exploration with no
/// locality - useful for escaping local optima but may disrupt good solutions.
///
/// **Example:** Gene value `50.0` in range `0.0..=100.0` could become any value
/// in that range with equal probability.
///
/// **Use case:** Initial exploration phase, problems with disconnected optima,
/// or when no assumptions about solution locality can be made.
///
/// ## `Relative(RangeInclusive<T>)`
/// Modifies the current gene value by adding a random delta sampled from the
/// relative range. The result is clamped to stay within the allele range.
/// Preserves locality - small changes to genotype produce small changes to phenotype.
///
/// **Example:** With relative range `-10.0..=10.0`, a gene value of `50.0` might
/// become `40.0` to `60.0` (clamped if necessary to the allele range).
///
/// **Use case:** Local search, fine-tuning solutions, problems where nearby
/// solutions have similar fitness (smooth fitness landscape).
///
/// ## `Scaled(Vec<RangeInclusive<T>>)`
/// Multi-scale mutation for progressive refinement. The vector contains ranges for each scale
/// level, from coarse to fine adjustments. During mutation, picks either the start or end value of
/// the current scale's range as the delta. So no sampling inside the scaled range, only the full
/// step up or down is applied in a mutation.
///
/// The scale is transitioned to the next level, when the algorithm reaches its
/// max_stale_generations.
///
/// **Example:** Scales `vec![-1.0..=1.0, -0.1..=0.1, -0.01..=0.01]` provide
/// three levels of precision. At scale 0, mutations use ±1.0; at scale 2, ±0.01.
///
/// **Use case:** Optimization requiring both exploration and exploitation,
/// systematic parameter sweeps, problems benefiting from coarse-to-fine search.
/// Also provides permutability for RangeGenotype and MultiRangeGenotype.
///
/// ## `Discrete`
/// Treats the numeric range as discrete integer values, useful for encoding
/// categorical data or enum variants as numbers. Values are floored to integers
/// during mutation. Only supported by MultiRangeGenotype.
///
/// **Example:** Range `0.0..=4.0` represents 5 discrete choices: `{0, 1, 2, 3, 4}`,
/// which could map to enum variants or categorical options.
///
/// **Use case:** Heterogeneous chromosomes mixing categorical choices with
/// continuous parameters, encoding finite state machines, or discrete optimization.
///
/// # Compatibility
///
/// - **RangeGenotype**: Supports Random, Relative, and Scaled
/// - **MultiRangeGenotype**: Supports all variants including Discrete
/// - Other genotypes (Binary, List, Unique) use fixed random mutation strategies
///
/// # Preference
///
/// Mutation type is defined by the most recent builder setting, so these can overwrite:
/// * `with_mutation_type(s)` → set directly (all mixed types, gene specific)
/// * `with_allele_mutation_scaled_range(s)` → legacy setting, scaled for all genes
/// * `with_allele_mutation_ranges(s)` → legacy setting, relative for all genes
/// * no setting → default, random for all genes
///
/// # Examples
///
/// ```
/// use genetic_algorithm::genotype::{Genotype, MutationType, RangeGenotype};
///
/// // Random mutation (default) - maximum exploration
/// let genotype = RangeGenotype::builder()
///     .with_mutation_type(MutationType::Random) // optional, default
///     .with_allele_range(0.0..=100.0)
///     .build();
///
/// // Relative mutation - local search with fixed neighborhood
/// let genotype = RangeGenotype::builder()
///     .with_allele_range(0.0..=100.0)
///     .with_mutation_type(MutationType::Relative(-5.0..=5.0))
///     .with_allele_mutation_range(-5.0..=5.0)  // legacy setting of the same
///     .build();
///
/// // Scaled mutation - progressive refinement
/// let genotype = RangeGenotype::builder()
///     .with_allele_range(0.0..=100.0)
///     .with_mutation_type(MutationType::Scaled(vec![
///         -10.0..=10.0,  // Coarse scale
///         -1.0..=1.0,    // Medium scale
///         -0.1..=0.1,    // Fine scale
///     ]))
///     .with_allele_mutation_scaled_range(vec![
///         -10.0..=10.0,  // Coarse scale
///         -1.0..=1.0,    // Medium scale
///         -0.1..=0.1,    // Fine scale
///     ]) // legacy setting of the same
///     .build();
/// ```
#[derive(Clone, PartialEq, Debug, Default)]
pub enum MutationType<T: Allele> {
    #[default]
    Random,
    Relative(RangeInclusive<T>),
    Scaled(Vec<RangeInclusive<T>>),
    Discrete, // Range acting as List encoding
}
