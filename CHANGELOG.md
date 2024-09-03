# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


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
  * Rename `allele_neighbour_range` to `allele_mutation_range` in `RangeGenoype` (incl. Multi) to define relative mutation
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
