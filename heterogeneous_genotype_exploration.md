# Heterogeneous Genotype Exploration

## Executive Summary

This document explores the feasibility, design considerations, and implementation strategies for supporting heterogeneous (mixed-type) genotypes in the genetic algorithm library. Currently, the library enforces type homogeneity within chromosomes, but there are valid use cases for mixing boolean, discrete, and continuous values within a single chromosome.

## Table of Contents

1. [Current Architecture](#current-architecture)
2. [Motivation for Heterogeneous Genotypes](#motivation-for-heterogeneous-genotypes)
3. [Existing Workarounds](#existing-workarounds)
4. [Implementation Strategies](#implementation-strategies)
5. [Performance Considerations](#performance-considerations)
6. [Recommendations](#recommendations)
7. [Code Examples](#code-examples)
8. [Future Roadmap](#future-roadmap)

## Current Architecture

### Type Homogeneity Constraint

The library's fundamental design enforces that all genes within a chromosome must be of the same Rust type:

```rust
pub struct Chromosome<T: Allele> {
    pub genes: Genes<T>,  // Vec<T> where all elements are type T
    pub fitness_score: Option<FitnessValue>,
    pub age: Option<u32>,
    pub genes_hash: Option<u64>,
}
```

This constraint propagates through the entire architecture:
- Genotype traits are generic over a single `Allele` type
- Crossover and mutation operations assume uniform gene types
- Memory layout is optimized for homogeneous arrays

### Existing Heterogeneity Support

Despite the type constraint, the library provides several patterns for heterogeneous behavior:

1. **Multi* Genotypes**: Different configurations per gene position
   - `MultiListGenotype`: Different discrete choices per position
   - `MultiRangeGenotype`: Different numeric ranges per position
   - `MultiUniqueGenotype`: Different permutation sets per position

2. **Tuple Alleles**: Heterogeneous types within a single gene
   - Supports tuples up to 12 elements
   - Each gene is a compound structure

3. **Custom Struct Alleles**: User-defined types with multiple fields

## Motivation for Heterogeneous Genotypes

### Real-World Use Cases

#### 1. **Hybrid Optimization Problems**
Many real-world problems naturally combine different variable types:

```
Robot Configuration:
- Gene 0: Use vision system? (boolean)
- Gene 1: Sensor type (discrete: lidar/radar/ultrasonic)
- Gene 2: Motor speed (continuous: 0.0-100.0 rpm)
- Gene 3: Algorithm choice (discrete: A*/Dijkstra/RRT)
- Gene 4: Safety margin (continuous: 0.1-2.0 meters)
```

#### 2. **Feature Selection with Hyperparameters**
Machine learning pipeline optimization:

```
ML Pipeline:
- Genes 0-9: Feature flags (boolean: include/exclude)
- Gene 10: Algorithm type (discrete: SVM/RandomForest/XGBoost)
- Gene 11: Learning rate (continuous: 0.001-1.0)
- Gene 12: Max depth (discrete: 3-20)
- Gene 13: Regularization (continuous: 0.0-10.0)
```

#### 3. **Mixed Integer Programming**
Classical optimization problems:

```
Supply Chain:
- Genes 0-4: Facility open? (boolean)
- Genes 5-9: Production levels (discrete integers)
- Genes 10-14: Shipping proportions (continuous: 0.0-1.0)
```

### Current Limitations

Users must currently choose between:
1. **Type erasure**: Encode everything as a single type (e.g., `f64`) and interpret
2. **Compound genes**: Use tuples/structs, but all genes have the same compound structure
3. **Multiple populations**: Run separate GAs and combine results manually

## Existing Workarounds

### Approach 1: Semantic Encoding

Encode all values in a common numeric type with interpretation layers:

```rust
// All genes are u32, but interpreted differently
struct MixedProblemEncoding;

impl MixedProblemEncoding {
    fn decode_gene(index: usize, value: u32) -> MixedValue {
        match index {
            0..=2 => MixedValue::Boolean(value > 0),  // Boolean flags
            3..=5 => MixedValue::Discrete(value % 10), // Discrete choices
            6..=8 => MixedValue::Continuous(value as f64 / u32::MAX as f64), // Normalized continuous
            _ => panic!("Invalid gene index"),
        }
    }
}
```

**Pros:**
- Works with current architecture
- No performance penalty
- Type-safe at chromosome level

**Cons:**
- Semantic overhead in fitness function
- Loss of type information at gene level
- Mutation/crossover may not respect semantic boundaries

### Approach 2: Tuple-Based Genes

Use tuple alleles where each gene contains all variable types:

```rust
type MixedGene = (Option<bool>, Option<u8>, Option<f32>);

// Each gene uses only one field
let chromosome = vec![
    (Some(true), None, None),        // Boolean gene
    (None, Some(5), None),           // Discrete gene
    (None, None, Some(0.75)),        // Continuous gene
];
```

**Pros:**
- Type information preserved
- Flexible representation

**Cons:**
- Memory overhead (unused fields)
- Complex mutation logic
- Awkward API

### Approach 3: Struct-Based Encoding

Define a custom struct that encompasses all needed types:

```rust
#[derive(Clone, Copy, PartialEq, Hash, Debug)]
struct HeterogeneousGene {
    boolean_part: bool,
    discrete_part: u8,
    continuous_part: f32,
}

impl Allele for HeterogeneousGene {}

// Use MultiRangeGenotype with custom mutation logic
impl HeterogeneousGene {
    fn mutate_at_position(&mut self, position: usize, rng: &mut impl Rng) {
        match position % 3 {
            0 => self.boolean_part = !self.boolean_part,
            1 => self.discrete_part = rng.gen_range(0..10),
            2 => self.continuous_part = rng.gen_range(0.0..1.0),
            _ => unreachable!(),
        }
    }
}
```

### Approach 4: Optional Per-Gene Scaling in MultiRangeGenotype

Use `MultiRangeGenotype` with optional scaling per gene position to naturally handle mixed continuous/discrete genes:

```rust
// Encode discrete choices as integer ranges, continuous as float ranges
let genotype = MultiRangeGenotype::<f32>::builder()
    .with_allele_ranges(vec![
        0.0..=1.0,    // Gene 0: Boolean flag (0 or 1)
        0.0..=4.0,    // Gene 1: Discrete choice (0-4)
        0.0..=100.0,  // Gene 2: Continuous value
        -1.0..=1.0,   // Gene 3: Continuous value
        0.0..=2.0,    // Gene 4: Discrete choice (0-2)
    ])
    .with_allele_mutation_scaled_ranges(vec![
        vec![
            None,                    // Gene 0: No scaling (discrete)
            None,                    // Gene 1: No scaling (discrete)
            Some(-10.0..=10.0),      // Gene 2: Coarse scale
            Some(-0.1..=0.1),        // Gene 3: Coarse scale
            None,                    // Gene 4: No scaling (discrete)
        ],
        vec![
            None,                    // Gene 0: No scaling
            None,                    // Gene 1: No scaling
            Some(-1.0..=1.0),        // Gene 2: Fine scale
            Some(-0.01..=0.01),      // Gene 3: Fine scale
            None,                    // Gene 4: No scaling
        ],
    ])
    .build()?;
```

**Key Insight**: When a gene position has `None` for its scaled range, it always gets random mutation from its full `allele_range`, effectively treating it as a discrete choice. Continuous genes with `Some(range)` get progressive refinement through scaling.

**Pros:**
- Works within existing architecture with minimal changes
- Clear semantics: `None` = discrete, `Some(range)` = continuous
- No performance overhead for discrete genes
- Natural integration with scaling mechanism

**Cons:**
- Requires encoding discrete values as numeric ranges
- Need to interpret integer values in fitness function
- Currently requires modification to `MultiRangeGenotype` to support `Option<RangeInclusive<T>>`

## Implementation Strategies

### Strategy 1: Enum-Based Allele Type

Create a sum type that can hold any gene type:

```rust
#[derive(Clone, Copy, PartialEq, Debug)]
enum HeterogeneousAllele {
    Boolean(bool),
    Discrete(u32),
    Continuous(f64),
}

impl Allele for HeterogeneousAllele {}

struct HeterogeneousGenotype {
    gene_types: Vec<GeneType>,  // Metadata about each position
    mutation_rates: Vec<f64>,    // Position-specific mutation rates
}

impl Genotype for HeterogeneousGenotype {
    type Allele = HeterogeneousAllele;

    fn mutate_chromosome_genes(&self, ...) {
        for index in mutation_indices {
            match self.gene_types[index] {
                GeneType::Boolean => {
                    if let HeterogeneousAllele::Boolean(ref mut b) = chromosome.genes[index] {
                        *b = !*b;
                    }
                }
                GeneType::Discrete { max } => {
                    if let HeterogeneousAllele::Discrete(ref mut d) = chromosome.genes[index] {
                        *d = rng.gen_range(0..max);
                    }
                }
                // ... etc
            }
        }
    }
}
```

**Pros:**
- Straightforward implementation
- Works within current trait system
- Explicit type handling

**Cons:**
- Runtime dispatch overhead
- Larger memory footprint (enum discriminant)
- Pattern matching in hot paths

### Strategy 2: Const Generic Type Maps

Use const generics to encode type information at compile time:

```rust
struct HeterogeneousGenotype<const N: usize, const TYPES: [GeneTypeId; N]> {
    // Type information encoded in const generic
}

// Usage would look like:
type MyGenotype = HeterogeneousGenotype<
    5,
    [BOOL, BOOL, DISCRETE, CONTINUOUS, DISCRETE]
>;
```

**Pros:**
- Compile-time type checking
- No runtime overhead
- Type-safe operations

**Cons:**
- Complex implementation
- Requires stable const generics features
- Verbose type signatures

### Strategy 3: Trait Object Array

Use dynamic dispatch with trait objects:

```rust
trait Gene: Clone + Debug {
    fn mutate(&mut self, rng: &mut dyn Rng);
    fn crossover(&mut self, other: &mut dyn Gene);
    fn as_any(&self) -> &dyn Any;
}

struct HeterogeneousChromosome {
    genes: Vec<Box<dyn Gene>>,
}
```

**Pros:**
- Maximum flexibility
- Easy to extend with new gene types
- Clean separation of concerns

**Cons:**
- Significant performance overhead (boxing, dynamic dispatch)
- Loss of cache locality
- Complex cloning/comparison

### Strategy 4: Code Generation

Use macros or build-time code generation:

```rust
heterogeneous_genotype! {
    MyProblem {
        use_feature: bool,
        algorithm_choice: Discrete(0..5),
        learning_rate: Continuous(0.001..1.0),
        max_iterations: Discrete(10..1000),
    }
}

// Generates:
// - struct MyProblemChromosome
// - impl Genotype for MyProblemGenotype
// - Specialized mutation/crossover
```

**Pros:**
- Zero runtime overhead
- Type-safe
- Ergonomic API

**Cons:**
- Compile-time complexity
- Debugging macro-generated code
- Less flexible than runtime approaches

## Performance Considerations

### Memory Layout

| Approach                     | Memory Overhead    | Cache Efficiency   | SIMD Friendly   |
| ----------                   | ----------------   | ------------------ | --------------- |
| Current (homogeneous)        | None               | Excellent          | Yes             |
| Optional per-gene scaling    | ~8 bytes/gene¹     | Excellent          | Yes             |
| Enum-based                   | ~8 bytes/gene      | Good               | Partial         |
| Tuple with Options           | ~12-16 bytes/gene  | Poor               | No              |
| Trait objects                | ~16 bytes/gene     | Poor               | No              |
| Const generics               | None               | Excellent          | Yes             |

¹ Only for the Option wrapper in scaled ranges metadata, not in the chromosome itself

### Computational Overhead

```
Benchmark estimates (relative to homogeneous):
- Optional per-gene scaling: ~1.0-1.05x slower (minimal overhead, only Option checking)
- Enum dispatch: 1.2-1.5x slower
- Trait objects: 2-3x slower
- Tuple Options: 1.3-1.8x slower
- Const generics: ~1.0x (no overhead)
- Semantic encoding: 1.1-1.2x slower
```

### Parallelization Impact

The current homogeneous design enables:
- Efficient vectorization
- Simple work distribution
- Predictable memory access patterns

Heterogeneous designs may impact:
- Rayon's ability to partition work
- SIMD optimization opportunities
- GPU compatibility (especially for matrix genotypes)

## Recommendations

### Short-Term (Use Current Architecture)

For immediate needs with mixed continuous/discrete genes, I recommend **MultiRangeGenotype with optional per-gene scaling** (Approach 4):

1. **For problems with scaling requirements**: Modify `MultiRangeGenotype` to support `Option<RangeInclusive<T>>` in scaled ranges
2. **For simpler discrete/continuous mixes**: Use `MultiListGenotype` with semantic encoding

```rust
// Define semantic layers
enum GeneSemantics {
    BooleanFlag { index: usize },
    DiscreteChoice { index: usize, options: usize },
    ContinuousValue { index: usize, min: f64, max: f64 },
}

// Use MultiListGenotype<usize> with interpretation
let genotype = MultiListGenotype::<usize>::builder()
    .with_allele_lists(vec![
        vec![0, 1],           // Boolean as 0/1
        vec![0, 1, 2, 3, 4],  // Discrete with 5 choices
        (0..100).collect(),   // Continuous discretized to 100 levels
    ])
    .build();

// Interpret in fitness function
fn interpret_gene(index: usize, value: usize, semantics: &[GeneSemantics]) -> Value {
    match semantics[index] {
        GeneSemantics::BooleanFlag { .. } => Value::Bool(value > 0),
        GeneSemantics::DiscreteChoice { .. } => Value::Discrete(value),
        GeneSemantics::ContinuousValue { min, max, .. } => {
            Value::Continuous(min + (value as f64 / 99.0) * (max - min))
        }
    }
}
```

### Medium-Term (Library Extension)

If heterogeneous genotypes become a common need, implement an **optional enum-based extension**:

```rust
// New module: src/genotype/heterogeneous.rs
pub mod heterogeneous {
    pub enum MixedAllele { ... }
    pub struct MixedGenotype { ... }
    // Specialized implementations
}
```

This keeps the core library performant while offering flexibility when needed.

### Long-Term (Architecture Evolution)

Consider a **two-tier genotype system**:

1. **Performance tier**: Current homogeneous genotypes
2. **Flexibility tier**: New heterogeneous trait family

```rust
// Separate trait hierarchies
trait HomogeneousGenotype { ... }  // Current
trait HeterogeneousGenotype { ... } // New

// Strategies generic over either
impl<G> Strategy for Evolve<G>
where
    G: HomogeneousGenotype + ?Sized
    // OR
    G: HeterogeneousGenotype + ?Sized
```

## Code Examples

### Example 1: Robot Path Planning

```rust
use genetic_algorithm::prelude::*;

// Define semantic encoding
#[derive(Clone, Copy, Debug)]
struct RobotConfig {
    use_vision: bool,
    sensor_type: SensorType,
    max_speed: f32,
    path_algorithm: PathAlgorithm,
}

// Encode as homogeneous genes
impl From<Vec<u32>> for RobotConfig {
    fn from(genes: Vec<u32>) -> Self {
        RobotConfig {
            use_vision: genes[0] > 0,
            sensor_type: match genes[1] % 3 {
                0 => SensorType::Lidar,
                1 => SensorType::Radar,
                _ => SensorType::Ultrasonic,
            },
            max_speed: (genes[2] as f32 / 1000.0) * 100.0,
            path_algorithm: match genes[3] % 3 {
                0 => PathAlgorithm::AStar,
                1 => PathAlgorithm::Dijkstra,
                _ => PathAlgorithm::RRT,
            },
        }
    }
}

// Use MultiListGenotype for different ranges
let genotype = MultiListGenotype::<u32>::builder()
    .with_allele_lists(vec![
        vec![0, 1],                  // Boolean
        vec![0, 1, 2],               // Sensor type
        (0..=1000).collect(),        // Speed (0-100 mapped)
        vec![0, 1, 2],               // Algorithm
    ])
    .build();
```

### Example 2: Feature Selection with Hyperparameters

```rust
// Using tuple alleles for mixed types
type MLGene = (bool, Option<f32>);  // (feature_enabled, hyperparameter)

struct MLPipeline;

impl MLPipeline {
    fn decode_chromosome(chromosome: &Chromosome<MLGene>) -> Pipeline {
        let mut features = vec![];
        let mut hyperparams = vec![];

        for (i, gene) in chromosome.genes.iter().enumerate() {
            if i < 10 {
                // First 10 genes are feature flags
                if gene.0 {
                    features.push(i);
                }
            } else {
                // Remaining genes are hyperparameters
                if let Some(value) = gene.1 {
                    hyperparams.push(value);
                }
            }
        }

        Pipeline { features, hyperparams }
    }
}
```

### Example 3: Custom Heterogeneous Wrapper

```rust
// Wrapper providing heterogeneous interface over homogeneous implementation
pub struct HeterogeneousWrapper {
    boolean_genes: BitGenotype,
    discrete_genes: ListGenotype<u8>,
    continuous_genes: RangeGenotype<f64>,

    // Mapping from logical index to (genotype, index)
    index_map: Vec<(GenotypeType, usize)>,
}

impl HeterogeneousWrapper {
    pub fn get_gene(&self, index: usize) -> MixedValue {
        let (genotype_type, local_index) = self.index_map[index];
        match genotype_type {
            GenotypeType::Boolean => {
                MixedValue::Boolean(self.boolean_genes.get(local_index))
            }
            GenotypeType::Discrete => {
                MixedValue::Discrete(self.discrete_genes.get(local_index))
            }
            GenotypeType::Continuous => {
                MixedValue::Continuous(self.continuous_genes.get(local_index))
            }
        }
    }

    pub fn evolve(&mut self) {
        // Run evolution on each genotype separately
        // Combine results based on fitness
    }
}
```

### Example 4: Mixed Continuous/Discrete with Optional Scaling

```rust
use genetic_algorithm::prelude::*;

// Problem: Optimize a polynomial with discrete order selection
// Gene 0: Use linear term? (boolean as 0.0 or 1.0)
// Gene 1: Polynomial order (discrete: 0-2 for constant/linear/quadratic)
// Gene 2: Coefficient A (continuous: -10.0 to 10.0)
// Gene 3: Coefficient B (continuous: -5.0 to 5.0)

#[derive(Clone, Debug)]
struct PolynomialConfig {
    use_linear: bool,
    order: usize,
    coeff_a: f32,
    coeff_b: f32,
}

impl PolynomialConfig {
    fn from_chromosome(chromosome: &Chromosome<f32>) -> Self {
        PolynomialConfig {
            use_linear: chromosome.genes[0] > 0.5,
            order: chromosome.genes[1].round() as usize,
            coeff_a: chromosome.genes[2],
            coeff_b: chromosome.genes[3],
        }
    }
}

// Setup genotype with optional scaling
let genotype = MultiRangeGenotype::<f32>::builder()
    .with_allele_ranges(vec![
        0.0..=1.0,    // Boolean flag
        0.0..=2.0,    // Discrete order (0, 1, or 2)
        -10.0..=10.0, // Continuous coefficient A
        -5.0..=5.0,   // Continuous coefficient B
    ])
    .with_allele_mutation_scaled_ranges(vec![
        // Coarse scale
        vec![
            None,                  // Boolean: always random
            None,                  // Order: always random
            Some(-2.0..=2.0),      // Coeff A: coarse adjustment
            Some(-1.0..=1.0),      // Coeff B: coarse adjustment
        ],
        // Fine scale
        vec![
            None,                  // Boolean: always random
            None,                  // Order: always random
            Some(-0.5..=0.5),      // Coeff A: fine adjustment
            Some(-0.1..=0.1),      // Coeff B: fine adjustment
        ],
        // Ultra-fine scale
        vec![
            None,                  // Boolean: always random
            None,                  // Order: always random
            Some(-0.05..=0.05),    // Coeff A: ultra-fine
            Some(-0.01..=0.01),    // Coeff B: ultra-fine
        ],
    ])
    .build()?;

// Fitness function
impl Fitness for PolynomialProblem {
    fn calculate_for_chromosome(&mut self, chromosome: &Chromosome<f32>) -> Option<FitnessValue> {
        let config = PolynomialConfig::from_chromosome(chromosome);

        // Evaluate polynomial quality based on configuration
        let fitness = match config.order {
            0 => evaluate_constant(config.coeff_a),
            1 if config.use_linear => evaluate_linear(config.coeff_a, config.coeff_b),
            2 => evaluate_quadratic(config.coeff_a, config.coeff_b),
            _ => 0.0,
        };

        Some(fitness)
    }
}

// Evolution strategy with progressive refinement
let mut evolve = Evolve::builder()
    .with_genotype(genotype)
    .with_fitness(PolynomialProblem::new())
    .with_fitness_ordering(FitnessOrdering::Maximize)
    .build()?;

// Run coarse scale optimization
evolve.run_for(100)?;

// Switch to fine scale for refinement
evolve.genotype.increment_scale_index();
evolve.run_for(100)?;

// Final ultra-fine tuning
evolve.genotype.increment_scale_index();
evolve.run_for(100)?;
```

This example demonstrates:
- Mixed gene types in a single chromosome
- Discrete genes (boolean, order) always get random mutations
- Continuous genes (coefficients) get progressively refined through scaling
- Clean separation of concerns without complex workarounds

## Future Roadmap

### Phase 1: Documentation and Examples (Immediate)
- [ ] Add examples showing semantic encoding patterns
- [ ] Document MultiList/MultiRange for quasi-heterogeneous use cases
- [ ] Create cookbook for common mixed-type problems
- [ ] Implement optional per-gene scaling in MultiRangeGenotype (Approach 4)

### Phase 2: Helper Utilities (3-6 months)
- [ ] Semantic interpretation helpers
- [ ] Conversion utilities between encodings
- [ ] Validation tools for gene constraints
- [ ] Builder ergonomics for mixed gene specifications

### Phase 3: Experimental Module (6-12 months)
- [ ] Implement `heterogeneous` feature flag
- [ ] Add enum-based `MixedGenotype` as optional
- [ ] Benchmark performance impact
- [ ] Gather user feedback

### Phase 4: Architecture Decision (12+ months)
Based on adoption and performance data:
- **Option A**: Keep heterogeneous as optional module
- **Option B**: Redesign core traits for native support
- **Option C**: Maintain status quo with better tooling

## Conclusion

While the current architecture doesn't directly support mixed-type genotypes, it provides sufficient flexibility through:
1. Multi* genotype variants for position-specific behavior
2. Tuple and struct alleles for compound genes
3. Semantic encoding patterns
4. **Optional per-gene scaling (Approach 4) - a particularly elegant solution for mixed continuous/discrete problems**

The optional per-gene scaling approach stands out as the most promising near-term solution, requiring minimal changes to the existing architecture while providing clean semantics for differentiating between discrete (None) and continuous (Some) genes. This approach maintains excellent performance characteristics while solving the exact problem of mixing gene types that need different mutation strategies.

True heterogeneous support would require architectural changes with performance
trade-offs. For most use cases, the existing patterns with proper encoding
strategies offer a pragmatic solution that maintains the library's performance
characteristics.

The key insight is that **genetic algorithms operate on the search space**, not
the problem space. As long as we can map between them efficiently, type
homogeneity in the search space doesn't limit the heterogeneity of the problems
we can solve.

## References

- [Library Documentation](https://docs.rs/genetic_algorithm)
- [Examples Directory](./examples/)
- [Genotype Module Source](./src/genotype/)
- [Multi* Genotype Implementations](./src/genotype/multi_*.rs)
