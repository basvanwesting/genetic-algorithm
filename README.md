# genetic-algorithm
A genetic algorithm implementation for Rust.
Inspired by the book [Genetic Algorithms in Elixir](https://pragprog.com/titles/smgaelixir/genetic-algorithms-in-elixir/)

There are three main elements to this approach:
* The [Genotype](#genotypes) (the search space)
* The [Fitness](#fitness) function (the search goal)
* The [Evolve](#evolve-strategy) strategy (the search strategy)

## Examples

* N-Queens puzzle https://en.wikipedia.org/wiki/Eight_queens_puzzle.
    * See [example/evolve_nqueens](../main/examples/evolve_nqueens.rs)
    * `UniqueDiscreteGenotype<u8>` with a 64x64 chess board setup and custom `NQueensFitness` fitness
    * `cargo run --example evolve_nqueens --release`
* Knapsack problem: https://en.wikipedia.org/wiki/Knapsack_problem
    * See [example/evolve_knapsack](../main/examples/evolve_knapsack.rs)
    * `DiscreteGenotype<(weight, value)>` with a custom `KnapsackFitness(weight_limit)` fitness
    * `cargo run --example evolve_knapsack --release`
* Infinite Monkey theorem: https://en.wikipedia.org/wiki/Infinite_monkey_theorem
    * See [example/evolve_monkeys](../main/examples/evolve_monkeys.rs)
    * `DiscreteGenotype<u8>` 100 monkeys randomly typing characters in a loop
    * `cargo run --example evolve_monkeys --release`
* Custom Fitness function with LRU cache
    * See [example/evolve_binary_lru_cache_fitness](../main/examples/evolve_binary_lru_cache_fitness.rs)
    * Note: doesn't help performance much in this case...
    * `cargo run --example evolve_binary_lru_cache_fitness --release`

## Quick Usage

```rust
// the search space
let genotype = BinaryGenotype::builder() // boolean genes
    .with_gene_size(100)                 // 100 of them
    .build()
    .unwrap();

println!("{}", genotype);

// the search goal to optimize towards (maximize or minimize)
#[derive(Clone, Debug)]
pub struct SimpleCount;
impl Fitness for SimpleCount {
    type Genotype = BinaryGenotype;
    fn call_for_chromosome(&mut self, chromosome: &Chromosome<Self::Genotype>) -> Option<FitnessValue> {
        Some(chromosome.genes.iter().filter(|&value| *value).count() as FitnessValue)
    }
}

// the search strategy
let mut rng = SmallRng::from_entropy();
let evolve = Evolve::builder()
    .with_genotype(genotype)
    .with_population_size(100)          // evolve with 100 chromosomes
    .with_target_fitness_score(100)     // goal is 100 times true in the best chromosome
    .with_crossover(CrossoverAll(true)) // crossover all individual genes between 2 chromosomes for offspring
    .with_mutate(MutateOnce(0.2))       // mutate a single gene with a 20% probability per chromosome
    .with_fitness(SimpleCount)          // count the number of true values in the chromosomes
    .with_compete(CompeteElite)         // sort the chromosomes by fitness to determine crossover order
    .build()
    .unwrap()
    .call(&mut rng);

println!("{}", evolve);
```

## Genotypes
The following genotypes are implemented:

* `BinaryGenotype`
    * List of true|false values with 50% probability
    * Builder `with_gene_size(usize)`, mandatory, number of genes per chromosome
    * Permutable
* `DiscreteGenotype<T>`
    * List of items with uniform probability
    * Builder `with_gene_size(usize)`, mandatory, number of genes per chromosome
    * Builder `with_gene_values(Vec<T>)`, mandatory, possible values for genes
    * Permutable
* `UniqueDiscreteGenotype<T>`
    * List of items with uniform probability, each item occurs exactly once
    * Builder `with_gene_values(Vec<T>)`, mandatory, possible values for genes
    * Builder gene size is derived, same as gene values length
    * Permutable
* `MultiDiscreteGenotype<T>`
    * List of separate item lists (different sizes, but same type), where each gene has its own item list with a weighted probability depending on the list size
    * Builder `with_gene_multi_values(<Vec<VecT>>)`, mandatory, possible values for each individual gene.
    * Builder gene size is derived, same as number of item lists
    * Permutable
* `ContinuousGenotype`
    * list of float ranges (f32) with uniform probability
    * Builder `with_gene_size(usize)`, mandatory, number of genes per chromosome
    * Builder `with_gene_range(Range<f32>)`, mandatory, possible values for genes
    * Not-Permutable
* `MultiContinuousGenotype`
    * List of separate float ranges (different sizes, but same type = f32), where each gene has it's own range with a weighted probability depending on the range size
    * Builder `with_gene_ranges(Vec<Range<f32>>)`, mandatory, possible values for each individual gene.
    * Builder gene size is derived, same as number of ranges
    * Not-Permutable

* General initialization options for all Genotypes:
    * Builder `with_seed_genes(Vec<_>)`, optional, start genes of all chromosomes in the population
      (instead of the default random genes). Sometimes it is efficient to start with a certain population 
      (e.g. [Knapsack problem](../main/examples/evolve_knapsack.rs) with no items in it)

## Fitness

The fitness function has an associated Genotype.
It returns an `Option<FitnessValue = isize>`. When a `None` is returned the chromosome ends
up last in the competition phase, regardless whether the fitness is maximized or minimized.
It is usually better to add a penalty to invalid or unwanted solutions instead
of returning a `None`, so "less" invalid chromosomes are preferred over "more"
invalid ones. This usually conditions the population towards a solution faster.
See [example/evolve_knapsack](../main/examples/evolve_knapsack.rs) for an example of a penalty and a `None`.

The trait Fitness needs to be implemented for a fitness function. It only requires one method.
The example below is taken from the Infinite Monkey Theorem, see [example/evolve_monkeys](../main/examples/evolve_monkeys.rs):

```rust
const TARGET_TEXT: &str =
  "Be not afraid of greatness! Some are great, some achieve greatness, and some have greatness thrust upon 'em.";

#[derive(Clone, Debug)]
struct MyFitness;
impl Fitness for MyFitness {
    type Genotype = DiscreteGenotype<u8>;
    fn call_for_chromosome(&mut self, chromosome: &Chromosome<Self::Genotype>) -> Option<FitnessValue> {
        let string = String::from_utf8(chromosome.genes.clone()).unwrap();
        println!("{}", string); // not needed, but it looks cool!
        Some(hamming(&string, TARGET_TEXT).unwrap() as FitnessValue)
    }
}
```

## Evolve strategy
The Evolve strategy is build with the following options:

* Mandatory
    * `with_genotype(Genotype)`: the genotype initialized earlier
    * `with_population_size(usize)`: the number of chromosomes in the population
    * one or more ending conditions (first one met stops searching):
        * `with_target_fitness_score(FitnessValue)`, if you know the ultimate goal in terms of fitness score, stop searching when met.
        * `with_max_stale_generations(usize)`, if you don't know the ultimate goal and depend on some convergion threshold, stop searching when improvement stalls.
    * `with_fitness(Fitness)`: the Fitness function
    * `with_mutate(Mutate)`: the mutation strategy, very important for avoiding local optimum lock-in. But don't overdo it, as it degenerates the
      population too much if overused. Mutation probability generally between 5% and 20%. Choose one:
        * `MutateOnce(mutation_probabilty: f32)`: Each genotype has a specific mutation strategy (e.g. non-unique chromosome just pack a random now gene value, but unique chromosomes swap two genes, in order to stay valid). With this approach each chromosome has the mutation probability to be mutated once.
    * `with_crossover(Crossover)`: the crossover strategy. Every two parents create two children. The competition phase determines the order of the
      parent pairing (overall with fitter first). If you choose to keep the parents, the parents will compete with their own
      children and the population is temporarily overbooked and half of it will be discarded in the competition phase. Choose one:
        * `CrossoverAll(keep_parents: bool)`: 50% probability for each gene to come from one of the two parents. Not allowed for unique genotypes
        * `CrossoverClone(keep_parents: bool)`: Children are clones, effectively doubling the population if you keep the parents. Acts as no-op if the parents are not kept. Allowed for unique genotypes
        * `CrossoverRange(keep_parents: bool)`: Single random position in the genes, after which the genes are switched from one parent to the other. Not allowed for unique genotypes
        * `CrossoverSingle(keep_parents: bool)`: Random position in the genes, where a single gene is taken from the other parent. Not allowed for unique genotypes
    * `with_compete(Compete`)
        * `CompeteElite`: Simply sort the chromosomes with fittest first. This approach has the risk of locking in to a local optimum.
        * `CompeteTournament(tournament_size: usize)`: Run tournaments with randomly chosen chromosomes and pick a single winner. Do this
          population size times until the required population level is reached. This approach kind of sorts the fitness first, but not very strictly. This preserves a level of diversity, which
          avoids local optimum lock-in.
* Optional
    * `with_fitness_ordering(FitnessOrdering)`: defaults to `FitnessOrdering::Maximize`, so is not mandatory. Set to `FitnessOrdering::Minimize` when the search goal is to minimize the fitness function.
    * `with_degeneration_range(Range<F32>)`: When approacking a (local) optimum in the fitness score, the variation in the population goes down dramatically. This slows down the efficiency, but also has the risk of local optimum lock-in. Set this parameter to simulate a cambrian explosion, where there is only mutation until the population diversity is large enough again.
         The controlling metric is fitness score standard deviation in the population. The degeneration has a hysteresis switch, where the degeneration is activated at the start bound of the range, and deactivated at the end bound of the range.
         A typical value is `(0.005..0.995)`
* Used in context of meta, see [Heterogeneous Genotypes and Meta performance analysis](#heterogeneous-genotypes-and-meta-performance-analysis) below for more context:
    * `with_max_stale_generations_option`: the max_stale_generations value wrapped in an option to allow for a `None` value next to `Some(usize)` values.
    * `with_target_fitness_score_option`: the target_fitness_score value wrapped in an option to allow for a `None` value next to `Some(FitnessValue)` values.
    * `with_degeneration_range_option`: the degeneration_range value wrapped in an option to allow for a `None` value next to `Some(Range<F32>)` values.
    * Mutate-/Crossover-/CompeteDispatch implementations.

## Permutate strategy (as alternative to Evolve)
Sometimes the population size is small enough to simply check all possible solutions.
No randomness, mutation, crossover, competition strategies needed.

```rust
let genotype = BinaryGenotype::builder()
    .with_gene_size(16)
    .build()
    .unwrap();

println!("{}", genotype);

let permutate = Permutate::builder()
    .with_genotype(genotype)
    .with_fitness(FitnessSimpleCount)                 // count true values for fitness
    .with_fitness_ordering(FitnessOrdering::Minimize) // goal is zero true values
    .build()
    .unwrap()
    .call();

println!("{}", permutate);
```

## Heterogeneous Genotypes and Meta performance analysis

One cool thing to do with genotypes is to make a meta-genotype of all the Crossover/Mutate/Compete strategies and other Evolve parameters. This could be
used to optimize the parameters of some other genetic algorithm. Yes, a simple nested for loop would also work, but where is the fun in that? But I wasn't
able to find an elegant approach to creating such a heterogene setup. It was tried with Trait objects, Any and Enums, but all didn't work well:

* Genotype wasn't allowed to become a Trait object due to it's other traits and generics.
* Any worked, but you still need to know all possible Genotypes up front for downcasting, defeating the flexible purpose
* Enum worked, but you still need to know all possible Genotypes up front for wrapping, defeating the flexible purpose

So, after some consideration I settled on using an nested index based Genotype `MultiDiscreteGenotype<usize>` indices of external vectors of arbitrary types, which should then be retrieved in the fitness function.
Only one type is allowed per external vector, so the Crossover/Mutate/Compete strategies all have a Dispatch implementation forwarding to the underlying types (e.g. `CompeteDispatch(Competes::Tournament, 4)`)

See example meta_evolve_binary for an meta analysis of the evolution strategy:

* `cargo run --example meta_evolve_binary --release`
* `cargo run --example meta_evolve_nqueens --release`

Currently implemented as a permutation, but with caching an evolve strategy could also be used for larger search spaces.

```rust
let rounds = 10;
let population_sizes = vec![1, 2, 3, 4, 5, 10];
let max_stale_generations_options = vec![Some(100)];
let target_fitness_score_options = vec![Some(0)];
let degeneration_range_options = vec![None, Some(0.001..0.995)];
let mutates = vec![
    MutateDispatch(Mutates::Once, 0.05),
    MutateDispatch(Mutates::Once, 0.1),
    MutateDispatch(Mutates::Once, 0.2),
    MutateDispatch(Mutates::Once, 0.3),
    MutateDispatch(Mutates::Once, 0.4),
    MutateDispatch(Mutates::Once, 0.5),
];
let crossovers = vec![
    CrossoverDispatch(Crossovers::Single, true),
    CrossoverDispatch(Crossovers::Single, false),
    CrossoverDispatch(Crossovers::All, true),
    CrossoverDispatch(Crossovers::All, false),
    CrossoverDispatch(Crossovers::Range, true),
    CrossoverDispatch(Crossovers::Range, false),
    CrossoverDispatch(Crossovers::Clone, true),
    CrossoverDispatch(Crossovers::Clone, false),
];
let competes = vec![
    CompeteDispatch(Competes::Elite, 0),
    CompeteDispatch(Competes::Tournament, 2),
    CompeteDispatch(Competes::Tournament, 4),
    CompeteDispatch(Competes::Tournament, 8),
];

let genotype = BinaryGenotype::builder()
    .with_gene_size(10)
    .build()
    .unwrap();
let fitness = FitnessSimpleCount;
let evolve_builder = EvolveBuilder::new()
    .with_genotype(genotype)
    .with_fitness(fitness)
    .with_fitness_ordering(FitnessOrdering::Minimize);
let evolve_fitness_to_micro_second_factor = 1_000_000;

let config = MetaConfig::builder()
    .with_evolve_builder(evolve_builder)
    .with_evolve_fitness_to_micro_second_factor(evolve_fitness_to_micro_second_factor)
    .with_rounds(rounds)
    .with_population_sizes(population_sizes)
    .with_max_stale_generations_options(max_stale_generations_options)
    .with_target_fitness_score_options(target_fitness_score_options)
    .with_degeneration_range_options(degeneration_range_options)
    .with_mutates(mutates)
    .with_crossovers(crossovers)
    .with_competes(competes)
    .build()
    .unwrap();

let permutate = MetaPermutate::new(&config).call();
println!();
println!("{}", permutate);
```

```
meta-permutate population_size: 2304

[...]

meta-permutate:
  best_population_size: 2
  best_max_stale_generations: Some(100)
  best_target_fitness_score: Some(0)
  best_degeneration_range: None
  best_mutate: Some(MutateDispatch(Once, 0.5))
  best_crossover: Some(CrossoverDispatch(Clone, true))
  best_compete: Some(CompeteDispatch(Elite, 0))
```

## Tests
Run tests with `cargo test`

## Benchmarks
Implemented using criterion.
Run benchmarks with `cargo bench`

## Profiling
Implemented using criterion and pprof. find the flamegraph in: ./target/criterion/profile_evolve_binary/profile/flamegraph.svg

`cargo run --example profile_evolve_binary --release -- --bench --profile-time 5`

## TODO
* Setup prelude
* Maybe seed best_chromosome back into population after degenerate?
* Make duration stats return Duration, so we can choose sec/milli/micro afterwards.
* Make fitness/simple_sum generic
* Support genotypes with variable length (for knapsack problem). A Set type?
