# Population-Based Chromosome Recycling Implementation Plan

## Analysis: Centralized vs Distributed Recycling Approaches

### Centralized Track (Current)
- **Where**: ChromosomeManager trait on Genotype
- **Storage**: Genotype owns both data (flat Vec) and recycling bin
- **Chromosome**: Just a pointer (row_id) to genes location
- **Lifecycle**: Genotype manages entire chromosome lifecycle

### Distributed Track (Target)
- **Where**: Population struct manages recycling
- **Storage**: Chromosomes own their genes directly
- **Chromosome**: Self-contained with Vec<Allele>
- **Lifecycle**: Population manages chromosome recycling

## Key Design Decision

**Population-owned recycling** is the right approach for distributed track because:
1. Chromosomes own their genes (no central storage)
2. Population already manages chromosome lifecycle
3. Keeps Genotype immutable (architectural goal)
4. Localizes recycling to where truncation/expansion happens

## Detailed Implementation Plan

### Phase 1: Add Recycling Infrastructure to Population

```rust
// src/distributed/population.rs
#[derive(Clone, Debug)]
pub struct Population<T: Allele> {
    pub chromosomes: Vec<Chromosome<T>>,
    recycled: Vec<Chromosome<T>>,  // NEW: recycling bin
    recycling_enabled: bool,       // NEW: opt-in flag
    recycling_threshold: usize,    // NEW: minimum genes size to enable
}

impl<T: Allele> Population<T> {
    pub fn new_with_recycling(
        chromosomes: Vec<Chromosome<T>>, 
        genes_size: usize
    ) -> Self {
        let recycling_enabled = genes_size >= 1000; // Auto-enable for large chromosomes
        Self {
            chromosomes,
            recycled: Vec::new(),
            recycling_enabled,
            recycling_threshold: 1000,
        }
    }
    
    pub fn with_recycling(mut self, enabled: bool) -> Self {
        self.recycling_enabled = enabled;
        self
    }
}
```

### Phase 2: Replace Destructive Operations

#### Current truncate (destroys chromosomes):
```rust
// src/distributed/select/elite.rs:94
chromosomes.truncate(selection_size);
```

#### New truncate_with_recycling:
```rust
impl<T: Allele> Population<T> {
    pub fn truncate_with_recycling(&mut self, keep_size: usize) {
        if !self.recycling_enabled {
            self.chromosomes.truncate(keep_size);
            return;
        }
        
        // Move excess to recycling bin instead of dropping
        while self.chromosomes.len() > keep_size {
            if let Some(mut chromosome) = self.chromosomes.pop() {
                // Clear transient state but keep genes allocation
                chromosome.fitness_score = None;
                chromosome.age = 0;
                // genes_hash will be recalculated when genes are set
                self.recycled.push(chromosome);
            }
        }
    }
}
```

### Phase 3: Replace Clone Operations

#### Current expand (always clones):
```rust
// src/distributed/crossover.rs:56-66
fn expand_chromosome_population<T: Allele>(
    &self,
    chromosomes: &mut Vec<Chromosome<T>>,
    amount: usize,
) {
    let modulo = chromosomes.len();
    for i in 0..amount {
        let chromosome = chromosomes[i % modulo].clone();
        chromosomes.push(chromosome);
    }
}
```

#### New expand_with_recycling:
```rust
impl<T: Allele> Population<T> {
    pub fn expand_with_recycling(&mut self, amount: usize) {
        if !self.recycling_enabled {
            // Fall back to standard cloning
            let modulo = self.chromosomes.len();
            for i in 0..amount {
                let chromosome = self.chromosomes[i % modulo].clone();
                self.chromosomes.push(chromosome);
            }
            return;
        }
        
        let modulo = self.chromosomes.len();
        for i in 0..amount {
            let source = &self.chromosomes[i % modulo];
            
            // Try to reuse from recycling bin first
            let mut chromosome = if let Some(mut recycled) = self.recycled.pop() {
                // Reuse allocation, just copy genes
                recycled.genes.clear();
                recycled.genes.extend_from_slice(&source.genes);
                recycled
            } else {
                // No recycled available, clone normally
                source.clone()
            };
            
            // Reset state for new chromosome
            chromosome.fitness_score = None;
            chromosome.genes_hash = None;  // Will be recalculated
            chromosome.age = 0;
            
            self.chromosomes.push(chromosome);
        }
    }
}
```

### Phase 4: Update Select Implementations

```rust
// src/distributed/select/elite.rs
impl Select for Elite {
    fn call<G: EvolveGenotype, R: Rng, SR: StrategyReporter<Genotype = G>>(
        &mut self,
        _genotype: &G,
        state: &mut EvolveState<G>,
        config: &EvolveConfig,
        _reporter: &mut SR,
        _rng: &mut R,
    ) {
        // ... existing code ...
        
        // OLD: chromosomes.truncate(selection_size);
        // NEW:
        state.population.truncate_with_recycling(config.target_population_size);
        
        // ... rest of method
    }
}
```

### Phase 5: Update Crossover Implementations

```rust
// src/distributed/crossover/uniform.rs (and others)
impl Crossover for Uniform {
    fn call<G: EvolveGenotype, R: Rng, SR: StrategyReporter<Genotype = G>>(
        &mut self,
        genotype: &G,
        state: &mut EvolveState<G>,
        _config: &EvolveConfig,
        _reporter: &mut SR,
        rng: &mut R,
    ) {
        // ... existing code ...
        
        // OLD: self.expand_chromosome_population(&mut state.population.chromosomes, selected_population_size);
        // NEW:
        state.population.expand_with_recycling(selected_population_size);
        
        // ... rest of method
    }
}
```

### Phase 6: Memory Management & Monitoring

```rust
impl<T: Allele> Population<T> {
    /// Get current recycling statistics
    pub fn recycling_stats(&self) -> RecyclingStats {
        RecyclingStats {
            active: self.chromosomes.len(),
            recycled: self.recycled.len(),
            total_allocated: self.chromosomes.len() + self.recycled.len(),
            recycling_enabled: self.recycling_enabled,
        }
    }
    
    /// Shrink recycling pool if it grows too large
    pub fn compact_recycling_pool(&mut self, max_recycled: usize) {
        if self.recycled.len() > max_recycled {
            self.recycled.truncate(max_recycled);
        }
    }
    
    /// Clear recycling pool to free memory
    pub fn clear_recycling_pool(&mut self) {
        self.recycled.clear();
    }
}
```

### Phase 7: Configuration & Builder Support

```rust
// src/distributed/strategy/evolve/builder.rs
impl<G: EvolveGenotype, M: Mutate, C: Crossover, S: Select, E: Extension, SR: StrategyReporter<Genotype = G>>
    Builder<G, M, C, S, E, SR>
{
    pub fn with_chromosome_recycling(mut self, enabled: bool) -> Self {
        self.chromosome_recycling = Some(enabled);
        self
    }
    
    pub fn with_recycling_threshold(mut self, min_genes_size: usize) -> Self {
        self.recycling_threshold = Some(min_genes_size);
        self
    }
}
```

## Migration Strategy

### Step 1: Add Infrastructure (Non-Breaking)
- Add recycled field to Population (defaults to empty)
- Add new methods alongside existing ones
- No behavior changes yet

### Step 2: Gradual Adoption
- Update Select/Crossover traits to use new methods
- Keep recycling disabled by default
- Test with examples/benchmarks

### Step 3: Performance Validation
- Benchmark with evolve_scrabble example (1000+ genes)
- Compare memory usage and runtime
- Tune recycling_threshold

### Step 4: Enable Selectively
- Auto-enable for genes_size > 1000
- Document in examples
- Add to performance guide

## Key Differences from Centralized Approach

| Aspect | Centralized | Distributed (Proposed) |
|--------|------------|----------------------|
| Owner | Genotype (ChromosomeManager) | Population |
| Storage | Flat Vec with row_id | Direct chromosome ownership |
| Complexity | High (trait implementation) | Low (simple methods) |
| Flexibility | Per-genotype control | Per-population control |
| Memory | Always allocated | Can be freed via clear_recycling_pool |

## Memory Contiguity Benefits

When chromosomes are allocated upfront (either initially or from recycling pool), there's an additional performance benefit from **memory contiguity**:

### Initial Allocation Pattern
```rust
// When creating population initially:
let mut chromosomes = Vec::with_capacity(1000);
for _ in 0..1000 {
    chromosomes.push(Chromosome::new(genes));
}
```

This creates:
1. **Contiguous chromosome structs** - All Chromosome metadata (fitness_score, age, genes_hash) in sequential memory
2. **Clustered gene allocations** - Gene vectors likely allocated in nearby heap regions
3. **Better cache locality** - CPU can efficiently prefetch neighboring chromosomes
4. **Fewer TLB misses** - Accessing sequential memory uses fewer page table entries

### How Recycling Preserves Contiguity
Without recycling:
- Generation 1: Contiguous allocation ✅
- Generation 2: Drop 500, allocate 500 new (fragmented) ❌
- Generation 3: More fragmentation ❌
- Generation N: Highly fragmented memory layout ❌

With recycling:
- Generation 1: Contiguous allocation ✅
- Generation 2: Reuse same memory locations ✅
- Generation 3: Still using original memory ✅
- Generation N: Maintains initial locality ✅

### Cache Line Benefits
Modern CPUs load 64-byte cache lines. A Chromosome struct is ~40 bytes:
- `genes`: 24 bytes (Vec pointer/len/capacity)
- `fitness_score`: 9 bytes (Option<isize>)
- `genes_hash`: 9 bytes (Option<u64>)  
- `age`: 8 bytes (usize)

So ~1.5 chromosomes fit per cache line. When iterating through population:
- **Without recycling**: Random memory access after fragmentation
- **With recycling**: Sequential access through original layout

### Caveat: Genes Still Heap-Allocated
Important: The genes themselves (Vec<T> contents) are still separate heap allocations. However:
- Chromosome metadata benefits from contiguity (used in sorting, filtering)
- Gene vectors allocated together tend to cluster in memory
- Recycling preserves whatever locality exists

## Performance Expectations

Based on the Fitness mutability analysis (1.5x improvement) plus memory contiguity benefits:
- **Small genes (<100)**: Negligible impact, keep disabled
- **Medium genes (100-1000)**: 5-10% improvement (cache benefits)
- **Large genes (>1000)**: 10-20% improvement (allocation + cache)
- **Very large (>10000)**: 20-35% improvement (significant allocation savings)

## Risk Mitigation

1. **Opt-in by default**: No breaking changes
2. **Auto-threshold**: Only enable where beneficial
3. **Monitoring**: RecyclingStats for debugging
4. **Escape hatch**: clear_recycling_pool() if memory grows
5. **Compatibility**: Works with all existing code

## Conclusion

This implementation plan brings the memory efficiency benefits of centralized recycling to the distributed track while:
- Maintaining the simpler distributed architecture
- Keeping Genotype immutable
- Providing opt-in adoption
- Localizing complexity to Population where it belongs

The approach is pragmatic, following the same philosophy as Fitness mutability: optimize where it matters, keep it simple elsewhere.