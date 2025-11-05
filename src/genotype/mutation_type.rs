pub use crate::allele::Allele;

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
/// ## `RelativeRange(T)`
/// Modifies the current gene value by adding a random delta sampled from the relative range
/// bandwidth. The result after sampling is clamped to stay within the allele range. Preserves
/// locality - small changes to genotype produce small changes to phenotype.
///
/// **Example:** With relative range bandwidth `10.0` translates to sampling range `-10.0..=10.0`,
/// a gene value of `50.0` might become anywhere between `40.0` and `60.0` with equal probability
/// (clamped if necessary to the allele range).
///
/// **Use case:** Local search, fine-tuning solutions, problems where nearby
/// solutions have similar fitness (smooth fitness landscape).
///
/// ## `ScaledSteps(Vec<T>)`
/// Multi-scale mutation for progressive refinement. The vector contains steps for each scale
/// level, from coarse to fine adjustments. During mutation, steps either up or down with the
/// the current scale's step as the delta. So no sampling inside the down-to-up step range, just
/// the full step.
///
/// The scale is transitioned to the next level, when the algorithm reaches its
/// max_stale_generations, resetting the max_stale_generations to zero again.
///
/// **Example:** Scales `vec![1.0, 0.1, 0.01]` provide
/// three levels of precision. At scale 0, mutations use ±1.0; at scale 2, ±0.01.
///
/// **Use case:** Optimization requiring both exploration and exploitation,
/// systematic parameter sweeps, problems benefiting from coarse-to-fine search.
/// Also provides permutability for RangeGenotype and MultiRangeGenotype.
///
/// ## `Transition(usize, usize, T)`
/// Smoothly transitions from Random to RelativeRange mutation over a specified number of generations.
/// Provides a natural exploration-to-exploitation schedule without abrupt behavioral changes.
///
/// **Parameters:**
/// - `random_until_generation`: Use pure Random mutation until this generation
/// - `relative_range_from_generation`: Use pure RelativeRange mutation from this generation on
/// - `relative_mutation_range_bandwidth`: The final relative range bandwidth to use after full transition
///
/// **Example:** `Transition(100, 500, 5.0)` means:
/// - Generations 0-99: Pure Random mutation (full exploration)
/// - Generations 100-499: Gradual transition with decreasing mutation range
/// - Generations 500+: Pure RelativeRange mutation with uniform sampled [-5.0,5.0] range (exploitation)
///
/// **Use case:** Problems requiring initial exploration followed by convergence,
/// evolutionary algorithms with scheduled exploration decay, parameter optimization
/// where good regions are unknown initially.
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
/// continuous parameters, encoding finite state machines, or discrete optimization. When there are
/// only discrete parameters, plain [ListGenotype](crate::genotype::ListGenotype) or [MultiListGenotype](crate::genotype::MultiListGenotype) are better fits.
///
/// ### Boundary Sampling Behavior
///
/// Different mutation types handle allele range boundaries differently:
///
/// - [MutationType::Random]: Undersamples boundaries (infinitesimal probability of sampling exact boundary)
/// - [MutationType::RelativeRange]: Slightly oversamples boundaries (up to 50% when near boundary due to clamping)
/// - [MutationType::ScaledSteps]: Slightly oversamples boundaries (up to 50% when near boundary due to clamping)
/// - [MutationType::Transition]: Uses pre-clamped sampling ranges, undersampling boundaries during transition phase
/// - [MutationType::Discrete]: Uniform sampling, no over- or undersampling of boundaries
///
/// The transition phase uses centered sampling with a progressively shrinking range. The range
/// is pre-clamped before sampling to avoid excessive boundary oversampling that would occur with large
/// ranges. This provides consistent undersampling behavior across the entire transition spectrum,
/// which is acceptable since Random mutation also undersamples boundaries.
///
/// # Compatibility
///
/// * [RangeGenotype](crate::genotype::RangeGenotype): Supports all variants, except Discrete
///   because [ListGenotype](crate::genotype::ListGenotype) is always a better fit in that situation
/// * [MultiRangeGenotype](crate::genotype::MultiRangeGenotype): Supports all variants
/// * Other genotypes (Binary, List, Unique, ...) use fixed random mutation strategies
///
/// # Preference
///
/// Mutation type is defined by the most recent Genotype builder setting, so these can overwrite:
/// * `with_mutation_type(s)` → set directly (single or mixed type per gene)
/// * `with_allele_mutation_scaled_range(s)` → deprecated setting, scaled for all genes
/// * `with_allele_mutation_ranges(s)` → deprecated setting, relative for all genes
/// * no setting → default, random for all genes
///
/// # Examples
///
/// ```
/// use genetic_algorithm::genotype::{Genotype, MutationType, RangeGenotype, MultiRangeGenotype};
///
/// // Random mutation (default) - maximum exploration
/// let genotype = RangeGenotype::builder()
///     .with_allele_range(0.0..=100.0)
///     .with_mutation_type(MutationType::Random) // optional, default
///     .build();
///
/// // RelativeRange mutation - local search with fixed neighborhood
/// let genotype = RangeGenotype::builder()
///     .with_allele_range(0.0..=100.0)
///     .with_mutation_type(MutationType::RelativeRange(5.0))
///     .with_allele_mutation_range(-5.0..=5.0)  // legacy setting of the same, to be deprecated
///     .build();
///
/// // ScaledSteps mutation - progressive refinement
/// let genotype = RangeGenotype::builder()
///     .with_allele_range(0.0..=100.0)
///     .with_mutation_type(MutationType::ScaledSteps(vec![
///         10.0,  // Coarse scale
///         1.0,    // Medium scale
///         0.1,    // Fine scale
///     ]))
///     .with_allele_mutation_scaled_range(vec![
///         -10.0..=10.0,  // Coarse scale
///         -1.0..=1.0,    // Medium scale
///         -0.1..=0.1,    // Fine scale
///     ]) // legacy setting of the same, to be deprecated
///     .build();
///
/// // Transition mutation - exploration-to-exploitation (Random to RelativeRange)
/// let genotype = RangeGenotype::builder()
///     .with_allele_range(0.0..=100.0)
///     .with_mutation_type(MutationType::Transition(
///         1000,  // first 1000 generations pure random mutations
///         5000,  // after 5000 generations pure relative mutations
///         5.0    // target relative mutation range bandwidth, linearly transitions towards this from 1000th to 5000th generation
///     ))
///     .build();
///
/// // Mixed mutation types (incl. Discrete)
/// let genotype = MultiRangeGenotype::builder()
///     .with_allele_ranges(vec![
///         0.0..=1.0,    // Gene 0: Boolean flag
///         0.0..=4.0,    // Gene 1: Algorithm choice (5 options)
///         0.0..=100.0,  // Gene 2: Speed percentage
///     ])
///     .with_mutation_types(vec![
///         MutationType::Discrete,  // Boolean as 0 or 1
///         MutationType::Discrete,  // One of 5 algorithms
///         MutationType::ScaledSteps(vec![10.0, 1.0, 0.1]), // Continuous refinement
///     ])
///     // no legacy alternative
///     .build();
/// ```
#[derive(Clone, PartialEq, Debug, Default)]
pub enum MutationType<T: Allele> {
    #[default]
    Random,
    /// Relative range bandwidth (leads to [-T,T], uniformly sampled)
    RelativeRange(T),
    /// Vec of decreasing step sizes (applied up or down)
    ScaledSteps(Vec<T>),
    Discrete,
    /// random_until_generation, relative_range_from_generation, relative_mutation_range_bandwidth
    Transition(usize, usize, T),
}

impl<T: Allele> MutationType<T> {
    pub fn transition_progress(&self, current_generation: usize) -> f64 {
        match self {
            Self::Transition(random_until, relative_from, _) => {
                if current_generation <= *random_until {
                    0.0
                } else if current_generation >= *relative_from {
                    1.0
                } else {
                    (current_generation - random_until) as f64
                        / (relative_from - random_until) as f64
                }
            }
            _ => 0.0,
        }
    }
    // pub fn relative_range(&self) -> Option<&T> {
    //     match self {
    //         Self::RelativeRange(range) => Some(range),
    //         Self::Transition(_, _, range) => Some(range),
    //         _ => None,
    //     }
    // }
    // pub fn scaled_ranges(&self) -> Option<&Vec<T>> {
    //     match self {
    //         Self::ScaledSteps(ranges) => Some(ranges),
    //         _ => None,
    //     }
    // }
}
