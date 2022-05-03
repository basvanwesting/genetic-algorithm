use genetic_algorithm::compete::{CompeteDispatch, Competes};
use genetic_algorithm::crossover::{CrossoverDispatch, Crossovers};
use genetic_algorithm::fitness::FitnessSimpleSumBinaryGenotype;
use genetic_algorithm::genotype::BinaryGenotype;
use genetic_algorithm::mutate::{MutateDispatch, Mutates};
use genetic_algorithm::permutate_meta::PermutateMeta;

fn main() {
    let population_sizes = vec![10, 20, 50, 100];
    let max_stale_generations_options = vec![Some(1000)];
    let target_fitness_score_options = vec![Some(100)];
    let degeneration_range_options = vec![None, Some(0.001..0.995)];
    let mutates = vec![
        MutateDispatch(Mutates::Once, 0.05),
        MutateDispatch(Mutates::Once, 0.1),
        MutateDispatch(Mutates::Once, 0.2),
        MutateDispatch(Mutates::Once, 0.3),
        MutateDispatch(Mutates::Once, 0.4),
        MutateDispatch(Mutates::Once, 0.5),
    ];
    let crossovers = vec![
        CrossoverDispatch(Crossovers::Single, true),
        CrossoverDispatch(Crossovers::Single, false),
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
    let evolve_genotype = BinaryGenotype::new().with_gene_size(100).build();
    let evolve_fitness = FitnessSimpleSumBinaryGenotype;

    let permutate_meta = PermutateMeta {
        rounds: 10,
        evolve_genotype: evolve_genotype,
        evolve_fitness: evolve_fitness,
        population_sizes: population_sizes.clone(),
        max_stale_generations_options: max_stale_generations_options.clone(),
        target_fitness_score_options: target_fitness_score_options.clone(),
        degeneration_range_options: degeneration_range_options.clone(),
        mutates: mutates.clone(),
        crossovers: crossovers.clone(),
        competes: competes.clone(),
    };

    permutate_meta.call();

    //let fitness = FitnessMeta {
    //rounds: 10,
    //evolve_genotype: evolve_genotype,
    //evolve_fitness: evolve_fitness,
    //population_sizes: population_sizes.clone(),
    //max_stale_generations_options: max_stale_generations_options.clone(),
    //target_fitness_score_options: target_fitness_score_options.clone(),
    //degeneration_range_options: degeneration_range_options.clone(),
    //mutates: mutates.clone(),
    //crossovers: crossovers.clone(),
    //competes: competes.clone(),
    //};

    ////let rng = SmallRng::from_entropy();
    //let genotype = MultiIndexGenotype::new()
    //.with_gene_value_sizes(vec![
    //population_sizes.len(),
    //max_stale_generations_options.len(),
    //target_fitness_score_options.len(),
    //degeneration_range_options.len(),
    //mutates.len(),
    //crossovers.len(),
    //competes.len(),
    //])
    //.build();

    //println!("{}", genotype);

    //let permutate = Permutate::new(genotype).with_fitness(fitness).call();

    //println!();
    //println!("{}", permutate);

    //if let Some(best_chromosome) = permutate.best_chromosome {
    //println!("best chromosome:");
    //println!(
    //"  population_size: {}",
    //population_sizes[best_chromosome.genes[0]]
    //);
    //println!(
    //"  max_stale_generations: {:?}",
    //max_stale_generations_options[best_chromosome.genes[1]]
    //);
    //println!(
    //"  target_fitness_score: {:?}",
    //target_fitness_score_options[best_chromosome.genes[2]]
    //);
    //println!(
    //"  degeneration_range: {:?}",
    //degeneration_range_options[best_chromosome.genes[3]]
    //);
    //println!("  mutate: {:?}", mutates[best_chromosome.genes[4]]);
    //println!("  crossover: {:?}", crossovers[best_chromosome.genes[5]]);
    //println!("  compete: {:?}", competes[best_chromosome.genes[6]]);
    //}
}
