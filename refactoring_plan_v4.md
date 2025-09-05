# Genetic Algorithm Library - Refactoring Plan v4: Embrace the Paradigm Split

## Executive Summary

Stop fighting the fundamental differences between distributed (chromosome-owns-genes) and centralized (matrix-based) genetic algorithms. Instead of forcing unification, create two clean, optimized modules that excel at their respective paradigms. This solves the original extensibility problem while reducing overall complexity.

## The Core Problem (Restated)

Currently, users cannot easily create custom genetic operators because:
1. `Genotype` owns all the mutation/crossover logic
2. Operators are thin wrappers that just call genotype methods
3. Creating a custom operator requires creating an entire new Genotype
4. This is too much work for reasonable domain-specific extensions

The root cause: Trying to support both distributed and centralized paradigms with a single trait hierarchy.

## The Solution: Paradigm-Specific Modules

### New Module Structure

```
genetic_algorithm/
├── src/
│   ├── common/                    # ~10% of codebase
│   │   ├── fitness.rs            # FitnessValue, FitnessOrdering
│   │   ├── allele.rs             # Allele trait  
│   │   ├── selection.rs          # Works on fitness scores only
│   │   ├── termination.rs        # Termination conditions
│   │   └── reporter.rs           # Reporter traits
│   │
│   ├── distributed/               # ~45% of codebase
│   │   ├── genotypes/
│   │   │   ├── binary.rs         # Each chromosome owns Vec<bool>
│   │   │   ├── list.rs           # Each chromosome owns Vec<T>
│   │   │   ├── unique.rs         # Each chromosome owns unique permutation
│   │   │   └── range.rs          # Each chromosome owns Vec<numeric>
│   │   ├── operators/
│   │   │   ├── mutate/           # Direct chromosome manipulation
│   │   │   ├── crossover/        # Direct parent manipulation
│   │   │   └── traits.rs         # Simple operator traits
│   │   ├── strategies/
│   │   │   ├── evolve.rs
│   │   │   ├── hill_climb.rs
│   │   │   └── permutate.rs
│   │   ├── chromosome.rs         # Trait: ChromosomeOwned
│   │   ├── builder.rs            
│   │   └── prelude.rs
│   │
│   ├── centralized/               # ~45% of codebase
│   │   ├── genotypes/
│   │   │   ├── dynamic_matrix.rs # All genes in Vec<T>, chromosomes point
│   │   │   └── static_matrix.rs  # All genes in [T; N*M], chromosomes point
│   │   ├── operators/
│   │   │   ├── mutate/           # Genotype-mediated manipulation
│   │   │   ├── crossover/        # Genotype-mediated manipulation
│   │   │   └── traits.rs         # Operators need genotype reference
│   │   ├── strategies/
│   │   │   ├── evolve.rs         # Optimized for bulk operations
│   │   │   └── hill_climb.rs     # Steepest ascent with population matrix
│   │   ├── chromosome.rs         # Trait: ChromosomePointer
│   │   ├── memory.rs             # Centralized memory management
│   │   ├── builder.rs
│   │   └── prelude.rs
│   │
│   ├── compat.rs                  # Backward compatibility (temporary)
│   └── lib.rs                     # Module exports
```

## Phase 1: Internal Reorganization (Week 1-2)

**Goal**: Reorganize code without breaking public API

### Steps:

1. **Create module structure**
   ```bash
   mkdir -p src/{common,distributed,centralized}/operators/{mutate,crossover}
   ```

2. **Move files to appropriate modules**
   ```rust
   // src/distributed/genotypes/binary.rs
   pub(crate) struct BinaryGenotype { ... }  // Initially private
   
   // src/genotype/binary.rs (compatibility shim)
   pub use crate::distributed::genotypes::binary::BinaryGenotype;
   ```

3. **Extract common traits**
   ```rust
   // src/common/fitness.rs
   pub type FitnessValue = f64;
   pub enum FitnessOrdering { Maximize, Minimize }
   
   // src/common/selection.rs
   pub trait Selection {
       fn select(&self, scores: &[FitnessValue], size: usize) -> Vec<usize>;
   }
   ```

4. **Add module boundaries**
   ```rust
   // src/distributed/mod.rs
   pub(crate) mod genotypes;  // Initially private
   pub(crate) mod operators;
   pub(crate) mod strategies;
   ```

## Phase 2: New Clean APIs (Week 3-4)

**Goal**: Create extensible APIs in each module

### Distributed Module API

```rust
// src/distributed/chromosome.rs
pub trait ChromosomeOwned: Clone + Send {
    type Gene: Clone;
    fn genes(&self) -> &[Self::Gene];
    fn genes_mut(&mut self) -> &mut [Self::Gene];
}

// src/distributed/operators/traits.rs
pub trait Mutate<C: ChromosomeOwned> {
    fn mutate(&mut self, chromosome: &mut C, rng: &mut impl Rng);
}

pub trait Crossover<C: ChromosomeOwned> {
    fn crossover(&mut self, parent1: &mut C, parent2: &mut C, rng: &mut impl Rng);
}

// src/distributed/genotypes/simple.rs
pub struct SimpleGenotype<G: Clone> {
    genes_size: usize,
    random_gene: Box<dyn Fn(&mut dyn Rng) -> G>,
}

// No mutation/crossover logic in genotype!
impl<G: Clone> SimpleGenotype<G> {
    pub fn random_chromosome(&self, rng: &mut impl Rng) -> SimpleChromosome<G> {
        let genes = (0..self.genes_size)
            .map(|_| (self.random_gene)(rng))
            .collect();
        SimpleChromosome { genes }
    }
}
```

### Centralized Module API

```rust
// src/centralized/chromosome.rs
pub trait ChromosomePointer: Copy + Send {
    fn row_id(&self) -> usize;
}

// src/centralized/memory.rs
pub trait MemoryPool<T> {
    fn get(&self, row: usize, col: usize) -> T;
    fn set(&mut self, row: usize, col: usize, value: T);
    fn swap(&mut self, row1: usize, row2: usize, col: usize);
    fn row_slice(&self, row: usize) -> &[T];
}

// src/centralized/operators/traits.rs
pub trait Mutate<M: MemoryPool<T>, T> {
    fn mutate(&mut self, 
              pool: &mut M, 
              chromosome: impl ChromosomePointer, 
              rng: &mut impl Rng);
}

pub trait Crossover<M: MemoryPool<T>, T> {
    fn crossover(&mut self,
                 pool: &mut M,
                 parent1: impl ChromosomePointer,
                 parent2: impl ChromosomePointer,
                 rng: &mut impl Rng);
}
```

## Phase 3: Enable True Extensibility (Week 5)

**Goal**: Make custom operators trivial to implement

### Example: Custom Distributed Operator

```rust
use genetic_algorithm::distributed::prelude::*;

/// Domain-specific mutation for protein folding
struct ProteinFoldMutate {
    hydrophobic_bias: f32,
}

impl<C> Mutate<C> for ProteinFoldMutate 
where 
    C: ChromosomeOwned<Gene = AminoAcid>
{
    fn mutate(&mut self, chromosome: &mut C, rng: &mut impl Rng) {
        let genes = chromosome.genes_mut();
        let idx = rng.gen_range(0..genes.len());
        
        // Domain-specific logic - impossible with current architecture!
        if genes[idx].is_hydrophobic() && rng.gen::<f32>() < self.hydrophobic_bias {
            // Prefer mutations that maintain hydrophobicity
            genes[idx] = AminoAcid::random_hydrophobic(rng);
        } else {
            genes[idx] = AminoAcid::random(rng);
        }
    }
}

// Usage is simple
let genotype = SimpleGenotype::new(100, || AminoAcid::random(&mut rng));
let mutate = ProteinFoldMutate { hydrophobic_bias: 0.7 };
let evolve = Evolve::builder()
    .genotype(genotype)
    .mutate(mutate)  // Just works!
    .build();
```

### Example: Custom Centralized Operator

```rust
use genetic_algorithm::centralized::prelude::*;

/// Mutation that considers neighboring genes
struct SpatialMutate {
    influence_radius: usize,
}

impl<M, T> Mutate<M, T> for SpatialMutate
where
    M: MemoryPool<T>,
    T: RangeAllele,
{
    fn mutate(&mut self, pool: &mut M, chromosome: impl ChromosomePointer, rng: &mut impl Rng) {
        let row = chromosome.row_id();
        let idx = rng.gen_range(0..pool.width());
        
        // Consider neighbors - needs access to pool
        let mut sum = T::default();
        let mut count = 0;
        for j in idx.saturating_sub(self.influence_radius)..=idx + self.influence_radius {
            if j < pool.width() {
                sum = sum + pool.get(row, j);
                count += 1;
            }
        }
        
        let average = sum / T::from(count);
        let mutation = T::random_near(average, rng);
        pool.set(row, idx, mutation);
    }
}
```

## Phase 4: Migration Support (Week 6)

**Goal**: Smooth transition for existing users

### Compatibility Layer

```rust
// src/compat.rs
#[deprecated(since = "1.0.0", note = "Use distributed::BinaryGenotype or centralized::DynamicMatrix")]
pub type BinaryGenotype = crate::distributed::genotypes::BinaryGenotype;

#[deprecated(since = "1.0.0", note = "Use distributed::operators::FlipMutate")]
pub type MutateSingleGene = crate::distributed::operators::mutate::FlipMutate;

// Helper macro for migration
#[macro_export]
macro_rules! migrate_to_distributed {
    ($genotype:expr) => {
        compile_warning!("Consider using distributed::prelude::* for cleaner API");
        $genotype
    };
}
```

### Migration Guide

```rust
// OLD (current library)
use genetic_algorithm::strategy::evolve::prelude::*;

let genotype = BinaryGenotype::builder()
    .with_genes_size(100)
    .build()
    .unwrap();

let evolve = Evolve::builder()
    .with_genotype(genotype)
    .with_mutate(MutateSingleGene::new(0.2))
    .build()
    .unwrap();

// NEW (distributed - what they probably want)
use genetic_algorithm::distributed::prelude::*;

let genotype = BinaryGenotype::new(100);
let mutate = FlipMutate::with_rate(0.2);

let evolve = Evolve::builder()
    .genotype(genotype)
    .mutate(mutate)
    .build();  // No Result needed - can't fail!

// NEW (custom operator - now possible!)
struct MyCustomMutate;
impl<C: ChromosomeOwned<Gene = bool>> Mutate<C> for MyCustomMutate {
    fn mutate(&mut self, chromosome: &mut C, rng: &mut impl Rng) {
        // Direct access to genes!
        for gene in chromosome.genes_mut() {
            if rng.gen::<f32>() < 0.1 {
                *gene = !*gene;
            }
        }
    }
}
```

## Phase 5: Documentation & Examples (Week 7)

### Distributed Tutorial

```markdown
# Distributed Genetic Algorithms

Use this module when:
- Each solution is independent
- You need custom operators
- Genes can be any type
- You prioritize extensibility

## Quick Start
\```rust
use genetic_algorithm::distributed::prelude::*;

// 1. Define your gene type
#[derive(Clone, Debug)]
struct MyGene { ... }

// 2. Create a genotype
let genotype = SimpleGenotype::new(100, || MyGene::random());

// 3. Define custom operators (optional)
struct MyMutate;
impl<C: ChromosomeOwned<Gene = MyGene>> Mutate<C> for MyMutate {
    fn mutate(&mut self, chromosome: &mut C, rng: &mut impl Rng) {
        // Your domain logic here
    }
}
\```
```

### Centralized Tutorial

```markdown
# Centralized Genetic Algorithms

Use this module when:
- You need maximum performance
- Working with numeric genes
- Want GPU acceleration potential
- Have large populations

## Quick Start
\```rust
use genetic_algorithm::centralized::prelude::*;

// 1. Create a matrix genotype
let genotype = DynamicMatrix::<f32>::new(
    100,  // genes per chromosome
    0.0..=1.0  // range
);

// 2. Use optimized bulk operations
let evolve = Evolve::builder()
    .genotype(genotype)
    .population_size(1000)  // Large populations are efficient
    .enable_gpu()  // Future feature
    .build();
\```
```

## Benefits of This Refactoring

### 1. **Solves the Original Problem**
   - Users can easily create custom operators
   - No need to implement entire Genotypes
   - Direct access to genes in distributed module

### 2. **Reduces Complexity**
   - No enum dispatch or runtime type checking
   - Each module is simpler and focused
   - Clear separation of concerns

### 3. **Improves Performance**
   - Each paradigm optimized without compromise
   - Centralized can use GPU/SIMD
   - Distributed can parallelize freely

### 4. **Better Developer Experience**
   - Compile-time safety
   - Clear mental model
   - Focused documentation
   - Intuitive API

### 5. **Enables Innovation**
   - Easy to add GPU support to centralized
   - Easy to add new gene types to distributed
   - Community can contribute operators

## Success Metrics

1. **Extensibility**: Users can implement custom operators in <20 lines
2. **Performance**: No regression, 10-20% improvement possible
3. **Adoption**: 80% of users naturally choose the right module
4. **Simplicity**: 30% reduction in internal complexity
5. **Type Safety**: Zero runtime type errors

## Timeline

- **Week 1-2**: Internal reorganization
- **Week 3-4**: New APIs  
- **Week 5**: Extensibility examples
- **Week 6**: Migration support
- **Week 7**: Documentation
- **Week 8**: Beta release
- **Month 3**: Gather feedback
- **Month 4**: 1.0 release

## Conclusion

By embracing the fundamental differences between distributed and centralized genetic algorithms, we:
1. Enable true extensibility (solving the original problem)
2. Reduce overall complexity
3. Improve performance for both paradigms
4. Provide a clearer mental model
5. Make the library easier to use and extend

This isn't just a refactoring - it's acknowledging that we've been trying to unify things that are fundamentally different, and that separation actually serves users better.