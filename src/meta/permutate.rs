use crate::fitness::Fitness;
use crate::genotype::{Genotype, MultiIndexGenotype, PermutableGenotype};
use crate::meta::{MetaConfig, MetaFitness};
use crate::permutate;
use std::fmt;

pub struct Permutate<'a, G: Genotype, F: Fitness<Genotype = G>> {
    pub config: &'a MetaConfig<G, F>,
    pub inner_permutate: Option<permutate::Permutate<MultiIndexGenotype, MetaFitness<'a, G, F>>>,
}

impl<'a, G: Genotype, F: Fitness<Genotype = G>> Permutate<'a, G, F> {
    pub fn new(config: &'a MetaConfig<G, F>) -> Self {
        Self {
            config,
            inner_permutate: None,
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
        let genotype = self.config.build_genotype();
        let fitness = MetaFitness {
            config: self.config,
        };

        println!(
            "meta-permutate population_size: {}",
            genotype.population_factory_size()
        );

        let permutate = permutate::Permutate::builder()
            .with_genotype(genotype)
            .with_fitness(fitness)
            .with_fitness_ordering(self.config.evolve_config.fitness_ordering)
            .build()
            .call();

        self.inner_permutate = Some(permutate);
        self
    }
}

impl<'a, G: Genotype, F: Fitness<Genotype = G>> fmt::Display for Permutate<'a, G, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(inner_permutate) = &self.inner_permutate {
            writeln!(f, "inner-{}", inner_permutate)?;

            if let Some(best_chromosome) = &inner_permutate.best_chromosome {
                let best_evolve_config = self.config.evolve_config_for_chromosome(best_chromosome);

                writeln!(f, "meta-permutate:")?;
                writeln!(
                    f,
                    "  best_population_size: {:?}",
                    best_evolve_config.population_size
                )?;
                writeln!(
                    f,
                    "  best_max_stale_generations: {:?}",
                    best_evolve_config.max_stale_generations
                )?;
                writeln!(
                    f,
                    "  best_target_fitness_score: {:?}",
                    best_evolve_config.target_fitness_score
                )?;
                writeln!(
                    f,
                    "  best_degeneration_range: {:?}",
                    best_evolve_config.degeneration_range
                )?;
                writeln!(f, "  best_mutate: {:?}", best_evolve_config.mutate)?;
                writeln!(f, "  best_crossover: {:?}", best_evolve_config.crossover)?;
                writeln!(f, "  best_compete: {:?}", best_evolve_config.compete)
            } else {
                write!(f, "meta-permutate: no best chromosome")
            }
        } else {
            write!(f, "meta-permutate: no inner-permutate")
        }
    }
}
