use crate::chromosome::Chromosome;
use crate::fitness::Fitness;
use crate::genotype::{Genotype, MultiIndexGenotype};
use crate::meta::{MetaConfig, MetaFitness};
use crate::permutate;
use std::fmt;

pub struct Permutate<G: Genotype, F: Fitness<Genotype = G>> {
    pub config: MetaConfig<G, F>,
    pub best_chromosome: Option<Chromosome<MultiIndexGenotype>>,
}

impl<G: Genotype, F: Fitness<Genotype = G>> Permutate<G, F> {
    pub fn new(config: MetaConfig<G, F>) -> Self {
        Self {
            config,
            best_chromosome: None,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.config.is_valid()
    }

    pub fn call(self) -> Self {
        if !self.is_valid() {
            return self;
        }
        self.execute()
    }

    pub fn execute(mut self) -> Self {
        let fitness = MetaFitness {
            config: self.config.clone(),
        };

        let genotype = self.config.build_genotype();
        let permutate = permutate::Permutate::new(genotype)
            .with_fitness(fitness)
            .call();

        self.best_chromosome = permutate.best_chromosome;
        self
    }

    fn best_fitness_score(&self) -> Option<isize> {
        self.best_chromosome.as_ref().and_then(|c| c.fitness_score)
    }
}

impl<G: Genotype, F: Fitness<Genotype = G>> fmt::Display for Permutate<G, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(best_chromosome) = &self.best_chromosome {
            let best_evolve_config = self.config.evolve_config_for_chromosome(best_chromosome);

            writeln!(
                f,
                "best_population_size: {:?}",
                best_evolve_config.population_size
            )?;
            writeln!(
                f,
                "best_max_stale_generations: {:?}",
                best_evolve_config.max_stale_generations_option
            )?;
            writeln!(
                f,
                "best_target_fitness_score: {:?}",
                best_evolve_config.target_fitness_score_option
            )?;
            writeln!(
                f,
                "best_degeneration_range: {:?}",
                best_evolve_config.degeneration_range_option
            )?;
            writeln!(f, "best_mutate: {:?}", best_evolve_config.mutate)?;
            writeln!(f, "best_crossover: {:?}", best_evolve_config.crossover)?;
            writeln!(f, "best_compete: {:?}", best_evolve_config.compete)?;

            write!(f, "best fitness score: {:?}\n", self.best_fitness_score())?;
            write!(f, "best_chromosome: {:?}\n", best_chromosome)
        } else {
            write!(f, "no best_chromosome found!",)
        }
    }
}
