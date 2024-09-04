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
* Genes: storage trait of the genes for a chromosome, mostly `Vec<Allele>`, but alternatives possible
* Genotype: Knows how to generate, mutate and crossover chromosomes efficiently
* Fitness: knows how to determine the fitness of a chromosome

All multithreading mechanisms are implemented using
[rayon::iter](https://docs.rs/rayon/latest/rayon/iter/index.html) and
[std::sync::mpsc](https://doc.rust-lang.org/1.78.0/std/sync/mpsc/index.html).

## Documentation

See [docs.rs](https://docs.rs/genetic_algorithm/latest/genetic_algorithm)

## Quick Usage

```rust
use genetic_algorithm::strategy::evolve::prelude::*;

// the search space
let genotype = BinaryGenotype::builder() // boolean alleles
    .with_genes_size(100)                // 100 genes per chromosome
    .build()
    .unwrap();

println!("{}", genotype);

// the search goal to optimize towards (maximize or minimize)
#[derive(Clone, Debug)]
pub struct CountTrue;
impl Fitness for CountTrue {
   type Genotype = BinaryGenotype; // Genes = Vec<bool>
      fn calculate_for_chromosome(&mut self, chromosome: &Chromosome<Self::Genotype>) -> Option<FitnessValue> {
         Some(chromosome.genes.iter().filter(|&value| *value).count() as FitnessValue)
      }
   }
}

// the search strategy
let evolve = Evolve::builder()
    .with_genotype(genotype)
    .with_compete(CompeteElite::new(0.9))          // sort the chromosomes by fitness to determine crossover order and select 90% of the population for crossover (drop 10% of population)
    .with_crossover(CrossoverUniform::new())       // crossover all individual genes between 2 chromosomes for offspring (and restore back to 100% of target population size by keeping the best parents alive)
    .with_mutate(MutateSingleGene::new(0.2))       // mutate offspring for a single gene with a 20% probability per chromosome
    .with_fitness(CountTrue)                       // count the number of true values in the chromosomes
    .with_target_population_size(100)              // evolve with 100 chromosomes
    .with_target_fitness_score(100)                // goal is 100 times true in the best chromosome
    .with_reporter(EvolveReporterSimple::new(100)) // optional builder step, report every 100 generations
    .call();
    .unwrap()

println!("{}", evolve);
```

## Examples
Run with `cargo run --example [EXAMPLE_BASENAME] --release`

* N-Queens puzzle https://en.wikipedia.org/wiki/Eight_queens_puzzle.
    * See [examples/evolve_nqueens](../main/examples/evolve_nqueens.rs)
    * See [examples/hill_climb_nqueens](../main/examples/hill_climb_nqueens.rs)
    * `UniqueGenotype<u8>` with a 64x64 chess board setup
    * custom `NQueensFitness` fitness
* Knapsack problem: https://en.wikipedia.org/wiki/Knapsack_problem
    * See [examples/evolve_knapsack](../main/examples/evolve_knapsack.rs)
    * See [examples/permutate_knapsack](../main/examples/permutate_knapsack.rs)
    * `BinaryGenotype<Item(weight, value)>` each gene encodes presence in the knapsack
    * custom `KnapsackFitness(&items, weight_limit)` fitness
* Infinite Monkey theorem: https://en.wikipedia.org/wiki/Infinite_monkey_theorem
    * See [examples/evolve_monkeys](../main/examples/evolve_monkeys.rs)
    * `ListGenotype<char>` 100 monkeys randomly typing characters in a loop
    * custom fitness using hamming distance
* Permutation strategy instead of Evolve strategy for small search spaces, with a 100% guarantee
    * See [examples/permutate_knapsack](../main/examples/permutate_knapsack.rs)
* HillClimb strategy instead of Evolve strategy, when crossover is impossible or inefficient
    * See [examples/hill_climb_nqueens](../main/examples/hill_climb_nqueens.rs)
    * See [examples/hill_climb_table_seating](../main/examples/hill_climb_table_seating.rs)
* Explore vector genes (BinaryGenotype) versus other storage (BitGenotype)
    * See [examples/evolve_bit_v_binary](../main/examples/evolve_bit_v_binary.rs)
* Explore internal and external multithreading options
    * See [examples/explore_multithreading](../main/examples/explore_multithreading.rs)
* Custom Fitness function with LRU cache
    * See [examples/evolve_binary_lru_cache_fitness](../main/examples/evolve_binary_lru_cache_fitness.rs)
    * _Note: doesn't help performance much in this case..._
* Custom Reporting implementation
    * See [examples/permutate_scrabble](../main/examples/permutate_scrabble.rs)

## Performance considerations

For the Evolve strategy:

* Compete: no considerations. All competes are basically some form of in-place
  sorting of some kind. This is relatively fast compared to the rest of the
  operations.
* Crossover: the workhorse of internal parts. Crossover touches most genes each
  generation and clones up to the whole population if you keep all the parents
  around. See performance tips below.
* Mutate: no considerations. It touches genes like crossover does, but should
  be used sparingly anyway; with low gene counts (<10%) and low probability (5-20%)
* Fitness: can be anything. This fully depends on the user domain. Parallelize
  it using `with_par_fitness()` in the Builder. But beware that parallelization
  has it's own overhead and is not always faster.

**Performance Tips**
* Small genes sizes
  * It seems that CrossoverMultiGene with `number_of_crossovers = genes_size / 2`
  and `allow_duplicates = true` is the best tradeoff between performance and
  effect. CrossoverUniform is an alias for the same approach, taking the
  genes_size from the genotype at runtime.
  * Keeping the parents around doesn't matter that much as the cloning is relatively less
  pronounced (but becomes more prominent for larger population sizes)
* Large genes sizes
  * It seems that CrossoverMultiPoint with `number_of_crossovers = genes_size / 9` 
  and `allow_duplicates = false` is the best tradeoff between performance and effect.
  * Keeping the parents around has major performance effects and should be avoided. Use a high
  selection_rate or even 100%, so there is little parent cloning. Explore non-Vec based
  genotypes like BitGenotype.

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

## MAYBE
* Target cardinality range for Mutate Dynamic to avoid constant switching
* Default max_stale_generations to 1 for SteepestAscent
* Add scaling permutate? Can be done by grid search and then search within last grid with new scale
* Add scaling helper function
* Add simulated annealing strategy
* Add Roulette competition with and without duplicates (with fitness ordering)
* Add OrderOne crossover for UniqueGenotype?
* Add WholeArithmetic crossover for RangeGenotype?
* Add CountTrueWithWork instead of CountTrueWithSleep for better benchmarks?
* Explore more non-Vec genes: PackedSimd, ArrayVec

## ISSUES
* permutate (and possibly others) with gene_size 0 panics. Maybe it should just return a empty chromosome?
