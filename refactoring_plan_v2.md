# Genetic Algorithm Library - Architecture Refactoring Plan v2

## Understanding the Core Architectural Challenge

### The Matrix Genotype Constraint

The library supports two fundamentally different chromosome models:

1. **GenesOwner Chromosomes** (Binary, List, Unique, etc.)
   - Each chromosome owns its genes as a `Vec<Allele>`
   - Genes are scattered across memory in different chromosome instances
   - Standard OOP-style encapsulation

2. **GenesPointer Chromosomes** (DynamicMatrix, StaticMatrix)
   - All genes for entire population stored in single contiguous `Vec<T>` in the Genotype
   - Chromosomes just hold a `row_id` pointer
   - Enables GPU-friendly memory layout and bulk operations
   - **This is why Genotype has so many responsibilities!**

The matrix genotypes REQUIRE the Genotype to:
- Own all gene data centrally
- Manage chromosome lifecycle (they're just lightweight row pointers)
- Implement all mutation/crossover (needs direct access to the data matrix)
- Handle all gene manipulation (chromosomes can't do it themselves)

## The Real Problem

The current architecture tries to support both models with a single trait hierarchy, leading to:
- Genotype being overloaded with responsibilities (necessary for matrix types)
- Operators being thin wrappers (they can't access matrix data directly)
- Users unable to extend operators without creating new Genotypes

## Proposed Solution: Dual-Track Architecture

Instead of forcing all genotypes into one model, explicitly support both:

### Track 1: Traditional Genotypes (GenesOwner)
For Binary, List, Unique, Range, etc. where chromosomes own their genes.

```rust
// Chromosomes own their genes
trait StandardGenotype {
    type Chromosome: GenesOwner;
    // Minimal responsibilities
    fn random_genes<R: Rng>(&self, rng: &mut R) -> Self::Genes;
    fn validate_genes(&self, genes: &Self::Genes) -> bool;
}

// Operators work directly with chromosome genes
trait StandardMutate {
    fn mutate<C: GenesOwner, R: Rng>(
        &self,
        chromosome: &mut C,
        rng: &mut R
    );
}

// Users can easily extend
struct CustomMutate;
impl StandardMutate for CustomMutate {
    fn mutate<C: GenesOwner, R: Rng>(&self, chromosome: &mut C, rng: &mut R) {
        // Direct access to chromosome.genes
        let idx = rng.gen_range(0..chromosome.genes.len());
        chromosome.genes[idx] = !chromosome.genes[idx]; // example for binary
    }
}
```

### Track 2: Matrix Genotypes (GenesPointer)
For DynamicMatrix, StaticMatrix where genotype owns all data.

```rust
// Genotype owns all data
trait MatrixGenotype {
    type Chromosome: GenesPointer;
    type Allele: RangeAllele;
    
    // Matrix-specific data access
    fn get_gene(&self, chr: &Self::Chromosome, idx: usize) -> Self::Allele;
    fn set_gene(&mut self, chr: &Self::Chromosome, idx: usize, val: Self::Allele);
    fn swap_genes(&mut self, chr1: &Self::Chromosome, chr2: &Self::Chromosome, idx: usize);
    
    // Bulk operations for GPU potential
    fn get_population_data(&self) -> &[Self::Allele];
    fn get_population_data_mut(&mut self) -> &mut [Self::Allele];
}

// Matrix operators need genotype reference
trait MatrixMutate {
    fn mutate<G: MatrixGenotype, R: Rng>(
        &self,
        genotype: &mut G,
        chromosome: &G::Chromosome,
        rng: &mut R
    );
}

// Users can still extend, but need genotype access
struct CustomMatrixMutate;
impl MatrixMutate for CustomMatrixMutate {
    fn mutate<G: MatrixGenotype, R: Rng>(
        &self,
        genotype: &mut G,
        chromosome: &G::Chromosome,
        rng: &mut R
    ) {
        // Access through genotype
        let idx = rng.gen_range(0..genotype.genes_size());
        let new_val = genotype.random_allele(rng);
        genotype.set_gene(chromosome, idx, new_val);
    }
}
```

### Track 3: Unified Interface (Strategy Level)

At the strategy level, provide a unified interface that works with both:

```rust
enum GenotypeKind<S: StandardGenotype, M: MatrixGenotype> {
    Standard(S),
    Matrix(M),
}

enum MutateKind<SM: StandardMutate, MM: MatrixMutate> {
    Standard(SM),
    Matrix(MM),
}

// Strategy uses the unified interface
impl Evolve {
    fn mutate_population(&mut self) {
        match (&mut self.genotype, &self.mutate) {
            (GenotypeKind::Standard(g), MutateKind::Standard(m)) => {
                for chr in &mut self.population {
                    m.mutate(chr, &mut self.rng);
                }
            }
            (GenotypeKind::Matrix(g), MutateKind::Matrix(m)) => {
                for chr in &self.population {
                    m.mutate(g, chr, &mut self.rng);
                }
            }
            _ => panic!("Incompatible genotype/mutate combination"),
        }
    }
}
```

## Benefits of This Approach

### 1. Respects Fundamental Differences
- Acknowledges that matrix genotypes NEED centralized data management
- Doesn't force inappropriate abstractions

### 2. Enables Extensibility for Both Models
- Standard genotypes: Easy custom operators with direct gene access
- Matrix genotypes: Custom operators with genotype-mediated access
- Both can be extended without creating new Genotype types

### 3. Maintains Performance Benefits
- Matrix genotypes keep contiguous memory for GPU operations
- No overhead from unnecessary abstraction layers
- Bulk operations remain possible

### 4. Clear Mental Model
- Users understand there are two tracks
- Documentation can explain when to use each
- No confusion about why some operations need genotype access

## Implementation Strategy

### Phase 1: Split Existing Traits
1. Create `StandardGenotype` and `MatrixGenotype` traits
2. Create corresponding operator traits for each track
3. Keep existing `Genotype` trait as deprecated facade

### Phase 2: Migrate Existing Types
1. Implement `StandardGenotype` for Binary, List, Unique, Range
2. Implement `MatrixGenotype` for DynamicMatrix, StaticMatrix
3. Create operator implementations for both tracks

### Phase 3: Unification Layer
1. Create `GenotypeKind` enum for strategies
2. Update strategies to handle both tracks
3. Provide conversion utilities

### Phase 4: Enable Advanced Patterns
1. Operator composition for both tracks
2. Domain-specific operator libraries
3. GPU acceleration hooks for matrix track

## Migration Path

```rust
// Old way (still works via compatibility layer)
let genotype = BinaryGenotype::builder().with_genes_size(100).build();
let mutate = MutateSingleGene::new(0.2);

// New way - Standard Track
let genotype = StandardBinary::builder().with_genes_size(100).build();
let mutate = FlipMutate::new(0.2); // or custom implementation

// New way - Matrix Track  
let genotype = MatrixDynamic::<f32>::builder()
    .with_genes_size(100)
    .with_allele_range(0.0..=1.0)
    .build();
let mutate = RangeMutate::new(0.2); // or custom matrix operator
```

## Examples of User Extensions

### Custom Standard Operator
```rust
struct BiasedMutate {
    probability: f32,
    bias_towards_true: f32,
}

impl StandardMutate for BiasedMutate {
    fn mutate<C: GenesOwner<Genes = Vec<bool>>, R: Rng>(
        &self,
        chromosome: &mut C,
        rng: &mut R
    ) {
        for gene in &mut chromosome.genes {
            if rng.gen::<f32>() < self.probability {
                *gene = rng.gen::<f32>() < self.bias_towards_true;
            }
        }
    }
}
```

### Custom Matrix Operator
```rust
struct GradientMutate {
    step_size: f32,
}

impl MatrixMutate for GradientMutate {
    fn mutate<G: MatrixGenotype<Allele = f32>, R: Rng>(
        &self,
        genotype: &mut G,
        chromosome: &G::Chromosome,
        rng: &mut R
    ) {
        // Can access entire population data for context
        let pop_data = genotype.get_population_data();
        let pop_mean = pop_data.iter().sum::<f32>() / pop_data.len() as f32;
        
        // Mutate based on population context
        for i in 0..genotype.genes_size() {
            let current = genotype.get_gene(chromosome, i);
            if current > pop_mean {
                genotype.set_gene(chromosome, i, current - self.step_size);
            }
        }
    }
}
```

## Conclusion

This dual-track approach:
1. **Acknowledges the fundamental architectural constraint** of matrix genotypes needing centralized data
2. **Provides appropriate abstractions** for each model rather than forcing a compromise
3. **Enables user extensibility** for both standard and matrix genotypes
4. **Maintains performance benefits** of the matrix architecture for GPU-friendly operations
5. **Offers a clear migration path** with backward compatibility

The key insight is that trying to force both models into a single abstraction is what causes the problems. By explicitly supporting both tracks, we can provide the right abstractions for each use case.