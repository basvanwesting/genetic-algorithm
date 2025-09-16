# Complete Chromosome Recycling Analysis - Main Branch

## ChromosomeManager Trait Definition
- **Location**: `src/chromosome.rs:67-127`
- **Key Methods**:
  - `chromosome_bin_push()`: Push chromosome to recycling bin
  - `chromosome_bin_find_or_create()`: Get from bin or create new
  - `chromosome_destructor()`: Wrapper that calls `chromosome_bin_push()`
  - `chromosome_destructor_truncate()`: Truncate and recycle multiple chromosomes
  - `chromosome_cloner()`: Clone using recycled chromosome if available
  - `chromosome_cloner_expand()`: Expand population using recycling

## Implementations of ChromosomeManager

Each genotype implements ChromosomeManager with its own `chromosome_bin`:

1. **BinaryGenotype** - `src/genotype/binary.rs:288`
2. **BitGenotype** - `src/genotype/bit.rs:369`
3. **ListGenotype** - `src/genotype/list.rs:332`
4. **MultiListGenotype** - `src/genotype/multi_list.rs:384`
5. **UniqueGenotype** - `src/genotype/unique.rs:272`
6. **MultiUniqueGenotype** - `src/genotype/multi_unique.rs:408`
7. **RangeGenotype** - `src/genotype/range.rs:701`
8. **MultiRangeGenotype** - `src/genotype/multi_range.rs:780`
9. **DynamicMatrixGenotype** - `src/genotype/dynamic_matrix.rs:688`
10. **StaticMatrixGenotype** - `src/genotype/static_matrix.rs:682`

## CRITICAL: Parent/Offspring Split Pattern in Selection

Both Elite and Tournament selection follow the same pattern where chromosomes are temporarily moved out of the population:

```rust
// Lines 34-38 in Elite.rs and 36-40 in Tournament.rs
let (mut offspring, mut parents): (Vec<G::Chromosome>, Vec<G::Chromosome>) = state
    .population
    .chromosomes
    .drain(..)  // Population is now EMPTY
    .partition(|c| c.is_offspring());

// Lines 47-48: Truncate parents and offspring SEPARATELY
self.selection(&mut parents, new_parents_size, genotype, config, rng);
self.selection(&mut offspring, new_offspring_size, genotype, config, rng);

// Lines 50-52: Reassemble population
state.population.chromosomes.append(&mut elite_chromosomes);
state.population.chromosomes.append(&mut offspring);
state.population.chromosomes.append(&mut parents);

// Lines 54-60: Final truncation
self.selection(
    &mut state.population.chromosomes,
    config.target_population_size,
    genotype,
    config,
    rng,
);
```

### The `selection()` method (lines 74-98) does the recycling:
```rust
pub fn selection<G: EvolveGenotype, R: Rng>(
    &self,
    chromosomes: &mut Vec<G::Chromosome>,  // Detached vector!
    selection_size: usize,
    genotype: &mut G,
    config: &EvolveConfig,
    _rng: &mut R,
) {
    // ... sorting logic ...
    genotype.chromosome_destructor_truncate(chromosomes, selection_size);  // RECYCLING HERE
}
```

## Complete List: Where Chromosomes Are Recycled to Bin

### 1. Selection Phase - WITH DETACHED VECTORS
- **Elite Selection** (`src/select/elite.rs`)
  - Line 47: `self.selection(&mut parents, ...)` - truncates detached parent vector
  - Line 48: `self.selection(&mut offspring, ...)` - truncates detached offspring vector  
  - Line 54-60: Final selection on reassembled population
  - Each calls `genotype.chromosome_destructor_truncate()` at line 97
  
- **Tournament Selection** (`src/select/tournament.rs`)
  - Line 49: `self.selection(&mut parents, ...)` - processes detached parent vector
  - Line 50: `self.selection(&mut offspring, ...)` - processes detached offspring vector
  - Line 56-62: Final selection on reassembled population
  - Line 139: `genotype.chromosome_destructor_truncate(chromosomes, 0)` clears all losing chromosomes

### 2. Crossover Phase
- **Rejuvenate Crossover** (`src/crossover/rejuvenate.rs:32`)
  - Calls `genotype.chromosome_destructor_truncate()` to drop non-selected parents

### 3. Extension Phase
- **Mass Genesis** (`src/extension/mass_genesis.rs:48`)
  - Calls `genotype.chromosome_destructor_truncate()` to clear entire population when triggered

- **Mass Extinction** (`src/extension/mass_extinction.rs:58`)
  - Calls `genotype.chromosome_destructor_truncate()` to remove portion of population

- **Mass Deduplication** (`src/extension/mass_deduplication.rs:44`)
  - Calls `genotype.chromosome_destructor_truncate()` when unique chromosomes drop below threshold

### 4. Strategy-Level Operations
- **Evolve Strategy** (`src/strategy/evolve.rs:605`)
  - Calls `genotype.chromosome_destructor()` for individual chromosomes exceeding max age

- **Hill Climb Strategy** (`src/strategy/hill_climb.rs:231`)
  - Calls `genotype.chromosome_destructor_truncate()` to clear population before generating neighbors

## Places Where Recycled Chromosomes Are Reused

### 1. Crossover Phase (Primary Consumer)
All crossover implementations use `genotype.chromosome_cloner_expand()` which internally uses recycled chromosomes:

- **Uniform** (`src/crossover/uniform.rs:39`)
- **SinglePoint** (`src/crossover/single_point.rs:36`)
- **MultiPoint** (`src/crossover/multi_point.rs:40`)
- **SingleGene** (`src/crossover/single_gene.rs:37`)
- **MultiGene** (`src/crossover/multi_gene.rs:40`)
- **Clone** (`src/crossover/clone.rs:29`)
- **Rejuvenate** (`src/crossover/rejuvenate.rs:37`)

### 2. Population Creation
- `chromosome_constructor_genes()` and `chromosome_constructor_random()` use `chromosome_bin_find_or_create()`

## Critical Insight: Memory Leak Risk

The main branch handles recycling for DETACHED vectors (chromosomes temporarily outside the population):
1. Population is drained into separate parent/offspring vectors
2. These vectors are truncated SEPARATELY using `genotype.chromosome_destructor_truncate()`
3. Without this, truncated chromosomes from detached vectors would be lost (memory leak)

This is why the main branch passes `genotype` to the selection methods - to enable recycling of detached chromosomes!