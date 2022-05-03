use crate::compete::CompeteDispatch;
use crate::crossover::CrossoverDispatch;
use crate::fitness::{Fitness, FitnessMeta};
use crate::genotype::{Genotype, MultiIndexGenotype};
use crate::mutate::MutateDispatch;
use crate::permutate::Permutate;
use std::ops::Range;
//use rand::rngs::SmallRng;

pub struct PermutateMeta<G: Genotype, F: Fitness<Genotype = G>> {
    pub rounds: usize,
    pub evolve_genotype: G,
    pub evolve_fitness: F,
    pub population_sizes: Vec<usize>,
    pub max_stale_generations_options: Vec<Option<usize>>,
    pub target_fitness_score_options: Vec<Option<isize>>,
    pub degeneration_range_options: Vec<Option<Range<f32>>>,
    pub mutates: Vec<MutateDispatch>,
    pub crossovers: Vec<CrossoverDispatch>,
    pub competes: Vec<CompeteDispatch>,
}

impl<G: Genotype, F: Fitness<Genotype = G>> PermutateMeta<G, F> {
    pub fn call(self) -> Self {
        let fitness = FitnessMeta {
            rounds: self.rounds,
            evolve_genotype: self.evolve_genotype.clone(),
            evolve_fitness: self.evolve_fitness.clone(),
            population_sizes: self.population_sizes.clone(),
            max_stale_generations_options: self.max_stale_generations_options.clone(),
            target_fitness_score_options: self.target_fitness_score_options.clone(),
            degeneration_range_options: self.degeneration_range_options.clone(),
            mutates: self.mutates.clone(),
            crossovers: self.crossovers.clone(),
            competes: self.competes.clone(),
        };

        //let rng = SmallRng::from_entropy();
        let genotype = MultiIndexGenotype::new()
            .with_gene_value_sizes(vec![
                self.population_sizes.len(),
                self.max_stale_generations_options.len(),
                self.target_fitness_score_options.len(),
                self.degeneration_range_options.len(),
                self.mutates.len(),
                self.crossovers.len(),
                self.competes.len(),
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
                self.population_sizes[best_chromosome.genes[0]]
            );
            println!(
                "  max_stale_generations: {:?}",
                self.max_stale_generations_options[best_chromosome.genes[1]]
            );
            println!(
                "  target_fitness_score: {:?}",
                self.target_fitness_score_options[best_chromosome.genes[2]]
            );
            println!(
                "  degeneration_range: {:?}",
                self.degeneration_range_options[best_chromosome.genes[3]]
            );
            println!("  mutate: {:?}", self.mutates[best_chromosome.genes[4]]);
            println!(
                "  crossover: {:?}",
                self.crossovers[best_chromosome.genes[5]]
            );
            println!("  compete: {:?}", self.competes[best_chromosome.genes[6]]);
        }
        self
    }
}
