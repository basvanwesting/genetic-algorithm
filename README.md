# genetic-algorithm

[![Crates.io MSRV][cmb]][cml]
[![Crates.io Version][cvb]][cvl]
[![Rust][rub]][rul]
[![Crates.io License][clb]][cll]

[cmb]: https://img.shields.io/crates/msrv/genetic-algorithm
[cvb]: https://img.shields.io/crates/v/genetic-algorithm
[clb]: https://img.shields.io/crates/l/genetic-algorithm
[rub]: https://github.com/basvanwesting/genetic-algorithm/actions/workflows/rust.yml/badge.svg

[cml]: https://www.rust-lang.org
[cvl]: https://crates.io/crates/genetic_algorithm
[cll]: COPYRIGHT.txt
[rul]: https://github.com/basvanwesting/genetic-algorithm/actions/workflows/rust.yml

A genetic algorithm implementation for Rust.
Inspired by the book [Genetic Algorithms in Elixir](https://pragprog.com/titles/smgaelixir/genetic-algorithms-in-elixir/)

Experimental branch with Centralized genetic algorithms with population-wide gene storage

Use this module for:
* DynamicRange, StaticRange, StaticBinary genotypes
* GPU/SIMD-ready operations
* Maximum performance with large populations

Branch is archived for now, as zero-copy transfer of genes to GPU proved impractical in practice

There are three main elements to this approach:
* The Genotype (the search space)
* The Fitness function (the search goal)
* The strategy (the search strategy)
    * Evolve (evolution strategy)
    * Permutate (for small search spaces, with a 100% guarantee)
    * HillClimb (when search space is convex with little local optima or when crossover is impossible/inefficient)

Terminology:
* Population: a population has `population_size` number of individuals (called chromosomes).
* Chromosome: a chromosome has `genes_size` number of genes
* Allele: alleles are the possible values of the genes
* Gene: a gene is a combination of position in the chromosome and value of the gene (allele)
* Genes: storage trait of the genes for a chromosome
* Genotype: Knows how to generate, mutate and crossover chromosomes efficiently and holds all the genes in a centralized manner
* Fitness: knows how to determine the fitness of a chromosome

All multithreading mechanisms are implemented using
[rayon::iter](https://docs.rs/rayon/latest/rayon/iter/index.html) and
[std::sync::mpsc](https://doc.rust-lang.org/1.78.0/std/sync/mpsc/index.html).

## Documentation

See [docs.rs](https://docs.rs/genetic_algorithm/latest/genetic_algorithm)

## Quick Usage

```rust
use genetic_algorithm::strategy::evolve::prelude::*;

const GENES_SIZE: usize = 100;
const POPULATION_SIZE: usize = 200;

// the search space
let genotype = StaticBinaryGenotype::<GENES_SIZE, POPULATION_SIZE>::builder() // boolean alleles (100 genes, 100 pop)
    .with_genes_size(GENES_SIZE)                                              // 100 genes per chromosome
    .build()
    .unwrap();

println!("{}", genotype);

// the search goal to optimize towards (maximize or minimize)
#[derive(Clone, Debug)]
pub struct CountTrue;
impl Fitness for CountTrue {
    type Genotype = StaticBinaryGenotype::<GENES_SIZE, POPULATION_SIZE>; // Genes = Vec<bool>
    fn calculate_for_population(
        &mut self,
        _population: &Population,
        genotype: &FitnessGenotype<Self>,
    ) -> Vec<Option<FitnessValue>> {
        // pure matrix data calculation on [[T; N] M]
        // the order of the rows needs to be preserved as it matches the row_id on the chromosome
        genotype
            .data
            .iter()
            .map(|genes| genes.iter().filter(|&value| *value).count() as FitnessValue)
            .map(Some)
            .collect()
    }
}

// the search strategy
let evolve = Evolve::builder()
    .with_genotype(genotype)
    .with_select(SelectElite::new(0.5, 0.02))         // sort the chromosomes by fitness to determine crossover order. Strive to replace 50% of the population with offspring. Allow 2% through the non-generational best chromosomes gate before selection and replacement
    .with_crossover(CrossoverUniform::new(0.7, 0.8))  // crossover all individual genes between 2 chromosomes for offspring with 70% parent selection (30% do not produce offspring) and 80% chance of crossover (20% of parents just clone)
    .with_mutate(MutateSingleGene::new(0.2))          // mutate offspring for a single gene with a 20% probability per chromosome
    .with_fitness(CountTrue)                          // count the number of true values in the chromosomes
    .with_fitness_ordering(FitnessOrdering::Maximize) // optional, default is Maximize, aim towards the most true values
    .with_target_population_size(100)                 // evolve with 100 chromosomes
    .with_target_fitness_score(100)                   // goal is 100 times true in the best chromosome
    .with_reporter(EvolveReporterSimple::new(100))    // optional builder step, report every 100 generations
    .call()
    .unwrap();

println!("{}", evolve);

// it's all about the best genes after all
let (best_genes, best_fitness_score) = evolve.best_genes_and_fitness_score().unwrap();
assert_eq!(best_genes, Box::new([true; 100]));
assert_eq!(best_fitness_score, 100);
```

## Examples
Run with `cargo run --example [EXAMPLE_BASENAME] --release`

* Knapsack problem: https://en.wikipedia.org/wiki/Knapsack_problem
    * See [examples/evolve_knapsack](../main/examples/evolve_knapsack.rs)
    * `StaticBinaryGenotype<Item(weight, value)>` each gene encodes presence in the knapsack
    * custom `KnapsackFitness(&items, weight_limit)` fitness
* HillClimb strategy instead of Evolve strategy, when crossover is impossible or inefficient
    * See [examples/hill_climb_range](../main/examples/hill_climb_range.rs)

## Performance considerations

For the Evolve strategy:

* Reporting: start with EvolveReporterSimple for basic understanding of:
  * fitness v. framework overhead
  * staleness and population characteristics (cardinality etc.)
* Select: no considerations. All selects are basically some form of in-place
  sorting of some kind. This is relatively fast compared to the rest of the
  operations.
* Crossover: the workhorse of internal parts. Crossover touches most genes each
  generation and clones up to the whole population to produce offspring
  (depending on selection-rate). It also calculates
  new genes hashes if enabled on the Genotype, which has a relatively high
  overhead on the main Evolve loop.
* Mutate: no considerations. It touches genes like crossover does, but should
  be used sparingly anyway; with low gene counts (<10%) and low probability (5-20%)
* Fitness: can be anything. This fully depends on the user domain.

**GPU acceleration**

Genes (N) and Population (M) are a stored in single contiguous memory range of
Alleles (T) with length N*M on the heap. A pointer to this data can be taken to
calculate the whole population at once.

Useful in the following strategies where a whole population is calculated:
* Evolve
* HillClimb-SteepestAscent

Possibly a GPU compatible memory layout still needs to be added. The current implementation
just provides all the basic building blocks to implement this. Please open a github issue for
further support.

## Tests
Run tests with `cargo test`

Use `.with_rng_seed_from_u64(0)` builder step to create deterministic tests results.

## Benchmarks
Implemented using criterion. Run benchmarks with `cargo bench`

## Profiling
Implemented using criterion and pprof.

Uncomment in Cargo.toml
```
[profile.release]
debug = 1
``````

Run with `cargo run --example profile_evolve_binary --release -- --bench --profile-time 5`

Find the flamegraph in: `./target/criterion/profile_evolve_binary/profile/flamegraph.svg`

## TODO
* One cannot permutate centralized static binary, yet. Need a window approach setting the matrix for each iteration. To calculate that matrix as a whole repeatedly

## MAYBE

## ISSUES
* hill_climb SteepestAscent actually has a population size requirement of
  neighbouring_population_size + 1, because of the working chromosome. This could
  overflow StaticRangeGenotype<T, N, M>, use StaticRangeGenotype<T, N, { M + 1 }>
  as workaround

