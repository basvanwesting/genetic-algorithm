# AGENTS.md — AI Agent Guide for genetic-algorithm

This file helps AI coding agents use this library correctly. It covers decision
guidance, API reference, gotchas, and copy-paste templates.

**Read the Gotchas section before writing code.** Gotchas 1, 2, 3, and 7 cause
compilation or runtime failures that are hard to debug after the fact.

## Quick Start

```rust
use genetic_algorithm::strategy::evolve::prelude::*;
```

This single import brings in all types needed for the Evolve strategy. Similar
preludes exist for other strategies:
- `genetic_algorithm::strategy::hill_climb::prelude::*`
- `genetic_algorithm::strategy::permutate::prelude::*`
- `genetic_algorithm::strategy::prelude::*` (superset, all strategies)

## Critical: FitnessValue is isize

`FitnessValue` is `isize`, **not** `f64`. This is by design: isize enables
equality checks needed for staleness detection (stale generations are detected
when the best fitness score stops changing).

For float-based fitness, scale to isize manually:

```rust
// divide by desired precision, then cast
let precision = 1e-5;
Some((score / precision) as FitnessValue)
```

Return `None` from `calculate_for_chromosome` to mark a chromosome as invalid
(it will rank last in selection regardless of fitness ordering).

## Decision Matrix: Problem Type → Configuration

### Which Genotype?

| Problem Type | Genotype | Example |
|---|---|---|
| Binary choices (include/exclude) | `BinaryGenotype` | Knapsack |
| Values from a fixed set | `ListGenotype<T>` | Monkeys typing Shakespeare |
| Permutation / ordering | `UniqueGenotype<T>` | N-Queens, TSP |
| Continuous values (uniform range) | `RangeGenotype<T>` | Function optimization |
| Per-gene value sets | `MultiListGenotype<T>` | Mixed categorical |
| Multiple permutation groups | `MultiUniqueGenotype<T>` | Multi-group assignment |
| Per-gene numeric ranges | `MultiRangeGenotype<T>` | Heterogeneous optimization |

**When to use Multi\* variants:** Use `Multi*` when each gene needs different
settings (different allele lists, different ranges, different mutation types).
Use the regular variant when all genes share the same configuration.

### Gene Types in Fitness Functions

When implementing `calculate_for_chromosome`, access genes via `chromosome.genes`:

| Genotype | `chromosome.genes` type | Notes |
|---|---|---|
| `BinaryGenotype` | `Vec<bool>` | |
| `ListGenotype<T>` | `Vec<T>` | Default T = usize |
| `UniqueGenotype<T>` | `Vec<T>` | Default T = usize, all values unique |
| `RangeGenotype<T>` | `Vec<T>` | Default T = f32 |
| `MultiListGenotype<T>` | `Vec<T>` | Default T = usize |
| `MultiUniqueGenotype<T>` | `Vec<T>` | Default T = usize, unique within each group |
| `MultiRangeGenotype<T>` | `Vec<T>` | Default T = f32, each gene has its own range |

### Which Strategy?

| Situation | Strategy | Why |
|---|---|---|
| General optimization | `Evolve` | Full GA with crossover + mutation |
| Single permutation (`UniqueGenotype`) | `HillClimb` | Standard crossovers swap genes between parents, breaking the uniqueness invariant. HillClimb uses neighbor generation instead. |
| Multi-group permutation (`MultiUniqueGenotype`) | `Evolve` | Point-based crossovers work at group boundaries without breaking within-group uniqueness. |
| Convex search space | `HillClimb` | Local search suffices |
| Small search space (<1M) | `Permutate` | Exhaustive, 100% guarantee |

### Which Crossover? (Evolve only)

| Genotype | Compatible Crossovers | Recommended |
|---|---|---|
| `BinaryGenotype` | All | `CrossoverUniform` or `CrossoverSinglePoint` |
| `ListGenotype<T>` | All | `CrossoverUniform` |
| `MultiListGenotype<T>` | All | `CrossoverUniform` |
| `UniqueGenotype<T>` | `CrossoverClone`, `CrossoverRejuvenate` ONLY | `CrossoverClone` |
| `MultiUniqueGenotype<T>` | Point-based + `CrossoverClone`, `CrossoverRejuvenate` (NO gene-based) | `CrossoverSinglePoint` |
| `RangeGenotype<T>` | All | `CrossoverMultiPoint` |
| `MultiRangeGenotype<T>` | All | `CrossoverSingleGene` |

**WARNING**: `UniqueGenotype` will cause a **runtime panic** with gene-based or
point-based crossovers. Use `CrossoverClone` (clones parents, relies on mutation
for diversity) or `CrossoverRejuvenate` (like Clone but optimized for less memory copying).
`MultiUniqueGenotype` supports point-based crossovers (`CrossoverSinglePoint`,
`CrossoverMultiPoint`) but panics on gene-based ones (`CrossoverUniform`,
`CrossoverSingleGene`, `CrossoverMultiGene`).

### Which Select?

| Type | When to use |
|---|---|
| `SelectElite` | Default choice. Deterministic, sorts by fitness. |
| `SelectTournament` | When you want stochastic pressure. Better diversity. |

### Which Mutate?

| Type | When to use |
|---|---|
| `MutateSingleGene` | Default choice. Simple, one gene per chromosome. |
| `MutateMultiGene` | When faster exploration is needed. Multiple genes. |
| `MutateMultiGeneRange` | When you want random variation in mutation count. |
| `MutateSingleGeneDynamic` | Auto-adjusts probability based on population cardinality. |
| `MutateMultiGeneDynamic` | Auto-adjusts probability based on cardinality. Multiple genes. |

### If unsure, start here

For binary/list genotypes:
```rust
.with_select(SelectTournament::new(0.5, 0.02, 4))
.with_crossover(CrossoverUniform::new(0.7, 0.8))
.with_mutate(MutateSingleGene::new(0.2))
```

For unique genotypes (permutation problems):
```rust
.with_select(SelectTournament::new(0.5, 0.02, 4))
.with_crossover(CrossoverClone::new(0.7))
.with_mutate(MutateMultiGene::new(2, 0.2))
```

## Constructor Parameter Reference

### Select

```rust
SelectElite::new(
    replacement_rate: f32,  // 0.3-0.7 typical. Fraction of population replaced by offspring.
    elitism_rate: f32,      // 0.01-0.05 typical. Fraction preserved as elite (bypass selection gate).
)

SelectTournament::new(
    replacement_rate: f32,  // 0.3-0.7 typical.
    elitism_rate: f32,      // 0.01-0.05 typical.
    tournament_size: usize, // 2-8 typical. Chromosomes compared per tournament.
)
```

### Crossover

```rust
CrossoverUniform::new(
    selection_rate: f32,    // 0.5-0.8 typical. Fraction of parents selected for reproduction.
    crossover_rate: f32,    // 0.5-0.9 typical. Probability parent pair actually crosses over (vs clone).
)

CrossoverSinglePoint::new(selection_rate: f32, crossover_rate: f32)
CrossoverSingleGene::new(selection_rate: f32, crossover_rate: f32)

CrossoverMultiPoint::new(
    selection_rate: f32,
    crossover_rate: f32,
    number_of_crossovers: usize, // Number of crossover points.
    allow_duplicates: bool,       // Allow same crossover point twice.
)

CrossoverMultiGene::new(
    selection_rate: f32,
    crossover_rate: f32,
    number_of_crossovers: usize, // Number of genes to exchange.
    allow_duplicates: bool,       // Allow same gene index twice.
)

CrossoverClone::new(
    selection_rate: f32,    // No actual crossover, parents are cloned. Use with UniqueGenotype.
)

CrossoverRejuvenate::new(
    selection_rate: f32,    // Like Clone but drops non-selected first, then refills. Less memory copying.
)
```

### Mutate

**Rate guidance depends on genotype size and type — see "Mutation tuning for
large float genomes" in Troubleshooting.**

```rust
MutateSingleGene::new(
    mutation_probability: f32,  // 0.05-0.3 typical for binary. See note above for floats.
)

MutateMultiGene::new(
    number_of_mutations: usize,   // Max genes mutated (sampled uniformly from 1..=n).
    mutation_probability: f32,    // Probability per chromosome.
)

MutateMultiGeneRange::new(
    number_of_mutations_range: RangeInclusive<usize>,  // e.g. 1..=5
    mutation_probability: f32,
)

MutateSingleGeneDynamic::new(
    mutation_probability_step: f32,  // Step size for probability adjustment.
    target_cardinality: usize,       // Target unique chromosomes in population.
)

MutateMultiGeneDynamic::new(
    number_of_mutations: usize,      // Max genes mutated.
    mutation_probability_step: f32,  // Step size for adjustment.
    target_cardinality: usize,       // Target unique chromosomes.
)
```

### Extension (Evolve only, all optional)

Extensions should not be needed when hyperparameters are properly tuned. They are
a fallback when the population keeps collapsing to clones despite reasonable
mutation/selection settings. Escalation order: `MassDegeneration` (least
disruptive) → `MassDeduplication`/`MassExtinction` → `MassGenesis` (last resort).

```rust
ExtensionMassExtinction::new(
    cardinality_threshold: usize,  // Trigger when unique chromosomes drop below this.
    survival_rate: f32,            // Fraction that survives (random selection + elite).
    elitism_rate: f32,             // Fraction of elite preserved before random reduction.
)
// Randomly trims population. Recovery happens naturally through offspring in following generations.

ExtensionMassGenesis::new(
    cardinality_threshold: usize,  // Trims to only 2 best (Adam & Eve). Most aggressive reset.
)
// Extreme version of MassExtinction. Population recovers through offspring in following generations.

ExtensionMassDegeneration::new(
    cardinality_threshold: usize,
    number_of_mutations: usize,    // Number of gene mutations applied per chromosome.
    elitism_rate: f32,             // Fraction of elite preserved before mutation.
)
// Only extension that actually mutates genes. No population trim, same size throughout.

ExtensionMassDeduplication::new(
    cardinality_threshold: usize,  // Trims to only unique chromosomes (by genes hash).
)
// Removes duplicates. Population recovers through offspring in following generations.
```

## Builder Methods (Evolve)

Required:
- `.with_genotype(genotype)` — the search space
- `.with_fitness(fitness)` — the evaluation function
- `.with_target_population_size(n)` — number of chromosomes (**defaults to 0, always set this**)
- `.with_select(select)` — parent selection strategy
- `.with_crossover(crossover)` — how parents combine
- `.with_mutate(mutate)` — how offspring are varied
- At least ONE ending condition (see below)

Ending conditions (at least one required):
- `.with_target_fitness_score(score)` — stop when best fitness reaches this value
- `.with_max_stale_generations(n)` — stop after n generations without improvement
- `.with_max_generations(n)` — stop after n total generations

Optional:
- `.with_fitness_ordering(FitnessOrdering::Minimize)` — default is Maximize
- `.with_par_fitness(true)` — parallelize fitness calculation
- `.with_fitness_cache(size)` — LRU cache for expensive fitness
- `.with_replace_on_equal_fitness(true)` — replace best even on equal score
- `.with_extension(extension)` — diversity management
- `.with_reporter(reporter)` — progress monitoring
- `.with_rng_seed_from_u64(seed)` — deterministic results (use 0 for tests)
- `.with_valid_fitness_score(score)` — only solutions with this score or better are valid
- `.with_max_chromosome_age(n)` — force replacement of old chromosomes

## Builder Methods (HillClimb)

Required:
- `.with_genotype(genotype)`
- `.with_fitness(fitness)`
- At least ONE ending condition

Optional:
- `.with_variant(HillClimbVariant::SteepestAscent)` — default is Stochastic.
  Stochastic: fast, one random neighbor per generation, good for large genomes.
  SteepestAscent: evaluates all neighbors, finds best improvement, slow for large genomes.
- `.with_fitness_ordering(FitnessOrdering::Minimize)` — default is Maximize
- `.with_par_fitness(true)` — parallelize fitness calculation
- `.with_fitness_cache(size)` — LRU cache for expensive fitness
- `.with_replace_on_equal_fitness(true)` — replace best even on equal score (default: true for HillClimb)
- `.with_valid_fitness_score(score)` — only solutions with this score or better are valid
- `.with_reporter(reporter)` — progress monitoring
- `.with_rng_seed_from_u64(seed)` — deterministic results

HillClimb has no Select/Crossover/Mutate — it generates neighbors from the genotype directly.

## Builder Methods (Permutate)

Required:
- `.with_genotype(genotype)`
- `.with_fitness(fitness)`

Optional:
- `.with_fitness_ordering(FitnessOrdering::Minimize)` — default is Maximize
- `.with_par_fitness(true)` — parallelize fitness calculation
- `.with_replace_on_equal_fitness(true)` — replace best even on equal score
- `.with_reporter(reporter)` — progress monitoring

Permutate has no ending conditions — it exhaustively evaluates all possibilities.

**Note**: `RangeGenotype`/`MultiRangeGenotype` only support Permutate with
`MutationType::Step`, `MutationType::StepScaled`, or `MutationType::Discrete`
(these make the search space countable).

## Running the Strategy

Two patterns:

```rust
// Pattern 1: One-shot (build + run in one call)
let evolve = Evolve::builder()
    .with_genotype(genotype)
    // ... other builder steps ...
    .call()      // builds AND runs
    .unwrap();

// Pattern 2: Build then run (inspect state after)
let mut evolve = Evolve::builder()
    .with_genotype(genotype)
    // ... other builder steps ...
    .build()     // only builds
    .unwrap();
evolve.call();   // runs separately
```

Additional Evolve builder call variants (return `(best_run, all_runs)` tuple):
- `.call_repeatedly(n)` — n independent runs, return best + all
- `.call_par_repeatedly(n)` — parallel version
- `.call_speciated(n)` — n species runs, then final run seeded with best genes
- `.call_par_speciated(n)` — parallel version

```rust
let (best, _all_runs) = Evolve::builder()
    // ... builder steps ...
    .call_repeatedly(5)
    .unwrap();
```

HillClimb also supports `.call_repeatedly(n)` and `.call_par_repeatedly(n)`.

Both `.call()` and `.build()` return `Result<_, TryFromEvolveBuilderError>`.
Builder validation catches: missing required fields, incompatible genotype +
crossover combinations, and missing ending conditions. The error message includes
an actionable fix suggestion.

### Common mistakes

```rust
// WRONG: UniqueGenotype + CrossoverUniform = RUNTIME PANIC
// FIX:   Use CrossoverClone or CrossoverRejuvenate

// WRONG: No ending condition = COMPILE/BUILD ERROR
// FIX:   Add .with_max_stale_generations(1000)

// WRONG: target_population_size not set (defaults to 0) = SILENT FAILURE
// FIX:   Add .with_target_population_size(100)

// WRONG: Fitness returns f64 = TYPE ERROR
// FIX:   Return Some((score / precision) as FitnessValue)

// WRONG: MutateSingleGene(0.2) with 1000+ float genes = DIVERSITY COLLAPSE
// FIX:   Use MutateMultiGene with higher mutation count, see Troubleshooting
```

## Retrieving Results

```rust
// Best genes and fitness score
let (best_genes, best_fitness_score) = evolve.best_genes_and_fitness_score().unwrap();

// Or separately (gene type depends on genotype, e.g. Vec<bool> for BinaryGenotype)
let best_genes = evolve.best_genes();
let best_fitness_score = evolve.best_fitness_score();
```

## Implementing Custom Fitness

```rust
#[derive(Clone, Debug)]
struct MyFitness;

impl Fitness for MyFitness {
    type Genotype = BinaryGenotype; // or any genotype type

    fn calculate_for_chromosome(
        &mut self,
        chromosome: &FitnessChromosome<Self>,
        _genotype: &FitnessGenotype<Self>,
    ) -> Option<FitnessValue> {
        // Access genes via chromosome.genes (Vec<bool> for BinaryGenotype)
        // Return Some(score) for valid solutions
        // Return None for invalid solutions (ranks last)
        let score = chromosome.genes.iter().filter(|&&v| v).count();
        Some(score as FitnessValue)
    }
}
```

The fitness struct must derive `Clone` and `Debug`.

**Why `&mut self`?** The `calculate_for_chromosome` method takes `&mut self` so
you can pre-allocate buffers and reuse them across evaluations for performance.
For simple fitness functions, just ignore the mutability. When using
`par_fitness(true)`, each thread gets its own clone via `ThreadLocal`.

## Copy-Paste Templates

### Binary Optimization (Knapsack)

```rust
use genetic_algorithm::strategy::evolve::prelude::*;

const ITEMS: [(isize, isize); 10] = [
    // (value, weight)
    (60, 10), (100, 20), (120, 30), (80, 15), (50, 10),
    (90, 25), (70, 18), (40, 8), (110, 22), (65, 12),
];
const MAX_WEIGHT: isize = 80;

#[derive(Clone, Debug)]
struct KnapsackFitness;
impl Fitness for KnapsackFitness {
    type Genotype = BinaryGenotype;
    fn calculate_for_chromosome(
        &mut self,
        chromosome: &FitnessChromosome<Self>,
        _genotype: &FitnessGenotype<Self>,
    ) -> Option<FitnessValue> {
        let (total_value, total_weight) = chromosome.genes.iter().enumerate()
            .filter(|(_, &included)| included)
            .fold((0, 0), |(v, w), (i, _)| (v + ITEMS[i].0, w + ITEMS[i].1));
        if total_weight > MAX_WEIGHT {
            Some(total_value - (total_weight - MAX_WEIGHT) * 10) // penalty
        } else {
            Some(total_value)
        }
    }
}

fn main() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(ITEMS.len())
        .build()
        .unwrap();

    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(100)
        .with_max_stale_generations(100)
        .with_fitness(KnapsackFitness)
        .with_select(SelectTournament::new(0.5, 0.02, 4))
        .with_crossover(CrossoverUniform::new(0.7, 0.8))
        .with_mutate(MutateSingleGene::new(0.2))
        .call()
        .unwrap();

    let (best_genes, best_fitness_score) = evolve.best_genes_and_fitness_score().unwrap();
    println!("score: {}", best_fitness_score);
}
```

### Continuous Optimization (RangeGenotype)

```rust
use genetic_algorithm::strategy::evolve::prelude::*;

#[derive(Clone, Debug)]
struct MinimizeDistance { target: f32, precision: f32 }
impl Fitness for MinimizeDistance {
    type Genotype = RangeGenotype<f32>;
    fn calculate_for_chromosome(
        &mut self,
        chromosome: &FitnessChromosome<Self>,
        _genotype: &FitnessGenotype<Self>,
    ) -> Option<FitnessValue> {
        let error: f32 = chromosome.genes.iter()
            .map(|v| (v - self.target).abs())
            .sum();
        // larger FitnessValue = worse when minimizing
        Some((error / self.precision) as FitnessValue)
    }
}

fn main() {
    let genotype = RangeGenotype::<f32>::builder()
        .with_genes_size(10)
        .with_allele_range(0.0..=1.0)
        .with_mutation_type(MutationType::StepScaled(vec![0.1, 0.01, 0.001]))
        .build()
        .unwrap();

    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(100)
        .with_max_stale_generations(1000)
        .with_fitness(MinimizeDistance { target: 0.5, precision: 1e-5 })
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_select(SelectTournament::new(0.5, 0.02, 4))
        .with_crossover(CrossoverMultiPoint::new(0.7, 0.8, 3, false))
        .with_mutate(MutateMultiGene::new(2, 0.2))
        .call()
        .unwrap();

    let (best_genes, best_fitness_score) = evolve.best_genes_and_fitness_score().unwrap();
    println!("genes: {:?}, score: {}", best_genes, best_fitness_score);
}
```

### Permutation Problem (HillClimb, recommended for permutations)

```rust
use genetic_algorithm::strategy::hill_climb::prelude::*;

#[derive(Clone, Debug)]
struct NQueensFitness;
impl Fitness for NQueensFitness {
    type Genotype = UniqueGenotype<u8>;
    fn calculate_for_chromosome(
        &mut self,
        chromosome: &FitnessChromosome<Self>,
        _genotype: &FitnessGenotype<Self>,
    ) -> Option<FitnessValue> {
        let mut score = 0;
        let n = chromosome.genes.len();
        for i in 0..n {
            for j in (i + 1)..n {
                let dy = chromosome.genes[i].abs_diff(chromosome.genes[j]) as usize;
                if dy == j - i { score += 1; }
            }
        }
        Some(score)
    }
}

fn main() {
    let genotype = UniqueGenotype::builder()
        .with_allele_list((0..8u8).collect())
        .build()
        .unwrap();

    let hill_climb = HillClimb::builder()
        .with_genotype(genotype)
        .with_variant(HillClimbVariant::SteepestAscent)
        .with_fitness(NQueensFitness)
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_target_fitness_score(0)
        .with_max_stale_generations(1000)
        .call()
        .unwrap();

    let (best_genes, best_fitness_score) = hill_climb.best_genes_and_fitness_score().unwrap();
    println!("queens: {:?}, conflicts: {}", best_genes, best_fitness_score);
}
```

### Exhaustive Search (Permutate)

```rust
use genetic_algorithm::strategy::permutate::prelude::*;

#[derive(Clone, Debug)]
struct MyFitness;
impl Fitness for MyFitness {
    type Genotype = BinaryGenotype;
    fn calculate_for_chromosome(
        &mut self,
        chromosome: &FitnessChromosome<Self>,
        _genotype: &FitnessGenotype<Self>,
    ) -> Option<FitnessValue> {
        Some(chromosome.genes.iter().filter(|&&v| v).count() as FitnessValue)
    }
}

fn main() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(16) // 2^16 = 65536 combinations, feasible
        .build()
        .unwrap();

    let permutate = Permutate::builder()
        .with_genotype(genotype)
        .with_fitness(MyFitness)
        .call()
        .unwrap();

    let (best_genes, best_fitness_score) = permutate.best_genes_and_fitness_score().unwrap();
    println!("best: {:?}, score: {}", best_genes, best_fitness_score);
}
```

### Heterogeneous Optimization (MultiRangeGenotype)

```rust
use genetic_algorithm::strategy::evolve::prelude::*;

// Optimize 4 parameters with different ranges and mutation behaviors:
//   Gene 0: boolean flag (0 or 1)
//   Gene 1: algorithm choice (0, 1, 2, 3, or 4)
//   Gene 2: learning rate (0.001 to 1.0, continuous)
//   Gene 3: batch size (16 to 512, discrete integer steps)
#[derive(Clone, Debug)]
struct HyperparamFitness { precision: f32 }
impl Fitness for HyperparamFitness {
    type Genotype = MultiRangeGenotype<f32>;
    fn calculate_for_chromosome(
        &mut self,
        chromosome: &FitnessChromosome<Self>,
        _genotype: &FitnessGenotype<Self>,
    ) -> Option<FitnessValue> {
        let flag = chromosome.genes[0];         // 0.0 or 1.0 (Discrete)
        let algorithm = chromosome.genes[1];    // 0.0..4.0 (Discrete)
        let learning_rate = chromosome.genes[2]; // 0.001..1.0 (continuous)
        let batch_size = chromosome.genes[3];   // 16.0..512.0 (Discrete)
        // ... your evaluation logic ...
        let score = learning_rate * flag + algorithm * 0.1 - batch_size * 0.001;
        Some((score / self.precision) as FitnessValue)
    }
}

fn main() {
    let genotype = MultiRangeGenotype::<f32>::builder()
        .with_allele_ranges(vec![
            0.0..=1.0,     // Gene 0: boolean
            0.0..=4.0,     // Gene 1: algorithm choice
            0.001..=1.0,   // Gene 2: learning rate
            16.0..=512.0,  // Gene 3: batch size
        ])
        .with_mutation_types(vec![
            MutationType::Discrete,                             // boolean: 0 or 1
            MutationType::Discrete,                             // enum: 0,1,2,3,4
            MutationType::StepScaled(vec![0.1, 0.01, 0.001]),  // continuous refinement
            MutationType::Discrete,                             // integer steps
        ])
        .build()
        .unwrap();

    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(100)
        .with_max_stale_generations(1000)
        .with_fitness(HyperparamFitness { precision: 1e-5 })
        .with_select(SelectTournament::new(0.5, 0.02, 4))
        .with_crossover(CrossoverSingleGene::new(0.7, 0.8))
        .with_mutate(MutateSingleGene::new(0.2))
        .call()
        .unwrap();

    let (best_genes, best_fitness_score) = evolve.best_genes_and_fitness_score().unwrap();
    println!("genes: {:?}, score: {}", best_genes, best_fitness_score);
}
```

## Troubleshooting

**Fitness not improving?**
- Increase `target_population_size` (more diversity)
- Increase `mutation_probability` (more exploration)
- Try `MutateSingleGeneDynamic` or `MutateMultiGeneDynamic` (auto-adjusts)
- For `RangeGenotype`/`MultiRangeGenotype`: use `MutationType::StepScaled` for
  progressive refinement instead of `MutationType::Random`
- Add an extension like `ExtensionMassGenesis` or `ExtensionMassDegeneration` to
  escape local optima when diversity drops

**Mutation tuning for large float genomes (RangeGenotype/MultiRangeGenotype)?**

The "typical" mutation rates assume small binary genomes. For large float genomes
(hundreds to thousands of genes), the effective per-gene mutation rate matters
more than the per-chromosome probability. Think in terms of what fraction of all
genes in the population actually change per generation.

- **Binary genes:** mutation flips 0↔1, which is a large relative change. Low
  rates (1-5% per chromosome) suffice.
- **Float genes:** mutation nudges a continuous value. Each mutation has less
  relative impact, so you need far more mutations to maintain diversity.

Concrete example for a 2000-gene float genome, population 100:
- `MutateSingleGene(0.2)` → 1 gene × 20% of offspring = effective 0.01% of all
  genes change per generation. **Population will collapse to near-clones.**
- `MutateMultiGene(10, 1.0)` → ~5.5 genes × 100% of offspring = effective 0.28%
  of all genes change per generation. **Maintains diversity.**

Rule of thumb for float genomes: target 0.1%-1.0% effective per-gene mutation
rate across the population. Use `MutateMultiGene` with high `mutation_probability`
and scale `number_of_mutations` with genome size.

**But high mutation prevents convergence.** Use scaled mutation types to get both
exploration early and convergence late:
- `MutationType::RangeScaled(vec![100.0, 100.0, 50.0, 10.0, 1.0])` — starts
  with full-range Random mutations (100% of allele range = effectively Random),
  then progressively narrows the mutation bandwidth. Best for float genomes:
  wide exploration phases first, then tight range-bound convergence.
- `MutationType::StepScaled(vec![10.0, 1.0, 0.1])` — fixed step sizes that
  decrease through phases. Better for grid-like or discrete problems.

Combine with `max_stale_generations` to trigger phase transitions automatically
(advances to next phase when fitness plateaus).

**Runtime too slow?**
- Use `.with_par_fitness(true)` for expensive fitness calculations
- Use `.with_fitness_cache(size)` if many chromosomes share the same genes
- Reduce `target_population_size` if fitness is cheap but framework overhead is high
- For `HillClimb::SteepestAscent`: the neighbourhood can be very large, consider
  `Stochastic` variant instead

**Getting `None` as best fitness?**
- All chromosomes returned `None` from `calculate_for_chromosome`
- Check your fitness function's validity constraints — they may be too strict
- Increase population size so some valid solutions appear in the initial population
- Prefer large penalties over `None`: returning `Some(very_bad_score)` lets the
  algorithm converge incrementally out of invalid space, while `None` provides
  no gradient signal and ranks last unconditionally

## Gotchas

1. **UniqueGenotype + standard crossover = runtime panic.** Use `CrossoverClone`. MultiUniqueGenotype supports point-based but not gene-based crossovers.
2. **FitnessValue is isize.** Scale floats manually: `(score / precision) as FitnessValue`.
3. **Ending condition required.** Evolve/HillClimb need at least one of: `target_fitness_score`, `max_stale_generations`, `max_generations`.
4. **Fitness struct must be `Clone + Debug`.**
5. **RangeGenotype requires a type parameter.** Use turbofish: `RangeGenotype::<f64>::builder()`, `RangeGenotype::<i32>::builder()`.
6. **Permutate + RangeGenotype** requires `MutationType::Step`, `StepScaled`, or `Discrete`.
7. **`target_population_size` defaults to 0.** Always set it for Evolve.
8. **Custom Crossover/Mutate/Extension must call `chromosome.reset_metadata(genotype.genes_hashing)`** after modifying genes directly.
9. **Custom Crossover must call `state.population.increment_age()`** on existing chromosomes.
10. **For deterministic tests:** use `.with_rng_seed_from_u64(0)`. Exact results may change between library versions, but deterministic within a version.
