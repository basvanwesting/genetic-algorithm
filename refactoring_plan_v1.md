# Genetic Algorithm Library - Architecture Refactoring Plan

## Current Architecture Analysis

### Problems Identified

1. **Genotype Monopolizes Genetic Operations**
   - `Genotype` directly implements mutation logic in `mutate_chromosome_genes()`
   - `Genotype` directly implements crossover logic in `crossover_chromosome_genes()` and `crossover_chromosome_points()`
   - Operators (Mutate/Crossover) are thin wrappers that just call genotype methods
   - Users must create entire new Genotype implementations for custom domain-specific operations

2. **High Coupling Between Components**
   - `Mutate` and `Crossover` traits depend on `EvolveGenotype`
   - Operators can't access gene data directly - they must go through Genotype
   - ChromosomeManager is mixed into Genotype, adding memory management responsibilities
   - Operators are tightly bound to specific Genotype implementations

3. **Limited Extensibility**
   - To implement custom mutation/crossover behavior, users must create entire new Genotype implementations
   - Can't mix and match mutation strategies with existing genotypes
   - Domain-specific operators require reimplementing the entire Genotype
   - No way to compose or combine different operator strategies

4. **Responsibility Overload in Genotype**
   - Gene representation and storage
   - Random gene generation
   - Mutation implementation
   - Crossover implementation  
   - Chromosome lifecycle management (bin, cloning, destruction)
   - Best genes tracking
   - Hashing logic
   - Population construction
   - Seed genes management

## Refactoring Plan: Decouple Genetic Operators from Genotype

### Goal
Make Mutate and Crossover truly pluggable and extensible by moving genetic operation logic out of Genotype and into the operators themselves, allowing users to implement custom domain-specific operators without creating new Genotypes.

### Phase 1: Extract Genetic Operations Interface

#### 1.1 Create GeneticOperations trait
```rust
trait GeneticOperations<G: GenotypeCore> {
    fn swap_genes(&mut self, chromosome: &mut G::Chromosome, idx1: usize, idx2: usize);
    fn flip_gene(&mut self, chromosome: &mut G::Chromosome, idx: usize);
    fn get_gene(&self, chromosome: &G::Chromosome, idx: usize) -> G::Allele;
    fn set_gene(&mut self, chromosome: &mut G::Chromosome, idx: usize, value: G::Allele);
    fn exchange_genes(&mut self, chr1: &mut G::Chromosome, chr2: &mut G::Chromosome, idx: usize);
    fn validate_constraints(&self, chromosome: &G::Chromosome) -> bool;
}
```

#### 1.2 Create GenotypeCore trait
Reduced Genotype with only essential responsibilities:
```rust
trait GenotypeCore {
    type Allele: Allele;
    type Genes: Genes;
    type Chromosome: Chromosome;
    
    // Core representation
    fn genes_size(&self) -> usize;
    fn random_allele<R: Rng>(&self, rng: &mut R) -> Self::Allele;
    fn random_genes<R: Rng>(&self, rng: &mut R) -> Self::Genes;
    
    // Constraints and validation
    fn validate_genes(&self, genes: &Self::Genes) -> bool;
    fn allele_range(&self) -> Option<Range<Self::Allele>>;
    
    // Remove: mutate_chromosome_genes, crossover_chromosome_*
}
```

### Phase 2: Refactor Operators to be Self-Contained

#### 2.1 Enhance Mutate trait
```rust
trait Mutate {
    fn mutate<G: GenotypeCore, R: Rng>(
        &self, 
        ops: &mut GeneticOperations<G>,
        chromosome: &mut G::Chromosome,
        rng: &mut R
    );
    
    fn supports_genotype<G: GenotypeCore>(&self) -> bool;
}

// Example implementation
struct FlipMutate;
impl Mutate for FlipMutate {
    fn mutate<G: GenotypeCore, R: Rng>(
        &self,
        ops: &mut GeneticOperations<G>, 
        chromosome: &mut G::Chromosome,
        rng: &mut R
    ) {
        let idx = rng.gen_range(0..ops.genes_size());
        ops.flip_gene(chromosome, idx);
    }
}
```

#### 2.2 Enhance Crossover trait
```rust
trait Crossover {
    fn crossover<G: GenotypeCore, R: Rng>(
        &self,
        ops: &mut GeneticOperations<G>,
        parent1: &mut G::Chromosome,
        parent2: &mut G::Chromosome,
        rng: &mut R
    );
    
    fn supports_genotype<G: GenotypeCore>(&self) -> bool;
}
```

### Phase 3: Separate Chromosome Management

#### 3.1 Extract ChromosomeManager
```rust
struct ChromosomePool<C: Chromosome> {
    bin: Vec<C>,
    // Lifecycle management separated from Genotype
}

impl<C: Chromosome> ChromosomePool<C> {
    fn get_or_create(&mut self) -> C;
    fn recycle(&mut self, chromosome: C);
    fn clone_chromosome(&mut self, source: &C) -> C;
}
```

### Phase 4: Backward Compatibility Layer

#### 4.1 Create adapter implementations
- Wrapper that implements new traits for existing Genotypes
- Default operator implementations that delegate to old methods
- Deprecation warnings guiding migration

#### 4.2 Migration path
```rust
// Old way (still works with adapter)
let genotype = BinaryGenotype::builder()
    .with_genes_size(100)
    .build();

// New way  
let genotype = BinaryGenotypeCore::builder()
    .with_genes_size(100)
    .build();
    
let mutate = CustomMutate::new();  // User's custom implementation
let crossover = UniformCrossover::new();  // Library or custom
```

### Phase 5: Enable Advanced Patterns

#### 5.1 Composable operators
```rust
struct CompositeMutate {
    operators: Vec<Box<dyn Mutate>>,
    weights: Vec<f32>,
}

struct AdaptiveMutate {
    base: Box<dyn Mutate>,
    adaptation_fn: Box<dyn Fn(usize) -> f32>,
}
```

#### 5.2 Domain-specific operators
```rust
// User can now easily create domain-specific operators
struct ProteinFoldingMutate;
impl Mutate for ProteinFoldingMutate {
    fn mutate<G: GenotypeCore, R: Rng>(...) {
        // Domain-specific logic without creating new Genotype
    }
}
```

## Benefits

1. **Improved Extensibility**
   - Users can implement custom Mutate/Crossover without touching Genotype
   - Mix and match operators with any compatible Genotype
   - Compose multiple operators together

2. **Better Separation of Concerns**
   - Genotype focuses on representation
   - Operators focus on manipulation
   - ChromosomeManager handles lifecycle

3. **Enhanced Testability**
   - Test operators in isolation
   - Mock GeneticOperations for unit tests
   - Clearer boundaries between components

4. **Code Reuse**
   - Share operators across different Genotypes
   - Build libraries of domain-specific operators
   - Reduce duplication in Genotype implementations

5. **Backward Compatibility**
   - Existing code continues to work
   - Gradual migration path
   - Clear deprecation strategy

## Implementation Timeline

1. **Phase 1** (Week 1-2): Extract interfaces, create new traits
2. **Phase 2** (Week 3-4): Refactor operators, implement for one Genotype
3. **Phase 3** (Week 5): Separate ChromosomeManager
4. **Phase 4** (Week 6): Compatibility layer and migration guide
5. **Phase 5** (Week 7-8): Advanced patterns and examples

## Testing Strategy

1. Maintain all existing tests
2. Add unit tests for new operator implementations
3. Integration tests for mixed old/new code
4. Performance benchmarks to ensure no regression
5. Example implementations demonstrating new patterns

## Documentation Updates

1. Architecture overview explaining new design
2. Migration guide from old to new API
3. Tutorial on creating custom operators
4. Best practices for operator implementation
5. Performance considerations guide