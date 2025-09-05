# Genetic Algorithm Library - Refactoring Plan v5: Complete Separation

## Core Philosophy

**Fork, don't unify.** Create two independent, optimized implementations that share terminology and builder patterns but have paradigm-specific internals. Users choose one paradigm and get a consistent, optimized experience.

## The Plan: Fork → Trim → Optimize

### Initial Structure
```
genetic_algorithm/
├── src/
│   ├── distributed/     # Complete fork of current codebase
│   │   └── [entire current codebase copied here]
│   │
│   ├── centralized/     # Complete fork of current codebase  
│   │   └── [entire current codebase copied here]
│   │
│   └── lib.rs           # Just exports both modules
```

No common module. No compatibility layer. Just two complete, independent implementations.

## Phase 1: Fork Everything (Day 1)

Simple mechanical copying:

```bash
# Fork entire codebase
cp -r src/* src/distributed/
cp -r src/* src/centralized/

# Update lib.rs
echo "pub mod distributed;" > src/lib.rs
echo "pub mod centralized;" >> src/lib.rs
```

Both modules now have everything. This compiles and works immediately.

## Phase 2: Trim Distributed Module (Week 1)

### Remove from distributed/
- `genotype/dynamic_matrix.rs`
- `genotype/static_matrix.rs`
- `chromosome/row.rs` (pointer-based)
- All matrix-specific code paths
- ChromosomePointer trait
- Population-wide memory management
- Chromosome recycling in ChromosomeManager

### Simplify in distributed/
```rust
// genotype.rs - Remove matrix-specific methods
trait Genotype {
    type Chromosome: Chromosome;
    type Allele: Allele;
    
    fn genes_size(&self) -> usize;
    fn random_genes<R: Rng>(&self, rng: &mut R) -> Vec<Self::Allele>;
    // Remove: mutate_chromosome_genes, crossover_chromosome_genes
    // Remove: chromosome recycling
}

// chromosome.rs - Chromosomes own their genes
trait Chromosome: Clone + Send {
    type Gene: Clone;
    fn genes(&self) -> &[Self::Gene];
    fn genes_mut(&mut self) -> &mut [Self::Gene];
    fn age(&self) -> usize;
    fn fitness_score(&self) -> Option<FitnessValue>;
}
```

## Phase 3: Trim Centralized Module (Week 1)

### Remove from centralized/
- `genotype/binary.rs`
- `genotype/list.rs`  
- `genotype/unique.rs`
- `genotype/range.rs`
- `genotype/multi_*.rs`
- `genotype/bit.rs`
- `chromosome/vector.rs`
- `chromosome/bit.rs`
- All per-chromosome allocation code
- ChromosomeOwner trait

### Simplify in centralized/
```rust
// genotype.rs - Only matrix operations
trait Genotype: Send + Sync {
    type Allele: RangeAllele;
    
    fn genes_size(&self) -> usize;
    fn population_capacity(&self) -> usize;
    fn data(&self) -> &[Self::Allele];
    fn data_mut(&mut self) -> &mut [Self::Allele];
    fn shape(&self) -> (usize, usize); // (rows, cols)
    
    // Keep: mutate_chromosome_genes (works on indices)
    // Keep: crossover_chromosome_genes (works on indices)
}

// chromosome.rs - Just pointers
#[derive(Copy, Clone)]
struct Chromosome {
    row_id: usize,
    fitness_score: Option<FitnessValue>,
    age: usize,
}
```

## Phase 4: Optimize Distributed (Week 2)

### Make operators work directly on chromosomes

```rust
// distributed/mutate.rs
trait Mutate: Clone + Send + Sync {
    fn mutate<C: Chromosome, R: Rng>(&mut self, chromosome: &mut C, rng: &mut R);
}

// Example implementation - user can now do this!
struct FlipMutate {
    rate: f32,
}

impl Mutate for FlipMutate {
    fn mutate<C: Chromosome<Gene = bool>, R: Rng>(&mut self, chromosome: &mut C, rng: &mut R) {
        // Direct access to genes!
        for gene in chromosome.genes_mut() {
            if rng.gen::<f32>() < self.rate {
                *gene = !*gene;
            }
        }
    }
}

// distributed/crossover.rs  
trait Crossover: Clone + Send + Sync {
    fn crossover<C: Chromosome, R: Rng>(
        &mut self, 
        parent1: &mut C, 
        parent2: &mut C, 
        rng: &mut R
    );
}

// distributed/fitness.rs
trait Fitness: Clone + Send + Sync {
    type Chromosome: Chromosome;
    
    fn calculate_for_chromosome(
        &mut self, 
        chromosome: &Self::Chromosome
    ) -> Option<FitnessValue>;
}
```

### Remove genotype from operator signatures in strategies

```rust
// distributed/strategy/evolve.rs
impl Evolve {
    fn mutate_population(&mut self) {
        for chromosome in &mut self.population.chromosomes {
            if chromosome.is_offspring() {
                self.mutate.mutate(chromosome, &mut self.rng);
            }
        }
    }
    
    fn crossover_population(&mut self) {
        for (parent1, parent2) in self.population.chromosomes.chunks_mut(2) {
            self.crossover.crossover(parent1, parent2, &mut self.rng);
        }
    }
}
```

## Phase 5: Optimize Centralized (Week 2)

### Optimize for bulk operations

```rust
// centralized/fitness.rs - Population-wide calculation
trait Fitness: Clone + Send + Sync {
    type Genotype: Genotype;
    
    fn calculate_for_population(
        &mut self, 
        genotype: &Self::Genotype
    ) -> Vec<FitnessValue>;
}

// Example GPU-ready implementation
struct MatrixFitness;

impl Fitness for MatrixFitness {
    type Genotype = DynamicMatrix<f32>;
    
    fn calculate_for_population(&mut self, genotype: &Self::Genotype) -> Vec<FitnessValue> {
        let (rows, cols) = genotype.shape();
        let data = genotype.data(); // Contiguous memory!
        
        // Can pass directly to SIMD/GPU
        gpu::batch_evaluate(data, rows, cols)
    }
}

// centralized/mutate.rs - Works through genotype
trait Mutate: Clone + Send + Sync {
    fn mutate<G: Genotype, R: Rng>(
        &mut self, 
        genotype: &mut G,
        chromosome_id: usize,
        rng: &mut R
    );
    
    // Optional: batch mutation
    fn mutate_batch<G: Genotype, R: Rng>(
        &mut self,
        genotype: &mut G,
        chromosome_ids: &[usize],
        rng: &mut R
    ) {
        for &id in chromosome_ids {
            self.mutate(genotype, id, rng);
        }
    }
}
```

### Enable SIMD/GPU patterns

```rust
// centralized/strategy/evolve.rs
impl Evolve {
    fn calculate_fitness(&mut self) {
        // Calculate entire population at once!
        let fitness_scores = self.fitness.calculate_for_population(&self.genotype);
        
        // Update chromosomes
        for (chromosome, score) in self.population.chromosomes.iter_mut().zip(fitness_scores) {
            chromosome.fitness_score = Some(score);
        }
    }
    
    fn mutate_population(&mut self) {
        let offspring_ids: Vec<usize> = self.population.chromosomes
            .iter()
            .enumerate()
            .filter(|(_, c)| c.is_offspring())
            .map(|(i, _)| i)
            .collect();
            
        // Could batch this for GPU
        for id in offspring_ids {
            self.mutate.mutate(&mut self.genotype, id, &mut self.rng);
        }
    }
}
```

## Phase 6: Consistent User Experience (Week 3)

### Similar builder APIs across both paradigms

```rust
// distributed/strategy/evolve.rs
Evolve::builder()
    .population_size(100)
    .mutation_rate(0.2)
    .crossover_rate(0.8)
    .selection(Tournament::new(3))
    .max_generations(1000)
    .build()

// centralized/strategy/evolve.rs  
Evolve::builder()
    .population_size(100)     // Same!
    .mutation_rate(0.2)       // Same!
    .crossover_rate(0.8)      // Same!
    .selection(Tournament::new(3))  // Same!
    .max_generations(1000)    // Same!
    .build()
```

### Clear prelude exports

```rust
// distributed/prelude.rs
pub use crate::distributed::{
    genotype::{Binary, List, Unique, Range},
    chromosome::Chromosome,
    fitness::Fitness,
    mutate::{Mutate, FlipMutate, SwapMutate},
    crossover::{Crossover, UniformCrossover, SinglePointCrossover},
    select::{Select, Tournament, Elite},
    strategy::evolve::Evolve,
};

// centralized/prelude.rs
pub use crate::centralized::{
    genotype::{DynamicMatrix, StaticMatrix},
    chromosome::Chromosome,  // Different type, same name!
    fitness::Fitness,        // Different trait, same name!
    mutate::{Mutate, RangeMutate, BatchMutate},
    crossover::{Crossover, UniformCrossover, BatchCrossover},
    select::{Select, Tournament, Elite},  // Can share selection
    strategy::evolve::Evolve,
};
```

## User Experience

### Distributed User
```rust
use genetic_algorithm::distributed::prelude::*;

// Define custom gene type
#[derive(Clone, Debug)]
struct MyGene { value: f64, metadata: String }

// Create genotype
let genotype = ListGenotype::builder()
    .genes_size(100)
    .gene_factory(|| MyGene::random())
    .build();

// Custom operator - EASY!
struct MyMutate;
impl Mutate for MyMutate {
    fn mutate<C: Chromosome<Gene = MyGene>, R: Rng>(&mut self, chr: &mut C, rng: &mut R) {
        // Direct access to genes
        let gene = &mut chr.genes_mut()[rng.gen_range(0..chr.genes().len())];
        gene.value *= rng.gen_range(0.8..1.2);
    }
}

// Custom fitness
struct MyFitness;
impl Fitness for MyFitness {
    type Chromosome = ListChromosome<MyGene>;
    
    fn calculate_for_chromosome(&mut self, chr: &Self::Chromosome) -> Option<FitnessValue> {
        Some(chr.genes().iter().map(|g| g.value).sum())
    }
}

// Run
let evolve = Evolve::builder()
    .genotype(genotype)
    .mutate(MyMutate)
    .fitness(MyFitness)
    .build();
```

### Centralized User
```rust
use genetic_algorithm::centralized::prelude::*;

// Create matrix genotype
let genotype = DynamicMatrix::<f32>::builder()
    .genes_size(100)
    .population_capacity(1000)
    .allele_range(0.0..=1.0)
    .build();

// Custom fitness - calculates whole population
struct MyFitness;
impl Fitness for MyFitness {
    type Genotype = DynamicMatrix<f32>;
    
    fn calculate_for_population(&mut self, genotype: &Self::Genotype) -> Vec<FitnessValue> {
        let (rows, cols) = genotype.shape();
        let data = genotype.data();
        
        // Process entire population at once
        // Perfect for SIMD/GPU operations
        (0..rows).map(|r| {
            let row_start = r * cols;
            let row_end = row_start + cols;
            data[row_start..row_end].iter().sum()
        }).collect()
    }
}

// Run
let evolve = Evolve::builder()
    .genotype(genotype)
    .mutate(RangeMutate::new(0.2))
    .fitness(MyFitness)
    .population_size(1000)  // Large populations are efficient!
    .build();
```

### Switching Paradigms

User wants to switch from distributed to centralized:

```rust
// Before (distributed)
use genetic_algorithm::distributed::prelude::*;

struct MyFitness;
impl Fitness for MyFitness {
    type Chromosome = BinaryChromosome;
    fn calculate_for_chromosome(&mut self, chr: &Self::Chromosome) -> Option<FitnessValue> {
        Some(chr.genes().iter().filter(|&&g| g).count() as f64)
    }
}

// After (centralized)
use genetic_algorithm::centralized::prelude::*;

struct MyFitness;
impl Fitness for MyFitness {
    type Genotype = DynamicMatrix<u8>;  // Using 0/1 instead of bool
    fn calculate_for_population(&mut self, genotype: &Self::Genotype) -> Vec<FitnessValue> {
        let (rows, cols) = genotype.shape();
        let data = genotype.data();
        (0..rows).map(|r| {
            let row_start = r * cols;
            let row_end = row_start + cols;
            data[row_start..row_end].iter().filter(|&&g| g == 1).count() as f64
        }).collect()
    }
}

// Builders remain almost identical
let evolve = Evolve::builder()
    .population_size(100)
    .mutation_rate(0.2)
    .build();
```

## Benefits of Complete Separation

1. **Maximum Optimization**: Each paradigm optimized without compromise
2. **Clear Mental Model**: Pick one paradigm, everything is consistent
3. **True Extensibility**: Distributed users get direct gene access
4. **Performance**: Centralized gets SIMD/GPU without overhead
5. **Simplicity**: No enum dispatch, no runtime checks, no compatibility layers
6. **Type Safety**: Incompatible combinations impossible
7. **Clean Code**: Each module is focused and coherent

## Timeline

- **Day 1**: Fork codebase
- **Week 1**: Trim both modules
- **Week 2**: Optimize each paradigm
- **Week 3**: Polish user experience
- **Week 4**: Documentation and examples
- **Month 2**: Beta release and feedback
- **Month 3**: 1.0 release

## Post-1.0 Considerations

Only after everything works perfectly separated:

1. **Maybe extract truly common code** (FitnessValue, FitnessOrdering)
2. **Maybe share selection algorithms** (they only need fitness scores)
3. **Maybe create common benchmark suite**

But these are afterthoughts. The separation comes first.

## Conclusion

By completely separating the paradigms:
- Users get exactly what they need, optimized for their use case
- The library becomes simpler, not more complex
- Each paradigm can evolve independently
- True extensibility becomes possible
- Performance optimization has no constraints

The key insight: **These paradigms are fundamentally different. Embrace it.**