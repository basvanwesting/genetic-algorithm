# Refactoring Plan v5 - Phase 2a & 3a: Parallel Trimming (CORRECTED)

## Core Principle: Symmetric Trimming

**If removed from distributed → Must stay in centralized**  
**If removed from centralized → Must stay in distributed**

This ensures both modules remain complete but focused on their paradigm.

## File-Level Trimming

### Files to DELETE from distributed/
```
distributed/genotype/dynamic_matrix.rs     ✓ Keep in centralized
distributed/genotype/static_matrix.rs      ✓ Keep in centralized  
distributed/chromosome/row.rs              ✓ Keep in centralized
```

### Files to DELETE from centralized/
```
centralized/genotype/binary.rs             ✓ Keep in distributed
centralized/genotype/bit.rs                ✓ Keep in distributed
centralized/genotype/list.rs               ✓ Keep in distributed
centralized/genotype/unique.rs             ✓ Keep in distributed
centralized/genotype/range.rs              ✓ Keep in distributed
centralized/genotype/multi_list.rs         ✓ Keep in distributed
centralized/genotype/multi_unique.rs       ✓ Keep in distributed
centralized/genotype/multi_range.rs        ✓ Keep in distributed
centralized/chromosome/vector.rs           ✓ Keep in distributed
centralized/chromosome/bit.rs              ✓ Keep in distributed
```

## Code-Level Trimming

### 1. distributed/genotype.rs

#### Remove module declarations and re-exports:
```rust
// Remove these lines:
- mod dynamic_matrix;
- mod static_matrix;
- pub use self::dynamic_matrix::DynamicMatrix as DynamicMatrixGenotype;
- pub use self::static_matrix::StaticMatrix as StaticMatrixGenotype;
```

### 2. centralized/genotype.rs

#### Remove module declarations and re-exports:
```rust
// Remove these lines:
- mod binary;
- mod bit;
- mod list;
- mod unique;
- mod range;
- mod multi_list;
- mod multi_unique;
- mod multi_range;
- pub use self::binary::Binary as BinaryGenotype;
- pub use self::bit::Bit as BitGenotype;
- pub use self::list::List as ListGenotype;
- pub use self::unique::Unique as UniqueGenotype;
- pub use self::range::Range as RangeGenotype;
- pub use self::multi_list::MultiList as MultiListGenotype;
- pub use self::multi_unique::MultiUnique as MultiUniqueGenotype;
- pub use self::multi_range::MultiRange as MultiRangeGenotype;
```

### 3. distributed/chromosome.rs

#### Remove module declarations and re-exports:
```rust
// Remove these lines:
- mod row;
- pub use self::row::Row as RowChromosome;
```

### 4. centralized/chromosome.rs

#### Remove module declarations and re-exports:
```rust
// Remove these lines:
- mod vector;
- mod bit;
- pub use self::vector::Vector as VectorChromosome;
- pub use self::bit::Bit as BitChromosome;
```

### 5. Update References in Tests and Examples

After deleting the files and removing module declarations, you'll need to:
1. Remove or update any tests that use the deleted genotypes/chromosomes
2. Remove or update any examples that use the deleted genotypes/chromosomes
3. Update any benchmarks that reference deleted types

## Additional Trimming Opportunities

### Trait methods that might be paradigm-specific:

#### In Genotype trait:
- `update_population_fitness_scores()` - This method panics for non-GenesPointer chromosomes, suggesting it's matrix-specific and could potentially be removed from distributed

However, since the operators (mutate, crossover, select) are generic and work with the Genotype trait, most trait methods need to stay to maintain compatibility.

## What NOT to Trim

**These stay in BOTH modules because they're generic:**
- All mutate operators (SingleGene, MultiGene, etc.)
- All crossover operators (Uniform, SinglePoint, MultiPoint, etc.)
- All select operators (Tournament, Elite, etc.)
- All strategies (Evolve, HillClimb, Permutate)
- Extension operators (MassExtinction, MassGenesis, etc.)
- Core traits (Mutate, Crossover, Select, Fitness)

The operators work with generic type parameters like `G: EvolveGenotype`, so they adapt to whatever genotype types are available.

## Implementation Order

1. **Delete genotype implementation files**
2. **Delete chromosome implementation files**
3. **Remove module declarations and re-exports**
4. **Fix compilation errors in tests/examples**
5. **Verify everything still compiles and tests pass**

## Testing After Trimming

```bash
# Test distributed module
cargo test --lib distributed::

# Test centralized module  
cargo test --lib centralized::

# Run example with distributed genotype
cargo run --example distributed_evolve_nqueens --release

# Run example with centralized genotype (if any examples use matrix genotypes)
cargo run --example centralized_evolve_matrix --release
```

## Success Criteria

1. **Both modules compile independently**
2. **No cross-paradigm genotype/chromosome types remain**
3. **Generic operators still work with remaining types**
4. **Tests pass for the remaining functionality**
5. **Examples run successfully**

## Notes

- The beauty of this approach is its simplicity - we're mostly just deleting files and removing imports
- The generic operators automatically adapt to work with whatever genotypes remain
- No need to modify operator logic since they're already generic
- This is purely subtractive refactoring - we're only removing, not changing behavior
