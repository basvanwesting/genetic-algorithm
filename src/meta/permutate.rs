use crate::fitness::Fitness;
use crate::genotype::{Genotype, MultiDiscreteGenotype, PermutableGenotype};
use crate::meta::{MetaConfig, MetaFitness};
use crate::strategy::permutate;
use crate::strategy::Strategy;
use std::fmt;

pub struct Permutate<'a, G: Genotype + Sync, F: Fitness<Genotype = G> + Sync> {
    pub config: &'a MetaConfig<G, F>,
    pub inner_permutate: Option<permutate::Permutate<MultiDiscreteGenotype, MetaFitness<'a, G, F>>>,
}

impl<'a, G: Genotype + Sync, F: Fitness<Genotype = G> + Sync> Permutate<'a, G, F> {
    pub fn new(config: &'a MetaConfig<G, F>) -> Self {
        Self {
            config,
            inner_permutate: None,
        }
    }

    pub fn call(mut self) -> Self {
        let genotype = self.config.build_genotype();
        let fitness = MetaFitness {
            config: self.config,
        };
        let fitness_ordering = self.config.evolve_builder.fitness_ordering;

        log::info!(
            "meta-permutate population_size: {}",
            genotype.chromosome_permutations_size()
        );

        let mut rng = rand::thread_rng();
        let mut permutate = permutate::Permutate::builder()
            .with_genotype(genotype)
            .with_fitness(fitness)
            .with_fitness_ordering(fitness_ordering)
            .build()
            .unwrap();

        permutate.call(&mut rng);

        self.inner_permutate = Some(permutate);
        self
    }
}

impl<'a, G: Genotype + Sync, F: Fitness<Genotype = G> + Sync> fmt::Display for Permutate<'a, G, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(inner_permutate) = &self.inner_permutate {
            writeln!(f, "inner-{}", inner_permutate)?;

            if let Some(best_chromosome) = &inner_permutate.best_chromosome() {
                let best_evolve_builder =
                    self.config.evolve_builder_for_chromosome(best_chromosome);

                writeln!(f, "meta-permutate:")?;
                writeln!(
                    f,
                    "  best_population_size: {:?}",
                    best_evolve_builder.population_size
                )?;
                writeln!(
                    f,
                    "  best_max_stale_generations: {:?}",
                    best_evolve_builder.max_stale_generations
                )?;
                writeln!(
                    f,
                    "  best_target_fitness_score: {:?}",
                    best_evolve_builder.target_fitness_score
                )?;
                writeln!(
                    f,
                    "  best_degeneration_range: {:?}",
                    best_evolve_builder.degeneration_range
                )?;
                writeln!(
                    f,
                    "  best_mass_extinction: {:?}, {:?}",
                    best_evolve_builder.mass_extinction_uniformity_threshold,
                    best_evolve_builder.mass_extinction_survival_rate,
                )?;
                writeln!(f, "  best_mutate: {:?}", best_evolve_builder.mutate)?;
                writeln!(f, "  best_crossover: {:?}", best_evolve_builder.crossover)?;
                writeln!(f, "  best_compete: {:?}", best_evolve_builder.compete)
            } else {
                write!(f, "meta-permutate: no best chromosome")
            }
        } else {
            write!(f, "meta-permutate: no inner-permutate")
        }
    }
}
