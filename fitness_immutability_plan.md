# Plan: Make Fitness Trait Immutable in Distributed Track

## Current State

The Fitness trait currently requires `&mut self` in all its methods:
- `calculate_for_chromosome(&mut self, ...)`
- `call_for_chromosome(&mut self, ...)`
- `call_for_population(&mut self, ...)`
- `call_for_state_population(&mut self, ...)`
- `call_for_state_chromosome(&mut self, ...)`

This causes complexity in parallel evaluation, requiring `ThreadLocal<RefCell<Self>>` gymnastics.

## Why Fitness Should Be Immutable

1. **Fitness calculation is pure** - Same genes → same fitness score
2. **Thread safety** - No need for ThreadLocal/RefCell complexity
3. **Consistency** - Matches other immutable traits (Genotype, Crossover, Mutate, Select, Extension)
4. **Simpler parallelization** - Direct parallel iteration without cloning
5. **No valid use case** - Fitness functions shouldn't maintain state between evaluations

## Implementation Plan

### Phase 1: Update Fitness Trait
**File:** `src/distributed/fitness.rs`

```rust
// Change all &mut self to &self
pub trait Fitness: Clone + Send + Sync + std::fmt::Debug {
    type Genotype: Genotype;
    
    fn call_for_state_population<S: StrategyState<Self::Genotype>, C: StrategyConfig>(
        &self,  // Changed from &mut self
        genotype: &Self::Genotype,
        state: &mut S,
        config: &C,
        // Remove thread_local parameter - no longer needed!
    ) {
        // Simplified implementation
    }
    
    fn call_for_population(
        &self,  // Changed from &mut self
        population: &mut FitnessPopulation<Self>,
        genotype: &Self::Genotype,
        // Remove thread_local parameter
        cache: Option<&FitnessCache>,
    ) {
        // Much simpler parallel implementation:
        population
            .chromosomes
            .par_iter_mut()
            .filter(|c| c.fitness_score().is_none())
            .for_each(|chromosome| {
                self.call_for_chromosome(chromosome, genotype, cache);
            });
    }
    
    fn call_for_chromosome(
        &self,  // Changed from &mut self
        chromosome: &mut FitnessChromosome<Self>,
        genotype: &Self::Genotype,
        cache: Option<&FitnessCache>,
    ) {
        // Same logic, just with &self
    }
    
    fn calculate_for_chromosome(
        &self,  // Changed from &mut self
        chromosome: &FitnessChromosome<Self>,
        genotype: &Self::Genotype,
    ) -> Option<FitnessValue>;
}
```

### Phase 2: Remove ThreadLocal Complexity

**Current (complex due to mutability):**
```rust
fn call_for_population(&mut self, ..., thread_local: Option<&ThreadLocal<RefCell<Self>>>, ...) {
    if let Some(thread_local) = thread_local {
        population.chromosomes.par_iter_mut()
            .for_each_init(
                || thread_local.get_or(|| RefCell::new(self.clone())).borrow_mut(),
                |fitness, chromosome| {
                    fitness.call_for_chromosome(chromosome, genotype, cache);
                },
            );
    } else {
        // Sequential version
    }
}
```

**New (simple with immutability):**
```rust
fn call_for_population(&self, ...) {
    population.chromosomes
        .par_iter_mut()
        .filter(|c| c.fitness_score().is_none())
        .for_each(|chromosome| {
            self.call_for_chromosome(chromosome, genotype, cache);
        });
}
```

### Phase 3: Update Fitness Implementations

**Files to update:**
- `src/distributed/fitness/placeholders.rs` - All placeholder fitness implementations
- Any user-defined fitness implementations in tests/examples

```rust
// Example update for CountTrue
impl Fitness for CountTrue {
    type Genotype = BinaryGenotype;
    fn calculate_for_chromosome(
        &self,  // Changed from &mut self
        chromosome: &FitnessChromosome<Self>,
        _genotype: &Self::Genotype,
    ) -> Option<FitnessValue> {
        Some(chromosome.genes.iter().filter(|&value| *value).count() as FitnessValue)
    }
}
```

### Phase 4: Update Strategy Calls

Update any place that calls fitness methods:

**In `src/distributed/strategy/evolve.rs`:**
```rust
// Remove thread_local parameter
fitness.call_for_state_population(genotype, state, config);
```

**In `src/distributed/strategy/hill_climb.rs`:**
```rust
// Same updates - remove thread_local
fitness.call_for_state_chromosome(genotype, state, config);
```

### Phase 5: Clean Up Imports

Remove no longer needed imports:
```rust
// Remove these:
- use std::cell::RefCell;
- use thread_local::ThreadLocal;
```

## Benefits After Implementation

### Before (Mutable)
```rust
// Complex parallel evaluation
let thread_local = ThreadLocal::new();
fitness.call_for_population(
    population,
    genotype,
    Some(&thread_local),  // Required for parallelization
    cache
);
```

### After (Immutable)
```rust
// Simple parallel evaluation
fitness.call_for_population(
    population,
    genotype,
    cache
);
```

## Testing Strategy

1. **Unit Tests:** Ensure all fitness implementations work with `&self`
2. **Parallel Tests:** Verify parallel evaluation works without ThreadLocal
3. **Benchmarks:** Compare performance (should be faster without RefCell overhead)

## Potential Issues & Solutions

### Issue: User-defined fitness needs state
**Solution:** State should be external (in Strategy) or passed as parameter, not in Fitness itself

### Issue: Counting evaluations
**Solution:** Add counter to Strategy/State, not Fitness:
```rust
// In EvolveState
pub evaluation_count: usize,

// In fitness.call_for_chromosome
state.evaluation_count += 1;
```

### Issue: Random number generation in fitness
**Solution:** Pass RNG as parameter if needed:
```rust
fn calculate_for_chromosome(
    &self,
    chromosome: &FitnessChromosome<Self>,
    genotype: &Self::Genotype,
    rng: Option<&mut R>,  // Optional RNG parameter
) -> Option<FitnessValue>;
```

## Migration Guide for Users

If users have custom Fitness implementations:

```rust
// Before
impl Fitness for MyFitness {
    fn calculate_for_chromosome(&mut self, ...) -> Option<FitnessValue> {
        self.counter += 1;  // Won't work anymore
        // ...
    }
}

// After - Option 1: Stateless
impl Fitness for MyFitness {
    fn calculate_for_chromosome(&self, ...) -> Option<FitnessValue> {
        // Pure calculation only
    }
}

// After - Option 2: External state
struct MyFitnessWithState {
    fitness: MyFitness,
    counter: AtomicUsize,
}
```

## Estimated Effort

- **Phase 1-2:** 2 hours (update trait and remove ThreadLocal)
- **Phase 3:** 1 hour (update implementations)
- **Phase 4-5:** 1 hour (update callers and clean up)
- **Testing:** 1 hour

**Total:** ~5 hours

## Success Criteria

- [ ] All `&mut self` removed from Fitness trait
- [ ] ThreadLocal/RefCell complexity removed
- [ ] Parallel evaluation works without cloning fitness
- [ ] All tests pass
- [ ] Performance improved or unchanged