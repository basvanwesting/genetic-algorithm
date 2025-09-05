# Refactoring Plan v5 - Phase 1: Fork Everything (No Backward Compatibility)

## Goal
Create two complete, independent copies of the codebase in `distributed/` and `centralized/` modules. Break backward compatibility intentionally - users must choose a paradigm.

## Current State Analysis

### Codebase Statistics
- **Total lines**: ~14,488
- **Root modules**: 12 files
- **Subdirectories**: 11 directories
- **Total files**: ~65 .rs files

### Directory Structure
```
src/
├── chromosome/        (3 files: bit.rs, row.rs, vector.rs)
├── crossover/        (8 files)
├── extension/        (6 files)
├── fitness/          (3 files)
├── genotype/         (11 files)
├── mutate/           (6 files)
├── select/           (3 files)
├── strategy/         (4 files + 3 subdirs)
│   ├── evolve/       (3 files)
│   ├── hill_climb/   (3 files)
│   └── permutate/    (3 files)
├── allele.rs
├── chromosome.rs
├── crossover.rs
├── errors.rs
├── extension.rs
├── fitness.rs
├── genotype.rs
├── lib.rs
├── mutate.rs
├── population.rs
├── select.rs
└── strategy.rs
```

## Step-by-Step Execution Plan

### Step 1: Create Module Directories (2 minutes)

```bash
# Create the two main module directories
mkdir -p src/distributed
mkdir -p src/centralized

# Verify
ls -la src/
# Should show: distributed/, centralized/, and all original files
```

### Step 2: Fork Entire Codebase (5 minutes)

```bash
# Copy everything except lib.rs to distributed
rsync -av --exclude='lib.rs' src/ src/distributed/

# Copy everything except lib.rs to centralized  
rsync -av --exclude='lib.rs' src/ src/centralized/

# Clean up nested copies
rm -rf src/distributed/distributed
rm -rf src/distributed/centralized
rm -rf src/centralized/distributed
rm -rf src/centralized/centralized

# Verify structure
find src/distributed -type f -name "*.rs" | wc -l  # Should be ~65 files
find src/centralized -type f -name "*.rs" | wc -l  # Should be ~65 files
```

### Step 3: Create Module Root Files (10 minutes)

#### Create `src/distributed/mod.rs`:
```rust
//! Distributed genetic algorithms where each chromosome owns its genes
//! 
//! Use this module for:
//! - Binary, List, Unique, and Range genotypes
//! - Custom genetic operators with direct gene access
//! - Maximum extensibility

pub mod allele;
pub mod chromosome;
pub mod crossover;
pub mod errors;
pub mod extension;
pub mod fitness;
pub mod genotype;
pub mod mutate;
pub mod population;
pub mod select;
pub mod strategy;

// Module prelude - users should use this
pub mod prelude {
    pub use crate::distributed::{
        allele::*,
        chromosome::*,
        crossover::*,
        extension::*,
        fitness::*,
        genotype::*,
        mutate::*,
        population::*,
        select::*,
        strategy::*,
        strategy::evolve::prelude::*,
    };
}
```

#### Create `src/centralized/mod.rs`:
```rust
//! Centralized genetic algorithms with population-wide gene storage
//! 
//! Use this module for:
//! - DynamicMatrix and StaticMatrix genotypes
//! - GPU/SIMD-ready operations
//! - Maximum performance with large populations

pub mod allele;
pub mod chromosome;
pub mod crossover;
pub mod errors;
pub mod extension;
pub mod fitness;
pub mod genotype;
pub mod mutate;
pub mod population;
pub mod select;
pub mod strategy;

// Module prelude - users should use this
pub mod prelude {
    pub use crate::centralized::{
        allele::*,
        chromosome::*,
        crossover::*,
        extension::*,
        fitness::*,
        genotype::*,
        mutate::*,
        population::*,
        select::*,
        strategy::*,
        strategy::evolve::prelude::*,
    };
}
```

### Step 4: Create Minimal lib.rs (NO backward compatibility)

Replace entire `src/lib.rs` with:

```rust
//! Genetic Algorithm Library - v0.20.5-cd.0 - This is a major breaking change, but use pre-release tags for now, to keep track of base version
//! 
//! This library provides two paradigms for genetic algorithms:
//! 
//! # Distributed
//! Each chromosome owns its genes. Use for maximum extensibility.
//! ```ignore
//! use genetic_algorithm::distributed::prelude::*;
//! ```
//! 
//! # Centralized  
//! Population-wide gene storage. Use for maximum performance.
//! ```ignore
//! use genetic_algorithm::centralized::prelude::*;
//! ```
//! 
//! Choose ONE paradigm for your application.

pub mod distributed;
pub mod centralized;

// NO re-exports at crate root - force explicit paradigm choice
// Users MUST choose: distributed::prelude::* OR centralized::prelude::*
```

### Step 5: Fix Internal Imports (30 minutes)

#### Script to update distributed module:
```bash
# Update all crate:: references to crate::distributed:: in distributed module
find src/distributed -name "*.rs" -type f -exec sed -i.bak \
  -e 's/use crate::/use crate::distributed::/g' \
  -e 's/pub(crate)/pub/g' \
  {} \;

# Fix any absolute paths that don't start with crate::distributed
find src/distributed -name "*.rs" -type f -exec sed -i.bak2 \
  -e 's/\bcrate::\([a-z]\)/crate::distributed::\1/g' \
  {} \;

# Special case: fix double distributed references
find src/distributed -name "*.rs" -type f -exec sed -i.bak3 \
  -e 's/crate::distributed::distributed::/crate::distributed::/g' \
  {} \;

# Clean up backup files
find src/distributed -name "*.bak*" -delete
```

#### Script to update centralized module:
```bash
# Update all crate:: references to crate::centralized:: in centralized module
find src/centralized -name "*.rs" -type f -exec sed -i.bak \
  -e 's/use crate::/use crate::centralized::/g' \
  -e 's/pub(crate)/pub/g' \
  {} \;

# Fix any absolute paths that don't start with crate::centralized
find src/centralized -name "*.rs" -type f -exec sed -i.bak2 \
  -e 's/\bcrate::\([a-z]\)/crate::centralized::\1/g' \
  {} \;

# Special case: fix double centralized references
find src/centralized -name "*.rs" -type f -exec sed -i.bak3 \
  -e 's/crate::centralized::centralized::/crate::centralized::/g' \
  {} \;

# Clean up backup files
find src/centralized -name "*.bak*" -delete
```

### Step 6: Delete Original Source Files (5 minutes)

**This is the breaking change - no going back!**

```bash
# Delete all original files (keep only distributed/ and centralized/)
find src -maxdepth 1 -name "*.rs" ! -name "lib.rs" -delete
rm -rf src/chromosome
rm -rf src/crossover
rm -rf src/extension
rm -rf src/fitness
rm -rf src/genotype
rm -rf src/mutate
rm -rf src/select
rm -rf src/strategy

# Verify only lib.rs and two modules remain
ls -la src/
# Should only show: lib.rs, distributed/, centralized/
```

### Step 7: Update ALL Examples (30 minutes)

Every example must be duplicated and updated:

```bash
# Create distributed and centralized example directories
mkdir -p examples/distributed
mkdir -p examples/centralized

# Copy relevant examples to distributed
cp examples/evolve_binary.rs examples/distributed/
cp examples/evolve_nqueens.rs examples/distributed/
cp examples/evolve_monkeys.rs examples/distributed/
cp examples/permutate_knapsack.rs examples/distributed/
# ... etc for non-matrix examples

# Copy relevant examples to centralized
cp examples/evolve_binary.rs examples/centralized/evolve_matrix_binary.rs
# Will need modification for matrix representation

# Delete original examples
rm examples/*.rs
```

Update distributed examples:
```rust
// examples/distributed/evolve_binary.rs
use genetic_algorithm::distributed::prelude::*;  // Changed!

// Rest of code stays the same
```

Update centralized examples (more work - changing to matrix):
```rust
// examples/centralized/evolve_matrix_binary.rs
use genetic_algorithm::centralized::prelude::*;  // Changed!

fn main() {
    // Need to rewrite using DynamicMatrix<u8> instead of BinaryGenotype
    let genotype = DynamicMatrixGenotype::<u8>::builder()
        .with_genes_size(100)
        .with_allele_range(0..=1)  // 0 or 1 instead of bool
        .build()
        .unwrap();
    
    // Fitness needs to work with matrix
    // ...
}
```

### Step 8: Update ALL Tests (45 minutes)

Tests must be split and updated:

```bash
# Create test directories
mkdir -p tests/distributed
mkdir -p tests/centralized

# Move existing tests
mv tests/*_test.rs tests/distributed/  # Most current tests
```

Update test imports:
```rust
// tests/distributed/binary_test.rs
use genetic_algorithm::distributed::prelude::*;

#[test]
fn test_binary_genotype() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(10)
        .build()
        .unwrap();
    // Test continues...
}
```

Create new centralized tests:
```rust
// tests/centralized/matrix_test.rs
use genetic_algorithm::centralized::prelude::*;

#[test]
fn test_dynamic_matrix_genotype() {
    let genotype = DynamicMatrixGenotype::<f32>::builder()
        .with_genes_size(10)
        .with_allele_range(0.0..=1.0)
        .build()
        .unwrap();
    // Test continues...
}
```

### Step 9: Update Cargo.toml (5 minutes)

```toml
[package]
name = "genetic_algorithm"
version = "0.20.5-cd.0"
edition = "2021"
# ... rest stays same

[[example]]
name = "distributed_evolve_binary"
path = "examples/distributed/evolve_binary.rs"

[[example]]
name = "distributed_evolve_nqueens"
path = "examples/distributed/evolve_nqueens.rs"

[[example]]
name = "centralized_evolve_matrix"
path = "examples/centralized/evolve_matrix.rs"

# ... etc for all examples

[[test]]
name = "distributed_tests"
path = "tests/distributed/main.rs"

[[test]]
name = "centralized_tests"
path = "tests/centralized/main.rs"
```

### Step 10: Verify Everything (15 minutes)

```bash
# This WILL fail initially - that's expected!
cargo build

# Fix compilation errors (mostly imports)
# Common fixes:
# - Remove any remaining crate-level re-exports
# - Fix test/example imports
# - Fix macro paths

# Once it compiles:
cargo test distributed::
cargo test centralized::

# Run examples
cargo run --example distributed_evolve_binary
cargo run --example centralized_evolve_matrix
```

### Step 11: Create Migration Guide (10 minutes)

Create `MIGRATION.md`:

```markdown
# Migration Guide: v0.20.5 to v0.20.5-cd.0

## Breaking Changes

The library now requires choosing between two paradigms:
- `distributed`: For Binary, List, Unique, Range genotypes
- `centralized`: For DynamicMatrix, StaticMatrix genotypes

## Migration Steps

### Old Code
```rust
use genetic_algorithm::strategy::evolve::prelude::*;

let genotype = BinaryGenotype::builder()
    .with_genes_size(100)
    .build()
    .unwrap();
```

### New Code - Distributed
```rust
use genetic_algorithm::distributed::prelude::*;

let genotype = BinaryGenotype::builder()
    .with_genes_size(100)
    .build()
    .unwrap();
```

### New Code - Centralized
```rust
use genetic_algorithm::centralized::prelude::*;

let genotype = DynamicMatrixGenotype::<u8>::builder()
    .with_genes_size(100)
    .with_allele_range(0..=1)
    .build()
    .unwrap();
```

## Which Paradigm to Choose?

**Use Distributed when:**
- You need custom genetic operators
- Working with non-numeric genes
- Need maximum flexibility

**Use Centralized when:**
- You need maximum performance
- Working with large populations
- Want GPU acceleration (future)
```

## Success Criteria

✅ **Phase 1 is complete when:**

1. ✓ Two complete copies exist in `src/distributed/` and `src/centralized/`
2. ✓ Original source files are DELETED (no backward compatibility)
3. ✓ lib.rs provides NO re-exports at crate root
4. ✓ All examples are split into distributed/ and centralized/
5. ✓ All tests are split and updated
6. ✓ `cargo build` succeeds
7. ✓ `cargo test` passes for both paradigms
8. ✓ Examples run for both paradigms
9. ✓ MIGRATION.md exists

## Time Estimate

- **Total time**: ~2.5 hours (more than before due to no compatibility)
- **Automated steps**: 45 minutes
- **Manual fixes**: 1.5 hours (updating examples and tests)
- **Verification**: 15 minutes

## Next Phase Preview

Phase 2 will:
- Remove matrix genotypes from `distributed/`
- Remove vector genotypes from `centralized/`
- Simplify traits in each module
- Each module becomes focused on its paradigm

## Important Notes

- **This breaks everything** - All existing user code will break
- **No gradual migration** - Users must update immediately
- **Version 0.20.5-cd.0** - This is a major breaking change, but use pre-release tags for now, to keep track of base version
- **No compatibility layer** - Clean break, clean start
- **Examples and tests are critical** - They prove the new structure works
