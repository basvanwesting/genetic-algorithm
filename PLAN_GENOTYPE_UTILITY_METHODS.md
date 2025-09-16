# Plan: Genotype Utility Methods for Custom Mutate/Crossover Implementations

## Overview
To support custom mutate and crossover implementations with the new associated types approach, genotypes need to expose utility methods that provide safe, ergonomic access to their internals. These methods should handle the complexity of different genotype constraints while providing a consistent API.

## **Shared Utility Methods** (All Genotypes)

### Gene Index Sampling
1. **`sample_gene_index(&self, rng) -> usize`**
   - Returns a random valid gene index
   - Applies to: ALL genotypes

2. **`sample_gene_indices(&self, count, allow_duplicates, rng) -> Vec<usize>`**
   - Returns multiple random gene indices
   - With or without duplicates
   - Applies to: ALL genotypes

3. **`sample_gene_index_pair(&self, rng) -> (usize, usize)`**
   - Returns two different random gene indices
   - Applies to: ALL genotypes

### Gene Access
4. **`get_gene(&self, chromosome, index) -> &Allele`**
   - Safe gene access with bounds checking
   - Applies to: ALL genotypes

5. **`get_gene_mut(&self, chromosome, index) -> &mut Allele`**
   - Safe mutable gene access
   - Applies to: ALL genotypes

6. **`swap_genes(&self, chromosome, index1, index2)`**
   - Swap two genes within a chromosome
   - Automatically maintains constraints (e.g., uniqueness for UniqueGenotype)
   - Default implementation just swaps, UniqueGenotype overrides to maintain uniqueness
   - Applies to: ALL genotypes

### Chromosome Operations
7. **`swap_genes_between(&self, chrom1, chrom2, index)`**
   - Swap genes at same index between chromosomes
   - Applies to: ALL genotypes

8. **`reset_chromosome_state(&self, chromosome)`**
   - Reset chromosome state after modification
   - Applies to: ALL genotypes

## **Genotype-Specific Utility Methods**

### BinaryGenotype
9. **`flip_gene(&self, chromosome, index)`**
   - Flip a boolean gene value
   - Applies to: BinaryGenotype

10. **`set_gene(&self, chromosome, index, value: bool)`**
    - Set a specific boolean value
    - Applies to: BinaryGenotype

### RangeGenotype / MultiRangeGenotype
11. **`get_allele_range(&self, gene_index) -> &RangeInclusive<T>`**
    - Get valid range for a specific gene
    - Applies to: RangeGenotype, MultiRangeGenotype

12. **`sample_random_allele(&self, gene_index, rng) -> T`**
    - Generate random value in valid range
    - Applies to: RangeGenotype, MultiRangeGenotype

13. **`mutate_gene_relative(&self, chromosome, index, delta)`**
    - Apply relative mutation with automatic clamping to valid range
    - Handles bounds checking internally
    - Applies to: RangeGenotype, MultiRangeGenotype

14. **`mutate_gene_scaled(&self, chromosome, index, scale_index, rng)`**
    - Apply scaled mutation based on current scale
    - Applies to: RangeGenotype, MultiRangeGenotype

15. **`get_mutation_range(&self, gene_index, scale_index) -> RangeInclusive<T>`**
    - Get mutation range for current scale
    - Applies to: RangeGenotype, MultiRangeGenotype

### ListGenotype / MultiListGenotype  
16. **`get_allele_list(&self, gene_index) -> &Vec<T>`**
    - Get valid alleles for a gene position
    - Applies to: ListGenotype, MultiListGenotype

17. **`sample_random_allele(&self, gene_index, rng) -> T`**
    - Pick random valid allele
    - Applies to: ListGenotype, MultiListGenotype

18. **`set_gene_to_allele(&self, chromosome, gene_index, allele_index)`**
    - Set gene to specific allele by index
    - Applies to: ListGenotype, MultiListGenotype

### UniqueGenotype / MultiUniqueGenotype
19. **`find_gene_position(&self, chromosome, allele) -> Option<usize>`**
    - Find position of specific allele
    - Applies to: UniqueGenotype, MultiUniqueGenotype

20. **`validate_uniqueness(&self, chromosome) -> bool`**
    - Check if chromosome maintains uniqueness
    - Applies to: UniqueGenotype, MultiUniqueGenotype

21. **`rotate_genes(&self, chromosome, start, mid, end)`**
    - Rotate a section of genes (useful for order-based problems)
    - Applies to: UniqueGenotype, MultiUniqueGenotype

22. **`reverse_gene_segment(&self, chromosome, start, end)`**
    - Reverse a segment (2-opt style operation)
    - Applies to: UniqueGenotype, MultiUniqueGenotype

## **Crossover-Specific Utilities**

23. **`generate_crossover_points(&self, count, allow_duplicates, rng) -> Vec<usize>`**
    - Generate crossover points for multi-point crossover
    - Applies to: ALL genotypes except Unique variants

24. **`can_crossover(&self) -> bool`**
    - Check if genotype supports crossover
    - Applies to: ALL genotypes (returns false for Unique)

25. **`exchange_gene_segments(&self, father, mother, start, end)`**
    - Exchange continuous segments between chromosomes
    - Applies to: ALL genotypes except Unique variants

## Implementation Strategy

### Phase 1: Core Shared Methods
Start with the most commonly needed methods that apply to all genotypes:
- `sample_gene_index`
- `sample_gene_indices`

### Phase 2: Type-Specific Methods
Add methods specific to each genotype type:
- (Multi)Range: `mutate_gene_relative`
- (Multi)List: `sample_random_allele`

### Phase 3: Advanced Utilities
Add more sophisticated operations:
- Crossover point generation
- Segment operations
- Validation methods

## Usage Example

With these utility methods, custom implementations become much cleaner:

```rust
impl Mutate for MyCustomMutate {
    type Genotype = RangeGenotype<f32>;
    
    fn call<R: Rng, SR: StrategyReporter<Genotype = Self::Genotype>>(
        &mut self,
        genotype: &Self::Genotype,
        state: &mut EvolveState<Self::Genotype>,
        _config: &EvolveConfig,
        _reporter: &mut SR,
        rng: &mut R,
    ) {
        for chromosome in state.population.chromosomes.iter_mut() {
            // Use utility methods instead of direct manipulation
            let index = genotype.sample_gene_index(rng);
            let current = genotype.get_gene(chromosome, index);
            let delta = rng.gen_range(-0.1..=0.1);
            genotype.mutate_gene_relative(chromosome, index, delta, rng);
            genotype.reset_chromosome_state(chromosome);
        }
    }
}
```

## Benefits

1. **Safety**: Bounds checking, constraint enforcement
2. **Ergonomics**: Clean API without exposing internals
3. **Consistency**: Same patterns across all genotypes
4. **Discoverability**: Methods on genotype make capabilities clear
5. **Maintainability**: Changes to internals don't break custom implementations

## Notes

- Methods should be added to the `Genotype` trait where possible for consistency
- Type-specific methods can be added directly to the concrete genotype structs
- All methods should maintain genotype invariants (uniqueness, ranges, etc.)
- Consider performance implications - inline small methods
- Document which methods are available for each genotype type
