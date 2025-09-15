# Refactoring Plan v6: From Dual Tracks to Purpose-Built Modules

## Executive Summary

Transform the confusing "centralized vs distributed" dual-track architecture into purpose-built modules:
1. **Classic**: Current main branch (simple, mutable, educational)
2. **Parallel**: Evolve from distributed track (immutable, thread-parallel)
3. **Compute**: Evolve from centralized track (matrix-based, GPU-ready)
4. **Distributed**: Future addition (network-scale)

This plan focuses on implementing Parallel and Compute modules while preserving Classic.

---

## Phase 1: Parallel Module Implementation
*Timeline: 1-2 weeks*  
*Base: Current distributed track*

### Step 1.1: Achieve True Immutability
**Goal**: Remove all `&mut self` from Genotype trait and implementations

```rust
// Current (distributed/genotype.rs)
trait Genotype {
    fn set_random_genes<R: Rng>(&mut self, chromosome: &mut Chromosome, rng: &mut R);
    fn mutate_chromosome_genes(&mut self, chromosome: &mut Chromosome, ...);
}

// Target (parallel/genotype.rs)
trait Genotype {
    fn set_random_genes<R: Rng>(&self, chromosome: &mut Chromosome, rng: &mut R);
    fn mutate_chromosome_genes(&self, chromosome: &mut Chromosome, ...);
}
```

**Tasks:**
- [ ] Audit all `&mut self` usage in `src/distributed/genotype/`
- [ ] Move mutable state to parameters or return new instances
- [ ] For distributions/caches, use interior mutability (`RefCell`) if needed
- [ ] Update all genotype implementations:
  - [ ] binary.rs - remove `&mut self` from all methods
  - [ ] list.rs - remove `&mut self` from all methods  
  - [ ] unique.rs - remove `&mut self` from all methods
  - [ ] range.rs - remove `&mut self` from all methods
  - [ ] multi_list.rs - remove `&mut self` from all methods
  - [ ] multi_unique.rs - remove `&mut self` from all methods
  - [ ] multi_range.rs - remove `&mut self` from all methods

### Step 1.2: Enhance Parallelization
**Goal**: Ensure all operations leverage Rayon effectively

```rust
// Add parallel-first methods to Population
impl Population {
    // Default to parallel
    pub fn evolve(&self) -> Population {
        self.par_evolve()
    }
    
    // Explicit parallel version
    pub fn par_evolve(&self) -> Population {
        self.chromosomes
            .par_iter()
            .map(|c| self.evolve_chromosome(c))
            .collect()
    }
}
```

**Tasks:**
- [ ] Add `par_` variants for all population operations
- [ ] Make parallel versions the default
- [ ] Add benchmarks comparing sequential vs parallel
- [ ] Optimize chunk sizes for Rayon

### Step 1.3: Functional API Improvements
**Goal**: Leverage immutability for better APIs

```rust
// Add functional combinators
impl Strategy {
    fn map<F>(self, f: F) -> Map<Self, F>;
    fn and_then<S>(self, next: S) -> Chain<Self, S>;
    fn retry(self, n: usize) -> Retry<Self>;
}

// Enable strategy composition
let strategy = mutate
    .and_then(crossover)
    .retry(3)
    .map(|pop| pop.with_elitism(0.1));
```

**Tasks:**
- [ ] Design combinator traits for strategies
- [ ] Implement basic combinators (map, and_then, or_else)
- [ ] Add retry and timeout mechanisms
- [ ] Create examples showing composition

### Step 1.4: Module Restructure
**Goal**: Rename and reorganize for clarity

```
src/
├── parallel/           # New name (was distributed)
│   ├── genotype/      
│   │   ├── mod.rs     # Immutable genotype trait
│   │   ├── binary.rs  # All discrete genotypes
│   │   ├── list.rs
│   │   ├── unique.rs
│   │   ├── range.rs   # Include range here too
│   │   └── multi_*.rs
│   ├── strategy/
│   │   ├── evolve.rs  # Auto-parallelized
│   │   ├── hill_climb.rs
│   │   └── permutate.rs
│   ├── population.rs  # Parallel-first operations
│   └── mod.rs
```

**Tasks:**
- [ ] Rename `distributed` → `parallel`
- [ ] Update all imports and module paths
- [ ] Update documentation to reflect parallel focus
- [ ] Add module-level docs explaining immutability benefits

### Step 1.5: Performance Validation
**Goal**: Ensure immutability doesn't regress performance

**Tasks:**
- [ ] Create comprehensive benchmark suite
- [ ] Compare with current distributed implementation
- [ ] Profile memory usage (Arc overhead)
- [ ] Optimize hot paths if needed

---

## Phase 2: Compute Module Implementation
*Timeline: 2-3 weeks*  
*Base: Current centralized track*

### Step 2.1: Complete Matrix Genotypes
**Goal**: Ensure all matrix genotypes work correctly

```rust
// Centralized already has these:
- dynamic_range.rs  → MatrixRangeGenotype
- static_range.rs   → StaticMatrixRangeGenotype  
- static_binary.rs  → MatrixBinaryGenotype
```

**Tasks:**
- [ ] Rename genotypes to clarify matrix nature
- [ ] Ensure consistent matrix storage (column-major for GPU)
- [ ] Add batch operations for entire populations
- [ ] Implement efficient slicing for subpopulations

### Step 2.2: Make Matrix Genotypes Immutable
**Goal**: Apply immutability lessons from Parallel

```rust
// Current (centralized)
impl Genotype for DynamicRangeGenotype {
    fn mutate_chromosome_genes(&mut self, ...) { }
}

// Target (compute)  
impl Genotype for MatrixRangeGenotype {
    fn mutate_chromosome_genes(&self, ...) { }
}
```

**Tasks:**
- [ ] Remove `&mut self` from all matrix genotype methods
- [ ] Use interior mutability for necessary caches
- [ ] Ensure thread-safety for parallel matrix operations
- [ ] Add tests for concurrent access

### Step 2.3: SIMD Optimization
**Goal**: Leverage CPU vector instructions

```rust
// Add SIMD operations
impl MatrixRangeGenotype {
    #[cfg(target_arch = "x86_64")]
    fn mutate_batch_avx(&self, chromosomes: &mut [Chromosome]) {
        use std::arch::x86_64::*;
        // AVX implementation
    }
    
    fn mutate_batch(&self, chromosomes: &mut [Chromosome]) {
        #[cfg(target_arch = "x86_64")]
        if is_x86_feature_detected!("avx2") {
            return self.mutate_batch_avx(chromosomes);
        }
        // Fallback implementation
    }
}
```

**Tasks:**
- [ ] Identify SIMD opportunities (mutation, crossover, fitness)
- [ ] Implement AVX2 variants for x86_64
- [ ] Add ARM NEON support if applicable
- [ ] Benchmark SIMD vs scalar performance

### Step 2.4: GPU Preparation (Foundation Only)
**Goal**: Structure code for future GPU integration

```rust
// Prepare for GPU without implementing yet
pub trait GpuCompatible {
    fn to_device_memory(&self) -> DeviceBuffer;
    fn from_device_memory(buffer: &DeviceBuffer) -> Self;
}

impl GpuCompatible for MatrixRangeGenotype {
    // Stub implementations for now
}
```

**Tasks:**
- [ ] Design GPU-compatible traits
- [ ] Ensure memory layout is GPU-friendly (aligned, contiguous)
- [ ] Add device memory abstractions (stubs for now)
- [ ] Document GPU integration points

### Step 2.5: Batch Processing APIs
**Goal**: Optimize for throughput over latency

```rust
impl BatchEvolve {
    // Process entire population as matrix
    pub fn evolve_batch(&self, population: MatrixPopulation) -> MatrixPopulation {
        // Matrix operations on entire population
    }
    
    // Streaming for huge populations
    pub fn evolve_streaming(&self, 
        input: impl Stream<Item = Batch>,
    ) -> impl Stream<Item = Batch> {
        // Process in chunks
    }
}
```

**Tasks:**
- [ ] Implement batch mutation operations
- [ ] Implement batch crossover operations
- [ ] Add streaming APIs for huge populations
- [ ] Optimize memory access patterns

### Step 2.6: Module Structure
**Goal**: Organize for compute focus

```
src/
├── compute/            # New name (was centralized)
│   ├── genotype/
│   │   ├── mod.rs     # Matrix-based genotype trait
│   │   ├── matrix_range.rs
│   │   ├── matrix_binary.rs
│   │   └── matrix_sparse.rs  # Future
│   ├── strategy/
│   │   ├── batch_evolve.rs
│   │   ├── batch_hill_climb.rs
│   │   └── streaming.rs
│   ├── simd/          # SIMD implementations
│   │   ├── x86_64.rs
│   │   └── aarch64.rs
│   ├── gpu/           # GPU stubs for future
│   │   └── traits.rs
│   ├── population.rs  # Matrix-based population
│   └── mod.rs
```

**Tasks:**
- [ ] Rename `centralized` → `compute`  
- [ ] Reorganize for compute focus
- [ ] Add module docs explaining use cases
- [ ] Create examples for scientific computing

---

## Phase 3: Classic Module Preservation
*Timeline: 1 week*  
*Base: Current main branch*

### Step 3.1: Import from Main Branch
**Goal**: Preserve simple, educational implementation

**Tasks:**
- [ ] Copy main branch code to `src/classic/`
- [ ] Keep all existing genotypes
- [ ] Maintain mutable, simple implementation
- [ ] No parallelization complexity

### Step 3.2: Clean Up
**Goal**: Remove any distributed/centralized concepts

**Tasks:**
- [ ] Remove any references to tracks
- [ ] Simplify trait bounds where possible
- [ ] Focus on clarity over performance
- [ ] Add educational documentation

---

## Phase 4: Integration & Migration
*Timeline: 1 week*

### Step 4.1: Common Traits
**Goal**: Share core abstractions

```rust
// src/lib.rs
pub trait Genotype {
    type Allele;
    // Minimal shared interface
}

pub mod classic;   // Simple implementation
pub mod parallel;  // Immutable + Rayon
pub mod compute;   // Matrix + SIMD
```

**Tasks:**
- [ ] Define minimal shared traits
- [ ] Ensure all modules implement them
- [ ] Add conversion functions between modules
- [ ] Document migration paths

### Step 4.2: Examples & Documentation
**Goal**: Show when to use each module

**Tasks:**
- [ ] Create example: Migrate from Classic to Parallel
- [ ] Create example: When to use Compute
- [ ] Add decision tree to main README
- [ ] Write migration guide

### Step 4.3: Benchmarks
**Goal**: Validate performance characteristics

**Tasks:**
- [ ] Benchmark Classic vs Parallel vs Compute
- [ ] Test with different problem sizes
- [ ] Measure memory usage
- [ ] Document performance characteristics

---

## Success Criteria

### For Parallel Module:
- [ ] Zero `&mut self` in Genotype trait
- [ ] All operations parallelizable without locks
- [ ] Performance within 10% of current distributed
- [ ] Clean functional API with combinators

### For Compute Module:
- [ ] Matrix operations 2x faster than individual
- [ ] SIMD provides measurable speedup
- [ ] Memory layout GPU-compatible
- [ ] Batch APIs reduce allocation overhead

### Overall:
- [ ] Clear separation of concerns
- [ ] Each module has distinct use case
- [ ] Migration between modules is straightforward
- [ ] No more "centralized vs distributed" confusion

---

## Risk Mitigation

### Risk: Performance Regression from Immutability
**Mitigation**: 
- Profile before/after each change
- Use interior mutability where critical
- Accept some overhead for architectural benefits

### Risk: API Breaking Changes
**Mitigation**:
- Keep Classic module 100% compatible
- Provide migration utilities
- Extensive documentation

### Risk: Complexity Explosion
**Mitigation**:
- Each module stays focused
- Share only minimal traits
- Resist feature creep

---

## Timeline Summary

**Week 1-2**: Parallel module (immutability + parallelization)  
**Week 3-4**: Compute module (matrix + SIMD)  
**Week 5**: Classic module + Integration  
**Week 6**: Documentation + Benchmarks

Total: 6 weeks to transform confusing dual tracks into three purpose-built modules with clear use cases.