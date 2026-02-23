# AGENTS_TEMPLATES.md — Copy-Paste Templates

See [AGENTS.md](AGENTS.md) for decision matrices, constructor parameter reference,
and troubleshooting.

## Binary Optimization (Knapsack)

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

## Discrete Selection (ListGenotype)

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

## Continuous Optimization (RangeGenotype)

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
        .with_genes_size(100)
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
        .with_mutate(MutateMultiGene::new(10, 1.0))
        .call()
        .unwrap();

    let (best_genes, best_fitness_score) = evolve.best_genes_and_fitness_score().unwrap();
    println!("genes: {:?}, score: {}", best_genes, best_fitness_score);
}
```

## Permutation Problem (HillClimb, recommended for permutations)

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

## Traveling Salesman (HillClimb with call_repeatedly)

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

## Exhaustive Search (Permutate)

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

## Heterogeneous Optimization (MultiRangeGenotype)

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

## Multi-Group Assignment (MultiUniqueGenotype)

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

## Per-Gene Categorical (MultiListGenotype)

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
