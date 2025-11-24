# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.26.0] - 2025-11-24

> [!CAUTION]
> Custom `Crossover` implementations need to be updated to increment the age of
> chromosomes in their implementations of `call()` (e.g.
> `state.population.increment_age()`)

### Added
* Add multiple extension points to the `Evolve` flow in the `Extension` type
  * `after_selection_complete()`
  * `after_crossover_complete()`
  * `after_mutation_complete()`
  * `after_generation_complete()`
* Add symmetrical reporter hooks, called before the extension hooks
  * `on_selection_complete()` (existing)
  * `on_crossover_complete()`
  * `on_mutation_complete()`
  * `on_generation_complete()` (existing)
* Add `before()` and `after()` hooks to `Select`, `Crossover`, and `Mutate` traits
  * Allows full customization of population lifecycle operations in Evolve
  * Default implementations handle age filtering and cardinality updates

### Changed
* Extension trait methods now receive `&mut Self::Genotype` instead of `&Self::Genotype`
  * Allows extensions to modify genotype parameters during evolution
* Extension trait `call()` method is now `after_selection_complete()`
  * `call()` still exists and delegates to `after_selection_complete()` for backward compatibility
* Move population lifecycle operations from inline `EvolveState` methods to trait hooks
  * Age filtering moved to `Select::before()`
  * Population cardinality update moved to `Select::after()`
  * Age increment handled internally by crossover implementations

## [0.25.1] - 2025-11-12

### Changed
* Fix github build dependencies, as `v0.25.0` was marked as failed to build due
  to missing `libfontconfig1` dependency for plotter. No changes to code.

## [0.25.0] - 2025-11-12

### Changed
* (naming only) Refactor `MutationType::ScaledSteps(Vec<T>)` to
  `MutationType::StepScaled(Vec<T>)` for naming consistency
* (naming only) Refactor `MutationType::RelativeRange(T)` to
  `MutationType::Range(T)`, as `MutationType::Step(T)` is also relative.
* Refactor `MutationType::Transition(usize, usize, T)` to `MutationType::RangeScaled(Vec<T>)`:
  * Drop the generation based transition configuration params (as it couldn't handle staleness)
  * Replace params with bandwidth per scale (just like step-size per scale in `MutationType::StepScaled`)
  * Use bandwidths equal to the full `allele_range` to achieve effective `MutationType::Random` mutation
  * Now `max_generations` and `max_stale_generations` handle scale transitions in the same manner everywhere
  * It is a good idea to use a prolonged period of full Random sampling (exploration phase, so
    several 100% bandwidth scales), which then transitions quite fast to the smaller bandwidths
    (exploitation phase, a few smaller bandwidth scales).

### Added
* Add `MutationType::Step(T)` for completeness
* Add `max_generations` as scale trigger as well in `Evolve` and `HillClimb`. Now
  `max_generations` and `max_stale_generations` behave the same: 
  * Ending condition for current scale, go to next scale and restart counting
    generations
  * Final ending condition if no scales left (or no scaling at all)
  * Both `max_generations` and `max_stale_generations` can be set at the same
    time, the first to reach the condition triggers and reset both
* Support unsigned integers for `MutationType::Range` and `MutationType::Step` as well (and scaled versions)
* Allow for zero bandwidth/step in `MutationType::Range` and
  `MutationType::Step` (and scaled versions), which lets the mutation die out (more for robustness than usefulness)
* Add `examples/visualize_evolve_mutation_types` and `examples/visualize_permutate_mutation_types`, which
  generate visualizations showing exploration patterns of different mutation strategies

### Removed
* Remove `increment_generation()` and `reset_generation()` from `Genotype` and remove hook
  in Strategies, as all scaled MutationTypes now work in the same manner
* Remove unused `Vec<Chromosome<T>>` into `Population<T>` implementation

## [0.24.0] - 2025-11-05

### Changed
* Change `MutationType::Scaled(Vec<RangeInclusive<T>)` to `MutationType::ScaledSteps(Vec<T>)`
  * removes the range as the steps are symmetrical by definition: -T, +T (also less error prone)
  * removes potential confusion about inner range samping v. step up/down behaviour by explicit naming
* Change `MutationType::Relative(RangeInclusive<T>)` to `MutationType::RelativeRange(T)`
  * removes the range as it is symmetrical by definition, only the bandwidth is needed to make a [-T,T] range (also less error prone)
  * removes potential confusion about inner range samping v. step up/down behaviour by explicit naming
* Change `MutationType::Transition(usize, usize, RangeInclusive<T>)` to `MutationType::Transition(usize, usize, T)`
  * follow the `MutationType::RelativeRange` parameters

## [0.23.0] - 2025-11-04

### New MutationType::Transition
`Transition(random_until_generation, relative_from_generation, relative_mutation_range)`
Smoothly transitions from Random to Relative mutation over a specified generation range.
Provides a natural exploration-to-exploitation schedule without abrupt behavioral changes.

**Parameters:**
- `random_until_generation`: Use pure Random mutation until this generation
- `relative_from_generation`: Use pure Relative mutation from this generation on
- `relative_mutation_range`: The final relative range to use after transition

**Example:** `Transition(100, 500, -5.0..=5.0)` means:
- Generations 0-99: Pure Random mutation (full exploration)
- Generations 100-499: Gradual transition with decreasing mutation range
- Generations 500+: Pure Relative mutation with ±5.0 range (exploitation)

**Use case:** Problems requiring initial exploration followed by convergence,
evolutionary algorithms with scheduled exploration decay, parameter optimization
where good regions are unknown initially.

### Added
* Implement `MutationType::Transition` for `RangeGenotype` and
  `MultiRangeGenotype`
* Add associated type `Genotype` to `Extension` as well for better custom
  implementations (just like `Mutate` & `Crossover`)
* Add associated type `Genotype` to `Select` as well for symmetry reasons (it
  was the only one without associated type left)
* Add `increment_generation()` and `reset_generation()` to `Genotype` and hook
  up in Strategies (for keeping track of `MutationType::Transition` progess
  inside the genotype)
* Add `SelectEvent(String)` and `CrossoverEvent(String)` tuple struct and
  handlers to reporting (`on_select_event` & `on_crossover_event`)

### Changed
* Add mutation details to `MutationType` enum as tuple structs:
  * Relative now holds the mutation range for a gene:
    `MutationType::Relative(RangeInclusive<T>`
  * Scaled now holds the mutation scales for a gene:
    `MutationType::Scaled(Vec<RangeInclusive<T>>)`
  * Random and Discrete don't have details, so they stay the same
  * `.with_mutation_type(s)` replaces the deprecated
    `with_allele_mutation_range(s)` and `with_allele_mutation_scaled_range(s)`
    builder steps, now able to hold all the mutation details by itself
* Remove `MutateEvent` and `ExtensionEvent` enums, all events are just plain
  texts wrapped in a single `MutateEvent(String)` or `ExtensionEvent(String)`
  tuple struct
* Rename `Genotype` custom method `sample_allele` to `sample_gene_random` for
  symmetry reasons with `sample_gene_delta,` even though technically it is not
  always a gene (no index)
* Drop `u64`, `u128`, `usize`, `i64`, `i128` & `isize` support for
  `RangeAllele` and add more numeric Traits
  (`Add`,`Sub`,`AddAssign`,`Into<f64>`,`SampleUniform`) to allow for more
  arithmetic in the genotype
* Rename `on_new_generation` reporting event in `Evolve` to
  `on_selection_complete` to clearly state intent. Rename other
  `on_new_generation` reporting events to `on_generation_complete`
  (`HillClimb`, `Permutate` and new for `Evolve`). For `Evolve` the
  `on_selection_complete` hook, is a more interesting point in the loop, as the
  population and cardinality are refreshed after selection. The
  `on_generation_complete` hook contains all the new mutations, most of which
  will be immediately selected out. This gives the false impression of good
  cardinality while there is actually little.

### Removed
* Drop `Genotype`'s `gene_slice` method as all genes are owned anyway

### Deprecated
* `with_allele_mutation_range` & `with_allele_mutation_scaled_range` now have
  deprecation warnings, as they are replaced by `with_mutation_type`
* `with_allele_mutation_ranges` & `with_allele_mutation_scaled_ranges` now have
  deprecation warnings, as they are replaced by `with_mutation_types`

## [0.22.0] - 2025-10-30

### Heterogeneous Genotype Support
`MultiRangeGenotype` now supports heterogeneous chromosomes that mix different
gene semantics (continuous values, discrete choices, booleans) within a single
numeric type `T`. See `MultiRangeGenotype` documentation for example.

Use `.with_mutation_types(vec![...])` to specify behavior for each gene individually:

* `MutationType::Random` - Samples uniformly from the full allele range
* `MutationType::Relative` - Mutates within a relative range around the current
  value (set for gene through `allele_mutation_ranges`)
* `MutationType::Scaled` - Progressive refinement through multiple scale levels
  around the current value (set for gene through `allele_mutation_scaled_ranges`)
* `MutationType::Discrete` - Rounded-to-integer values with uniform selection
  (like `ListGenotype`).
  * Mutations ignore current value - all rounded-to-integer in range equally likely
  * Range `0.0..=4.0` yields values: 0.0, 1.0, 2.0, 3.0, 4.0 (with equal probability)
  * Useful for encoding: enums (0.0..=4.0), booleans (0.0..=1.0), or discrete choices
  * Neighbours and permutations include all integer values in the allele range

For backward compatibility, mutation type per gene is auto-detected from
provided ranges in the following order (as it was before, implicitly):
* Has `allele_mutation_scaled_ranges` → scaled for all genes
* Has `allele_mutation_ranges` → relative for all genes
* Else → random for all genes

Explicit `.with_mutation_types()` overrides any auto-detected mutation range settings.

### Added
* Add optional `.with_mutation_types()` to MultiRangeGenotype builder

### Changed
* Move `.mutation_type()` from `Genotype` Trait to individual genotype
  implementations, as it is no longer a single uniform value for all genotypes.

## [0.21.0] - 2025-10-06

### Design choices and internal impact (API changes listed separately below)
The primary goal was easier custom `Mutate` and `Crossover` implementations,
which would require a high level of Genotype unification and simplification.
This was mostly blocked by the GPU optimization premise provided by the
centralized `DynamicMatrixGenotype` and `StaticMatrixGenotype` with all their
added centralized complexity (`ChromosomeManager` and `GenesPointer` v. `GenesOwner`
etc...). 

The first attempt was to split the library in two: centralized v. distributed,
where the distributed track could become less genotype-heavy and more
flexible. But it also happens the GPU zero-copy optimization premise was flawed
as memory transfers are required regardless of pre-transfer layout. So on the
end we just dropped the whole centralized approach and archived it in
[archive/centralized-gpu-experiment branch](https://github.com/basvanwesting/genetic-algorithm/tree/archive/centralized-gpu-experiment) 
for later use, when zero-copy actually becomes viable.

Now the library is restructured to a simpler form, moving a lot of
responsibilities away from `Genotype` which was becoming too heavy and
centralized: All genes are now `Vec<Allele>` and owned by the `Chromosome`
(which now only has one implementation, no genotype specific variants anymore).

Chromosome recycling has been moved from the `Genotype` (`ChromosomeManager`)
to the `Population`, the enabling flag is on `Genotype`. So when making custom
implementations remember to use the population's `new_chromosome()`,
`drop_chromomsome()`, `truncate()`, `truncate_external()` &
`extend_from_within()` methods for the chromosomes. Or just disable the
recycling (no risk of errors).

However, `Genotype` unification still proved impossible - each type has
fundamentally different requirements. So the best route to allow for easier
custom `Mutate` and `Crossover` implementations, was to make them user-genotype
specific using an associated type `Genotype` on `Mutate` and `Crossover` traits
(following existing `Fitness` pattern). The Genotypes now also have some
implementation-specific helper methods to support custom implementations:
`sample_allele()`, `sample_gene_delta()`, `sample_gene_index()`,
`sample_gene_indices()`.

Skip the associated type `Genotype` on `Select` and `Extension` for now, as
these mainly work with the chromosome metadata and are not genotype specific.

General usage by client has little impact, most is internal.

### API Impact
* Replace `BinaryChromosome` with `Chromosome<bool>`
* Remove all other `Chromosome` struct genotype specific prefixes (e.g. `ListChromosome<..>` -> `Chromosome<..>`)
* Use `genetic_algorithm::impl_allele!(CustomAllele);` to implement `Allele` for your CustomAllele

### Changed
* Add associated type `Genotype` to `Mutate` and `Crossover` traits (following
  existing `Fitness` pattern).
* Move chromosome recycling from `ChromosomeManager` to `Population`
* Move genes hashing from the `Genotype` to `Chromosome` making the `Allele`
  trait responsible for the hashing implementation (with `impl_allele!` macro
  for default implementation)
* Moved `current_scale_index` from `StrategyState` to `Genotype`. This is the
  only change adding `Genotype` logic. The scaling is only implemented by
  `RangeGenotype` and `MultiRangeGenotype`, so it felt more genotype specific.
  It does make the `Genotype` mutable again, which is an accepted tradeoff.
> [!CAUTION]
> Remember to `.reset_scale_index()` when reusing your genotype with a custom
> implementation of multiple runs! (the library's implementation for multiple
> runs, e.g. `call_repeatedly()`, clones the builder and then casts it to the
> specific strategy, so each run starts with a cloned unused genotype)

### Removed
* Remove matrix genotypes (`DynamicMatrixGenotype`, `StaticMatrixGenotype`)
* Remove `ChromosomeManager` trait
* Remove `GenesOwner` and `GenesPointer` traits
* Remove `BitGenotype` - use `BinaryGenotype` with `Vec<bool>` instead
* Remove `calculate_for_population()` as the population-level fitness calculation user
  hook. All is now `calculate_for_chromosome()` which can now be compile time guarded

## [0.20.5] - 2025-05-21
### Added
* Add `PermutableGenotype` support for scaled `RangeGenotype` and scaled
  `MultiRangeGenotype.` This approach implements a increasingly localized grid
  search with increasing precision using the `allele_mutation_scaled_range(s)`
  to define the search scope and grid steps
  * First scale (index = 0) traverses the whole `allele_range(s)` with the
    upper bound of the first scale as step size.
  * Other scales (index > 0) center around the best chromosome of the previous
    scale, traversing the previous scale bounds around the best chromosome with
    the upper bound of the current scale as step size.
  * Scale down and repeat after grid is fully traversed
  * _Note: Not implemented for relative or random mutation types_
  * _Note: Not implemented for `DynamicMatrixGenotype` and
    `StaticMatrixGenotype`, as matrix has no use in permutation_
  * _Note: Scales should be symmetrical around zero, as always, but good to
    remember_
  * _Note: As an added benefit the generic `StrategyBuilder` with deferred
    specialization can now also be used for `RangeGenotype` and
    `MultiRangeGenotype`_
* Keep track of `scale_generation` next to `current_generation` in
  `StrategyState`, resets every scale increment, no use yet
* Add `mutation_type_allows_permutation()` guard on `Genotype` and check in
  `Permutate` strategy builder

### Changed
* Add `chromosome` and `scale_index` parameters to `PermutateGenotype`
  `chromosome_permutations_into_iter()` function, ignore for all existing
  implementations, used by `RangeGenotype` and `MultiRangeGenotype`

## [0.20.4] - 2025-05-15
### Fixed
* The `replacement_rate` in selection was not working, as the age was
  incremented before selection leading to only parents and no offspring during
  selection. Effectively the whole population was selected against without
  partitioninig. Fix by moving the age increment from the start of the new
  evolve generation (before selection) to just before crossover (after
  selection and after extension, as extension is form of selection where
  offspring information might still be relevant as well).
* For `GenotypeMultiRange`, remove the weighted probability of mutating a gene,
  depending on its allele_range size. The assumption of weighted mutation
  probability holds for `GenotypeMultiList` and `GenotypeMultiUnique`, in order
  to avoid over mutating the smaller sets with regard to the larger sets. But
  it does not for `GenotypeMultiRange`. We cannot make the assumption that the
  range size says anything about the search space size. Now probability of
  mutating a gene is uniform, regardless of its allele_range size in
  `GenotypeMultiRange`.

### Changed
* `MutateMultiGene` and `MutateMultiGeneDynamic` now avoid mutating the same
  genes twice
* `MutateMultiGene` and `MutateMultiGeneDynamic` now mutate the exact
  `number_of_mutations` times if mutation is occuring

### Added
* Add `MutateMultiGeneRange` to provide the previous `MutateMultiGene`
  behaviour. This samples the `number_of_mutations` from the provided
  `number_of_mutations_range`. And it allows for duplicate mutations of the
  same gene (as it is less strict anyway)
* Add parents and offspring size to `on_new_generation` reporting of
  `EvolveReporterSimple`

## [0.20.3] - 2025-05-14
### Added
* Initialize first `HillClimb` `SteepestAscent` population with the
  `seed_genes_list`, to ensure the best seed is taken as the starting point
  when seeds are provided.

## [0.20.2] - 2025-05-12
### Added
* Add `max_generations` ending condition on `Evolve` and `HillClimb` strategies
  (also blocked by optional `valid_fitness_score,` as `max_stale_generations` is)
* Add `ExtensionMassDeduplication` which requires `with_genes_hashing(true)` on
  the `Genotype` (otherwise ignored)
* Add `Population` `unique_chromosome_indices()` and
  `best_unique_chromosome_indices()` support functions

### Changed
* Nuance `ExtensionMassGenesis` trying to use unique Adam en Eve Chromosomes
  when genes_hashes are availble, otherwise just take 2 best chromosomes
  (possibly duplicates)
* Decided not to use unique best chromosomes for the elitism_rate in
  `ExtensionMassDegeneration` and `ExtensionMassExtinction`. Just use the best
  chromosomes (possibly duplicates), as uniqueness hard-limits the amount of selected
  elites in the typical low cardinality situation, which seems unwanted behaviour

## [0.20.1] - 2025-05-08
### Added
* Add `elitism_rate` to `ExtensionMassExtinction` and
  `ExtensionMassDegeneration`. The `elitism_rate` ensures the passing of the best
  chromosomes before extinction or degeneration is applied

### Fixed
* Fix `best_chromosome_indices()` on `Population` in case where there are no
  fitness values other than None

## [0.20.0] - 2025-05-08

### Changed
* Completely redo `Selection` in order to align with general genetic-algorithm
  terminology:
  * `replacement_rate`: the target fraction of the population which exists of
    children. Generational Replacement and Steady-State Replacement can both be
    modelled with this parameter by setting it respectively to 1.0 and 0.2-0.8.
    High values converge faster, but risk losing good solutions. Low values
    convergence slower. If there is a shortage of population after the ideal
    fraction, firstly remaining non-selected children and secondly remaining
    non-selected parents will be used to fill the shortage to avoid population
    collapse.
  * `elitism_rate`: a non-generational elite gate, which ensures passing of the
    best chromosomes before selection and replacement takes place. Value should
    typically be very low, between 0.01 and 0.05. Relevant for
    `SelectTournament` where the best chromosome is not guaranteed to be
    selected for a tournament if the `population_size` is larger than the
    `target_population_size`
* Completely redo `Crossover` in order to align with general genetic-algorithm
  terminology:
  * `selection_rate`: the fraction of parents which are selected for
    reproduction. This selection adds offspring to the population, the other
    parents do not. The population now grows by the added offspring, as the
    parents are not replaced yet. Value should typically be between 0.4 and
    0.8. High values risk of premature convergence. Low values reduce diversity
    if overused.
  * `crossover_rate` (or recombination-rate): the fraction of selected parents
    to crossover, the remaining parents just clone as offspring. Value should
    typically be between 0.5 and 0.8. High values converge faster, but risk
    losing good solutions. Low values have poor exploration and risk of
    premature convergence
* Note that `max_chromosome_age` is implemented at the `EvolveBuilder` level,
  it will remove old chromosomes in the evolve loop before selection. The
  `elitism_rate` will not save them.
* Note that `population_size` now grows with added offspring during crossover
  and no longer replaces parents in-place with children. This means the
  `StaticMatrixGenotype` needs to reserve extra population space beyond the
  `target_population_size`
* Note that in previous releases all parents had an in-place crossover. The
  behaviour from previous releases can be achieved by setting the
  replacement_rate to 1.0 (drop parents), the selection_rate to 1.0 (all
  crossover) and the crossover_rate to 1.0 (all crossover). The new release is
  a bit slower as in-place crossover was faster than keeping the parents around
  and dropping them during selection
* Rename and refactor some internal `Chromosome` recycling methods

### Added
* Add `best_chromosome_indices()` on `Population`, used to implement
  `elitism_rate` without the need for a full sort
* Add `CrossoverRejuvenate` as new limited cloning implementation for the old
  `CrossoverClone` behaviour, which had limited cloning. The new
  `CrossoverClone` now adds actual clones as offspring without removing the
  parents, which is much more cloning than the original. For
  `CrossoverRejuvenate`: drop non-selected parents, then clone top parents to
  repopulate, then rejuvenate selected parents to children in place. No copying
  of chromosomes for creating the offspring itself, only for repopulating the
  dropped non-selected parents (smaller fraction). However the cloning of
  top-parents is crucial for driving the Evolve process as experimenting with
  the evolve_nqueens example demonstrated.

### Removed
* Remove `reference_id: usize` from `Chromosome` as user controlled alternative
  to genes_hash, as genes_hash is now formally supported for all types.

## [0.19.4] - 2025-05-05

### Added
* Add `with_fitness_cache(size)` builder step to the `HillClimbBuider` as well.
  There are some use cases with long tails and `replace_on_equal_fitness(true)`
  where caching information is useful.
* Add `FitnessGenes<Self>` type alias for better fitness API (next to
  `FitnessChromosome<Self>` etc.)

## [0.19.3] - 2025-04-15

### Fixed
* Forgot to `calculate_genes_hash()` after `chromosome_constructor_genes()`,
  refactor a bunch to ensure this never happens again.

## [0.19.2] - 2025-04-15

### Changed
* Cycle through the `seed_genes_list` to fill the initial population for
  `Evolve` strategy (instead of random sampling from the seed genes). This is
  done to ensure all seed genes reach the initial population (if the
  `target_population_size` is larger than the `seed_genes_list`). The
  `HillClimb` strategy still samples a single random starting seed gene as the
  starting point for each run (not cycling through them in repeated runs)

## [0.19.1] - 2025-04-15

### Changed
* Silently ignore zero cache size in `with_fitness_cache(size)`, to support
  superset builder strategies where cache size is derived from unset parameters
  (defaulting to zero)

## [0.19.0] - 2025-04-15

### Added
* Add `with_fitness_cache(size)` builder step to the `EvolveBuilder`. When
  applying this step, a thread-safe `FitnessCache` object is stored on the
  `EvolveConfig` which manages an `Arc`-wrapped LRU cache of fitness values for
  genes.
  * Note that caching only works when the `genes_hash` is stored on chromosome
    as well (through the `with_genes_hashing()` builder step), as this is the
    cache key.
  * Note the `FitnessCache` is stored on `EvolveConfig`, not `EvolveState`, as
    the cache is external to the strategy (and reused over multiple repeated
    runs).
  * Note that caching is only useful for long stale runs, but it is better to
    avoid those in general. This makes the cache hit/miss reported in the
    `EvolveReporterSimple` more of a hint where the hyperparameters should be
    adjusted to increase population diversity. I don't think caching is the
    proper solution to ovelry revisiting the same genes. Keeping the feature
    for now though, as the hint is valuable in itself.
  * Note that +0.0 and -0.0 hash differently on floats when using
    `with_genes_hashing()`.
  * Decided not to support the fitness cache in the `Permutate` and `HillClimb`
    strategies as these should (almost) never revisit the same genes anyway.

### Changed
* Update pprof to v0.14.0 due to security issue on v0.13.0
* Change some internal `Fitness` trait method parameters as the
  `StrategyConfig` and `FitnessCache` need to be passed around.

## [0.18.1] - 2025-01-13

### Changed
* Use `rustc_hash::FxHasher` instead of `DefaultHasher` for 2x-5x faster genes hashing

## [0.18.0] - 2025-01-12

### Added
* Option to store `genes_hash` on `Chromosome` by setting
  `with_genes_hashing(true)` on the `Genotype.` This can be used for better
  population cardinality estimation (with respect to the default fitness score
  based estimate), but has a relatively high overhead on the main `Evolve` loop
  (mostly noticable in `Crossover` duration).
* Store `population_cardinality` on `EvolveState`, using `genes_hash`
  cardinality if set, otherwise fallback to fitness score cardinality. Update the
  `population_cardinality` just after selection in the `Evolve` loop and use for
  rest of the generation. `Crossover` only extends existing cardinality if
  present. `Mutation` always adds cardinality, but might also all be dropped in
  selection again. So using the cardinality at the beginning of the `Evolve` loop
  is a good enough estimate

### Changed
* No longer count `Nones` as unique fitness score cardinality. Instead make
  `fitness_score_cardinality()` return `None` of no values can be used to
  estimating the cardinality (ensuring we keep avoiding the immediate `Extension`
  trigger at the start of the iteration)
* Make `MutateSingleGeneDynamic`, `MutateMultiGeneDynamic`, `ExtensionMassGenesis`,
  `ExtensionMassExtinction` and `ExtensionMassDegeneration` use `EvolveState`
  `population_cardinality` as guard check

### Dropped
* Remove old `GenesKey` type and `genes_key()` function, replaced by `GenesHash`
* Disallow `f32` and `f64` as `Allele` for `List` and `Unique` genotypes by
  requiring the `Allele` to implement `Hash` for those `Genotypes` (including
  tuple Alleles). Now we can use standard Hashing for normal Alleles and use
  `bytemuck` Hashing for `RangeAllele`. This allows for easier implementation of
  genes based cardinality

## [0.17.1] - 2024-10-10

### Changed
* Make `MutateMultiGene` and `MutateMultiGeneDynamic` mutate up to the provided
  `number_of_mutations` (instead of always the exact amount of mutations)

## [0.17.0] - 2024-10-04

### Added
* Add `on_exit()` event to `StrategyReporter`
* Add `StrategyVariant` info in `on_enter()` and `on_exit()` events
* Add `fitness_duration_rate()` to `StrategyState` and use in `StrategyReporter`

### Changed
* Permutation with `seed_genes_list` now only permutates over the seeded genes (useful to calculate only a specific predefined set)
* Add cleanup step in strategies. As the repeated/speciated calls keep the runs around, it seems better to cleanup the population and genotype storage
* Rename `StrategyAction::Init` to `StrategyAction::SetupAndCleanup`
* Rename `on_init()` event to `on_enter()` in `StrategyReporter`
* Move duration reporting to `on_exit()` in reporters to include the cleanup duration

## [0.16.0] - 2024-09-18

### Changed
* Add a buffered option to all Reporters (through `new_with_buffer()`)
  * When the buffer is off (default), the reporter will print to stdout
  * When the buffer is on, the reporter will print to the internal buffer,
    which can be read out through `flush_reporter()` on the strategy later
* Change `call_repeatedly()`, `call_par_repeatedly()`, `call_speciated()` and
  `call_par_speciated()` to return a tuple `(best_run, other_runs)`. This way the
  reporter buffers of the other runs can be read out as well afterwards

### Dropped
* Drop `StrategyReporterBuffer`, as all reporters now have a buffered option

## [0.15.1] - 2024-09-17

### Added
* Add `StrategyReporterBuffer` implementation with an internal buffer instead of stdout
* Add `flush_reporter()` in `Strategy` as the strategy can be boxed and we need
  a way to get to the buffer of the `Reporter`

## [0.15.0] - 2024-09-17

### Changed
* Implement `StrategyReporter` trait 
  * It was held off in previous releases, because of the generics in the API,
    but it is needed for the superset `StrategyBuilder`
  * Provide `StrategyReporterNoop`, `StrategyReporterDuration` and
    `StrategyReporterSimple` reporters, usable by all strategies (but less
    informative)
  * Re-export `StrategyReporterNoop` as `EvolveReporterNoop,` `HillClimbReporterNoop`
    and `PermutateReporterNoop`
  * Re-export `StrategyReporterDuration` as `EvolveReporterDuration,`
    `HillClimbReporterDuration` and `PermutateReporterDuration`
  * Re-implement `EvolveReporterSimple,` `HillClimbReporterSimple,`
    `PermutateReporterSimple` as strategy specialized versions
* Implement `EvolveGenotype` for all genotypes (as it was implicit) and move
  crossover functions over from `Genotype`
* Rename `IncrementalGenotype` to `HillClimbGenotype`
* Rename `PermutableGenotype` to `PermutateGenotype`
* Move `Extension` step to follow after `Select` step, so the `fitness_score_cardinality`
  of the selected population can be taken as a trigger
* Move reporter event `on_new_generation` to after selection for `Evolve`, as it
  is a more informative location in the loop

### Added
* Add `StrategyBuilder,` a superset builder which delays specialization to
  `Evolve`, `HillClimb` or `Permutate`. Only usable by genotypes that implement
  all strategies (i.e. not for `RangeGenotype` and friends)
* Add `call()`, `call_repeatedly()`, `call_par_repeatedly()`, `call_speciated()` and
  `call_par_speciated()` to `StrategyBuilder` (fallback to equivalent of not
  available on specialized strategy)
* Add `EvolveVariant` (Standard only) and `PermutateVariant` (Standard only) for
  symmetry next to `HillClimbVariant`
* Implement `mutation_type()` for all genotypes. Use it to trigger scaling logic
  in `Evolve` and `HillClimb.`
* Implement `MutationType::Random` in `fill_neighbouring_population()` for
  `RangeGenotype`, `MultiRangeGenotype`, `DynamicMatrixGenotype` and `StaticMatrixGenotype`.
  It is not advised to use in `HillClimb` context, but a panic is worse.

### Dropped
* Drop `EvolveReporterLog`, `HillClimbReporterLog` and `PermutateReporterLog`

## [0.14.0] - 2024-09-12

### Design choices and internal impact (API changes listed separately below)
* In order to support future GPU acceleration, the possibilty for the Genotype
  to store the genes of the whole population in a single contiguous memory
  location has been added. This has the following effects:
  * The Genotype has to be mutable and passed along for most operations.
    Affects a lot of interal function paramters
  * Chromosomes no longer always own their own genes, but can also just point
    to a section of the genotype's data storage. New triats `GenesOwner` (has
    genes) and `GenesPointer` (has row_id) for chromosomes
  * Each Genotype now has its own type of Chromosome, which implements the
    Genotype's genes storage model. Genotypes get an associated Chromosome type,
    all with their own alias. This leads to three core chromosome
    implementations:
      * `VectorChromosome`, stores `Vec<Allele>` in genes field (the original chromosome)
      * `BitChromosome`, stores `FixedBitSet` in genes field
      * `RowChromosome`, stores genotype data row in row_id fields
  * Chromosomes can't just be created, cloned and dropped, as the genotype
    needs to keep track of which sections of the data storage are in use.
    Therefore Genotype now is the constructor and destructor of chromosomes.
    Added `ChromosomeManager` trait to implement on the Genotoype. The
    strategies and plugins now need to properly call population reduction and
    regrow methods through this ChromosomeManager trait
  * The `Fitness` now has two implementation points, the normal
    `calculate_for_chromosome(...)` and a population level
    `calculate_for_population(...)`. The latter is optional and can be used to
    calculate the genotype data strucure as a whole.
  * Because the chromosomes no longer always hold the genes, returning a
    `best_chromosome()` as end result from the strategies is not really suitable
    anymore. The method still is available when using standard chromosomes, but
    the new `best_genes_and_fitness_score()` is the preferred method as it works
    for all chromosome types.
  * The best_genes are now stored on the Genotype, instead of in a chromosome clone
    on the StrategyState. The latter would require the Genotype internal storage
    to keep a row reserved for the best_chromosome clone, which isn't nice.
* Because the Genotype now constructs and destructs chromosomes, this feature
  can be leveraged to recycle chromosome allocations for all Genotypes. This leads
  to an overal performance improvement, especially noticable for large genes
  sizes and low survival rates.

### Changed (API)
* The `calculate_for_chromosome(...)` client implementation now gets a
  reference to `Genotype`, which can subsequently be ignored for standard use.
* The `EvolveReporter`, `HillClimbReporter` and `PermutateReporter` traits now
  also get a reference to `Genotype` on all functions, for the same reason
* Add `FitnessGenotype<Self>`, `FitnessChromosome<Self>` &
  `FitnessPopulation<Self>` type aliases for better fitness API

### Added (API)
* Add `DynamicMatrixGenotype`, storing the population's genes in a single
  contiguous memory `Vec<Allele>` on the heap. All other features are like
  `RangeGenotype`.
* Add `StaticMatrixGenotype`, storing the population's genes with a single
  contiguous  memory `Box<[[T; N]; M]>` on the heap. All other features are like
  `RangeGenotype`. `N` is the genes size and `M` is the population size:
  * For `Evolve`, `M` would be the `target_population_size`
  * For `HillClimbVariant::SteepesAscent`, `M` would be the `neighbouring_population_size`
* Add `calculate_for_population(...)` client implementation option in
  `Fitness`, only usable for the new matrix genotypes above.
* Add `best_genes_and_fitness_score()` function on Evolve, HillClimb and
  Permutate. Prefer this use over `best_chromosome()`.
* Add utility methods `genes_slice(chromosome)` and `best_genes_slice()` to
  `Genotype`, returning the genes as a slice for all chromosome types
  (`GenesOwned` or `GenesPointer`)
* Add `Fitness` placeholder `SumDynamicMatrix` for `DynamicMatrixGenotype` (with optional precision)
* Add `Fitness` placeholder `SumStaticMatrix` for `StaticMatrixGenotype` (with optional precision)
* Add `EvolveReporterDuration`, `HillClimbReporterDuration` and
  `PermutateReporterDuration` to report only on the duration of the different
  phases of the strategies

### Removed (API)
* Drop `HillClimbVariant::StochasticSecondary` and
  `HillClimbVariant::SteepestAscentSecondary` as `call_repeatedly(...)` on the
  basic variant is much more efficient
* Drop `CrossoverParMultiPoint` as it conflicts with storage owning
  `Genotypes`. You would have to provide a mutable Genotype in a Mutex, which is
  not worth the effort


## [0.13.0] - 2024-09-11

### Changed
* Rename `Compete` to `Select`
* Redo `Select`/`Crossover` selection & survival rates:
  * Remove `Crossover` `parent_survival_rate` and add `Select` `selection_rate`
  * `Select` now reduces the population and `Crossover` restores the population with the best parents after creating new offspring
  * Simulate the old behaviour of keeping all the parents by setting the `selection_rate = 0.5` and doubling the target_population_size
* Implement `Allele` for `()` and set `BitGenotype::Allele = ()` as it is not used

### Removed
* Drop `BinaryAllele` type alias for `bool`. It is the only of it's kind and never used

### Fixed
* Fix `SelectElite` sorting (should be best first, was best last). Effect was
  that the best part of population was dropped instead of kept, resulting in slow to
  no solution in `Evolve`

## [0.12.1] - 2024-09-07

### Fixed
* Fix `CompeteElite` sorting (should be best first, was best last). Effect was
  that the best part of population was dropped instead of kept, resulting in slow to
  no solution in `Evolve`

## [0.12.0] - 2024-09-03
This is a major breaking release (back to pre-v0.9.0 API), see Changed:

### Changed
* Add formal `Genes` trait: `Genes: Clone + Send + Sync + Debug`
* Change associated type from `Allele` to `Genotype` for: `Fitness`,
  `EvolveReporter`, `HillClimbReporter` and `PermutateReporter`
* Change generic type `Allele` to `Genotype` for: `Chromosome`, `Population`
  and other structs/functions using these types
* Store `Genotype::Genes` instead of `Vec<Genotype::Allele>` in the `Chromosome`
  `genes` field

### Addded
* Allow for non-`Vec` based genes in `Genotype.` Most existing `Genotype`
  implementations use `Vec<Allele>` as genes, but now alternatives are possible
* Add `BitGenotype` using `FixedBitSet` for genes storage. Functionally the same as
  `BinaryGenotype,` but better for large genes sizes as storage is much more
  efficient than `Vec<bool>`.
* Add `Fitness` placeholder `CountOnes` for `BitGenotype`

## [0.11.1] - 2024-09-07

### Fixed
* Fix `CompeteElite` sorting (should be best first, was best last). Effect was
  that the best part of population was dropped instead of kept, resulting in slow to
  no solution in `Evolve`

## [0.11.0] - 2024-09-02

### Changed
* Change `Crossover`'s `keep_parent` parameter to `parent_survival_rate`. This
  keeps a fraction of the top parents, instead of the previous all or nothing boolean.
* Add duration tracking of interal actions for `Evolve`, `HillClimb` & `Permutate` 
* Add duration tracking results in `EvolveReporterSimple`,
  `HillClimbReporterSimple` and `PermutateReporterSimple`
* Remove `Genotype` parameter from `EvolveReporter`, `HillClimbReporter` and
  `PermutateReporter` `on_start` event, use `on_init` event for reporting about
  `Genotype`

### Removed
* Drop `ExtensionMassInvasion` as reseeding the population conflicts with
  scaling. And reseeding can better be done by `call_repeatedly` or
  `call_speciated` anyway

### Fixed
* Remove yanked package warning for `bytemuck v1.16.1` in Cargo.lock by updating all dependencies to latest versions

## [0.10.3] - 2024-08-29

### Added
* Add performance considerations in documentation
* Add `CrossoverMultiGene`, `CrossoverMultiPoint` and `CrossoverParMultiPoint`
* Add `Fitness` placeholders `Countdown` and `CountdownNoisy`

### Changed
* Move all crossover and mutation logic to `Genotype`. The goal is to limit the
  knowledge of the internal genes structure to the Genotype only. Lots of internal
  changes, not relevant for API.
* Improve `CrossoverSinglePoint` and `CrossoverMultiPoint` performance by avoiding cloning of genes
* Reimplement `CrossoverUniform` as `CrossoverMultiGene` with
  `number_of_crossovers = genes_size / 2` and `allow_duplicates = true`, 
  no API change

### Removed
* Drop `CrossoverParUniform` in favor of `CrossoverParMultiPoint` as a more useful
  example implementation, although parallel execution of crossovers have no
  performance benefits for most situations

## [0.10.2] - 2024-08-26

### Added
* Add `with_rng_seed_from_u64_option()` function to `EvolveBuilder` and `HillClimbBuider` for more flexible API

## [0.10.1] - 2024-08-26

### Changed
* Move `PartialEq` requirement from general `Allele` to `ListGenotype` and `MultiListGenotype` specific allele requirements

### Added
* Implement `Allele` trait for tuple sizes 1 to 12, as 12 is the limit for `PartialEq` in tuples

### Removed
* Remove `Default` requirement on `Allele` for `RangeGenotype` and `MultiRangeGenotype` (no longer used)
* Remove `Zero` requirement on `Allele` for `RangeGenotype` and `MultiRangeGenotype` (no longer used)

## [0.10.0] - 2024-08-26

### Changed
* Add Copy to Allele trait, this drops support for String Alleles, but is fine for primitives, enum and structs
* Make the randomness provider internal to the API. You no longer need to provide it in the `call()` methods
* Add `with_rng_seed_from_u64()` functions to `EvolveBuilder` and `HillClimbBuider` for reproducible runs (e.g. testing)
* Align all multithreading approaches of `Evolve`, `HillClimb` & `Permutate`
  using [rayon::iter](https://docs.rs/rayon/latest/rayon/iter/index.html) and
  [std::sync::mpsc](https://doc.rust-lang.org/1.78.0/std/sync/mpsc/index.html)
* Distinguish between internal and external multithreading:
  * Internal multithreading means: parallel execution within an `Evolve`,
    `HillClimb` or `Permutate` run (mainly `Fitness` calculations)
  * External multithreading means: parallel execution of multiple independent
    `Evolve` or `HillClimb` runs.
  * Note that `Permutate` only has internal multithreading as repeated calls make no sense
  * Note that internal and external multithreading can be combined
  * Note that internal multithreading has been explored for `Compete`,
    `Crossover` and `Mutate`. But the overhead of parallel execution was too
    high, generally resulting in degradation of performance. The breakeven point was found
    only for huge populations or genes_sizes, and only where each gene was part
    of the calculation (e.g. `CrossoverUniform`). Since `Fitness` is a client
    implementation which could be very heavy depending on the domain, an explicit
    `with_par_fitness()` is used for enabling internal multithreading of the
    fitness calculation only. Adding `with_par_crossover()` and friends has been
    considered, but due to the little expected gain, separate implementations
    where possibly beneficial are added instead (e.g. `CrossoverParUniform`).
* Rename `with_multithreading` to `with_par_fitness()`, as to properly reflect it's effects only in the fitness calculations (internal multithreading)
* Require `Send + Sync` to Compete, Crossover, Extension and Mutate
* Change `chromosome_permutations_into_iter()` return type from `Box<dyn Iterator>` to `impl Iterator`
* Rename `with_multithreading()` to explicit `with_par_fitness()` for clarity of the effect

### Added
* Add `call_par_repeatedly()` and `call_par_speciated()` to `EvolveBuilder` (external multithreading)
* Add `call_par_repeatedly()` to `HillClimbBuilder` (external multithreading)
* Add short-circuit for `call_speciated()` and `call_par_speciated()` when
  target_fitness_score is reached during speciation
* Add `CountTrueWithSleep` fitness placeholder for use in multithreading examples and benchmarking
* Add `CrossoverParUniform` for a multithreaded implemenation of `CrossoverUniform`
* Add `reference_id: usize` to `Chromosome` as user controlled alternative to
  `genes_key()` for the GPU calculation use case described in [issue 5](https://github.com/basvanwesting/genetic-algorithm/issues/5)

## [0.9.0] - 2024-08-20
This is a major breaking release, see Changed:

### Changed
* Add formal `Allele` trait: `Allele: Clone + Send + Sync + PartialEq + Debug`
* Change associated type from `Genotype` to `Allele` for: `Fitness`, `EvolveReporter`, `HillClimbReporter` and `PermutateReporter`
* Change generic type `Genotype` to `Allele` for: `Chromosome`, `Population` and other structs/functions using these types
* Rename `DiscreteGenotype` to `ListGenotype` (incl. Multi)
* Rename `ContinuousGenotype` to `RangeGenotype` (incl Multi)
* Generalize `RangeGenotype` for numeric types (incl. Multi, default still f32, but other float and integer types are now supported)
* Replace `Range` with `RangeInclusive` for all ranges in `RangeGenotype` in order to handle integer ranges more intuitively (incl. Multi)
* Change `Fitness` placeholders `SumContinuousAllele` and `SumDiscreteAllele` to generalized `SumGenes` (with optional precision)
* Reimplement scaling completely now `RangeGenotype` is generalized. 
  * Drop f32 `Scaling` logic
  * Set `allele_mutation_scaled_range` on `RangeGenotype` to define scaling (incl. Multi) instead of `with_scaling()` in `HillClimb` build step
  * Mutation distance only on edges of current scale (e.g. -1 and +1 for -1..-1 scale)
  * Scale down after `max_stale_generations` is reached and reset new `stale_generations` counter to zero
  * Only trigger `max_stale_generations` ending condition when already reached the smallest scale
* How to mutate now fully controlled by `Genotype` with random, relative or scaled mutations options (relative and scaled only possible for RangeGenotype, incl. Multi)
  * Rename `MutateSingleGeneRandom` to `MutateSingleGene` as it just calls `mutate_chromosome()` on `Genotype`
  * Rename `MutateSingleGeneRandomDynamic` to `MutateSingleGeneDynamic` as it just calls `mutate_chromosome()` on `Genotype`
  * Rename `MutateMultiGeneRandom` to `MutateMultiGene` as it just calls `mutate_chromosome()` on `Genotype`
  * Rename `MutateMultiGeneRandomDynamic` to `MutateMultiGeneDynamic`as it just calls `mutate_chromosome()` on `Genotype`
  * Rename `allele_neighbour_range` to `allele_mutation_range` in `RangeGenotype` (incl. Multi) to define relative mutation
  * Add `allele_mutation_scaled_range` to `RangeGenotype` (incl. Multi) to define scaled mutation
* All changes to `RangeGenotype` are reflected in `MultiRangeGenotype` as well

### Added
* Allow relative mutations for `Evolve` as well, as it is a `Genotype` responsibility now
* Allow scaled mutations for `Evolve` as well, as it is a `Genotype` responsibility now
  * Scale down after `max_stale_generations` is reached and reset `stale_generations` to zero
  * Only trigger `max_stale_generations` ending condition when already reached the smallest scale
* Add `replace_on_equal_fitness` to builders to allow for lateral moves in search space
  * `Evolve`: defaults to false, maybe useful to avoid repeatedly seeding with the same best chromosomes after mass extinction events
  * `HillClimb`: defaults to true, crucial for some type of problems with discrete fitness steps like nqueens
  * `Permutate`: defaults to false, makes no sense to use in this strategy

### Removed
* Drop `MutateSingleGeneDistance` as random, relative or scaled mutations are now handled by `Genotype` and not the caller

## [0.8.2] - 2024-08-09
### Added
* Improve bootstrapped reporter outputs `EvolveReporterSimple`, `HillClimbReporterSimple` and `PermutateReporterSimple`
* Implement `ExtensionNoop` default for `EvolveBuilder` and remove now optional `with_extension()` steps in examples
* Implement `EvolveReporterNoop` default for `EvolveBuilder` and remove now optional `with_reporter()` steps in examples
* Implement `HillClimbReporterNoop` default for `HillClimbBuilder` and remove now optional `with_reporter()` steps in examples
* Implement `PermutateReporterNoop` default for `PermutateBuilder` and remove now optional `with_reporter()` steps in examples

### Changed
* Align `EvolveReporter`, `HillClimbReporter` and `PermutateReporter` traits to
  take `EvolveState` and `EvolveConfig` as parameters in further aligment with
  `Mutate`, `Compete`, `Crossover` and `Extension` traits.
* Add `Sync` trait everywhere where `Send` trait was required.

### Fixed
* Fix major issue where cardinality starts out as 0 as there are no fitness
  calculations yet. This triggers the optional extension event, if set, at the
  start of the evolve loop (killing seed population and diversity). Issue was
  introduced in v0.8.0 with `fitness_score_cardinality()`. Solve by adding None
  fitness counts to cardinality.

## [0.8.1] - 2024-08-08
### Added
* Always implement `new()` next to `default()`. Use `new()` in public API examples

### Changed
* Rename `new()` to `new_with_flags()` for more verbose reporting in `EvolveReporterSimple`, `HillClimbReporterSimple` and `PermutateReporterSimple`
* Add simpler `new()` to only take `period: usize` and set all flags to false (as this is the sensible less noisy default) in `EvolveReporterSimple`, `HillClimbReporterSimple` and `PermutateReporterSimple`

## [0.8.0] - 2024-08-07
### Added
* Add `PermutateConfig` and `PermutateState` to align structure with `Evolve` and `HillClimb`
* Extract `StrategyConfig` trait and use for `EvolveConfig`, `HillClimbConfig` and `PermutateConfig`
* Extract `StrategyState` trait and use for `EvolveState`, `HillClimbState` and `PermutateState`
* Add pluggable `EvolveReporter` to `Evolve` strategy
    * Set in builder using `with_reporter()`
    * Custom implementations by client are encouraged, the API resembles the Fitness API
    * Add bootstrap implementations `EvolveReporterNoop`, `EvolveReporterSimple` and `EvolveReporterLog`
* Add pluggable `HillClimbReporter` to `HillClimb` strategy
    * Set in builder using `with_reporter()`
    * Custom implementations by client are encouraged, the API resembles the Fitness API
    * Add bootstrap implementations `HillClimbReporterNoop`, `HillClimbReporterSimple` and `HillClimbReporterLog`
* Add pluggable `PermutateReporter` to `Permutate` strategy
    * Set in builder using `with_reporter()`
    * Custom implementations by client are encouraged, the API resembles the Fitness API
    * Add bootstrap implementations `PermutateReporterNoop`, `PermutateReporterSimple` and `PermutateReporterLog`
* Add `fitness_score_cardinality()` to `Population`
* Add `MutateMultiGeneRandomDynamic` (generalize to any number of mutations)
* Add `MutateSingleGeneDistance` (only for `ContinuousGenotype`)

### Removed
* Drop `fitness_score_uniformity()` and `fitness_score_prevalence()` from `Population`
* Drop `MutateDynamicRounds`

### Changed
* Align `Mutate`, `Compete`, `Crossover` and `Extension` traits to take `EvolveState`, `EvolveConfig`, `EvolveReporter` as parameters
* Reimplement `MutateOnce` as `MutateSingleGeneRandom`
* Reimplement `MutateTwice` as `MutateMultiGeneRandom` (generalize to any number of mutations)
* Reimplement `MutateDynamicOnce` as `MutateSingleGeneRandomDynamic` (also fix InvalidProbabilty issue)
* Replace `target_uniformity` with `target_cardinality` in `MutateSingleGeneRandomDynamic` and `MutateMultiGeneRandomDynamic` as uniformity is ill defined
* Replace `uniformity_threshold` with `cardinality_threshold` in `Extension` implementations, as uniformity is ill defined
* Move permutation `total_population_size` from `PermutateConfig` to `PermutateState`, so progress can be reported on in `PermutateReporterSimple`
* Move `env_logger` dependency to dev-dependencies as this crate is a library, not an executable

### Note
* Note that `HillClimb` scaling needs review as it doesn't feel right in its design approach. Possibly align with `MutateSingleGeneDistance` approach?
* Extract `StrategyReporter` trait, but don't use because of error E0658: associated type defaults are unstable. So for `EvolveReporter`, `HillClimbReporter` and `PermutateReporter` the trait is shadowed as if it is implemented

## [0.7.2] - 2024-07-27
### Added
* Add `Wrapper`s instead of `Dispatcher`s as they keep state, behaviour is the same using `into()` (e.g. `MutateOnce::new(0.2).into()`)

### Removed
* Extract Meta logic to separate crate [genetic_algorithm_meta](https://docs.rs/genetic_algorithm_meta/latest/genetic_algorithm_meta)
* Phase out the `Dispatcher`s as they are replaced by `Wrapper`s

## [0.7.1] - 2024-07-23
### Changed
* MSRV bumped to 1.71.1
* Solve [RUSTSEC-2021-0145](https://rustsec.org/advisories/RUSTSEC-2021-0145)

## [0.7.0] - 2023-05-25
### Added
* Add `Mutate` implementations:
  * `MutateTwice`, support some form of swap-like behaviour where `UniqueGenotype` doesn't match with the problem space
  * `MutateDynamicOnce`, increase mutation probability when population uniformity is above threshold and vice versa
  * `MutateDynamicRounds`, increase mutation rounds when population uniformity is above threshold and vice versa
* Add `HillClimbVariant::StochasticSecondary` and `HillClimbVariant::SteepestAscentSecondary` as well for the same reasons as `MutateTwice`
* Add `call_speciated` next to the existing `call_repeatedly` in `EvolveBuilder`. This runs multiple independent
  evolve strategies and then competes their best chromosomes as starting population against each other in one final evolve strategy
* Add `Chromosome` age and optional `with_max_chromosome_age` to `EvolveBuilder`. Filtering chromosomes past the maximum age from the next generation
* Add `best_generation()` and `best_fitness_score()` to `Strategy`, so client implementation can report and switch more easily over different strategies.
  Return zero for `Permutate::best_generation()` as there is no concept of best generation there
* Add `Extension` step to `Evolve`, adding `with_extension` to `EvolveBuilder`, with several implementations:
  * `ExtensionNoop`, for no extension
  * `ExtensionMassExtinction`, trigger mass extinction to allow for cambrian explosion (no competition for a while, which allows for more diversity)
  * `ExtensionMassGenesis`, like `ExtensionMassExtinction`, but only a pair of best chromosomes (adam and eve) are taken as the start for the next generations
  * `ExtensionMassInvasion`, like `ExtensionMassExtinction`, but replace extinct population with random population (respecting seed_genes if present)
  * `ExtensionMassDegeneration`, simulate cambrian explosion by apply several rounds of uncontrolled mutation directly
* Add `Population::fitness_score_unformity()` as measure for uniformity (fraction between 0 and 1). Use as triggers for `MutateDynamic*` and `Extension`
* Add dispatch `From` to `Evolve` plugins for use in `MetaConfigBuilder`, instead of manual wrapping (e.g. `MutateOnce::new(0.2).into()` instead of `MutateDispatch(MutateOnce::new(0.2)`)

### Changed
* Refactor `Compete`, `Crossover` and `Mutate` from tuple structs to struct, initialize with `::new()`, because the structs now have some mutable internal properties (e.g. `MutateDynamicOnce`). Make all plugins mutable for consistency
* Split off internal config and state structs for `Evolve` and `HillClimb`, leave `Permutate` untouched weighing overkill v. symmetry different there
* Split off internal plugins for `Evolve` (i.e. `Mutate`/`Crossover`/`Compete`/`Extension`)
* Change `seed_genes` to `seed_genes_list` to allow for multiple seed genes taken randomly (used in `call_speciated`)
* Only mutate children in the `Mutate` step, in earlier versions parents and children were mutated equally
* Refactor `Evolve` `population_size` property to `target_population_size`, thus also replacing `with_population_size` with `with_target_population_size`
* Add `env_logger::init()` to all examples, so the `RUST_LOG` environment variable works as expected
* Change `HillClimbBuilder::with_scaling` parameter from tuple to struct `Scaling`

### Removed
* Phase out the `with_mass_degeneration` in `EvolveBuilder` as it is replaced by `ExtensionMassDegeneration`

## [0.6.0] - 2022-10-14
### Changed
* Calculate initial chromosome fitness in `HillClimb` to lock in on original seed if present

### Removed
* Remove `random_chromosome_probability` to `HillClimb` as it was hackish

## [0.5.4] - 2022-10-14
### Added
* Add `valid_fitness_score` to block ending conditions until met for `Evolve` and `HillClimb` strategies

## [0.5.3] - 2022-10-14
### Changed
* Tweak TRACE logging

## [0.5.2] - 2022-10-14
### Added
* Add env_logger and some INFO/DEBUG/TRACE logging

### Changed
* Count generation zero based

## [0.5.1] - 2022-09-10
### Fixed
* Solve lock-in to single best chromosome in stale `HillClimbVariant::SteepestAscent` by shuffling chromosomes before taking best

## [0.5.0] - 2022-07-07
### Added
* Add `IncrementalGenotype` Trait with neighbouring chromosome implementations
* Implement `IncrementalGenotype` for all `Genotype`s
* Add `allele_neighbour_range` to `ContinuousGenotype`
* Add `allele_neighbour_ranges` for `MultiContinuousGenotype`
* Add `HillClimbVariant::Stochastic` and `HillClimbVariant::SteepestAscent`
* Add `HillClimb` scaling (for `ContinuousGenotype` & `MultiContinuousGenotype`) to scale down neighbours on each round and use as ending condition
* Add `random_chromosome_probability` to `HillClimb` to avoid local optima
* Add multithreading to `Permutate` (parallel processing of chromosome generator)
* Add multithreading to `Evolve` (fitness execution for population)
* Add multithreading to `HillClimb` (fitness execution for `HillClimbVariant::SteepestAscent` population only)
* Add `call_repeatedly` for `EvolveBuilder` and `HillClimbBuilder`
* Add examples/evolve_milp.rs
* Add examples/evolve_scrabble.rs
* Add examples/hill_climb_scrabble.rs
* Add examples/hill_climb_milp.rs
* Add examples/permutate_scrabble.rs

### Changed
* Require `IncrementalGenotype` for `HillClimb` strategy
* Refactor `allele_values` to `allele_list`
* Refactor `allele_multi_values` to `allele_lists`
* Refactor `allele_multi_range` to `allele_ranges`
* Add median/mean/stddev to `report_round` in `Evolve` and `HillClimb`
* Add precision to `SumContinuousGenotype` and `SumMultiContinuousGenotype` placeholders for better handling of decimal changes on cast to isize

## [0.4.1] - 2022-06-14
### Documentation
* Use SPDX license in Cargo.toml as the existing LICENSE file (MIT) was marked as non-standard by crates.io
* Add Apache 2.0 license

## [0.4.0] - 2022-06-14
### Documentation
* Note degeneration_range use as case by case configuration

### Added
* Add `Strategy` trait and implement for `Evolve` and `Permutate`
* Add `HillClimb` strategy for when crossover is impossible or inefficient
* Add `MultiUniqueGenotype`
* Add table_seating example (hill_climb and evolve)

### Changed
* Move `Evolve` & `Permutate` to `strategy` module
* Remove `Genotype::is_unique` and `Crossover::allow_unique_genotype` methods
  * Replace with `Genotype::crossover_indexes` and `Crossover::require_crossover_indexes`
  * Replace with `Genotype::crossover_points` and `Crossover::require_crossover_points`
* Rename `UniqueDiscreteGenotype` to `UniqueGenotype` as it is discrete by definition
* Rename `PermutableGenotype::allele_values` to `PermutableGenotype::allele_values_for_chromosome_permutations` for clarity of purpose
* Hide `Evolve` and `Permutate` internal fields (to align with `Strategy` trait)

## [0.3.1] - 2022-05-16
### Fixed
* forgot to update version in `Cargo.toml`

## [0.3.0] - 2022-05-16
### Documentation
* Make proper distinction between gene and allele as in the book "Genetic Algorithms in Elixir"

### Added
* Add option to `call()` from `EvolveBuilder` & `PermutateBuilder` directly
* Add `Fitness::Zero` placeholder

### Changed
* Refactor `Evolve` & `Permutate` to `call(&mut self, ...)`
* Refactor `Fitness`, `Crossover`, `Mutate` & `Compete` to take mutable population reference
* Improve performance in `Crossover` when not keeping parents
* Rename `gene_value*` to `allele_value*`
* Rename `gene_ranges` to `allele_multi_range` for symmetry reasons with `allele_multi_values`
* Rename `gene_size` to `genes_size` as it is not the size of a gene
* Rename `CrossoverSingle` to `CrossoverSingleGene`
* Rename `CrossoverRange` to `CrossoverSinglePoint`
* Rename `CrossoverAll` to `CrossoverUniform`

### Removed
* Drop SetGenotype as it is always better implemented using BinaryGenotype
* Cleanup examples

## [0.2.0] - 2022-05-13
### Added
* Add `SetGenotype<T>`, with `examples/evolve_knapsack_set.rs` and `examples/permutate_knapsack_set.rs`

### Changed
* Refactor fitness placeholders to placeholders module to emphasize that production use is not intended
* Rename `Fitness::call_for_chromosome()` to `Fitness::calculate_for_chromosome()`
* Replaced `PermutableGenotype::population_factory()` with `PermutableGenotype::chromosome_permutations_into_iter()`
* Use `PermutableGenotype::chromosome_permutations_into_iter()` in `Permutate::call()` instead of fully instantiated population
* Rename `PermutableGenotype::population_factory_size()` to `PermutableGenotype::chromosome_permutations_size()`
* Use `num::BigUint` for `PermutableGenotype::chromosome_permutations_size()` as it overflows easily
* Rename existing `examples/evove_knapsack_set.rs` to `examples/evolve_knapsack_discrete.rs` to note is uses `DiscreteGenotype<T>`

### Documentation
- Improve rustdocs, refer to docs.rs documentation from general README.md

## [0.1.1] - 2022-05-11
### Documentation
- Added rustdocs, refer to crate.io documentation from general README.md

## [0.1.0] - 2022-05-10
Initial version
