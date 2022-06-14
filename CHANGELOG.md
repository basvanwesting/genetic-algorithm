# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
