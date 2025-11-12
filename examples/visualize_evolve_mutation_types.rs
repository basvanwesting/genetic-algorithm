use genetic_algorithm::strategy::evolve::prelude::*;
use plotters::prelude::*;
use std::sync::{Arc, Mutex};

const SEED_POINT: [f32; 2] = [0.0, 0.0];
const TARGET_POINT: [f32; 2] = [66.666, 77.777];

/// Fitness function targeting the point in 2D space
#[derive(Clone, Debug)]
struct TargetPointFitness {
    target: [f32; 2],
    precision: f32,
}

impl TargetPointFitness {
    fn new(target: [f32; 2], precision: f32) -> Self {
        Self { target, precision }
    }
}

impl Fitness for TargetPointFitness {
    type Genotype = RangeGenotype<f32>;

    fn calculate_for_chromosome(
        &mut self,
        chromosome: &FitnessChromosome<Self>,
        _genotype: &FitnessGenotype<Self>,
    ) -> Option<FitnessValue> {
        let dx = chromosome.genes[0] - self.target[0];
        let dy = chromosome.genes[1] - self.target[1];
        let score = (dx * dx + dy * dy).sqrt() / self.precision;
        Some(score as FitnessValue)
    }
}

/// Custom reporter that collects all exploration points during evolution
#[derive(Clone)]
struct ExplorationReporter {
    explored_points: Arc<Mutex<Vec<(f32, f32, usize)>>>, // (x, y, generation)
    best_points: Arc<Mutex<Vec<(f32, f32, usize)>>>,     // (x, y, generation)
}

impl ExplorationReporter {
    fn new() -> Self {
        Self {
            explored_points: Arc::new(Mutex::new(Vec::new())),
            best_points: Arc::new(Mutex::new(vec![(SEED_POINT[0], SEED_POINT[1], 0)])),
        }
    }

    fn get_explored_points(&self) -> Vec<(f32, f32, usize)> {
        self.explored_points.lock().unwrap().clone()
    }
    fn get_best_points(&self) -> Vec<(f32, f32, usize)> {
        self.best_points.lock().unwrap().clone()
    }
}

impl StrategyReporter for ExplorationReporter {
    type Genotype = RangeGenotype<f32>;

    fn on_generation_complete<S: StrategyState<Self::Genotype>, C: StrategyConfig>(
        &mut self,
        _genotype: &Self::Genotype,
        state: &S,
        _config: &C,
    ) {
        // Collect all explored points
        state
            .population_as_ref()
            .chromosomes
            .iter()
            .for_each(|chromosome| {
                let mut explored_points = self.explored_points.lock().unwrap();
                explored_points.push((
                    chromosome.genes[0],
                    chromosome.genes[1],
                    state.current_generation(),
                ));
            });

        // Collect best point
        if state.best_generation() == state.current_generation() {
            let best_genes = state.best_genes().unwrap();
            let mut best_points = self.best_points.lock().unwrap();
            best_points.push((best_genes[0], best_genes[1], state.current_generation()));
        }
    }

    // fn on_selection_complete<S: StrategyState<Self::Genotype>, C: StrategyConfig>(
    //     &mut self,
    //     genotype: &Self::Genotype,
    //     state: &S,
    //     config: &C,
    // ) {
    //     if state.current_generation() % 10 == 0 {
    //         let fitness_cache_hit_miss_ratio = config.fitness_cache().map(|c| c.hit_miss_stats().2);
    //         let (parents_size, offspring_size) =
    //             state.population_as_ref().parents_and_offspring_size();
    //
    //         println!(
    //             "periodic - current_generation: {}, stale_generations: {}, best_generation: {}, scale_index: {:?}, population_cardinality: {:?}, current_population_size: {} ({}p/{}o,{}r), fitness_cache_hit_miss_ratio: {:.2?}",
    //             state.current_generation(),
    //             state.stale_generations(),
    //             state.best_generation(),
    //             genotype.current_scale_index(),
    //             state.population_cardinality(),
    //             state.population_as_ref().size(),
    //             parents_size,
    //             offspring_size,
    //             state.population_as_ref().recycled_size(),
    //             fitness_cache_hit_miss_ratio,
    //         );
    //     }
    // }
}

/// Run evolution with a specific mutation type and collect exploration points
fn run_evolution(
    mutation_type: MutationType<f32>,
    mutation_type_name: String,
    max_stale_generations: usize,
) -> ExplorationReporter {
    let fitness = TargetPointFitness::new(TARGET_POINT, 0.001);
    let reporter = ExplorationReporter::new();

    // Create genotype with 2 genes for 2D visualization
    let genotype = RangeGenotype::<f32>::builder()
        .with_genes_size(2)
        .with_allele_range(0.0..=100.0)
        .with_mutation_type(mutation_type)
        .with_seed_genes_list(vec![SEED_POINT.to_vec()]) // Start from origin
        .build()
        .unwrap();

    // Build evolution strategy
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_fitness(fitness)
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_target_population_size(1) // Single individual to clearly show mutation behavior
        .with_max_stale_generations(max_stale_generations) // triggers scaling
        .with_select(SelectElite::new(0.0, 1.0)) // Always select the best
        .with_crossover(CrossoverClone::new(1.0)) // Clone parent to offspring, as only offspring are mutated
        .with_mutate(MutateMultiGene::new(2, 1.0)) // Always mutate all genes to show exploration
        .with_reporter(reporter)
        .with_par_fitness(false) // No parallelization needed for pop size 1
        .with_rng_seed_from_u64(42) // Fixed seed for reproducibility
        .call()
        .unwrap();

    println!(
        "Completed {} with {} exploration points, max_stale_generations: {}, best_generation: {}, best_fitness_score: {:?}",
        mutation_type_name,
        evolve.reporter.get_explored_points().len(),
        max_stale_generations,
        evolve.best_generation(),
        evolve.best_fitness_score(),
    );

    evolve.reporter
}

/// Generate visualization plot showing exploration patterns
fn generate_plot(
    reporters: Vec<(String, ExplorationReporter)>,
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Ensure the output path is created from the project root
    let output_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(output_path);
    let root = BitMapBackend::new(&output_path, (1800, 1200)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart_builders = root.split_evenly((2, 3));

    for ((name, reporter), chart_area) in reporters.iter().zip(chart_builders.iter_mut()) {
        let explored_points = reporter.get_explored_points();
        let best_points = reporter.get_best_points();

        let mut chart = ChartBuilder::on(chart_area)
            .caption(name, ("sans-serif", 30))
            .margin(10)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(0f32..100f32, 0f32..100f32)?;

        chart.configure_mesh().draw()?;

        // Draw target point
        chart
            .draw_series(PointSeries::of_element(
                [(TARGET_POINT[0], TARGET_POINT[1])],
                5,
                &RED,
                &|c, s, _st| {
                    EmptyElement::at(c)
                        + Cross::new((0, 0), s * 2, ShapeStyle::from(&RED).stroke_width(2))
                },
            ))?
            .label("Target")
            .legend(|(x, y)| Cross::new((x, y), 5 * 2, ShapeStyle::from(&RED).stroke_width(2)));

        // Draw exploration points with color gradient based on generation
        if !explored_points.is_empty() {
            let max_gen = explored_points.iter().map(|(_, _, g)| *g).max().unwrap() as f32;

            // Draw points
            for (x, y, gen) in &explored_points {
                let color_intensity = (*gen as f32 / max_gen * 200.0) as u8;
                let color = RGBColor(50, 50 + color_intensity, 255 - color_intensity);

                chart.draw_series(PointSeries::of_element(
                    [(*x, *y)],
                    2,
                    &color,
                    &|c, s, st| Circle::new(c, s, st.filled()),
                ))?;
            }
        }

        // Draw best points with color gradient based on generation
        if !best_points.is_empty() {
            let max_gen = best_points.iter().map(|(_, _, g)| *g).max().unwrap() as f32;

            // Draw lines connecting consecutive points
            for window in best_points.windows(2) {
                let (x1, y1, g1) = window[0];
                let (x2, y2, _) = window[1];

                let color_intensity = (g1 as f32 / max_gen * 200.0) as u8;
                let color = RGBColor(50, 50 + color_intensity, 255 - color_intensity);

                chart.draw_series(LineSeries::new(vec![(x1, y1), (x2, y2)], &color))?;
            }
        }

        // Draw ending point
        if let Some(ending_point) = best_points.last() {
            chart
                .draw_series(PointSeries::of_element(
                    [(ending_point.0, ending_point.1)],
                    4,
                    &RED,
                    &|c, s, st| Circle::new(c, s, st.filled()),
                ))?
                .label("End")
                .legend(|(x, y)| Circle::new((x, y), 3, RED.filled()));
        }

        // Draw starting point
        chart
            .draw_series(PointSeries::of_element(
                [(SEED_POINT[0], SEED_POINT[1])],
                4,
                &BLACK,
                &|c, s, st| Circle::new(c, s, st.filled()),
            ))?
            .label("Start")
            .legend(|(x, y)| Circle::new((x, y), 3, BLACK.filled()));

        chart.configure_series_labels().draw()?;
    }

    root.present()?;
    println!("Plot saved to {}", output_path.display());

    Ok(())
}

fn main() {
    println!("=== Visualizing Evolve Mutation Types in 2D Search Space ===\n");
    println!("Starting point: {:?}", SEED_POINT);
    println!("Target point: {:?}", TARGET_POINT);
    println!("Search space: [0.0, 100.0] x [0.0, 100.0]\n");

    let mut reporters = Vec::new();

    // Random mutation - complete random replacement
    println!("Running Random mutation...");
    let reporter = run_evolution(MutationType::Random, "Random".to_string(), 50);
    reporters.push(("Random".to_string(), reporter));

    // Range mutation - fixed bandwidth
    println!("Running Range(10.0) mutation...");
    let reporter = run_evolution(MutationType::Range(10.0), "Range(10.0)".to_string(), 20);
    reporters.push(("Range(±10)".to_string(), reporter));

    // RangeScaled mutation - dropping by 1/5th each scale
    println!("Running RangeScaled mutation...");
    let ranges = vec![100.0, 100.0, 100.0, 90.0, 30.0, 1.0];
    let reporter = run_evolution(
        MutationType::RangeScaled(ranges.clone()),
        format!("RangeScaled({:?})", ranges),
        10,
    );
    reporters.push(("RangeScaled (slow starting sigmoid)".to_string(), reporter));

    // Step mutation - fixed step
    println!("Running Step(1.0) mutation...");
    let reporter = run_evolution(MutationType::Step(1.0), "Step(1.0)".to_string(), 20);
    reporters.push(("Step(±1)".to_string(), reporter));

    // StepScaled mutation - halving each scale
    println!("Running StepScaled mutation...");
    let steps = vec![50.0, 25.0, 12.5, 6.25, 3.125, 1.5625];
    let reporter = run_evolution(
        MutationType::StepScaled(steps.clone()),
        format!("StepScaled({:?})", steps),
        10,
    );
    reporters.push(("StepScaled (halving)".to_string(), reporter));

    // Discrete mutation - like ListGenotype for categories
    println!("Running Discrete mutation...");
    let reporter = run_evolution(MutationType::Discrete, "Discrete".to_string(), 50);
    reporters.push((
        "Discrete (floored integers, map to categories)".to_string(),
        reporter,
    ));

    // Generate visualization
    println!("\nGenerating visualization...");
    if let Err(e) = generate_plot(reporters, "examples/visualize_evolve_mutation_types.png") {
        eprintln!("Failed to generate plot: {}", e);
    }

    println!("\n=== Analysis Summary ===");
    println!("- Random: Chaotic exploration, can jump anywhere in search space");
    println!("- Range: Local search with fixed radius, smooth but may get stuck");
    println!("- RangeScaled: Funnel-like convergence, broad exploration then fine-tuning");
    println!("- Step: Local search with fixed step, smooth but may get stuck");
    println!("- StepScaled: Grid-like exploration with progressively finer resolution");
    println!("- Discrete: ListGenotype behaviour, for categories in heterogeneous genotypes");
    println!("Color gradient: Blue (early) → Green (late) generations");
}
