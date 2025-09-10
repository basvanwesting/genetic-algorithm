# Distributed Track Chromosome Simplification Plan

## Executive Summary
A comprehensive plan to unify chromosome types and clarify responsibilities between Chromosome, Genotype, and Strategy components in the distributed track.

## Current State Analysis

### Problems Identified

1. **Redundant Type Aliases**
   - All chromosome types (BinaryChromosome, RangeChromosome, ListChromosome, etc.) are just aliases for `Vector<T>`
   - No behavioral differences between types
   - Confusing indirection requiring users to define structurally identical types

2. **Misplaced Responsibilities**
   - Genotype stores `best_genes` which is actually search progress, not search space definition
   - Genotype manages hash calculation config (`genes_hashing: bool`) which should be a strategy decision
   - ChromosomeManager has methods that should be chromosome's own responsibility

3. **Tight Coupling**
   - Mutate/Crossover strategies must work through genotype methods
   - Limited flexibility for custom strategies
   - Unclear separation of concerns

## Proposed Responsibility Model

### Chromosome<T> (unified type, renamed from Vector<T>)
**IS**: A candidate solution container
**DOES**: 
- Store genes and metadata (fitness, age, hash)
- Always calculate and store hash when genes change
- Manage own state (get/set genes, copy_from)
**DOESN'T**: 
- Know about constraints or validity
- Store configuration

### Genotype
**IS**: Search space definition
**DOES**:
- Generate valid random genes
- Define mutation/crossover rules
- Validate gene combinations
- Store seed genes list
**DOESN'T**:
- Store best solutions
- Manage hash configuration
- Track search progress

### Strategies (Evolve, HillClimb, Permutate)
**ARE**: Search algorithms
**DO**:
- Track best solutions (as cloned chromosomes)
- Use hash for fitness caching (when enabled)
- Use hash for deduplication (when needed)
- Manage search progress
**DON'T**:
- Define search space rules
- Control whether hash is calculated (always done by chromosome)

## Implementation Plan

### Phase 1: Extend Allele Trait
1. Add `hash_slice` method to Allele trait
2. Implement for all existing Allele types:
   - bool, char, integers (i8, i16, i32, i64, i128, isize)
   - unsigned integers (u8, u16, u32, u64, u128, usize)  
   - floats (f32, f64) with special bytemuck handling
3. This enables type-specific hash behavior while maintaining single Chromosome type

### Phase 2: Unify Chromosome Type
1. Rename `Vector<T>` to `Chromosome<T>` for clarity
2. Remove all type aliases:
   - BinaryChromosome
   - RangeChromosome
   - ListChromosome
   - UniqueChromosome
   - MultiRangeChromosome
   - MultiListChromosome
   - MultiUniqueChromosome
3. Update all genotypes to use `type Chromosome = Chromosome<Self::Allele>`
4. Update all imports throughout the codebase

### Phase 3: Move Hash Calculation to Chromosome
1. Use the extended Allele trait in Chromosome implementation:
   ```rust
   impl<T: Allele> Chromosome<T> {
       pub fn calculate_hash(&self) -> GenesHash {
           let mut hasher = FxHasher::default();
           T::hash_slice(&self.genes, &mut hasher);
           hasher.finish()
       }
       
       pub fn set_genes(&mut self, genes: Vec<T>) {
           self.genes = genes;
           let hash = self.calculate_hash();
           self.genes_hash = Some(hash);
           self.fitness_score = None;
           self.age = 0;
       }
   }
   ```
2. Remove `calculate_genes_hash` from Genotype trait
3. Remove `genes_hashing: bool` field from all genotypes
4. **Always calculate hash**: Hash is now always calculated and stored when genes change
   - This is a core chromosome responsibility
   - No configuration needed - it's always available
5. **Important**: The hash serves multiple purposes:
   - **Fitness Cache Key**: Used by FitnessCache to cache expensive fitness calculations
   - **Population Deduplication**: Used to track unique chromosomes in the population
   - **Population Cardinality**: Better estimation of population diversity

### Phase 4: Remove Best Genes from Genotype
1. Remove from Genotype trait:
   - `save_best_genes(&mut self, chromosome: &Self::Chromosome)`
   - `load_best_genes(&mut self, chromosome: &mut Self::Chromosome)`
   - `best_genes(&self) -> &Self::Genes`
   - `best_genes_slice(&self) -> &[Self::Allele]`
2. Remove `best_genes` field from all genotype implementations
3. Add to strategy states:
   ```rust
   best_chromosome: Option<Chromosome<T>>
   ```
4. Update strategies to clone and store best chromosome:
   ```rust
   // When finding better solution
   self.best_chromosome = Some(chromosome.clone())
   
   // When needing to restore
   chromosome = self.best_chromosome.clone().unwrap()
   ```

### Phase 5: Simplify ChromosomeManager Further
1. Remove from ChromosomeManager:
   - `reset_chromosome_state` (becomes direct chromosome operation)
   - `copy_chromosome_state` (use chromosome.copy_from)
2. ChromosomeManager reduced to essentials:
   - `random_genes_factory` (needs genotype knowledge)
   - `genes_capacity` (provides capacity hint)
3. Update helper methods to use chromosome's own capabilities

### Phase 6: Clean Up and Test
1. Update all imports to use unified `Chromosome<T>`
2. Fix test code that relies on old patterns
3. Update examples
4. Run full test suite
5. Update documentation

## Expected Benefits

### Code Reduction
- **~50% reduction** in chromosome-related type definitions
- **~30% reduction** in genotype implementation code
- **Elimination** of redundant type aliases

### Architecture Improvements
- **Clearer separation** of concerns
- **More intuitive** API (chromosomes manage themselves)
- **Greater flexibility** for custom strategies
- **Simpler mental model** (one chromosome type for all)

### Maintainability
- **Easier to extend** with new genotypes
- **Less coupling** between components
- **Clearer responsibilities** for each trait

## Example Transformations

### Before
```rust
// chromosome.rs
pub type BinaryChromosome = Vector<bool>;
pub type RangeChromosome<T> = Vector<T>;

// genotype.rs
pub struct Binary {
    pub best_genes: Vec<bool>,
    pub genes_hashing: bool,
    // ...
}

impl Genotype for Binary {
    type Chromosome = BinaryChromosome;
    
    fn save_best_genes(&mut self, chromosome: &Self::Chromosome) {
        self.best_genes.clone_from(&chromosome.genes);
    }
    
    fn calculate_genes_hash(&self, chromosome: &Self::Chromosome) -> Option<GenesHash> {
        if self.genes_hashing {
            // calculate hash
        }
    }
}

// strategy.rs
genotype.save_best_genes(chromosome);
```

### After
```rust
// chromosome.rs
pub struct Chromosome<T: Allele> {
    pub genes: Vec<T>,
    // ...
}

impl<T: Allele> Chromosome<T> {
    pub fn calculate_hash(&self) -> GenesHash {
        let mut hasher = FxHasher::default();
        T::hash_slice(&self.genes, &mut hasher);
        hasher.finish()
    }
    
    pub fn set_genes(&mut self, genes: Vec<T>) {
        self.genes = genes;
        // Always calculate and store hash
        let hash = self.calculate_hash();
        self.genes_hash = Some(hash);
        // Reset other state
        self.fitness_score = None;
        self.age = 0;
    }
}

// genotype.rs
pub struct Binary {
    // no best_genes
    // no genes_hashing configuration
    // ...
}

impl Genotype for Binary {
    type Chromosome = Chromosome<bool>;
    // no save_best_genes
    // no calculate_genes_hash
}

// strategy.rs
self.best_chromosome = Some(chromosome.clone());
// Hash is already calculated and stored by chromosome
```

## Risk Mitigation

### Breaking Changes
- This is a significant breaking change for users
- Mitigation: Clear migration guide with examples

### Performance Impact
- Additional cloning of best chromosome
- Mitigation: Chromosomes are already cloned frequently; one more clone for best is negligible

### Hash Calculation
- Always calculating hash adds small overhead to every gene change
- Mitigation: Hash calculation is fast compared to fitness evaluation; always having it available simplifies code
- Type-specific handling achieved through Allele trait method
- Each Allele type implements its own hash_slice method (more boilerplate but type-safe)
- Float types use bytemuck::cast_slice for deterministic hashing

### Fitness Cache Compatibility
- Hash calculation MUST be stable and consistent for cache correctness
- Float types need special handling (cast to bytes) to ensure deterministic hashing
- Since hash is always calculated, fitness cache can rely on it being available
- Mitigation: Document hash requirements clearly, provide type-specific implementations if needed

## Conclusion

This simplification will result in a much cleaner, more maintainable, and more flexible architecture. The distributed track will have:
- One unified chromosome type
- Clear separation of responsibilities
- Simpler genotype implementations
- More powerful strategy capabilities

The changes align with Rust's philosophy of zero-cost abstractions and clear ownership semantics.
