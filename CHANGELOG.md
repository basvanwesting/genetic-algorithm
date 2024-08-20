# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.9.0] - 2024-08-19
### Added
### Changed

* (HEAD -> feature/generic_continuous_genotype, origin/feature/generic_continuous_genotype) Rename MutateSingleGeneRandom to MutateSingleGene Rename MutateSingleGeneRandomDynamic to MutateSingleGeneDynamic Rename MutateMult iGeneRandom to MutateMultiGene Rename MutateMultiGeneRandomDynamic to MutateMultiGeneDynamic
* Remove MutateSingleGeneNeighbour as random v. neighbour is no longer decided by caller
* Move decision between random, neighbour-scaled and neighbour-unscaled mutation from caller to genotype internal implementation. Allow for fallback to complete allele_range when allele_neighbour_(scaled_)range not set
* Revert "refactor reset and increment stale generations to strategy"
* refactor reset and increment stale generations to strategy, but don't like the result, reverting in next commit
* Add replace_on_equal_fitness to HillClimb, default to true as the best chromosome is take as base for next step, which sometimes crucial Add replace_on_equal_fitness to Evolve, default to false, since the best chromosome is less important, but useful for mass extinction events Add replace_on_equal_fitness to Permutate, default to false, since it makes no sense at all
* Add scaling to Evolve as well (just like HillClimb)
* Refactor DiscreteGenotype to ListGenotype Refactor MultiDiscreteGenotype to MultiListGenotype Refactor ContinuousGenotype to RangeGenotype Refactor MultiContinuousGenotype to MultiRangeGenotype
* Add scaling documentation
* remove all unneeded precision in tests
* Add increment_stale_generations on no valid fitness for HillClimb and Evolve Implement max_scale_index for MultiContinuousGenotype
* Align MultiContinuousGenotype scaling implementation with ContinuousGenotype
* Generalize fitness summing placeholders into SumGenes with optional precision
* remove unused allele_neighbour_scaled_sampler remove unused TryFromGenotypeBuilderError in evolve test
* Align all StrategyState implementations and add shadowed StrategyReporter trait function update_best_chromosome_and_report Use stale_generations for is_finished_by_max_stale_generations for Evolve as well
* Implement new scaling implementation in HillClimb Use separate stale_generations to keep track of staleness per scale
* Ensure range.sample(rng) for unscaled neighbour always has a smalller and larger neighbour for the neighbouring_population set
* Add rng to Genotype::neighbouring_population Always use range start/end for scaled neighour (incl. single mutation) Always use range.sample(rng) for unscaled neighbour (incl. population)
* Add show_equal_fitness flag to HillClimbReporterSimple as example/hill_climb_continuous sometimes doesn't resolve
* Remove legacy f32 scaling logic in HillClimb
* Hookup scale_index in HillClimb process
* Add with_allele_neighbour_scaled_range for ContinuousGenotype with index addressing from caller
* Change example/hill_climb_continuous and example/evolve_continuous_float to optimize the distance-to 0.5 for each gene, instead of max (which can't overshoot) Use this distance-to to prove HillClimbVariant SteepestAscent needs scaling, otherwise it can't find a target fitness
* Replace MultiContinuousGenotype with static f32 implementation with generic implementation
* Add multi_continuous_t implementation next to multi_continuous_f32
* Replace ContinuousGenotype with static f32 implementation with generic implementation #3
* Change SumContinuousAllele to SumF32 Change SumDiscreteAllele to SumUsize and SumIsize
* Replace Range with RangeInclusive for allele ranges in order to handle integer ranges more intuitively
* Enable continuous_t module instead of fixed continuous_f32 module Issue with exclusive range end on integer range
* Revert "Remove Associated trait Allele from StrategyReporter, use simple generic functions" The client API will use only one specific implementation of Allele for Fitness and Reporting. So an associated type fits the API best
* Remove Associated trait Allele from StrategyReporter, use simple generic functions
* Change StrategyReporter to take Allele instead of Genotype as well
* cargo tests and bench green
* cargo build runs
* Change Chromosome, Population and Fitness to take Allele instead of Genotype, RED EOD
* Add formal Allele trait
* Explicit type list doesn't help
* Added first attempt at ContinuousGenotype<T>, but having Send + Sync issues with SampleUniform
* (origin/main, main) Rename MutateSingleGeneDistance to MutateSingleGeneNeighbour
* Reimplement MutateSingleGeneDistance to use mutate_chromosome_neighbour where the genotype already defines the distance range through allele_neighbour_range Move mutate_chromosome_neighbour from the IncrementalGenotype tr ait to the Genotype trait and provide blanked fallback to mutate_chromosome_random Only truly implement mutate_chromosome_neighbour for ContinuousGenotype and MultiContinuousGenotype (as is)


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
