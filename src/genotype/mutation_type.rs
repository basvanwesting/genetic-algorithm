use crate::allele::Allele;

/// Controls mutation behavior for numeric genotypes (Range and MultiRange).
///
/// Determines how genes are modified during mutation operations, from completely
/// random replacement to fine-grained local adjustments. Different mutation types
/// enable different search strategies and optimization characteristics.
///
/// # Design Philosophy
///
/// MutationType defines **what** mutations to apply (random, range-based, step-based)
/// and **how** they scale over phases. The **when** to advance phases is determined
/// by the strategy layer, not the genotype. This separation allows strategies to use
/// different triggers (performance-based, time-based, or custom) with the same
/// mutation types.
///
/// # Categories
///
/// Mutation types fall into two categories:
///
/// ## Static Mutations
/// Fixed behavior throughout the evolution process:
/// - `Random`: Full range replacement
/// - `Range(T)`: Fixed bandwidth range mutation (uniform sampling within ±bandwidth)
/// - `Step(T)`: Fixed step mutation (exactly +step or -step)
/// - `Discrete`: Integer-only mutations for categorical data
///
/// ## Scaled Mutations
/// Progress through phases based on strategy-determined triggers:
/// - `RangeScaled(Vec<T>)`: Range bandwidths that decrease through phases
/// - `StepScaled(Vec<T>)`: Step sizes that decrease through phases
///
/// The strategy decides when to advance phases based on its own criteria:
/// - Performance-based: Advance when `max_stale_generations` reached
/// - Time-based: Advance at `max_generations` intervals
/// - Custom: Any other trigger mechanism the strategy implements
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
/// Multi-phase range mutation with strategy-controlled progression. Each element
/// in the vector represents the mutation bandwidth for that phase. The strategy
/// determines when to advance to the next phase based on its own criteria.
///
/// **Example:** `RangeScaled(vec![100, 100, 50, 20, 5, 1])` on range `0..=100` provides
/// six phases of progressively focused search.
///
/// **Phase progression (strategy-dependent):**
/// - Performance-triggered: Advance when fitness plateaus (`max_stale_generations`)
/// - Time-triggered: Advance at fixed intervals (`max_generations`)
/// - Custom: Any other trigger the strategy implements
///
/// **Behavior by phase:**
/// - Phase 0: Mutations uniformly within full allele range (same as Random, pre-clamped)
/// - Phase 1: Mutations uniformly within full allele range (extended round, more exploration)
/// - Phase 2: Mutations uniformly within ±50 (pre-clamped)
/// - Phase 3: Mutations uniformly within ±20 (pre-clamped)
/// - Phase 4: Mutations uniformly within ±5 (pre-clamped)
/// - Phase 5: Mutations uniformly within ±1 (post-clamped, final phase)
///
/// **Fully Random mutation:** Random mutations can be achieved by setting the bandwidth to the
/// full allowed allele range (or higher, as it is pre-clamped).
///
/// **Use case:** Adaptive exploration-exploitation balance, coarse-to-fine search,
/// problems requiring different search granularities at different stages.
///
/// Prolonged periods of the same bandwidth can be achieved by setting multiple scales with the
/// same bandwidth value. You could also alternate between exploration and exploitation several
/// times (provide alternating high & low bandwidths in the scales)
///
/// ## `StepScaled(Vec<T>)`
/// Multi-phase step mutation with strategy-controlled progression. Like `RangeScaled`
/// but uses fixed step sizes instead of uniform ranges. Mutations apply the step
/// value either up or down (50/50 probability).
///
/// **Example:** `StepScaled(vec![10, 1])` on an integer range provides two
/// precision levels.
///
/// **Behavior by phase:**
/// - Phase 0: Mutations of exactly ±10
/// - Phase 1: Mutations of exactly ±1
///
/// **Use case:** Grid-like search spaces, systematic parameter sweeps, problems requiring exact
/// step sizes. Allows for [Permutation](crate::strategy::permutate) and
/// [HillClimb](crate::strategy::hill_climb)/steepest-ascent for (Multi)RangeGenotype
///
/// Prolonged periods of the same step size can be achieved by setting multiple scales with the
/// same bandwidth value. You could also alternate between exploration and exploitation several
/// times (provide alternating high & low step sizes in the scales)
///
/// ## `Discrete`
/// Treats the numeric range as discrete integer values, useful for encoding
/// categorical data or enum variants as numbers. Values are floored to integers
/// during mutation. Only supported by MultiRangeGenotype.
///
/// **Example:** Range `0.0..=4.0` represents 5 discrete choices: `{0, 1, 2, 3, 4}`,
/// which could map to enum variants or categorical options.
///
/// **Use case:** Heterogeneous chromosomes mixing categorical choices with continuous parameters.
///
/// ** Note: ** When all parameters are discrete, prefer
/// [ListGenotype](crate::genotype::ListGenotype) or
/// [MultiListGenotype](crate::genotype::MultiListGenotype) as these are more optimized and also
/// balance the mutation probablity per allowed value, not per gene.
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
/// Range mutations use different clamping strategies to handle under- and oversampling of the
/// allele boundaries.
///
/// ### Pre-clamping (Exploration Phases)
/// Used for all phases except the final one in `RangeScaled`:
/// - First constrains the sampling range to valid values: `[max(min, current-bandwidth), min(max, current+bandwidth)]`
/// - Then samples uniformly from this constrained range
/// - **Benefit**: Avoids boundary oversampling during exploration, when ranges are large
/// - **Drawback**: Cannot sample exact boundaries (zero probability for continuous values)
///
/// ### Post-clamping (Final Phase / Local Search)
/// Used for:
/// - Static `Range(T)` mutations (always)
/// - Final phase of `RangeScaled`
///
/// - First samples from the full range: `[current-bandwidth, current+bandwidth]`
/// - Then clamps the result to allele bounds
/// - **Benefit**: Can reach exact boundaries (important for fine-tuning)
/// - **Drawback**: Slight boundary oversampling when near edges
///
/// ### Example
/// For `RangeScaled(vec![50.0, 20.0, 5.0, 1.0])` on allele range `[0.0, 100.0]`:
/// - Phases 0-2: Use pre-clamping (avoid boundary oversampling)
/// - Phase 3: Uses post-clamping (ensure boundaries reachable)
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
/// - `RangeScaled`:
///   - Non-final phases: Pre-clamped, boundaries undersampled
///   - Final phase: Post-clamped, slight boundary oversampling
/// - `StepScaled`: Always clamped, slight boundary oversampling
/// - `Discrete`: Uniform sampling, no boundary bias
///
/// # Phase Management
///
/// For scaled mutations (`RangeScaled` and `StepScaled`), the current phase is
/// determined by a `current_scale` index provided by the strategy. The strategy
/// is responsible for:
/// - Tracking when to advance phases
/// - Providing the current scale index to the genotype
/// - Implementing the advancement logic (performance-based, time-based, etc.)
///
/// The genotype simply:
/// - Accepts the current scale index
/// - Applies the corresponding mutation parameters
/// - Stays at the final phase if the scale exceeds the vector length
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
/// * Other genotypes use fixed mutation strategies (always Random)
///
/// For time-based or performance-based scaling:
/// - Use `RangeScaled` or `StepScaled` with appropriate values
/// - Configure the strategy's trigger mechanism (`max_stale_generations` or `max_generations`)
/// - The strategy will handle phase advancement automatically
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
/// // Scaled exploration with proper clamping strategy
/// // Strategy controls when to advance phases
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
/// // Progressive step refinement
/// // Strategy decides when to move to finer steps
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
///         MutationType::Discrete,     // Boolean as 0 or 1
///         MutationType::Discrete,     // One of 5 algorithms
///         MutationType::Range(10.0),  // Uniform search within ±10.0 (post-clamped)
///         MutationType::StepScaled(vec![0.5, 0.1, 0.01]), // Decreasing steps
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
    /// Range bandwidths for scaled mutations (strategy controls phase advancement)
    RangeScaled(Vec<T>),
    /// Step sizes for scaled mutations (strategy controls phase advancement)
    StepScaled(Vec<T>),
    Discrete,
}
