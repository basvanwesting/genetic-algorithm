# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Common Development Commands

```bash
# Build the project
cargo build
cargo build --release

# Run all tests
cargo test

# Run a specific test
cargo test test_name

# Run tests with verbose output
cargo test -- --nocapture

# Run examples (in release mode for performance)
cargo run --example evolve_nqueens --release
cargo run --example hill_climb_nqueens --release
cargo run --example permutate_knapsack --release

# Run benchmarks
cargo bench
cargo bench bench_name

# Generate documentation
cargo doc --open

# Check code without building
cargo check

# Format code
cargo fmt

# Lint code
cargo clippy
```

## High-Level Architecture

This is a Rust genetic algorithm library with three core abstractions:

### 1. **Genotype** (Search Space)
Located in `src/genotype/`, defines the representation of solutions:
- `BinaryGenotype`: Boolean alleles (Vec<bool>)
- `BitGenotype`: Bit-packed boolean alleles (more memory efficient)
- `ListGenotype<T>`: List of values from a fixed set of alleles
- `UniqueGenotype<T>`: Permutation of unique values
- `RangeGenotype<T>`: Numeric values within ranges
- `DynamicRangeGenotype<T>` / `StaticRangeGenotype<T>`: Matrix representations for GPU-friendly operations

### 2. **Fitness** (Search Goal)
Located in `src/fitness/`, evaluates chromosome quality:
- Trait `Fitness` with method `calculate_for_chromosome()`
- Returns `FitnessValue` (f64) to be maximized or minimized
- Can be parallelized with `with_par_fitness()`
- Cache wrapper available for expensive fitness calculations

### 3. **Strategy** (Search Method)
Located in `src/strategy/`, implements the search algorithm:

#### **Evolve** (`src/strategy/evolve/`)
Classical genetic algorithm with:
- **Select**: Tournament or Elite selection to choose parents
- **Crossover**: Uniform, SinglePoint, MultiPoint, etc. to combine parents
- **Mutate**: SingleGene, MultiGene with configurable rates
- **Extensions**: MassExtinction, MassGenesis, MassDegeneration for diversity management

#### **HillClimb** (`src/strategy/hill_climb/`)
Local search when crossover is inefficient:
- Variants: Stochastic, SteepestAscent
- Works with neighbor generation instead of crossover

#### **Permutate** (`src/strategy/permutate/`)
Exhaustive search for small spaces with 100% guarantee

## Key Design Patterns

1. **Builder Pattern**: All major components use builders for configuration
2. **Trait-based Architecture**: Core behaviors defined as traits (Genotype, Fitness, Select, Crossover, Mutate)
3. **Parallelization**: Built on rayon for automatic parallelization
4. **Reporter Pattern**: Strategies accept reporters for monitoring progress

## Testing Approach

- Integration tests in `tests/` directory organized by module
- Use `.with_rng_seed_from_u64(0)` for deterministic test results
- Benchmarks in `benches/` using criterion
- Examples serve as both documentation and integration tests

## Dual Module System

The library has two parallel module hierarchies:

### **Centralized** (`src/centralized/`)
Standard single-machine implementation where chromosomes own their genes directly

### **Distributed** (`src/distributed/`)
For distributed/parallel computing where chromosomes reference genes stored separately (e.g., in matrices for GPU operations)

Both systems share the same API surface but differ in internal implementation details.

## Performance Considerations

- Prefer `BitGenotype` over `BinaryGenotype` for large boolean chromosomes
- Use `with_par_fitness()` for expensive fitness calculations
- Matrix genotypes enable GPU-friendly memory layout
- Monitor with reporters to identify bottlenecks (fitness vs framework overhead)