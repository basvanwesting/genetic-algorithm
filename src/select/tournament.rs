use super::Select;
use crate::chromosome::Chromosome;
use crate::fitness::FitnessOrdering;
use crate::fitness::FitnessValue;
use crate::genotype::EvolveGenotype;
use crate::strategy::evolve::{EvolveConfig, EvolveState};
use crate::strategy::{StrategyAction, StrategyReporter, StrategyState};
use rand::prelude::*;
use std::marker::PhantomData;
use std::time::Instant;

/// Run tournaments with randomly chosen chromosomes and pick a single winner. Do this untill the
/// target_population_size (or full population when in shortage) of the population is reached and
/// drop excess chromosomes. This approach kind of sorts the fitness first, but not very strictly.
/// This preserves a level of diversity, which avoids local optimum lock-in.
#[derive(Clone, Debug)]
pub struct Tournament<G: EvolveGenotype> {
    _phantom: PhantomData<G>,
    pub replacement_rate: f32,
    pub elitism_rate: f32,
    pub tournament_size: usize,
}

impl<G: EvolveGenotype> Select for Tournament<G> {
    type Genotype = G;

    fn call<R: Rng, SR: StrategyReporter<Genotype = G>>(
        &mut self,
        _genotype: &G,
        state: &mut EvolveState<G>,
        config: &EvolveConfig,
        _reporter: &mut SR,
        rng: &mut R,
    ) {
        let now = Instant::now();

        let mut elite_chromosomes =
            self.extract_elite_chromosomes(state, config, self.elitism_rate);

        #[allow(clippy::type_complexity)]
        let (mut offspring, mut parents): (
            Vec<Chromosome<G::Allele>>,
            Vec<Chromosome<G::Allele>>,
        ) = state
            .population
            .chromosomes
            .drain(..)
            .partition(|c| c.is_offspring());

        let (new_parents_size, new_offspring_size) = self.parent_and_offspring_survival_sizes(
            parents.len(),
            offspring.len(),
            config.target_population_size - elite_chromosomes.len(),
            self.replacement_rate,
        );

        self.selection::<R>(
            &mut parents,
            new_parents_size,
            &mut state.population,
            config,
            rng,
        );
        self.selection::<R>(
            &mut offspring,
            new_offspring_size,
            &mut state.population,
            config,
            rng,
        );

        state.population.chromosomes.append(&mut elite_chromosomes);
        state.population.chromosomes.append(&mut offspring);
        state.population.chromosomes.append(&mut parents);

        // detach and attach chromosomes for general reuse of selection method
        let mut chromosomes = std::mem::take(&mut state.population.chromosomes);
        self.selection::<R>(
            &mut chromosomes,
            config.target_population_size,
            &mut state.population,
            config,
            rng,
        );
        state.population.chromosomes = chromosomes;

        state.add_duration(StrategyAction::Select, now.elapsed());
    }
}

impl<G: EvolveGenotype> Tournament<G> {
    /// Create a new Tournament selection strategy.
    /// * `replacement_rate` - fraction of population replaced by offspring (0.3-0.7 typical)
    /// * `elitism_rate` - fraction of best chromosomes preserved across generations (0.01-0.05 typical)
    /// * `tournament_size` - number of chromosomes competing per tournament (2-8 typical)
    pub fn new(replacement_rate: f32, elitism_rate: f32, tournament_size: usize) -> Self {
        Self {
            _phantom: PhantomData,
            replacement_rate,
            elitism_rate,
            tournament_size,
        }
    }

    pub fn selection<R: Rng>(
        &self,
        chromosomes: &mut Vec<Chromosome<G::Allele>>,
        selection_size: usize,
        population: &mut crate::population::Population<G::Allele>,
        config: &EvolveConfig,
        rng: &mut R,
    ) {
        let mut working_population_size = chromosomes.len();
        let tournament_size = std::cmp::min(self.tournament_size, working_population_size);
        let selection_size = std::cmp::min(selection_size, working_population_size);

        let mut selected_chromosomes: Vec<Chromosome<G::Allele>> =
            Vec::with_capacity(selection_size);
        let mut sample_index: usize;
        let mut winning_index: usize;
        let mut sample_fitness_value: FitnessValue;
        let mut winning_fitness_value: FitnessValue;

        match config.fitness_ordering {
            FitnessOrdering::Maximize => {
                for _ in 0..selection_size {
                    winning_index = 0;
                    winning_fitness_value = FitnessValue::MIN;

                    for _ in 0..tournament_size {
                        sample_index = rng.gen_range(0..working_population_size);
                        sample_fitness_value = chromosomes[sample_index]
                            .fitness_score()
                            .unwrap_or(FitnessValue::MIN);

                        if sample_fitness_value >= winning_fitness_value {
                            winning_index = sample_index;
                            winning_fitness_value = sample_fitness_value;
                        }
                    }
                    let chromosome = chromosomes.swap_remove(winning_index);
                    selected_chromosomes.push(chromosome);
                    working_population_size -= 1;
                }
            }
            FitnessOrdering::Minimize => {
                for _ in 0..selection_size {
                    winning_index = 0;
                    winning_fitness_value = FitnessValue::MAX;

                    for _ in 0..tournament_size {
                        sample_index = rng.gen_range(0..working_population_size);
                        sample_fitness_value = chromosomes[sample_index]
                            .fitness_score()
                            .unwrap_or(FitnessValue::MAX);

                        if sample_fitness_value <= winning_fitness_value {
                            winning_index = sample_index;
                            winning_fitness_value = sample_fitness_value;
                        }
                    }
                    let chromosome = chromosomes.swap_remove(winning_index);
                    selected_chromosomes.push(chromosome);
                    working_population_size -= 1;
                }
            }
        };
        // Recycle all losing chromosomes to population's recycling bin
        population.truncate_external(chromosomes, 0);
        chromosomes.append(&mut selected_chromosomes);
    }
}
