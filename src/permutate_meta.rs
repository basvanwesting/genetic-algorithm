use crate::compete::CompeteDispatch;
use crate::crossover::CrossoverDispatch;
use crate::fitness::{Fitness, FitnessMeta};
use crate::genotype::{Genotype, MultiIndexGenotype};
use crate::meta_config::MetaConfig;
use crate::mutate::MutateDispatch;
use crate::permutate::Permutate;
use std::ops::Range;
//use rand::rngs::SmallRng;

pub struct PermutateMeta<G: Genotype, F: Fitness<Genotype = G>> {
    pub config: MetaConfig<G, F>,
}

impl<G: Genotype, F: Fitness<Genotype = G>> PermutateMeta<G, F> {
    pub fn call(self) -> Self {
        let fitness = FitnessMeta {
            config: self.config.clone(),
        };

        //let rng = SmallRng::from_entropy();
        let genotype = MultiIndexGenotype::new()
            .with_gene_value_sizes(vec![
                self.config.population_sizes.len(),
                self.config.max_stale_generations_options.len(),
                self.config.target_fitness_score_options.len(),
                self.config.degeneration_range_options.len(),
                self.config.mutates.len(),
                self.config.crossovers.len(),
                self.config.competes.len(),
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
                self.config.population_sizes[best_chromosome.genes[0]]
            );
            println!(
                "  max_stale_generations: {:?}",
                self.config.max_stale_generations_options[best_chromosome.genes[1]]
            );
            println!(
                "  target_fitness_score: {:?}",
                self.config.target_fitness_score_options[best_chromosome.genes[2]]
            );
            println!(
                "  degeneration_range: {:?}",
                self.config.degeneration_range_options[best_chromosome.genes[3]]
            );
            println!(
                "  mutate: {:?}",
                self.config.mutates[best_chromosome.genes[4]]
            );
            println!(
                "  crossover: {:?}",
                self.config.crossovers[best_chromosome.genes[5]]
            );
            println!(
                "  compete: {:?}",
                self.config.competes[best_chromosome.genes[6]]
            );
        }
        self
    }
}
