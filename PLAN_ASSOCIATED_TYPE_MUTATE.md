# Final Plan: Associated Types for Mutate, Crossover, Select, Extension

## Critical Insight: No Boxing Required!

The Builder already handles Fitness with associated types WITHOUT boxing:
```rust
pub struct Builder<
    G: EvolveGenotype,
    M: Mutate,
    F: Fitness<Genotype = G>,  // Associated type, no boxing!
    S: Crossover,
    C: Select,
    ...
>
```

Therefore, Mutate/Crossover/Select with associated types would work the same way - no performance penalty!

## The Change

### Current
```rust
pub trait Mutate {
    fn call<G: EvolveGenotype, R: Rng, SR: StrategyReporter<Genotype = G>>(
        &mut self,
        genotype: &G,
        state: &mut EvolveState<G>,
        ...
    );
}
```

### Proposed
```rust
pub trait Mutate {
    type Genotype: EvolveGenotype;
    
    fn call<R: Rng, SR: StrategyReporter<Genotype = Self::Genotype>>(
        &mut self,
        genotype: &Self::Genotype,  // Concrete type!
        state: &mut EvolveState<Self::Genotype>,
        ...
    );
}
```

## Implementation Pattern

### Generic Implementations (Most Common)
```rust
// Works for ANY genotype - like Fitness placeholders
pub struct MutateSingleGene<G: EvolveGenotype> {
    mutation_rate: f32,
    _phantom: PhantomData<G>,
}

impl<G: EvolveGenotype> Mutate for MutateSingleGene<G> {
    type Genotype = G;
    
    fn call<R: Rng, SR: StrategyReporter<Genotype = G>>(
        &mut self,
        genotype: &G,
        state: &mut EvolveState<G>,
        config: &EvolveConfig,
        reporter: &mut SR,
        rng: &mut R,
    ) {
        // Can call genotype methods directly
        for chromosome in &mut state.population.chromosomes {
            if rng.gen::<f32>() < self.mutation_rate {
                genotype.mutate_chromosome_genes(1, false, chromosome, None, rng);
            }
        }
    }
}

// Constructor stays simple
impl<G: EvolveGenotype> MutateSingleGene<G> {
    pub fn new(mutation_rate: f32) -> Self {
        Self {
            mutation_rate,
            _phantom: PhantomData,
        }
    }
}
```

### Type-Specific Implementations (When Needed)
```rust
// Only for UniqueGenotype
pub struct MutateSwapAdjacent;

impl<T: Allele> Mutate for MutateSwapAdjacent {
    type Genotype = UniqueGenotype<T>;
    
    fn call<...>(...) {
        // Direct access to UniqueGenotype methods!
        let idx = genotype.sample_gene_index(rng);
        // ...
    }
}
```

## User Experience

### Before
```rust
let evolve = Evolve::builder()
    .with_genotype(UniqueGenotype::<u8>::builder().build().unwrap())
    .with_mutate(MutateSingleGene::new(0.2))
    .with_fitness(NQueensFitness)
    .build();
```

### After
```rust
let evolve = Evolve::builder()
    .with_genotype(UniqueGenotype::<u8>::builder().build().unwrap())
    .with_mutate(MutateSingleGene::new(0.2))  // Type inference still works!
    .with_fitness(NQueensFitness)
    .build();
```

**Key: Type inference from the builder constraint means users don't need to specify types explicitly in most cases!**

The builder's type constraint `M: Mutate<Genotype = G>` combined with `G` being known from the genotype means Rust can infer the type parameter for `MutateSingleGene<_>`.

## Builder Updates

```rust
pub struct Builder<
    G: EvolveGenotype,
    M: Mutate<Genotype = G>,      // Add constraint
    F: Fitness<Genotype = G>,     // Already has it
    S: Crossover<Genotype = G>,   // Add constraint
    C: Select<Genotype = G>,      // Add constraint
    E: Extension,                  // No genotype needed
    SR: StrategyReporter<Genotype = G>,
>
```

## Benefits

1. **Type Safety** - Concrete genotype access in implementations
2. **Clean Custom Implementations** - No unsafe/casting/workarounds
3. **Consistency** - All traits follow same pattern as Fitness
4. **No Performance Penalty** - No boxing, no dynamic dispatch
5. **Better IDE Support** - Concrete types mean better autocomplete

## Costs

1. **Breaking Change** - All implementations need updating
2. **More Verbose Generics** - Need PhantomData for generic impls
3. **Migration Effort** - Users must update custom operators

## Migration Guide

### For Library Implementations
```rust
// Old
pub struct MutateSingleGene {
    mutation_rate: f32,
}

impl Mutate for MutateSingleGene {
    fn call<G: EvolveGenotype, ...>(...) { ... }
}

// New
pub struct MutateSingleGene<G: EvolveGenotype> {
    mutation_rate: f32,
    _phantom: PhantomData<G>,
}

impl<G: EvolveGenotype> Mutate for MutateSingleGene<G> {
    type Genotype = G;
    fn call<...>(...) { ... }
}
```

### For User Custom Implementations
```rust
// Old
impl Mutate for MyCustomMutate {
    fn call<G: EvolveGenotype, ...>(...) {
        // Can't access genotype-specific methods
    }
}

// New
impl Mutate for MyCustomMutate {
    type Genotype = UniqueGenotype<u8>;  // Specify exact type
    fn call<...>(...) {
        // Can access all UniqueGenotype methods!
        let idx = genotype.sample_gene_index(rng);
    }
}
```

## Which Traits Should Change?

| Trait                | Change?      | Rationale                                          |
| -------              | ---------    | -----------                                        |
| **Mutate**           | ✅ Yes       | Needs genotype methods for mutations               |
| **Crossover**        | ✅ Yes       | Needs genotype methods for crossover               |
| **Select**           | ❌ No        | Only works with populations, not genotype-specific |
| **Extension**        | ❌ No        | High-level operations, not genotype-specific       |
| **Fitness**          | Already done | Already uses associated types                      |
| **StrategyReporter** | Already done | Already uses associated types                      |

## Implementation Order

### Phase 1: Changes
1. Update Mutate trait
2. Update all library mutations (7 files)
3. Update Crossover trait
4. Update all library crossovers (7 files)
5. Update builder constraints
6. Update tests and examples

### Phase 2: Documentation
- Migration guide for users
- Update all examples
- Update trait documentation

## Example: Custom Mutate After Change

```rust
// Clean implementation with full type access!
impl Mutate for MutateEvenIndicesOnly {
    type Genotype = UniqueGenotype<u8>;
    
    fn call<R: Rng, SR: StrategyReporter<Genotype = UniqueGenotype<u8>>>(
        &mut self,
        genotype: &UniqueGenotype<u8>,  // Concrete type!
        state: &mut EvolveState<UniqueGenotype<u8>>,
        config: &EvolveConfig,
        reporter: &mut SR,
        rng: &mut R,
    ) {
        for chromosome in &mut state.population.chromosomes {
            if rng.gen::<f32>() < self.mutation_rate {
                // Direct access to UniqueGenotype methods!
                let idx1 = genotype.sample_gene_index(rng);
                let idx2 = genotype.sample_gene_index(rng);
                chromosome.genes.swap(idx1, idx2);
                chromosome.reset_state();
            }
        }
    }
}
```

## Final Recommendation

### Why:
1. **No performance penalty** - Builder doesn't require boxing
2. **Better API** - Type-safe access to genotype methods
3. **Consistency** - Matches Fitness pattern
4. **Worth the breaking change** - Significant API improvement

### Why NOT Select/Extension:
1. **Select** works with populations, not genotype-specific operations
2. **Extension** does high-level operations, rarely needs genotype details
3. **Less benefit** for the breaking change

## Conclusion

The initial concern about boxing was unfounded. The builder already handles associated types elegantly for Fitness, and will handle them the same way for Mutate and Crossover. This change provides a cleaner, more type-safe API that allows custom implementations to access genotype-specific methods directly. The breaking change is justified by the significant improvement in API quality and consistency.
