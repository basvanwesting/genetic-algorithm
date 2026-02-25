# AGENTS.md — AI Agent Guide for genetic-algorithm

This file helps AI coding agents use this library correctly. It covers decision
guidance, API reference, gotchas, and copy-paste templates.

## Table of Contents

- [Quick Start](#quick-start)
- [Critical: FitnessValue is isize](#critical-fitnessvalue-is-isize)
- [Copy-Paste Templates](#copy-paste-templates) (more in [AGENTS_TEMPLATES.md](https://github.com/basvanwesting/genetic-algorithm/blob/main/AGENTS_TEMPLATES.md))
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
- [Troubleshooting](#troubleshooting)
- [Gotchas](#gotchas)

## Quick Start

Add to your `Cargo.toml`:
```toml
[dependencies]
genetic_algorithm = "0.27.0"
```

```rust,ignore
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

**Critical gotchas** (see [Gotchas](#gotchas) for full list):
1. `FitnessValue` is `isize`, not `f64`. Scale floats: `Some((score / precision) as FitnessValue)`.
2. Evolve/HillClimb require at least one ending condition (`target_fitness_score`, `max_stale_generations`, or `max_generations`).

## Critical: FitnessValue is isize

`FitnessValue` is `isize`, **not** `f64`. This is by design: isize enables
equality checks needed for staleness detection (stale generations are detected
when the best fitness score stops changing).

**This library uses f32 as the default float type** (e.g. `RangeGenotype<f32>`,
`MultiRangeGenotype<f32>`). GAs don't need f64 precision — the stochastic search
process dominates any floating-point rounding.

For float-based fitness, scale to isize manually:

```rust,ignore
// divide by desired precision, then cast
let precision = 1e-5;
Some((score / precision) as FitnessValue)

// or use the helper function (accepts f32 and f64)
Some(fitness_value(score, precision))
```

Return `None` from `calculate_for_chromosome` to mark a chromosome as invalid
(it will rank last in selection regardless of fitness ordering).

## Copy-Paste Templates

Minimal end-to-end example showing the general flow (genotype → fitness → strategy → result):

```rust,ignore
use genetic_algorithm::strategy::evolve::prelude::*;

#[derive(Clone, Debug)]
struct CountTrue;
impl Fitness for CountTrue {
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
        .with_genes_size(10)
        .build()
        .unwrap();

    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(100)
        .with_max_stale_generations(100)
        .with_fitness(CountTrue)
        .with_select(SelectTournament::new(0.5, 0.02, 4))
        .with_crossover(CrossoverUniform::new(0.7, 0.8))
        .with_mutate(MutateSingleGene::new(0.2))
        .call()
        .unwrap();

    let (best_genes, best_fitness_score) = evolve.best_genes_and_fitness_score().unwrap();
    println!("genes: {:?}, score: {}", best_genes, best_fitness_score);
}
```

See [AGENTS_TEMPLATES.md](https://github.com/basvanwesting/genetic-algorithm/blob/main/AGENTS_TEMPLATES.md) for more examples covering all
genotype types, strategies, and patterns (call_repeatedly, HillClimb, Permutate, etc.).

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
| `UniqueGenotype<T>` | `Vec<T>` | Default T = usize, positional permutation (swap-only mutation), values do not have to be unique, they are only treated as such |
| `RangeGenotype<T>` | `Vec<T>` | Default T = f32 |
| `MultiListGenotype<T>` | `Vec<T>` | Default T = usize, each gene has its own set of possible values |
| `MultiUniqueGenotype<T>` | `Vec<T>` | Default T = usize, positional permutation (swap-only mutation) within each group, values do not have to be unique, they are only treated as such |
| `MultiRangeGenotype<T>` | `Vec<T>` | Default T = f32, each gene has its own range |

### Which Strategy?

| Situation | Strategy | Why |
|---|---|---|
| General optimization | `Evolve` | Full GA with crossover + mutation |
| Convex search space | `HillClimb` | Local search suffices |
| Small search space | `Permutate` | Exhaustive, 100% guarantee |

**Scaling warning:** Permutate evaluates all possible gene combinations in a stream, so no memory issues, but serious duration issues

### Which HillClimb Variant?

| Situation | Variant | Why |
|---|---|---|
| Large genome | `Stochastic` (default) | One random neighbor per step, fast iterations |
| Small genome | `SteepestAscent` | Evaluates all neighbors, guarantees best local move |
| Plateau traversal needed | `Stochastic` + high `max_stale_generations` | Stochastic can escape via `replace_on_equal_fitness` |
| Exact local optimum needed | `SteepestAscent` + `call_repeatedly(n)` | Deterministic per start, multiple random restarts |

**Scaling warning:** SteepestAscent evaluates n*(n-1)/2 neighbors for
`UniqueGenotype` of size n (e.g., 2016 neighbors for n=64). Use Stochastic
with `call_repeatedly` for genomes >20 genes.

### Which Crossover? (Evolve only)

| Genotype | Compatible Crossovers | Recommended |
|---|---|---|
| `BinaryGenotype` | All | `CrossoverUniform` or `CrossoverSinglePoint` |
| `ListGenotype<T>` | All | `CrossoverUniform` or `CrossoverSinglePoint` |
| `MultiListGenotype<T>` | All | `CrossoverUniform` or `CrossoverSinglePoint` |
| `UniqueGenotype<T>` | `CrossoverClone`, `CrossoverRejuvenate` ONLY (others are compile errors) | `CrossoverClone` |
| `MultiUniqueGenotype<T>` | Point-based + `CrossoverClone`, `CrossoverRejuvenate` (gene-based are compile errors) | `CrossoverSinglePoint` |
| `RangeGenotype<T>` | All | `CrossoverUniform` or `CrossoverSinglePoint` |
| `MultiRangeGenotype<T>` | All | `CrossoverUniform` or `CrossoverSinglePoint` |

**Compile-time safety**: `UniqueGenotype` does not implement `SupportsGeneCrossover`
or `SupportsPointCrossover`, so incompatible crossovers are **compile errors**.
Use `CrossoverClone` (clones parents, relies on mutation for diversity) or
`CrossoverRejuvenate` (like Clone but optimized for less memory copying).
`MultiUniqueGenotype` implements `SupportsPointCrossover` only, so gene-based
crossovers (`CrossoverUniform`, `CrossoverSingleGene`, `CrossoverMultiGene`) are
compile errors.

**Note on UniqueGenotype + Evolve:** `CrossoverClone` with `UniqueGenotype`
produces valid code but is almost always less efficient than `HillClimb` +
`call_repeatedly(n)`. Only use `Evolve` + `CrossoverClone` for `UniqueGenotype`
when you need Extensions or speciation.

### Which Select?

| Type | When to use |
|---|---|
| `SelectElite` | Deterministic, sorts by fitness. Less diversity. |
| `SelectTournament` | Default choice. When you want stochastic pressure. Better diversity. |

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
```rust,ignore
// also requires: genotype, fitness, target_population_size, ending condition
.with_select(SelectTournament::new(0.5, 0.02, 4))
.with_crossover(CrossoverUniform::new(0.7, 0.8))
.with_mutate(MutateSingleGene::new(0.2))
```

For range/float genotypes (>50 genes, see Troubleshooting for tuning):
```rust,ignore
// also requires: genotype, fitness, target_population_size, ending condition
.with_select(SelectTournament::new(0.5, 0.02, 4))
.with_crossover(CrossoverUniform::new(0.7, 0.8))
.with_mutate(MutateMultiGene::new(10, 0.8))
```

For unique genotypes (swap-only problems):
```rust,ignore
// also requires: genotype, fitness, target_population_size, ending condition
.with_select(SelectTournament::new(0.5, 0.02, 4))
.with_crossover(CrossoverClone::new(0.7))
.with_mutate(MutateSingleGene::new(0.8))
```

## Constructor Parameter Reference

### Select

```rust,ignore
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

```rust,ignore
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

**Rate guidance depends on genotype size and type — see "Mutation tuning" in
Troubleshooting.** For float genomes >50 genes: use `MutateMultiGene` with
`mutation_probability` near 1.0 and scale `number_of_mutations` with genome
size. The "typical" mutation rates assume small genomes. For large genomes
(hundreds to thousands of genes), the effective per-gene mutation rate matters
more than the per-chromosome probability. Think in terms of what fraction of
all genes in the population actually change per generation.

```rust,ignore
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
mutation/selection settings. Escalation order: `MassDegeneration` (most bang for buck)
→ `MassDeduplication`/`MassExtinction`/`MassGenesis` (mostly for completeness and the cambrian explosion metaphor).

```rust,ignore
ExtensionMassExtinction::new(
    cardinality_threshold: usize,  // Trigger when unique chromosomes drop below this.
    survival_rate: f32,            // Fraction that survives (random selection + elite).
    elitism_rate: f32,             // Fraction of elite preserved before random reduction.
)
// Randomly trims population. Recovery happens naturally through offspring in following generations.

ExtensionMassGenesis::new(
    cardinality_threshold: usize,  // Trims to only 2 unique best (Adam & Eve).
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
- `.with_extension(extension)` — diversity management (fallback, hyperparameters smell)
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
- At least ONE ending condition (see below)

Ending conditions (at least one required):
- `.with_target_fitness_score(score)` — stop when best fitness reaches this value
- `.with_max_stale_generations(n)` — stop after n generations without improvement
- `.with_max_generations(n)` — stop after n total generations

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

```rust,ignore
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
} else if is_permutation_problem {
    StrategyVariant::HillClimb(HillClimbVariant::Stochastic)
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

```rust,ignore
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
- `.call_par_repeatedly(n)` — parallel version, combine with with_par_fitness(false) to avoid double parallelization
- `.call_speciated(n)` — n species runs, then final run seeded with best genes
- `.call_par_speciated(n)` — parallel version, combine with with_par_fitness(false) to avoid double parallelization, but will hurt final run

```rust,ignore
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
| `call_repeatedly(n)` | Results vary across runs (local optima). Typical n: 10. |
| `call_par_repeatedly(n)` | Parallel version of above. Beware of double parallelization with_par_fitness |
| `call_speciated(n)` | Multiple runs seed a final refinement pass. Best for complex combinatorial problems, where characteristics of alternative refined solutions need combining |
| `call_par_speciated(n)` | Parallel version of above. Beware of double parallelization with_par_fitness |

**Call variant availability by builder:**

| Variant | `Evolve::builder()` | `HillClimb::builder()` | `Permutate::builder()` | `StrategyBuilder` |
|---|---|---|---|---|
| `call()` | yes | yes | yes | yes |
| `call_repeatedly(n)` | yes | yes | no | yes |
| `call_par_repeatedly(n)` | yes | yes | no | yes |
| `call_speciated(n)` | yes | no | no | yes (falls back to `call_repeatedly`) |
| `call_par_speciated(n)` | yes | no | no | yes (falls back to `call_par_repeatedly`) |

Only Evolve performs true speciation (seeding a final run with best genes from
prior runs). Each run before the final run starts from a random population.

Permutate always falls back to `call()` for all variants.

Both `.call()` and `.build()` return `Result<_, TryFromEvolveBuilderError>`.
Builder validation catches: missing required fields and missing ending conditions.
Incompatible genotype + crossover combinations are caught at compile time via
trait bounds (`SupportsGeneCrossover`, `SupportsPointCrossover`).

## Common Mistakes

```text
WRONG: UniqueGenotype + CrossoverUniform = COMPILE ERROR
FIX:   Use CrossoverClone or CrossoverRejuvenate

WRONG: No ending condition = COMPILE/BUILD ERROR
FIX:   Add .with_max_stale_generations(1000)

WRONG: Fitness returns f64 = TYPE ERROR
FIX:   Return Some((score / precision) as FitnessValue)

WRONG: MutateSingleGene(0.2) with 1000+ float genes = DIVERSITY COLLAPSE
FIX:   Use MutateMultiGene with higher mutation count, see Troubleshooting
```

## Retrieving Results

These methods are available on all strategy types (`Evolve`, `HillClimb`,
`Permutate`):

```rust,ignore
// Best genes and fitness score (returns None if no valid fitness was found)
let (best_genes, best_fitness_score) = evolve.best_genes_and_fitness_score().unwrap();

// Or separately (gene type depends on genotype, e.g. Vec<bool> for BinaryGenotype)
let best_genes = evolve.best_genes();
let best_fitness_score = evolve.best_fitness_score();

// Generation when best was found (available on all strategies)
let best_generation = evolve.best_generation();
```

## Implementing Custom Fitness

```rust,ignore
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

```rust,ignore
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

| Genotype | Required Builder Methods | genes_size |
|---|---|---|
| `BinaryGenotype` | `with_genes_size(n)` | explicit |
| `ListGenotype<T>` | `with_genes_size(n)`, `with_allele_list(vec)` | explicit |
| `UniqueGenotype<T>` | `with_allele_list(vec)` | derived from list length |
| `RangeGenotype<T>` | `with_genes_size(n)`, `with_allele_range(range)` | explicit |
| `MultiListGenotype<T>` | `with_allele_lists(vec)` | derived from lists length |
| `MultiUniqueGenotype<T>` | `with_allele_lists(vec)` | derived from lists length |
| `MultiRangeGenotype<T>` | `with_allele_ranges(vec)` | derived from ranges length |

**Notes:** Setting `with_genes_size()` to a value conflicting with the derived
size is a build error. Use `with_allele_list` (singular) for `ListGenotype` and
`UniqueGenotype`; use `with_allele_lists` (plural) for `Multi*` variants. Using
the wrong variant gives a helpful build error.

All genotype builders support these optional settings:

- `.with_genes_hashing(true)` (default) — required for `fitness_cache` and
  `Extension` & `Mutation` cardinality_thresholds. Auto-disabled by HillClimb (unless `fitness_cache` is set).
- `.with_chromosome_recycling(true)` (default) — reuses chromosome memory
  allocations. Generally leave at default.

## Troubleshooting

**Fitness not improving?**
- Increase `target_population_size` (more diversity)
- Increase `mutation_probability` (more exploration)
- Try `MutateSingleGeneDynamic` or `MutateMultiGeneDynamic` (auto-adjusts)
- For `RangeGenotype`/`MultiRangeGenotype`: use `MutationType::RangeScaled` for
  progressive refinement instead of `MutationType::Random`
- Add an extension like `ExtensionMassDegeneration` to
  escape local optima when diversity drops

**Mutation tuning**

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
- Use `.with_fitness_cache(size)` if many chromosomes share the same genes (is hyperparameter smell though)
- Reduce `target_population_size` if fitness is cheap but framework overhead is high
- For `HillClimb::SteepestAscent`: the neighbourhood can be very large, consider
  `Stochastic` variant instead
- `par_fitness` has no effect with `HillClimbVariant::Stochastic` (sequential by
  nature). Use `call_par_repeatedly` to parallelize independent Stochastic runs. Disable `par_fitness` to avoid double parallelization.

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
