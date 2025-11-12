use genetic_algorithm::strategy::permutate::prelude::*;
use plotters::prelude::*;
use std::sync::{Arc, Mutex};

const TARGET_POINT: [f32; 2] = [6.6666, 7.7777];

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

/// Custom reporter that collects all exploration points during permutation
#[derive(Clone)]
struct PermutationReporter {
    explored_points: Arc<Mutex<Vec<(f32, f32, usize)>>>, // (x, y, iteration)
    best_points: Arc<Mutex<Vec<(f32, f32, usize)>>>,     // (x, y, iteration)
}

impl PermutationReporter {
    fn new() -> Self {
        Self {
            explored_points: Arc::new(Mutex::new(Vec::new())),
            best_points: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn get_explored_points(&self) -> Vec<(f32, f32, usize)> {
        self.explored_points.lock().unwrap().clone()
    }
    fn get_best_points(&self) -> Vec<(f32, f32, usize)> {
        self.best_points.lock().unwrap().clone()
    }
}

impl StrategyReporter for PermutationReporter {
    type Genotype = RangeGenotype<f32>;

    fn on_generation_complete<S: StrategyState<Self::Genotype>, C: StrategyConfig>(
        &mut self,
        _genotype: &Self::Genotype,
        state: &S,
        _config: &C,
    ) {
        // For permutation, we track the current chromosome being evaluated
        if let Some(chromosome) = state.chromosome_as_ref() {
            let mut explored_points = self.explored_points.lock().unwrap();
            explored_points.push((
                chromosome.genes[0],
                chromosome.genes[1],
                state.current_generation(),
            ));
        }

        // Collect best point
        if state.best_generation() == state.current_generation() {
            let best_genes = state.best_genes().unwrap();
            let mut best_points = self.best_points.lock().unwrap();
            best_points.push((best_genes[0], best_genes[1], state.current_generation()));
        }
    }
}

/// Run permutation with a specific mutation type and collect exploration points
fn run_permutation(
    mutation_type: MutationType<f32>,
    mutation_type_name: String,
) -> PermutationReporter {
    let fitness = TargetPointFitness::new(TARGET_POINT, 0.0001);
    let reporter = PermutationReporter::new();

    // Create genotype with 2 genes for 2D visualization
    // Using integer type for discrete steps
    let genotype = RangeGenotype::<f32>::builder()
        .with_genes_size(2)
        .with_allele_range(0.0..=10.0)
        .with_mutation_type(mutation_type)
        // No seed - let permutation explore the entire space
        .build()
        .unwrap();

    // Build permutation strategy
    let permutate = Permutate::builder()
        .with_genotype(genotype)
        .with_fitness(fitness)
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_reporter(reporter)
        .with_par_fitness(true) // maybe messes up graphs a bit
        .call()
        .unwrap();

    println!(
        "Completed {} with {} exploration points, best_generation: {}, best_fitness_score: {:?}",
        mutation_type_name,
        permutate.reporter.get_explored_points().len(),
        permutate.best_generation(),
        permutate.best_fitness_score(),
    );

    permutate.reporter
}

/// Generate visualization plot showing exploration patterns
fn generate_plot(
    reporters: Vec<(String, PermutationReporter)>,
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Ensure the output path is created from the project root
    let output_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(output_path);
    let root = BitMapBackend::new(&output_path, (1800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart_builders = root.split_evenly((1, 3));

    for ((name, reporter), chart_area) in reporters.iter().zip(chart_builders.iter_mut()) {
        let explored_points = reporter.get_explored_points();
        let best_points = reporter.get_best_points();

        let mut chart = ChartBuilder::on(chart_area)
            .caption(name, ("sans-serif", 30))
            .margin(10)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(0f32..10f32, 0f32..10f32)?;

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

        chart.configure_series_labels().draw()?;
    }

    root.present()?;
    println!("Plot saved to {}", output_path.display());

    Ok(())
}

fn main() {
    println!("=== Visualizing Permutation Mutation Types in 2D Search Space ===\n");
    println!("Target point: {:?}", TARGET_POINT);
    println!("Search space: [0.0, 10.0] x [0.0, 10.0]\n");

    let mut reporters = Vec::new();

    // Step mutation - fixed step
    // With step=5, this will explore a 20x20 grid (every 5th point)
    println!("Running Step(0.3) mutation...");
    let reporter = run_permutation(MutationType::Step(0.3), "Step(0.3)".to_string());
    reporters.push(("Step(±0.3, full exploration)".to_string(), reporter));

    // StepScaled mutation - halving each scale
    // This will first explore with large steps, then refine around the best
    println!("Running StepScaled mutation...");
    let steps = vec![5.00, 2.50, 1.25, 0.625, 0.3125, 0.15625];
    let reporter = run_permutation(
        MutationType::StepScaled(steps.clone()),
        format!("StepScaled({:?})", steps),
    );
    reporters.push(("StepScaled (nested halving grids)".to_string(), reporter));

    // Discrete mutation - like ListGenotype for categories
    // This explores a sampled subset of the space
    println!("Running Discrete mutation...");
    let reporter = run_permutation(MutationType::Discrete, "Discrete".to_string());
    reporters.push((
        "Discrete (full exploration of integers)".to_string(),
        reporter,
    ));

    // Generate visualization
    println!("\nGenerating visualization...");
    if let Err(e) = generate_plot(reporters, "examples/visualize_permutate_mutation_types.png") {
        eprintln!("Failed to generate plot: {}", e);
    }

    println!("\n=== Analysis Summary ===");
    println!("- Step: Explores all points within step distance systematically");
    println!("- StepScaled: Progressively refines search with smaller steps");
    println!("- Discrete: Jumps to any discrete value in the range");
    println!("Color gradient: Blue (early) → Green (late) generations");
}
