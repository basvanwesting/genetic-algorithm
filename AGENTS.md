# AGENTS.md — AI Agent Guide for genetic-algorithm

This file helps AI coding agents use this library correctly. It covers decision
guidance, API reference, gotchas, and copy-paste templates.

**Read the Gotchas section before writing code.** Gotchas 1 and 2 cause
compilation or runtime failures that are hard to debug after the fact.

## Table of Contents

- [Quick Start](#quick-start)
- [Critical: FitnessValue is isize](#critical-fitnessvalue-is-isize)
- [Decision Matrix: Problem Type → Configuration](#decision-matrix-problem-type--configuration)
- [Constructor Parameter Reference](#constructor-parameter-reference)
- [Builder Methods (Evolve)](#builder-methods-evolve)
- [Builder Methods (HillClimb)](#builder-methods-hillclimb)
- [Builder Methods (Permutate)](#builder-methods-permutate)
- [Builder Methods (StrategyBuilder)](#builder-methods-strategybuilder)
- [Running the Strategy](#running-the-strategy)
- [Common Mistakes](#common-mistakes)
- [Retrieving Results](#retrieving-results)
- [Implementing Custom Fitness](#implementing-custom-fitness)
- [Copy-Paste Templates](#copy-paste-templates)
- [Troubleshooting](#troubleshooting)
- [Gotchas](#gotchas)

## Quick Start

Add to your `Cargo.toml`:
```toml
[dependencies]
genetic_algorithm = "0.26"
```

```rust
use genetic_algorithm::strategy::evolve::prelude::*;
```

This single import brings in all types needed for the Evolve strategy. Similar
preludes exist for other strategies:
- `genetic_algorithm::strategy::hill_climb::prelude::*`
- `genetic_algorithm::strategy::permutate::prelude::*`
- `genetic_algorithm::strategy::prelude::*` (superset, all strategies)

**Logging:** Examples use `env_logger::init()` for log output. Add `env_logger`
to your `[dependencies]` and call `env_logger::init()` in `main()` to see
reporter output. Set `RUST_LOG=info` (or `debug`) when running.

## Critical: FitnessValue is isize

`FitnessValue` is `isize`, **not** `f64`. This is by design: isize enables
equality checks needed for staleness detection (stale generations are detected
when the best fitness score stops changing).

**This library uses f32 as the default float type** (e.g. `RangeGenotype<f32>`,
`MultiRangeGenotype<f32>`). GAs don't need f64 precision — the stochastic search
process dominates any floating-point rounding.

For float-based fitness, scale to isize manually:

```rust
// divide by desired precision, then cast
let precision = 1e-5;
Some((score / precision) as FitnessValue)

// or use the helper function (accepts f32 and f64)
Some(fitness_value(score, precision))
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
| `UniqueGenotype<T>` | `CrossoverClone`, `CrossoverRejuvenate` ONLY (others are compile errors) | `CrossoverClone` |
| `MultiUniqueGenotype<T>` | Point-based + `CrossoverClone`, `CrossoverRejuvenate` (gene-based are compile errors) | `CrossoverSinglePoint` |
| `RangeGenotype<T>` | All | `CrossoverMultiPoint` |
| `MultiRangeGenotype<T>` | All | `CrossoverSingleGene` |

**Compile-time safety**: `UniqueGenotype` does not implement `SupportsGeneCrossover`
or `SupportsPointCrossover`, so incompatible crossovers are **compile errors**.
Use `CrossoverClone` (clones parents, relies on mutation for diversity) or
`CrossoverRejuvenate` (like Clone but optimized for less memory copying).
`MultiUniqueGenotype` implements `SupportsPointCrossover` only, so gene-based
crossovers (`CrossoverUniform`, `CrossoverSingleGene`, `CrossoverMultiGene`) are
compile errors.

**Note on UniqueGenotype + Evolve:** `CrossoverClone` with `UniqueGenotype`
produces valid code but is almost always less efficient than `HillClimb`. Only
use `Evolve` + `CrossoverClone` for `UniqueGenotype` when you need Extensions,
speciation, or `call_repeatedly`.

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

### Which MutationType?

`MutationType` controls how gene values change during mutation (Range/MultiRange
genotypes only). Other genotypes ignore this setting.

| Variant | Behavior |
|---|---|
| `Random` (default) | Uniform random within allele range. |
| `Range(bandwidth)` | Uniform random within ±bandwidth of current value, clamped to range. |
| `Step(step)` | Exactly +step or -step, clamped to range. |
| `Discrete` | Integer-only mutations. Required for Permutate with Range genotypes. |
| `RangeScaled(vec)` | Like Range, but advances through multiple bandwidths. |
| `StepScaled(vec)` | Like Step, but advances through multiple step sizes. |

`Random` is the default when unspecified. For float genomes >50 genes, prefer
`RangeScaled` or `StepScaled` for progressive refinement.

**Scale advancement:** Scaled variants advance to the next phase when
`max_stale_generations` or `max_generations` is reached. Counters reset per
phase. A run with 3 phases and `max_stale_generations(100)` can run up to 300
stale generations before terminating.

### If unsure, start here

For binary/list genotypes:
```rust
// also requires: genotype, fitness, target_population_size, ending condition
.with_select(SelectTournament::new(0.5, 0.02, 4))
.with_crossover(CrossoverUniform::new(0.7, 0.8))
.with_mutate(MutateSingleGene::new(0.2))
```

For range/float genotypes (>50 genes, see Troubleshooting for tuning):
```rust
// also requires: genotype, fitness, target_population_size, ending condition
.with_select(SelectTournament::new(0.5, 0.02, 4))
.with_crossover(CrossoverMultiPoint::new(0.7, 0.8, 3, false))
.with_mutate(MutateMultiGene::new(10, 1.0))
```

For unique genotypes (permutation problems):
```rust
// also requires: genotype, fitness, target_population_size, ending condition
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
large float genomes" in Troubleshooting.** For float genomes >50 genes: use
`MutateMultiGene` with `mutation_probability` near 1.0 and scale
`number_of_mutations` with genome size.

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

Builder method order does not matter.

Required:
- `.with_genotype(genotype)` — the search space
- `.with_fitness(fitness)` — the evaluation function
- `.with_select(select)` — parent selection strategy
- `.with_crossover(crossover)` — how parents combine
- `.with_mutate(mutate)` — how offspring are varied
- At least ONE ending condition (see below)

Ending conditions (at least one required):
- `.with_target_fitness_score(score)` — stop when best fitness reaches this value
- `.with_max_stale_generations(n)` — stop after n generations without improvement
- `.with_max_generations(n)` — stop after n total generations

**`max_stale_generations` guidance:**
- HillClimb SteepestAscent without plateaus: 1-2 (use `call_repeatedly` for restarts)
- HillClimb SteepestAscent with plateaus (e.g. N-Queens): 1000+ (needs room to
  traverse equal-fitness states via `replace_on_equal_fitness`)
- Binary/list problems: 100-1000
- Permutation problems: 1000-10000
- Range with StepScaled/RangeScaled: multiply by number of phases (each phase
  resets the stale counter)

Optional:
- `.with_target_population_size(n)` — number of chromosomes (defaults to 100).
  Heuristic: small genomes (<50 genes): 100, medium (50-500): 200-500,
  large/complex: 1000+. Must be explicitly set for `StrategyBuilder` (defaults
  to 0). HillClimb does not use population size.
- `.with_fitness_ordering(FitnessOrdering::Minimize)` — default is Maximize
- `.with_par_fitness(true)` — parallelize fitness calculation
- `.with_fitness_cache(size)` — LRU cache for expensive fitness
- `.with_replace_on_equal_fitness(bool)` — replace best even on equal score (default: true)
- `.with_extension(extension)` — diversity management
- `.with_reporter(reporter)` — progress monitoring. Use `EvolveReporterSimple::new(100)`
  for progress every 100 generations, `EvolveReporterDuration::new()` for performance
  profiling. Each strategy has its own reporter types: `EvolveReporterSimple`/`Duration`,
  `HillClimbReporterSimple`/`Duration`, `PermutateReporterSimple`/`Duration`. Use
  `*ReporterNoop` for no reporting.
- `.with_rng_seed_from_u64(seed)` — deterministic results (use 0 for tests)
- `.with_valid_fitness_score(score)` — gates all ending conditions: no ending condition
  fires until best fitness reaches this threshold
- `.with_max_chromosome_age(n)` — removes chromosomes with age >= n from selection pool.
  Age resets to 0 for offspring, increments each generation.
- `.with_seed_genes_list(genes_list)` — seed initial population with known solutions
  (set on the genotype builder, not the strategy builder)

## Builder Methods (HillClimb)

Required:
- `.with_genotype(genotype)`
- `.with_fitness(fitness)`
- At least ONE ending condition

Optional:
- `.with_variant(HillClimbVariant::SteepestAscent)` — default is Stochastic.
  Stochastic: fast, one random neighbor per generation, good for large genomes.
  SteepestAscent: evaluates all neighbors, finds best improvement, slow for large genomes.
  **Warning:** SteepestAscent evaluates n*(n-1)/2 neighbors for UniqueGenotype of size n.
  Use only for small genomes (<20 genes).
- `.with_fitness_ordering(FitnessOrdering::Minimize)` — default is Maximize
- `.with_par_fitness(true)` — parallelize fitness calculation.
  **Note:** `par_fitness` has no effect with `HillClimbVariant::Stochastic` (sequential
  by nature). Only useful with `SteepestAscent`.
- `.with_fitness_cache(size)` — LRU cache for expensive fitness
- `.with_replace_on_equal_fitness(bool)` — replace best even on equal score (default: true).
  For minimization problems with plateaus (many solutions share the same fitness),
  this default is essential — without it, HillClimb cannot traverse plateaus to find
  improvements.
- `.with_valid_fitness_score(score)` — only solutions with this score or better are valid
- `.with_reporter(reporter)` — progress monitoring
- `.with_rng_seed_from_u64(seed)` — deterministic results

HillClimb auto-disables `genes_hashing` (unless `fitness_cache` is set), so you
don't need to set it manually.

HillClimb has no Select/Crossover/Mutate — it generates neighbors from the genotype directly.

## Builder Methods (Permutate)

Required:
- `.with_genotype(genotype)`
- `.with_fitness(fitness)`

Optional:
- `.with_fitness_ordering(FitnessOrdering::Minimize)` — default is Maximize
- `.with_par_fitness(true)` — parallelize fitness calculation
- `.with_replace_on_equal_fitness(bool)` — replace best even on equal score (default: true)
- `.with_reporter(reporter)` — progress monitoring

Permutate has no ending conditions — it exhaustively evaluates all possibilities.

**Note**: `RangeGenotype`/`MultiRangeGenotype` only support Permutate with
`MutationType::Step`, `MutationType::StepScaled`, or `MutationType::Discrete`
(these make the search space countable).

## Builder Methods (StrategyBuilder)

`StrategyBuilder` is a superset builder supporting all three strategies from one
configuration. Useful for dynamically sized problems where small spaces use
Permutate and larger ones use Evolve.

Requires `use genetic_algorithm::strategy::prelude::*;` and a genotype that
implements all three strategy traits. All genotypes are compatible:
`BinaryGenotype`, `ListGenotype`, `UniqueGenotype`, `MultiListGenotype`,
`MultiUniqueGenotype`, `RangeGenotype`, `MultiRangeGenotype`. Note that
`RangeGenotype`/`MultiRangeGenotype` only support the Permutate variant with
`MutationType::Step`, `StepScaled`, or `Discrete` (runtime check via
`allows_permutation()`).

Switch strategy with `.with_variant(variant)`:

```rust
use genetic_algorithm::strategy::prelude::*;

let builder = StrategyBuilder::new()
    .with_genotype(genotype)
    .with_target_population_size(100)
    .with_max_stale_generations(100)
    .with_fitness(my_fitness)
    .with_select(SelectTournament::new(0.5, 0.02, 4))
    .with_crossover(CrossoverUniform::new(0.7, 0.8))
    .with_mutate(MutateSingleGene::new(0.2));

// Switch strategy based on problem size
let variant = if search_space_size < 1_000_000 {
    StrategyVariant::Permutate(PermutateVariant::Standard)
} else {
    StrategyVariant::Evolve(EvolveVariant::Standard)
};

let strategy = builder.with_variant(variant).call().unwrap();
println!("best: {:?}", strategy.best_fitness_score());
```

Supports all 5 call variants (see "Running the Strategy" for availability table).
`call_speciated`/`call_par_speciated` fall back to `call_repeatedly`/`call_par_repeatedly`
for non-Evolve strategies.

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

**Note:** The best run is extracted from the runs vector — `all_runs` contains
N-1 results.

### Choosing a call variant

| Variant | When to use |
|---|---|
| `call()` | Default. Sufficient for most problems. |
| `call_repeatedly(n)` | Results vary across runs (local optima). Typical n: 3-10. |
| `call_par_repeatedly(n)` | Parallel version of above. |
| `call_speciated(n)` | Multiple runs seed a final refinement pass. Best for complex combinatorial problems. |
| `call_par_speciated(n)` | Parallel version of above. |

**Call variant availability by builder:**

| Variant | `Evolve::builder()` | `HillClimb::builder()` | `Permutate::builder()` | `StrategyBuilder` |
|---|---|---|---|---|
| `call()` | yes | yes | yes | yes |
| `call_repeatedly(n)` | yes | yes | no | yes |
| `call_par_repeatedly(n)` | yes | yes | no | yes |
| `call_speciated(n)` | yes | no | no | yes (falls back to `call_repeatedly`) |
| `call_par_speciated(n)` | yes | no | no | yes (falls back to `call_par_repeatedly`) |

Only Evolve performs true speciation (seeding a final run with best genes from
prior runs). Each run starts from a random population.

Both `.call()` and `.build()` return `Result<_, TryFromEvolveBuilderError>`.
Builder validation catches: missing required fields and missing ending conditions.
Incompatible genotype + crossover combinations are caught at compile time via
trait bounds (`SupportsGeneCrossover`, `SupportsPointCrossover`).

## Common Mistakes

```rust
// WRONG: UniqueGenotype + CrossoverUniform = COMPILE ERROR
// FIX:   Use CrossoverClone or CrossoverRejuvenate

// WRONG: No ending condition = COMPILE/BUILD ERROR
// FIX:   Add .with_max_stale_generations(1000)

// WRONG: Fitness returns f64 = TYPE ERROR
// FIX:   Return Some((score / precision) as FitnessValue)

// WRONG: MutateSingleGene(0.2) with 1000+ float genes = DIVERSITY COLLAPSE
// FIX:   Use MutateMultiGene with higher mutation count, see Troubleshooting
```

## Retrieving Results

These methods are available on all strategy types (`Evolve`, `HillClimb`,
`Permutate`):

```rust
// Best genes and fitness score (returns None if no valid fitness was found)
let (best_genes, best_fitness_score) = evolve.best_genes_and_fitness_score().unwrap();

// Or separately (gene type depends on genotype, e.g. Vec<bool> for BinaryGenotype)
let best_genes = evolve.best_genes();
let best_fitness_score = evolve.best_fitness_score();

// Generation when best was found (available on all strategies)
let best_generation = evolve.best_generation();
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

`FitnessChromosome<Self>` is `Chromosome<Allele>` — access genes via
`chromosome.genes`. `FitnessGenotype<Self>` provides genotype metadata (allele
ranges, lists).

**Prefer penalties over `None`:** Return `Some(score_with_penalty)` rather than
`None` for invalid solutions. `None` provides no gradient signal and ranks last
unconditionally, while penalties let the algorithm converge incrementally out of
invalid space.

The fitness struct must implement `Clone + Send + Sync + Debug`. Most structs
auto-derive `Send + Sync`; use `Arc` instead of `Rc` if you need shared references.

**Why `&mut self`?** The `calculate_for_chromosome` method takes `&mut self` so
you can pre-allocate buffers and reuse them across evaluations for performance.
For simple fitness functions, just ignore the mutability. When using
`par_fitness(true)`, each thread gets its own clone via `ThreadLocal`.

### Using Custom Allele Types

`ListGenotype<T>` and `UniqueGenotype<T>` accept custom types as alleles. Allele
trait bounds: `Clone + Copy + Send + Sync + Debug`. Additionally, types must
implement `Hash` (for genes hashing) via the `impl_allele!` macro.

```rust
use genetic_algorithm::strategy::evolve::prelude::*;

#[derive(Clone, Copy, PartialEq, Hash, Debug)]
enum Color { Red, Green, Blue }
genetic_algorithm::impl_allele!(Color);

let genotype = ListGenotype::<Color>::builder()
    .with_genes_size(10)
    .with_allele_list(vec![Color::Red, Color::Green, Color::Blue])
    .build()
    .unwrap();
```

### Genotype Builder Options

**Gene size derivation:** `UniqueGenotype`, `MultiUniqueGenotype`,
`MultiListGenotype`, and `MultiRangeGenotype` derive `genes_size` automatically.
Setting `with_genes_size()` to a conflicting value is a build error.
`BinaryGenotype`, `ListGenotype`, and `RangeGenotype` require explicit
`with_genes_size(n)`.

**Builder method naming:** Use `with_allele_list` (singular) for `ListGenotype`
and `UniqueGenotype`. Use `with_allele_lists` (plural) for `MultiListGenotype`
and `MultiUniqueGenotype`. Use `with_allele_ranges` for `MultiRangeGenotype`.
Using the wrong variant gives a helpful build error.

All genotype builders support these optional settings:

- `.with_genes_hashing(true)` (default) — required for `fitness_cache` and
  `ExtensionMassDeduplication`. Auto-disabled by HillClimb (unless `fitness_cache` is set).
- `.with_chromosome_recycling(true)` (default) — reuses chromosome memory
  allocations. Generally leave at default.

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

### Discrete Selection (ListGenotype)

```rust
use genetic_algorithm::strategy::evolve::prelude::*;

// Select best material for each of 5 components to maximize total strength
const MATERIALS: [&str; 4] = ["steel", "aluminum", "titanium", "carbon"];
const STRENGTH: [[isize; 4]; 5] = [
    // steel, aluminum, titanium, carbon — strength per component
    [80, 40, 95, 70],
    [60, 50, 85, 90],
    [75, 30, 90, 65],
    [55, 45, 80, 95],
    [70, 60, 75, 85],
];

#[derive(Clone, Debug)]
struct MaterialFitness;
impl Fitness for MaterialFitness {
    type Genotype = ListGenotype<usize>;
    fn calculate_for_chromosome(
        &mut self,
        chromosome: &FitnessChromosome<Self>,
        _genotype: &FitnessGenotype<Self>,
    ) -> Option<FitnessValue> {
        let total: isize = chromosome.genes.iter().enumerate()
            .map(|(component, &material)| STRENGTH[component][material])
            .sum();
        Some(total)
    }
}

fn main() {
    let genotype = ListGenotype::<usize>::builder()
        .with_genes_size(5)
        .with_allele_list((0..MATERIALS.len()).collect())
        .build()
        .unwrap();

    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(100)
        .with_max_stale_generations(100)
        .with_fitness(MaterialFitness)
        .with_select(SelectTournament::new(0.5, 0.02, 4))
        .with_crossover(CrossoverUniform::new(0.7, 0.8))
        .with_mutate(MutateSingleGene::new(0.2))
        .call()
        .unwrap();

    let (best_genes, best_fitness_score) = evolve.best_genes_and_fitness_score().unwrap();
    let names: Vec<_> = best_genes.iter().map(|&i| MATERIALS[i]).collect();
    println!("materials: {:?}, score: {}", names, best_fitness_score);
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
        let mut conflicts = 0;
        let n = chromosome.genes.len();
        for i in 0..n {
            for j in (i + 1)..n {
                let dy = chromosome.genes[i].abs_diff(chromosome.genes[j]) as usize;
                if dy == j - i { conflicts += 1; }
            }
        }
        Some(conflicts)
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

### Traveling Salesman (HillClimb with call_repeatedly)

```rust
use genetic_algorithm::strategy::hill_climb::prelude::*;

// Distance matrix for 5 cities
const DISTANCES: [[isize; 5]; 5] = [
    [0, 10, 15, 20, 25],
    [10, 0, 35, 25, 30],
    [15, 35, 0, 30, 20],
    [20, 25, 30, 0, 15],
    [25, 30, 20, 15, 0],
];

#[derive(Clone, Debug)]
struct TspFitness;
impl Fitness for TspFitness {
    type Genotype = UniqueGenotype<u8>;
    fn calculate_for_chromosome(
        &mut self,
        chromosome: &FitnessChromosome<Self>,
        _genotype: &FitnessGenotype<Self>,
    ) -> Option<FitnessValue> {
        let genes = &chromosome.genes;
        let mut distance: isize = 0;
        for i in 0..genes.len() {
            let from = genes[i] as usize;
            let to = genes[(i + 1) % genes.len()] as usize;
            distance += DISTANCES[from][to];
        }
        Some(distance)
    }
}

fn main() {
    let genotype = UniqueGenotype::builder()
        .with_allele_list((0..5u8).collect())
        .build()
        .unwrap();

    // call_repeatedly returns (best_run, remaining_runs) tuple
    let (best, _rest) = HillClimb::builder()
        .with_genotype(genotype)
        .with_variant(HillClimbVariant::SteepestAscent)
        .with_fitness(TspFitness)
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_max_stale_generations(2)
        .call_repeatedly(10)
        .unwrap();

    let (best_genes, best_fitness_score) = best.best_genes_and_fitness_score().unwrap();
    println!("route: {:?}, distance: {}", best_genes, best_fitness_score);
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

### Multi-Group Assignment (MultiUniqueGenotype)

```rust
use genetic_algorithm::strategy::evolve::prelude::*;

// Assign 6 workers to 2 shifts, 3 per shift. Minimize total cost.
const COSTS: [[isize; 3]; 2] = [
    // Worker costs per position within each shift
    [10, 20, 15],  // Shift 0
    [25, 10, 30],  // Shift 1
];

#[derive(Clone, Debug)]
struct ShiftFitness;
impl Fitness for ShiftFitness {
    type Genotype = MultiUniqueGenotype<usize>;
    fn calculate_for_chromosome(
        &mut self,
        chromosome: &FitnessChromosome<Self>,
        _genotype: &FitnessGenotype<Self>,
    ) -> Option<FitnessValue> {
        // genes is a flat Vec with group boundaries: [shift0_w0, shift0_w1, shift0_w2, shift1_w0, ...]
        let mut cost = 0;
        for (i, &worker) in chromosome.genes.iter().enumerate() {
            let shift = i / 3;
            let position = i % 3;
            cost += COSTS[shift][position] * worker as isize;
        }
        Some(cost)
    }
}

fn main() {
    let workers: Vec<usize> = (0..6).collect();
    // Each shift draws from the same pool; workers are unique within each group
    let allele_lists = vec![workers.clone(), workers];

    let genotype = MultiUniqueGenotype::builder()
        .with_allele_lists(allele_lists) // note: with_allele_lists (plural)
        .build()
        .unwrap();

    let (evolve, _) = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(100)
        .with_max_stale_generations(1000)
        .with_fitness(ShiftFitness)
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_select(SelectTournament::new(0.5, 0.02, 4))
        .with_crossover(CrossoverSinglePoint::new(0.7, 0.8)) // not Uniform (compile error)
        .with_mutate(MutateSingleGene::new(0.2))
        .call_repeatedly(10)
        .unwrap();

    let (best_genes, best_fitness_score) = evolve.best_genes_and_fitness_score().unwrap();
    println!("assignment: {:?}, cost: {}", best_genes, best_fitness_score);
}
```

### Per-Gene Categorical (MultiListGenotype)

```rust
use genetic_algorithm::strategy::evolve::prelude::*;

// Schedule 3 tasks to time slots. Each task has different valid slots.
#[derive(Clone, Debug)]
struct ScheduleFitness;
impl Fitness for ScheduleFitness {
    type Genotype = MultiListGenotype<usize>;
    fn calculate_for_chromosome(
        &mut self,
        chromosome: &FitnessChromosome<Self>,
        _genotype: &FitnessGenotype<Self>,
    ) -> Option<FitnessValue> {
        // Reward distinct slots (penalize collisions)
        let mut used = std::collections::HashSet::new();
        let mut score: isize = 0;
        for &slot in &chromosome.genes {
            if used.insert(slot) {
                score += 10; // unique slot
            } else {
                score -= 5; // collision penalty
            }
        }
        Some(score)
    }
}

fn main() {
    let genotype = MultiListGenotype::<usize>::builder()
        .with_allele_lists(vec![ // note: with_allele_lists (plural)
            vec![0, 1, 2],    // Task 0 can go in slots 0, 1, or 2
            vec![1, 2, 3],    // Task 1 can go in slots 1, 2, or 3
            vec![0, 3],       // Task 2 can only go in slots 0 or 3
        ])
        .build()
        .unwrap();

    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(100)
        .with_max_stale_generations(100)
        .with_fitness(ScheduleFitness)
        .with_select(SelectTournament::new(0.5, 0.02, 4))
        .with_crossover(CrossoverUniform::new(0.7, 0.8)) // Uniform works (implements SupportsGeneCrossover)
        .with_mutate(MutateSingleGene::new(0.2))
        .call()
        .unwrap();

    let (best_genes, best_fitness_score) = evolve.best_genes_and_fitness_score().unwrap();
    println!("schedule: {:?}, score: {}", best_genes, best_fitness_score);
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
- `par_fitness` has no effect with `HillClimbVariant::Stochastic` (sequential by
  nature). Use `call_par_repeatedly` to parallelize independent Stochastic runs.

**Getting `None` as best fitness?**
- All chromosomes returned `None` from `calculate_for_chromosome`
- Check your fitness function's validity constraints — they may be too strict
- Increase population size so some valid solutions appear in the initial population
- Prefer large penalties over `None` (see "Implementing Custom Fitness")

**Multi-objective optimization?**
This library optimizes a single `FitnessValue`. For multiple objectives, combine
them into a weighted sum: `Some(((w1 * obj1 + w2 * obj2) / precision) as FitnessValue)`.
Adjust weights to control tradeoffs.

## Gotchas

1. **FitnessValue is isize.** Scale floats manually: `(score / precision) as FitnessValue`.
2. **Ending condition required.** Evolve/HillClimb need at least one of: `target_fitness_score`, `max_stale_generations`, `max_generations`.
3. **Fitness struct must be `Clone + Send + Sync + Debug`.** Most structs auto-derive `Send + Sync`; use `Arc` instead of `Rc` if needed.
4. **Permutate + RangeGenotype** requires `MutationType::Step`, `StepScaled`, or `Discrete`.
5. **`target_population_size` defaults to 100.** Override with `.with_target_population_size(n)` if needed.
6. **Custom Crossover/Mutate/Extension must call `chromosome.reset_metadata(genotype.genes_hashing)`** after modifying genes directly.
7. **For deterministic tests:** use `.with_rng_seed_from_u64(0)`. Exact results may change between library versions, but deterministic within a version.
