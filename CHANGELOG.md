# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
