pub use crate::allele::Allele;

/// Controls mutation behavior for numeric genotypes (Range and MultiRange).
///
/// Determines how genes are modified during mutation operations, from completely
/// random replacement to fine-grained local adjustments. Different mutation types
/// enable different search strategies and optimization characteristics.
///
/// # Categories
///
/// Mutation types fall into three categories:
///
/// ## Static Mutations
/// Fixed behavior throughout the evolution process:
/// - `Random`: Full range replacement
/// - `Range(T)`: Fixed bandwidth range mutation (uniform sampling within ±bandwidth)
/// - `Step(T)`: Fixed step mutation (exactly +step or -step)
/// - `Discrete`: Integer-only mutations for categorical data
///
/// ## Scaled Mutations (Performance-Triggered)
/// Progress through phases based on `max_stale_generations`:
/// - `RangeScaled(Vec<T>)`: Range bandwidths that decrease when performance stalls
/// - `StepScaled(Vec<T>)`: Step sizes that decrease when performance stalls
///
/// ## Generational Mutations (Time-Triggered)
/// Progress through phases based on `max_generations`:
/// - `RangeGenerational(Vec<T>)`: Range bandwidths that change over time
/// - `StepGenerational(Vec<T>)`: Step sizes that change over time
///
/// # Variants
///
/// ## `Random` (default)
/// Replaces the current gene value with a completely new random value sampled
/// uniformly from the full allele range. Provides maximum exploration with no
/// locality - useful for escaping local optima but may disrupt good solutions.
///
/// **Example:** Gene value `50` in range `0..=100` could become any value
/// in that range with equal probability.
///
/// **Use case:** Initial exploration phase, problems with disconnected optima,
/// or when no assumptions about solution locality can be made.
///
/// ## `Range(T)`
/// Modifies the current gene value by adding a random delta sampled uniformly from
/// the range `[-bandwidth, +bandwidth]`. The result is clamped to stay within the
/// allele range. Preserves locality - small bandwidths for local search, large
/// bandwidths for broader exploration.
///
/// **Example:** With `Range(10)` on an integer range `0..=100`, a gene value
/// of `50` could become anywhere between `40` and `60` with equal probability
/// (clamped if necessary). With `Range(5.0)` on a float range, mutations are
/// uniformly sampled within ±5.0.
///
/// **Use case:** Local search (small bandwidth), controlled exploration (medium
/// bandwidth), problems where nearby solutions have similar fitness.
///
/// ## `Step(T)`
/// Modifies the current gene value by exactly +step or -step (50/50 probability).
/// Unlike `Range`, this doesn't sample within an interval but uses discrete steps.
/// The result is clamped to stay within the allele range.
///
/// **Note on clamping:** For step mutations, pre-clamping and post-clamping produce
/// identical behavior since each direction (up/down) is discretely chosen with equal
/// probability. The boundary is reached deterministically when stepping would exceed it.
///
/// **Example:** With `Step(5)` on range `0..=100`, a gene value of `50` becomes
/// either `45` or `55` (50% probability each), never values in between.
///
/// **Use case:** Grid-aligned search spaces, discrete optimization where specific
/// increments are meaningful, systematic exploration patterns.
///
/// ## `RangeScaled(Vec<T>)`
/// Performance-adaptive range mutation. Progresses through bandwidths when the
/// algorithm stalls (reaches `max_stale_generations` without improvement). Each
/// bandwidth represents the mutation range for that scale level.
///
/// **Example:** `RangeScaled(vec![50, 20, 5, 1])` on range `0..=100` starts
/// with ±50 mutations, then progressively focuses as performance plateaus.
///
/// **Behavior:**
/// - Scale 0: Mutations uniformly within ±50 (pre-clamped)
/// - After stalling → Scale 1: Mutations uniformly within ±20 (pre-clamped)
/// - After stalling → Scale 2: Mutations uniformly within ±5 (pre-clamped)
/// - After stalling → Scale 3: Mutations uniformly within ±1 (post-clamped, final phase)
/// - Total max runtime: `4 * max_stale_generations`
///
/// **Use case:** Problems requiring adaptive exploration-exploitation balance,
/// automatic focusing when stuck in local optima.
///
/// ## `RangeGenerational(Vec<T>)`
/// Time-scheduled range mutation. Progresses through bandwidths at fixed
/// intervals (`max_generations`). Each bandwidth is used for exactly
/// `max_generations` generations before moving to the next.
///
/// **Example:** `RangeGenerational(vec![50.0, 20.0, 5.0, 1.0])` with `max_generations=1000`:
/// - Generations 0-999: Mutations uniformly within ±50.0 (pre-clamped)
/// - Generations 1000-1999: Mutations uniformly within ±20.0 (pre-clamped)
/// - Generations 2000-2999: Mutations uniformly within ±5.0 (pre-clamped)
/// - Generations 3000+: Mutations uniformly within ±1.0 (post-clamped, final phase)
/// - Total runtime: `4 * 1000 = 4000` generations
///
/// **Use case:** Predictable exploration-exploitation schedules, benchmarking with
/// consistent behavior, problems with known convergence patterns.
///
/// ## `StepScaled(Vec<T>)`
/// Performance-adaptive step mutation. Like `RangeScaled` but uses fixed step
/// sizes instead of uniform ranges. Mutations apply the step value either up or
/// down (50/50 probability), not uniformly sampled within a range.
///
/// **Example:** `StepScaled(vec![10, 1])` on an integer range provides two
/// precision levels, advancing when performance stalls.
///
/// **Behavior:**
/// - Scale 0: Mutations of exactly ±10
/// - After stalling → Scale 1: Mutations of exactly ±1
///
/// **Use case:** Grid-like search spaces, systematic parameter sweeps, problems
/// requiring exact step sizes. Provides permutability for RangeGenotype when steps
/// align with value discretization.
///
/// ## `StepGenerational(Vec<T>)`
/// Time-scheduled step mutation. Like `RangeGenerational` but uses fixed steps
/// instead of uniform ranges. Progresses at fixed `max_generations` intervals.
///
/// **Example:** `StepGenerational(vec![10.0, 1.0, 0.1])` with `max_generations=1000`:
/// - Generations 0-999: Steps of exactly ±10.0
/// - Generations 1000-1999: Steps of exactly ±1.0
/// - Generations 2000+: Steps of exactly ±0.1
///
/// **Use case:** Scheduled coarse-to-fine search, consistent stepping behavior
/// across runs, problems with known scale requirements over time.
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
/// continuous parameters. When all parameters are discrete, prefer
/// [ListGenotype](crate::genotype::ListGenotype) or [MultiListGenotype](crate::genotype::MultiListGenotype).
///
/// # Key Differences: Range vs Step
///
/// The distinction between Range and Step mutations:
/// - **Range mutations**: Sample uniformly within `[-bandwidth, +bandwidth]`
///   - Example: `Range(10)` → mutations anywhere in -10 to +10
/// - **Step mutations**: Apply exactly `+step` or `-step` (50/50 probability)
///   - Example: `Step(10)` → mutations of exactly +10 or -10, nothing in between
///
/// # Boundary Sampling and Clamping Strategy
///
/// ## Pre-clamping vs Post-clamping
///
/// Range mutations use different clamping strategies to balance exploration and exploitation:
///
/// ### Pre-clamping (Exploration Phases)
/// Used for all phases except the final one in `RangeScaled` and `RangeGenerational`:
/// - First constrains the sampling range to valid values: `[max(min, current-bandwidth), min(max, current+bandwidth)]`
/// - Then samples uniformly from this constrained range
/// - **Benefit**: Avoids boundary oversampling during exploration
/// - **Drawback**: Cannot sample exact boundaries (zero probability for continuous values)
///
/// ### Post-clamping (Final Phase / Local Search)
/// Used for:
/// - Static `Range(T)` mutations (always)
/// - Final phase of `RangeScaled` and `RangeGenerational`
///
/// - First samples from the full range: `[current-bandwidth, current+bandwidth]`
/// - Then clamps the result to allele bounds
/// - **Benefit**: Can reach exact boundaries (important for fine-tuning)
/// - **Drawback**: Slight boundary oversampling when near edges
///
/// ### Example
/// For `RangeScaled(vec![50.0, 20.0, 5.0, 1.0])` on allele range `[0.0, 100.0]`:
/// - Phases 0-2: Use pre-clamping (exploration, avoid boundary oversampling)
/// - Phase 3: Uses post-clamping (exploitation, ensure boundaries reachable)
///
/// This design naturally matches the exploration→exploitation progression, where early
/// phases explore broadly without boundary bias, while the final phase can fine-tune
/// to exact boundary values.
///
/// ## Boundary Sampling Summary
///
/// - `Random`: Undersamples boundaries (infinitesimal probability)
/// - `Range`: Post-clamped, slight boundary oversampling when near edges
/// - `Step`: Always clamped, slight boundary oversampling when near edges
/// - `RangeScaled/RangeGenerational`:
///   - Non-final phases: Pre-clamped, boundaries undersampled
///   - Final phase: Post-clamped, slight boundary oversampling
/// - `StepScaled/StepGenerational`: Always clamped, slight boundary oversampling
/// - `Discrete`: Uniform sampling, no boundary bias
///
/// # Phase Progression
///
/// ## Scaled Mutations (Performance-Triggered)
/// - Advance to next phase when `current_stale_generations >= max_stale_generations`
/// - Reset `current_stale_generations` to 0 after advancing
/// - Stay at final phase once all values are exhausted
/// - Total phases: Length of provided vector
/// - Maximum runtime: `vector.len() * max_stale_generations`
///
/// ## Generational Mutations (Time-Triggered)
/// - Advance to next phase when `current_generation >= (phase_index + 1) * max_generations`
/// - No reset needed, purely time-based
/// - Stay at final phase once all values are exhausted
/// - Total phases: Length of provided vector
/// - Total runtime: `vector.len() * max_generations`
///
/// # Type Consistency
///
/// All bandwidth and step values use the same type `T` as the genotype's allele type.
/// This ensures type safety and intuitive behavior:
/// - For `RangeGenotype<i32>`: Use integer bandwidths like `Range(10)` or `Step(5)`
/// - For `RangeGenotype<f64>`: Use float bandwidths like `Range(10.0)` or `Step(0.5)`
///
/// # Compatibility
///
/// * [RangeGenotype](crate::genotype::RangeGenotype): All variants except Discrete
/// * [MultiRangeGenotype](crate::genotype::MultiRangeGenotype): All variants
/// * Other genotypes use fixed mutation strategies
///
/// # Migration Guide
///
/// From old types to new:
/// - `MutationType::Random` → `MutationType::Random` (unchanged)
/// - `MutationType::Range(bandwidth)` → `MutationType::Range(bandwidth)`
/// - `MutationType::ScaledSteps(vec![...])` → `MutationType::StepScaled(vec![...])`
/// - `MutationType::Transition(until, from, bandwidth)` → See example below
/// - `MutationType::Discrete` → `MutationType::Discrete` (unchanged)
///
/// Transition migration example:
/// ```ignore
/// // Old: Transition(1000, 5000, 5.0)
/// // Meant: Random for 1000 gens, transition over 4000 gens, then Range(5.0)
///
/// // New approach 1: Approximate with phases
/// MutationType::RangeGenerational(vec![
///     100.0,  // Full range (assuming allele range is ±100)
///     100.0,  // Continue full range (covers first 1000 gens with max_generations=500)
///     50.0,   // Start reducing
///     25.0,   // Continue reducing
///     10.0,   // Almost there
///     5.0,    // Final bandwidth
/// ])
/// // with .with_max_generations(833) to get ~5000 total
///
/// // New approach 2: Simpler two-phase
/// MutationType::RangeGenerational(vec![
///     100.0,  // Full exploration
///     5.0,    // Focused search
/// ])
/// // with .with_max_generations(2500) for abrupt transition at 2500
/// ```
///
/// # Examples
///
/// ```
/// use genetic_algorithm::genotype::{Genotype, MutationType, RangeGenotype, MultiRangeGenotype};
///
/// // Integer genotype with range mutations
/// let genotype = RangeGenotype::<i32>::builder()
///     .with_allele_range(0..=100)
///     .with_mutation_type(MutationType::Range(10)) // ±10 uniform mutations (post-clamped)
///     .build();
///
/// // Integer genotype with step mutations
/// let genotype = RangeGenotype::<i32>::builder()
///     .with_allele_range(0..=100)
///     .with_mutation_type(MutationType::Step(5)) // exactly +5 or -5
///     .build();
///
/// // Float genotype with range mutations
/// let genotype = RangeGenotype::<f64>::builder()
///     .with_allele_range(0.0..=100.0)
///     .with_mutation_type(MutationType::Range(10.0)) // ±10.0 uniform mutations (post-clamped)
///     .build();
///
/// // Performance-adaptive exploration with proper clamping strategy
/// let genotype = RangeGenotype::<i32>::builder()
///     .with_allele_range(0..=100)
///     .with_mutation_type(MutationType::RangeScaled(vec![
///         50,  // Broad exploration (pre-clamped, no boundary bias)
///         20,  // Medium range (pre-clamped)
///         5,   // Focused search (pre-clamped)
///         1,   // Fine-tuning (post-clamped, can reach boundaries)
///     ]))
///     .build();
///
/// // Time-scheduled exploration to exploitation
/// let genotype = RangeGenotype::<f64>::builder()
///     .with_allele_range(0.0..=100.0)
///     .with_mutation_type(MutationType::RangeGenerational(vec![
///         80.0,  // First max_generations: broad exploration (pre-clamped)
///         80.0,  // Second max_generations: continue exploration (pre-clamped)
///         40.0,  // Third max_generations: medium range (pre-clamped)
///         10.0,  // Fourth max_generations: focused search (pre-clamped)
///         2.0,   // Fifth+ max_generations: fine-tuning (post-clamped)
///     ]))
///     .build();
///
/// // Progressive step refinement (performance-based)
/// let genotype = RangeGenotype::<f64>::builder()
///     .with_allele_range(0.0..=100.0)
///     .with_mutation_type(MutationType::StepScaled(vec![
///         10.0,  // Coarse steps
///         1.0,   // Medium steps
///         0.1,   // Fine steps
///     ]))
///     .build();
///
/// // Mixed mutation types for heterogeneous chromosome
/// let genotype = MultiRangeGenotype::<f64>::builder()
///     .with_allele_ranges(vec![
///         0.0..=1.0,    // Gene 0: Boolean flag
///         0.0..=4.0,    // Gene 1: Algorithm choice (5 options)
///         0.0..=100.0,  // Gene 2: Continuous parameter
///         -1.0..=1.0,   // Gene 3: Direction parameter
///     ])
///     .with_mutation_types(vec![
///         MutationType::Discrete,            // Boolean as 0 or 1
///         MutationType::Discrete,            // One of 5 algorithms
///         MutationType::Range(10.0),         // Uniform search within ±10.0 (post-clamped)
///         MutationType::StepGenerational(vec![0.5, 0.1, 0.01]), // Decreasing steps
///     ])
///     .build();
/// ```
#[derive(Clone, PartialEq, Debug, Default)]
pub enum MutationType<T: Allele> {
    #[default]
    Random,
    /// Range mutation bandwidth (uniform sampling within ±bandwidth, post-clamped)
    Range(T),
    /// Step mutation size (exactly +step or -step, clamped)
    Step(T),
    /// Range bandwidths that change based on performance (pre-clamped except final phase)
    RangeScaled(Vec<T>),
    /// Range bandwidths that change over time (pre-clamped except final phase)
    RangeGenerational(Vec<T>),
    /// Step sizes that change based on performance (always clamped)
    StepScaled(Vec<T>),
    /// Step sizes that change over time (always clamped)
    StepGenerational(Vec<T>),
    Discrete,
}

impl<T: Allele> MutationType<T> {
    /// Returns the current range bandwidth for Range-based mutations.
    /// For scaled/generational variants, returns the bandwidth at the given scale/phase.
    /// Returns None for non-Range mutations.
    pub fn current_range_bandwidth(&self, scale_or_phase: usize) -> Option<&T> {
        match self {
            Self::Range(bandwidth) => Some(bandwidth),
            Self::RangeScaled(bandwidths) | Self::RangeGenerational(bandwidths) => {
                bandwidths.get(scale_or_phase.min(bandwidths.len().saturating_sub(1)))
            }
            _ => None,
        }
    }

    /// Returns the current step size for Step-based mutations.
    /// For scaled/generational variants, returns the step at the given scale/phase.
    /// Returns None for non-Step mutations.
    pub fn current_step(&self, scale_or_phase: usize) -> Option<&T> {
        match self {
            Self::Step(step) => Some(step),
            Self::StepScaled(steps) | Self::StepGenerational(steps) => {
                steps.get(scale_or_phase.min(steps.len().saturating_sub(1)))
            }
            _ => None,
        }
    }

    /// Determines which phase/scale should be active for generational mutations
    /// based on current generation and max_generations setting.
    pub fn generational_phase(&self, current_generation: usize, max_generations: usize) -> usize {
        match self {
            Self::RangeGenerational(bandwidths) => {
                let phase = current_generation / max_generations;
                phase.min(bandwidths.len().saturating_sub(1))
            }
            Self::StepGenerational(steps) => {
                let phase = current_generation / max_generations;
                phase.min(steps.len().saturating_sub(1))
            }
            _ => 0,
        }
    }

    /// Returns true if this mutation type uses performance-based scaling
    pub fn is_scaled(&self) -> bool {
        matches!(self, Self::RangeScaled(_) | Self::StepScaled(_))
    }

    /// Returns true if this mutation type uses generation-based progression
    pub fn is_generational(&self) -> bool {
        matches!(self, Self::RangeGenerational(_) | Self::StepGenerational(_))
    }

    /// Returns the maximum number of phases for scaled/generational mutations
    pub fn num_phases(&self) -> usize {
        match self {
            Self::RangeScaled(bandwidths) | Self::RangeGenerational(bandwidths) => bandwidths.len(),
            Self::StepScaled(steps) | Self::StepGenerational(steps) => steps.len(),
            _ => 1,
        }
    }

    /// Determines if the current phase should use post-clamping for Range mutations.
    /// - Static Range: always post-clamp (for local search)
    /// - RangeScaled/RangeGenerational: only post-clamp the final phase
    /// - Step mutations: always clamp
    /// - Others: not applicable
    pub fn should_post_clamp(&self, scale_or_phase: usize) -> bool {
        match self {
            Self::Range(_) => true, // Always post-clamp for static Range
            Self::RangeScaled(bandwidths) => {
                scale_or_phase >= bandwidths.len().saturating_sub(1) // Final phase only
            }
            Self::RangeGenerational(bandwidths) => {
                scale_or_phase >= bandwidths.len().saturating_sub(1) // Final phase only
            }
            Self::Step(_) | Self::StepScaled(_) | Self::StepGenerational(_) => true, // Always clamp steps
            _ => false,
        }
    }

    /// Returns true if this is the final phase for scaled/generational mutations
    pub fn is_final_phase(&self, scale_or_phase: usize) -> bool {
        match self {
            Self::RangeScaled(v) | Self::RangeGenerational(v) => {
                scale_or_phase >= v.len().saturating_sub(1)
            }
            Self::StepScaled(v) | Self::StepGenerational(v) => {
                scale_or_phase >= v.len().saturating_sub(1)
            }
            _ => true, // Static mutations are always "final"
        }
    }

    /// Checks if this mutation is equivalent to Random behavior for the given allele range.
    /// Useful for optimization - Random mutations can use more efficient sampling.
    pub fn is_full_range(&self, allele_min: &T, allele_max: &T) -> bool
    where
        T: PartialOrd + std::ops::Sub<Output = T> + Copy,
    {
        match self {
            Self::Random => true,
            Self::Range(bandwidth) => {
                let range_width = *allele_max - *allele_min;
                *bandwidth >= range_width
            }
            _ => false,
        }
    }

    /// Returns true if this is a Range-type mutation (uniform sampling within bandwidth)
    pub fn is_range_based(&self) -> bool {
        matches!(
            self,
            Self::Range(_) | Self::RangeScaled(_) | Self::RangeGenerational(_)
        )
    }

    /// Returns true if this is a Step-type mutation (discrete steps up or down)
    pub fn is_step_based(&self) -> bool {
        matches!(
            self,
            Self::Step(_) | Self::StepScaled(_) | Self::StepGenerational(_)
        )
    }
}
