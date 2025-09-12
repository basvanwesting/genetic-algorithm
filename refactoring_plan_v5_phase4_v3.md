# Phase 3: Lighten Genotype Trait - Simplified Plan

## Goal
Enable flexible Mutate & Crossover strategies in the distributed track by giving them direct access to chromosome genes, while keeping genotype constraints simple and clear.

## The Problem
Currently, all mutations and crossovers must go through genotype methods like `mutate_chromosome_genes()`. This prevents custom strategies from implementing domain-specific logic or using additional context.

## The Solution
Keep Genotype's performance optimizations (like cached Distributions) but remove its control over mutation/crossover logic. Let strategies access genes directly while still benefiting from genotype's cached state.

## Implementation

### Step 1: Simplify Genotype Trait

```rust
// Before: Heavy trait with many responsibilities
pub trait Genotype:
    Clone + Send + Sync + fmt::Debug + fmt::Display + TryFrom<GenotypeBuilder<Self>>
{
    fn mutate_chromosome_genes(&mut self, ...);  // Controls HOW to mutate
    fn crossover_chromosome_genes(&mut self, ...); // Controls HOW to crossover
    // ... many more methods
}

// After: Keep convenient method but expose building blocks
pub trait Genotype: Send + Sync {
    type Allele: Allele;
    
    // Core properties
    fn genes_size(&self) -> usize;
    
    // Performance optimizations (cached distributions)
    fn gene_index_sampler(&self) -> &Uniform<usize>;
    
    // Building blocks for custom mutations (exposed internals)
    fn mutate_gene_at(&self, genes: &mut [Self::Allele], index: usize, rng: &mut impl Rng);
    fn random_allele_at(&self, index: usize, rng: &mut impl Rng) -> Self::Allele;
    
    // Keep the convenient all-in-one method (but not &mut self!)
    fn mutate_chromosome_genes(
        &self,  // Note: &self, not &mut self
        chromosome: &mut Chromosome<Self::Allele>,
        mutation_count: usize,
        allow_duplicates: bool, 
        rng: &mut impl Rng
    ) {
        // Default implementation using building blocks
        if allow_duplicates {
            let sampler = self.gene_index_sampler();
            for _ in 0..mutation_count {
                let i = rng.sample(sampler);
                self.mutate_gene_at(&mut chromosome.genes, i, rng);
            }
        } else {
            let indices = rand::seq::index::sample(rng, self.genes_size(), mutation_count);
            for i in indices {
                self.mutate_gene_at(&mut chromosome.genes, i, rng);
            }
        }
        chromosome.refresh_metadata();
    }
    
    fn random_genes(&self, rng: &mut impl Rng) -> Vec<Self::Allele> {
        (0..self.genes_size())
            .map(|i| self.random_allele_at(i, rng))
            .collect()
    }
    
    // Constraints
    fn allows_crossover_points(&self) -> bool { true }
    fn allows_duplicate_genes(&self) -> bool { true }
}
```

### Step 2: Three Levels of Flexibility

```rust
// Level 1: Use the convenient built-in method
impl<T: Allele> Mutate<T> for StandardMultiGene {
    fn mutate(&self, chromosome: &mut Chromosome<T>, genotype: &impl Genotype<Allele=T>, rng: &mut impl Rng) {
        // Simple: just call the built-in method
        genotype.mutate_chromosome_genes(chromosome, self.count, self.allow_duplicates, rng);
    }
}

// Level 2: Use building blocks for custom patterns
impl<T: Allele> Mutate<T> for AdaptiveMutation {
    fn mutate(&self, chromosome: &mut Chromosome<T>, genotype: &impl Genotype<Allele=T>, rng: &mut impl Rng) {
        // Custom pattern: mutate more genes based on age
        let mutation_count = self.base_count * (1 + chromosome.age / 10);
        
        for _ in 0..mutation_count {
            let i = rng.sample(genotype.gene_index_sampler());
            genotype.mutate_gene_at(&mut chromosome.genes, i, rng);
        }
        chromosome.refresh_metadata();
    }
}

// Level 3: Direct gene manipulation when needed
impl<T: Allele> Mutate<T> for LocalSearchMutation {
    fn mutate(&self, chromosome: &mut Chromosome<T>, genotype: &impl Genotype<Allele=T>, rng: &mut impl Rng) {
        // Direct access for domain-specific logic
        let start = rng.gen_range(0..chromosome.genes.len() - 5);
        
        // Mutate a window of genes
        for i in start..start+5 {
            if rng.gen_bool(0.3) {
                // Can choose: use genotype's mutation or replacement
                if self.use_replacement {
                    chromosome.genes[i] = genotype.random_allele_at(i, rng);
                } else {
                    genotype.mutate_gene_at(&mut chromosome.genes, i, rng);
                }
            }
        }
        chromosome.refresh_metadata();
    }
}
```

### Step 3: Constraint-Specific Genotypes

Each genotype implements its specific mutation knowledge:

```rust
// BinaryGenotype: Knows to flip bits
impl Genotype for BinaryGenotype {
    type Allele = bool;
    
    fn mutate_gene_at(&self, genes: &mut [bool], index: usize, _rng: &mut impl Rng) {
        genes[index] = !genes[index]; // Flip the bit
    }
    
    fn random_allele_at(&self, _index: usize, rng: &mut impl Rng) -> bool {
        rng.gen()  // Index doesn't matter for binary
    }
}

// RangeGenotype: Knows to stay within bounds
impl Genotype for RangeGenotype<f32> {
    type Allele = f32;
    
    fn mutate_gene_at(&self, genes: &mut [f32], index: usize, rng: &mut impl Rng) {
        // Generate new value within the allowed range
        genes[index] = rng.sample(&self.range_samplers[index]);
    }
    
    fn random_allele_at(&self, index: usize, rng: &mut impl Rng) -> f32 {
        rng.sample(&self.range_samplers[index])  // Index matters! Different range per position
    }
}

// UniqueGenotype: Knows to swap to maintain uniqueness
impl<T: Allele> Genotype for UniqueGenotype<T> {
    type Allele = T;
    
    fn mutate_gene_at(&self, genes: &mut [T], index: usize, rng: &mut impl Rng) {
        // Swap with another position to maintain uniqueness
        let other = rng.gen_range(0..genes.len());
        genes.swap(index, other);
    }
    
    fn allows_crossover_points(&self) -> bool { 
        false // Only index swapping allowed
    }
    
    fn random_genes(&self, rng: &mut impl Rng) -> Vec<T> {
        // Shuffle to ensure uniqueness
        let mut genes = self.alleles.clone();
        genes.shuffle(rng);
        genes
    }
}

// ListGenotype: Knows the allowed values
impl<T: Allele> Genotype for ListGenotype<T> {
    type Allele = T;
    
    fn mutate_gene_at(&self, genes: &mut [T], index: usize, rng: &mut impl Rng) {
        // Pick another value from the allowed list
        genes[index] = self.alleles[rng.gen_range(0..self.alleles.len())].clone();
    }
    
    fn random_allele_at(&self, _index: usize, rng: &mut impl Rng) -> T {
        self.alleles[rng.gen_range(0..self.alleles.len())].clone()  // Index doesn't matter for list
    }
}
```

### Step 4: Handling Complex Genotypes (MultiUnique, MultiList)

Complex genotypes can expose their internal building blocks for custom implementations:

```rust
// MultiUniqueGenotype: Expose internal structure for custom mutations
impl MultiUniqueGenotype<T> {
    // Building blocks exposed as public methods
    pub fn subset_boundaries(&self) -> &[Range<usize>] {
        &self.subset_boundaries
    }
    
    pub fn subset_for_index(&self, index: usize) -> Range<usize> {
        // Find which subset this index belongs to
        self.subset_boundaries[self.subset_index(index)].clone()
    }
    
    // Override the default to handle swapping
    fn mutate_chromosome_genes(
        &self,
        chromosome: &mut Chromosome<T>,
        mutation_count: usize,
        allow_duplicates: bool,
        rng: &mut impl Rng
    ) {
        for _ in 0..mutation_count {
            let subset_idx = rng.gen_range(0..self.subset_boundaries.len());
            let subset = &self.subset_boundaries[subset_idx];
            let i = rng.gen_range(subset.start..subset.end);
            let j = rng.gen_range(subset.start..subset.end);
            chromosome.genes.swap(i, j);
        }
        chromosome.refresh_metadata();
    }
}

// MultiListGenotype: Expose weighted sampling
impl MultiListGenotype<T> {
    // Building blocks exposed
    pub fn allele_lists(&self) -> &[Vec<T>] {
        &self.allele_lists
    }
    
    pub fn allele_list_sizes(&self) -> &[usize] {
        &self.allele_list_sizes
    }
    
    pub fn weighted_gene_sampler(&self) -> &WeightedIndex<f64> {
        &self.weighted_gene_sampler
    }
    
    // Override to use weighted sampling
    fn mutate_chromosome_genes(
        &self,
        chromosome: &mut Chromosome<T>,
        mutation_count: usize,
        allow_duplicates: bool,
        rng: &mut impl Rng
    ) {
        if allow_duplicates {
            for _ in 0..mutation_count {
                let i = self.weighted_gene_sampler.sample(rng);
                let allele_list = &self.allele_lists[i];
                chromosome.genes[i] = allele_list[rng.gen_range(0..allele_list.len())].clone();
            }
        } else {
            // Use weighted sampling without replacement
            let indices = rand::seq::index::sample_weighted(
                rng, self.genes_size(), 
                |i| self.allele_list_sizes[i] as f64,
                mutation_count
            ).unwrap();
            
            for i in indices {
                let allele_list = &self.allele_lists[i];
                chromosome.genes[i] = allele_list[rng.gen_range(0..allele_list.len())].clone();
            }
        }
        chromosome.refresh_metadata();
    }
}

// Custom strategy can use exposed internals when needed
impl<T: Allele> Mutate<T> for CustomMultiUniqueMutation {
    fn mutate(&self, chromosome: &mut Chromosome<T>, genotype: &MultiUniqueGenotype<T>, rng: &mut impl Rng) {
        // Direct access to subset information
        let subsets = genotype.subset_boundaries();
        
        // Custom logic: mutate each subset differently
        for (idx, subset) in subsets.iter().enumerate() {
            if idx % 2 == 0 {
                // Even subsets: more aggressive mutation
                let i = rng.gen_range(subset.start..subset.end);
                let j = rng.gen_range(subset.start..subset.end);
                chromosome.genes.swap(i, j);
            }
        }
        chromosome.refresh_metadata();
    }
}
```

This balanced approach provides:
- **Convenience**: Built-in `mutate_chromosome_genes()` for standard use cases
- **Building blocks**: `mutate_gene_at()`, `random_allele_at()`, samplers for custom patterns
- **Full access**: Direct gene manipulation and exposed internals when needed
- **No forced abstractions**: Complex genotypes can expose what makes sense for them
- **Backward compatibility**: Existing code using `mutate_chromosome_genes()` still works

### Step 5: Custom Strategy Examples

With direct access, we can now implement sophisticated strategies:

```rust
// Domain-specific mutation for TSP
impl Mutate<City> for TwoOptMutation {
    fn mutate(&self, chromosome: &mut Chromosome<City>, _genotype: &impl Genotype<Allele=City>, rng: &mut impl Rng) {
        // Direct access to implement 2-opt local search
        let i = rng.gen_range(0..chromosome.genes.len());
        let j = rng.gen_range(0..chromosome.genes.len());
        chromosome.genes[i..j].reverse();
        chromosome.refresh_metadata();
    }
}

// Adaptive mutation: Control the pattern, let genotype handle the how
impl<T: Allele> Mutate<T> for AdaptiveMutation {
    fn mutate(&self, chromosome: &mut Chromosome<T>, genotype: &impl Genotype<Allele=T>, rng: &mut impl Rng) {
        // Strategy decides mutation intensity based on fitness history
        let stagnation = chromosome.age; // Direct access to metadata
        let mutation_count = (self.base_rate * stagnation as f32) as usize;
        
        // Use cached distribution for performance
        let sampler = genotype.gene_index_sampler();
        
        for _ in 0..mutation_count {
            let i = rng.sample(sampler);
            // Let genotype handle the actual mutation
            genotype.mutate_gene_at(&mut chromosome.genes, i, rng);
        }
        chromosome.refresh_metadata();
    }
}

// Custom mutation that still respects genotype constraints
impl<T: Allele> Mutate<T> for LocalSearchMutation {
    fn mutate(&self, chromosome: &mut Chromosome<T>, genotype: &impl Genotype<Allele=T>, rng: &mut impl Rng) {
        // Custom logic: mutate genes near each other
        let start = rng.gen_range(0..chromosome.genes.len());
        let window = 5.min(chromosome.genes.len() - start);
        
        for i in start..start+window {
            if rng.gen_bool(0.3) {
                // Use genotype's knowledge for valid mutations
                genotype.mutate_gene_at(&mut chromosome.genes, i, rng);
            }
        }
        chromosome.refresh_metadata();
    }
}

// Machine learning guided crossover
impl Crossover<f32> for NeuralCrossover {
    fn crossover(&self, father: &mut Chromosome<f32>, mother: &mut Chromosome<f32>, _genotype: &impl Genotype<Allele=f32>, model: &NeuralNet) {
        // Use ML model to decide crossover points
        let weights = model.predict_importance(&father.genes, &mother.genes);
        
        for (i, &weight) in weights.iter().enumerate() {
            if weight > 0.5 {
                std::mem::swap(&mut father.genes[i], &mut mother.genes[i]);
            }
        }
        
        father.refresh_metadata();
        mother.refresh_metadata();
    }
}
```

## Benefits

1. **Lighter Genotype**: Remove Clone, Debug, Display requirements; keep useful state
2. **No Strategy Specialization**: One Mutate implementation works for all genotypes including complex ones
3. **Flexible Patterns**: Strategies control mutation patterns while genotypes handle validity
4. **Performance Maintained**: Keep cached distributions for hot paths
5. **Direct Access**: Strategies can access genes directly when needed for custom logic
6. **Complex Genotypes Supported**: MultiUnique/MultiList work through mutation scope API
7. **Cleaner Architecture**: Clear separation between knowledge (genotype) and control (strategy)

## Migration Path

### Phase 1: Add New Trait (Non-breaking)
1. Define simplified `Genotype` trait
2. Add adapter for existing genotypes
3. Test with one strategy

### Phase 2: Update Strategies (Incremental)
1. Update `Mutate` and `Crossover` traits to use direct access
2. Migrate one strategy at a time
3. Keep backwards compatibility layer

### Phase 3: Migrate Genotypes (Per-genotype)
1. Start with `BinaryGenotype` (simplest)
2. Keep cached distributions (they're performance optimizations)
3. Remove heavy trait bounds (Clone, Debug, Display)
4. Remove control methods (mutate_chromosome_genes, etc.)
5. Implement new simplified trait

### Phase 4: Cleanup (Breaking)
1. Remove old trait methods
2. Remove compatibility layer
3. Update documentation

## Key Insights

1. **Keep Performance Optimizations**: Cached distributions (`Uniform<usize>`, etc.) are legitimate performance improvements for hot paths.

2. **Three Levels of Access**: 
   - **Level 1**: Use convenient `mutate_chromosome_genes()` for standard cases
   - **Level 2**: Use building blocks (`mutate_gene_at()`, `random_allele_at()`) for custom patterns
   - **Level 3**: Direct gene access and exposed internals for full control

3. **Pragmatic Design**:
   - **Keep what works**: `mutate_chromosome_genes()` stays but changes from `&mut self` to `&self`
   - **Expose internals**: Complex genotypes expose their structure (subsets, weights) as needed
   - **No forced abstractions**: Don't create complex generic APIs for edge cases
   - **Backward compatible**: Existing code continues to work

## Next Steps

1. Implement new `Genotype` trait in `src/distributed/genotype/mod.rs`
2. Create proof-of-concept with `BinaryGenotype`
3. Update one `Mutate` strategy to use direct access
4. Benchmark performance difference
5. Get approval before full migration