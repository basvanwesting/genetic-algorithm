# Migration Guide: v0.20.5 to v0.20.5-cd.0

## Breaking Changes

The library now requires choosing between two paradigms:
- `distributed`: For Binary, List, Unique, Range genotypes
- `centralized`: For DynamicMatrix, StaticMatrix genotypes

## Migration Steps

### Old Code
```rust
use genetic_algorithm::strategy::evolve::prelude::*;

let genotype = BinaryGenotype::builder()
    .with_genes_size(100)
    .build()
    .unwrap();
```

### New Code - Distributed
```rust
use genetic_algorithm::distributed::prelude::*;

let genotype = BinaryGenotype::builder()
    .with_genes_size(100)
    .build()
    .unwrap();
```

### New Code - Centralized
```rust
use genetic_algorithm::centralized::prelude::*;

let genotype = DynamicMatrixGenotype::<u8>::builder()
    .with_genes_size(100)
    .with_allele_range(0..=1)
    .build()
    .unwrap();
```

## Which Paradigm to Choose?

**Use Distributed when:**
- You need custom genetic operators
- Working with non-numeric genes
- Need maximum flexibility
- Using Binary, List, Unique, or Range genotypes

**Use Centralized when:**
- You need maximum performance
- Working with large populations
- Want GPU acceleration (future)
- Using DynamicMatrix or StaticMatrix genotypes

## Import Changes

| Old Import | New Import |
|------------|------------|
| `use genetic_algorithm::genotype::*;` | `use genetic_algorithm::distributed::genotype::*;` |
| `use genetic_algorithm::strategy::evolve::prelude::*;` | `use genetic_algorithm::distributed::prelude::*;` |
| `use genetic_algorithm::fitness::*;` | `use genetic_algorithm::distributed::fitness::*;` |

## API Differences

The Fitness trait has different signatures between paradigms:

### Distributed Fitness
```rust
trait Fitness {
    type Chromosome: Chromosome;
    
    fn calculate_for_chromosome(
        &mut self, 
        chromosome: &Self::Chromosome
    ) -> Option<FitnessValue>;
}
```

### Centralized Fitness (Future - Phase 2+)
```rust
trait Fitness {
    type Genotype: Genotype;
    
    fn calculate_for_population(
        &mut self, 
        genotype: &Self::Genotype
    ) -> Vec<FitnessValue>;
}
```

Note: In Phase 1, both paradigms still use the same Fitness trait. The divergence will come in Phase 2.

## Common Migration Patterns

### Binary to Distributed
```rust
// Old
use genetic_algorithm::genotype::BinaryGenotype;
use genetic_algorithm::strategy::evolve::Evolve;

// New
use genetic_algorithm::distributed::prelude::*;
// No other changes needed for Binary, List, Unique, Range
```

### Matrix stays in Centralized
```rust
// Old
use genetic_algorithm::genotype::DynamicMatrixGenotype;
use genetic_algorithm::strategy::evolve::Evolve;

// New  
use genetic_algorithm::centralized::prelude::*;
// No other changes needed for Matrix genotypes (yet)
```

## Troubleshooting

### Error: Cannot find type `BinaryGenotype` in this scope
**Solution**: You need to choose a paradigm. Add:
```rust
use genetic_algorithm::distributed::prelude::*;
```

### Error: Multiple candidates for `Evolve` found
**Solution**: You're importing both paradigms. Choose one:
- Either `use genetic_algorithm::distributed::prelude::*;`
- Or `use genetic_algorithm::centralized::prelude::*;`
- Not both!

### Error: Method `with_genotype` not found
**Solution**: The builder methods remain the same, but make sure you're importing from the correct paradigm.

## Future Changes (Phase 2+)

In future phases, the paradigms will diverge more:
- Distributed will remove matrix genotypes
- Centralized will remove vector-based genotypes
- Fitness traits will have different APIs
- Operators will be paradigm-specific

Start migrating now to prepare for these changes.