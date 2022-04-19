# genetic-algorithm
A genetic algorithm implementation for Rust

Run examples with e.g. `cargo run --example evolve_binary --release`

## Tests

Run tests with `cargo test`

## Benchmarks

Run benchmarks with `cargo bench`

## Profiling

`cargo run --example profile_evolve_binary --release -- --bench --profile-time 5`

find the flamegraph in: ./target/criterion/profile_evolve_binary/profile/flamegraph.svg

## TODO

* current nqueens example bottleneck is generating valid chromosomes, which can be mutated towards the solution, need a large population_size to do this. With cloning and elite competition you can then weed out the invalid chromosomes.
* need genotype concept, where a chromosome has more control over the internal structure of the genes.
    * factories / permutations / how to mutate consistently / how to crossover consistently
    * how to do this without losing the reuse of existing mutate/compete/crossover structure?
    * it seems context is slowly becoming the genotype, maybe population_size and rng don't belong there?
    * turn context into a Trait so we can specialize for nqueens, mainly the factory side of things need further specialization

* seed best_chromosome back into population after degenerate?

