use genetic_algorithm::chromosome::Chromosome;
use genetic_algorithm::compete::{CompeteDispatch, Competes};
use genetic_algorithm::crossover::{CrossoverDispatch, Crossovers};
use genetic_algorithm::evolve::Evolve;
use genetic_algorithm::evolve_stats::EvolveStats;
use genetic_algorithm::fitness;
use genetic_algorithm::fitness::Fitness;
use genetic_algorithm::genotype::{BinaryGenotype, MultiIndexGenotype};
use genetic_algorithm::mutate::{MutateDispatch, Mutates};
use genetic_algorithm::permutate::Permutate;
use rand::prelude::*;
use rand::rngs::SmallRng;
use std::ops::Range;
use std::time::Instant;

#[derive(Clone, Debug)]
struct MetaFitness {
    rounds: usize,
    population_sizes: Vec<usize>,
    degeneration_ranges: Vec<Range<f32>>,
    mutates: Vec<MutateDispatch>,
    crossovers: Vec<CrossoverDispatch>,
    competes: Vec<CompeteDispatch>,
}
impl Fitness for MetaFitness {
    type Genotype = MultiIndexGenotype;
    fn call_for_chromosome(&self, chromosome: &Chromosome<Self::Genotype>) -> isize {
        let population_size = self.population_sizes[chromosome.genes[0]];
        let degeneration_range = self.degeneration_ranges[chromosome.genes[1]].clone();
        let mutate = self.mutates[chromosome.genes[2]].clone();
        let crossover = self.crossovers[chromosome.genes[3]].clone();
        let compete = self.competes[chromosome.genes[4]].clone();

        let mut stats = EvolveStats::new();
        for _ in 0..self.rounds {
            let rng = SmallRng::from_entropy();
            let genotype = BinaryGenotype::new().with_gene_size(100).build();
            let now = Instant::now();

            let evolve = Evolve::new(genotype, rng)
                .with_population_size(population_size)
                .with_max_stale_generations(1000)
                .with_target_fitness_score(100)
                .with_degeneration_range(degeneration_range.clone())
                .with_mutate(mutate.clone())
                .with_fitness(fitness::SimpleSumBinaryGenotype)
                .with_crossover(crossover.clone())
                .with_compete(compete.clone())
                .call();

            stats.durations.push(now.elapsed());
            stats.best_generations.push(evolve.best_generation);
            stats.best_fitness_scores.push(evolve.best_fitness_score());
        }
        println!(
            "population_size: {} | degeneration_range {:?} | mutate: {:?} | crossover: {:?} | compete: {:?}",
            population_size, degeneration_range, mutate, crossover, compete
        );
        println!("  {}", stats);

        let mut score: isize = 0;
        if stats.best_fitness_score_mean() == 100.0 {
        } else {
            score -= 1_000_000_000
        }
        score -= (stats.duration_mean_subsec_micros()) as isize;
        score
    }
}

fn main() {
    let population_sizes = vec![10, 20, 50, 100];
    let degeneration_ranges = vec![0.0..0.0, 0.001..0.995];

    let mutates = vec![
        MutateDispatch(Mutates::SingleGene, 0.05),
        MutateDispatch(Mutates::SingleGene, 0.1),
        MutateDispatch(Mutates::SingleGene, 0.2),
        MutateDispatch(Mutates::SingleGene, 0.3),
        MutateDispatch(Mutates::SingleGene, 0.4),
        MutateDispatch(Mutates::SingleGene, 0.5),
    ];

    let crossovers = vec![
        CrossoverDispatch(Crossovers::Individual, true),
        CrossoverDispatch(Crossovers::Individual, false),
        CrossoverDispatch(Crossovers::All, true),
        CrossoverDispatch(Crossovers::All, false),
        CrossoverDispatch(Crossovers::Range, true),
        CrossoverDispatch(Crossovers::Range, false),
    ];
    let competes = vec![
        CompeteDispatch(Competes::Elite, 0),
        CompeteDispatch(Competes::Tournament, 2),
        CompeteDispatch(Competes::Tournament, 4),
        CompeteDispatch(Competes::Tournament, 8),
    ];

    let fitness = MetaFitness {
        rounds: 10,
        population_sizes: population_sizes.clone(),
        degeneration_ranges: degeneration_ranges.clone(),
        mutates: mutates.clone(),
        crossovers: crossovers.clone(),
        competes: competes.clone(),
    };

    //let rng = SmallRng::from_entropy();
    let genotype = MultiIndexGenotype::new()
        .with_gene_value_sizes(vec![
            population_sizes.len(),
            degeneration_ranges.len(),
            mutates.len(),
            crossovers.len(),
            competes.len(),
        ])
        .build();

    println!("{}", genotype);

    let permutate = Permutate::new(genotype).with_fitness(fitness).call();

    println!();
    println!("{}", permutate);

    if let Some(best_chromosome) = permutate.best_chromosome {
        println!("best chromosome:");
        println!(
            "  population_size: {}",
            population_sizes[best_chromosome.genes[0]]
        );
        println!(
            "  degeneration_range: {:?}",
            degeneration_ranges[best_chromosome.genes[1]]
        );
        println!("  mutate: {:?}", mutates[best_chromosome.genes[2]]);
        println!("  crossover: {:?}", crossovers[best_chromosome.genes[3]]);
        println!("  compete: {:?}", competes[best_chromosome.genes[4]]);
    }
}
