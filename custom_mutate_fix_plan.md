# Plan: Fix Custom Mutate Example Using Fitness Pattern

## Problem Analysis

### Current Mutate Trait (Generic)
```rust
pub trait Mutate {
    fn call<G: EvolveGenotype, R: Rng, SR: StrategyReporter<Genotype = G>>(
        &mut self,
        genotype: &G,  // Generic G - can't access UniqueGenotype methods!
        state: &mut EvolveState<G>,
        ...
    );
}
```

### Fitness Trait (Associated Type)
```rust
pub trait Fitness {
    type Genotype: Genotype;  // Concrete type!
    
    fn calculate_for_chromosome(
        &mut self,
        chromosome: &FitnessChromosome<Self>,
        genotype: &Self::Genotype,  // Concrete type - can access specific methods!
    ) -> Option<FitnessValue>;
}
```

## Why Fitness Pattern Works

1. **Associated type** - `type Genotype` is specified at implementation time
2. **Concrete access** - Gets `&Self::Genotype`, not generic `&G`
3. **Type-specific implementation** - Each Fitness impl knows its exact Genotype type
4. **Can access type-specific methods** - Like `UniqueGenotype::sample_gene_index()`

## Two Possible Solutions

### Solution A: Change Mutate Trait (Breaking Change)
```rust
pub trait Mutate {
    type Genotype: EvolveGenotype;  // Add associated type
    
    fn call<R: Rng, SR: StrategyReporter<Genotype = Self::Genotype>>(
        &mut self,
        genotype: &Self::Genotype,  // Now concrete!
        state: &mut EvolveState<Self::Genotype>,
        ...
    );
}
```

**Pros:**
- Clean, matches Fitness pattern
- Type-safe access to genotype-specific methods
- No workarounds needed

**Cons:**
- Breaking API change
- All existing Mutate implementations need updating
- Less flexible (can't have one Mutate work for multiple genotypes)

### Solution B: Custom Mutate for Specific Genotype (No API Change)
```rust
// Don't implement generic Mutate trait
// Create a specific mutate for UniqueGenotype<u8> only
#[derive(Clone, Debug)]
struct MutateEvenIndicesForNQueens {
    mutation_rate: f32,
}

// Implement Mutate ONLY for our specific case
impl Mutate for MutateEvenIndicesForNQueens {
    fn call<G: EvolveGenotype, R: Rng, SR: StrategyReporter<Genotype = G>>(
        &mut self,
        genotype: &G,
        state: &mut EvolveState<G>,
        ...
    ) where G: EvolveGenotype<Allele = u8> {
        // We know G must be UniqueGenotype<u8> from our usage
        // Can we cast or pattern match safely?
    }
}
```

**Still problematic:** Even with constraints, we can't access `UniqueGenotype::sample_gene_index()`

### Solution C: Work Within Current Constraints (Pragmatic)
```rust
// Accept we can't use genotype-specific methods
// Implement custom logic that doesn't need them
struct MutateEvenIndicesOnly {
    mutation_rate: f32,
    genes_size: usize,
    // Duplicate what we need from UniqueGenotype
}
```

**This is what we did** - It works but shows API limitations

## Recommendation

### Short Term (For Example)
Use Solution C with clear documentation:
```rust
// NOTE: Due to the generic nature of the Mutate trait, we cannot access
// UniqueGenotype-specific methods like sample_gene_index(). This example
// shows how to work within these constraints by reimplementing needed logic.
// 
// For comparison, the Fitness trait uses associated types which allows
// type-specific implementations with full access to genotype methods.
```

### Long Term (API Evolution)
Consider adding a new trait alongside Mutate:
```rust
// Keep existing Mutate for generic implementations
pub trait Mutate { ... }

// Add new trait for type-specific implementations
pub trait TypedMutate {
    type Genotype: EvolveGenotype;
    fn call<R: Rng, SR: StrategyReporter<Genotype = Self::Genotype>>(...);
}

// Allow either in the builder
impl Builder {
    fn with_mutate(mut self, mutate: impl Mutate) { ... }
    fn with_typed_mutate(mut self, mutate: impl TypedMutate) { ... }
}
```

## Conclusion

The example successfully demonstrates custom mutation but also reveals an API design issue. The Fitness trait's associated type pattern is superior for type-specific implementations. The Mutate trait's generic design prevents accessing genotype-specific functionality, forcing workarounds.

The example should:
1. Keep the current working implementation
2. Add clear documentation about the limitation
3. Point to Fitness as an example of better trait design for type-specific access