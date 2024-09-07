use constants::{fitting_constants::*, progress_bar_constants::*};
use integrators::leapfrog::integrate;
use math_helpers::MathHelpers;
use solvers::bisection::solve;

use std::{
    fs::{create_dir_all, File},
    io::{stdout, BufWriter, Write},
    path::Path,
    time::Instant,
};

mod constants;
mod integrators;
mod math_helpers;
mod solvers;

fn main() {
    let results: Vec<(f64, f64, f64)> = simulate_single_cpu();

    write_results(results);
}

/// Writes a [Vec<(f64, f64, f64)>] with a distance, velocity, and root to a [RESULTS_FILE] in the [RESULTS_FOLDER], formatted as a csv.
fn write_results(results: Vec<(f64, f64, f64)>) {
    let results_folder: &Path = Path::new(RESULTS_FOLDER);
    create_dir_all(results_folder).unwrap();

    let results_file: File = File::create(results_folder.join(RESULTS_FILE)).unwrap();
    let mut results_writer: BufWriter<File> = BufWriter::new(results_file);

    writeln!(results_writer, "distance,velocity,root").unwrap();

    for (distance, velocity, root) in results {
        writeln!(results_writer, "{},{},{}", distance, velocity, root).unwrap();
    }

    results_writer.flush().unwrap();

    println!(
        "Results written to {} in the {} folder.",
        RESULTS_FILE, RESULTS_FOLDER
    );
}

/// An implementation of the simulation which solves each distance and velocity combination
/// on a single thread and returns the results as a [Vec<(f64, f64, f64)>].
fn simulate_single_cpu() -> Vec<(f64, f64, f64)> {
    println!("Single-threaded CPU simulation starting...");

    // This creates a Vec<(f64, f64, f64)> that can hold TOTAL_STEPS entries before reallocating.
    // Since there will usually be impossible simulations, all of the space allocated will not be used.
    let mut results: Vec<(f64, f64, f64)> = Vec::with_capacity(TOTAL_STEPS as usize);
    let mut incompleteable: u64 = 0;

    let start: Instant = Instant::now();

    for i in 0..DISTANCE_STEPS {
        // The distance and velocity are rounded to the nearest step size to help with imprecision.
        let distance: f64 =
            ((DISTANCE_STEP * i as f64) + DISTANCE_MIN).round_nearest(DISTANCE_STEP);

        for j in 0..VELOCITY_STEPS {
            let velocity: f64 =
                ((VELOCITY_STEP * j as f64) + VELOCITY_MIN).round_nearest(VELOCITY_STEP);

            if let Some((root, _)) = solve(0.0, 0.0, distance, 2.1, velocity, integrate) {
                results.push((distance, velocity, root));
            } else {
                incompleteable += 1;
            }

            let progress_factor: f64 = (i * VELOCITY_STEPS + j + 1) as f64 / TOTAL_STEPS as f64;
            let progress_percent: f64 = progress_factor * 100.0;

            let progress_complete: String = PROGRESS_CHARACTER
                .repeat((PROGRESS_LENGTH as f64 * progress_factor).floor() as usize);

            let progress_incomplete: String = PROGRESS_CHARACTER
                .repeat((PROGRESS_LENGTH as f64 * (1.0 - progress_factor)).ceil() as usize);

            // \r goes to the start of the line, \x1b[2K clears it, \x1b[0m sets the color back to the default, and \x1b[?25l hides the cursor.
            print!(
                "\r\x1b[2K{}{}{}{}\x1b[0m {:.4?} | {:.2}%\x1b[?25l",
                COMPLETE_COLOR,
                progress_complete,
                INCOMPLETE_COLOR,
                progress_incomplete,
                start.elapsed(),
                progress_percent
            );

            stdout().flush().unwrap();
        }
    }

    // \n goes to a new line and \x1b[?25h shows the cursor.
    println!(
        "\nSingle-threaded CPU simulation finished with {} incompleteable simulations.\x1b[?25h",
        incompleteable
    );

    results
}

#[allow(dead_code)]
/// An implementation of the simulation which solves each distance and velocity combination
/// on multiple threads and returns the results as a [Vec<(f64, f64, f64)>].
fn simulate_multi_cpu() -> Vec<(f64, f64, f64)> {
    todo!("Implement multithreaded CPU simulation (with rayon).");
}

#[allow(dead_code)]
/// An implementation of the simulation which solves each distance and velocity combination
/// on multiple threads on the GPU and returns the results as a [Vec<(f64, f64, f64)>].
fn simulate_multi_gpu() -> Vec<(f64, f64, f64)> {
    todo!("Implement multithreaded GPU simulation.");
}
