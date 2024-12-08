use std::{path::Path, time::Instant};
use std::{fs::{create_dir_all, File}, io::{stdout, BufWriter, Write}};

use crate::constants::simulation::{*, progress_bar::*};
use crate::integrators::leapfrog::integrate;
use crate::math_helpers::MathHelpers;
use crate::solvers::bisection::solve;

/// Writes a [Vec<(f64, f64, f64)>] to the 'results.csv' in the 'results' folder with a 'distance,velocity root' header.
pub fn write_results(results: Vec<(f64, f64, f64)>) {
    let results_folder_path: &Path = Path::new("results");
    create_dir_all(results_folder_path).unwrap();

    let results_file: File = File::create(results_folder_path.join("results.csv")).unwrap();
    let mut results_writer: BufWriter<File> = BufWriter::new(results_file);

    writeln!(results_writer, "distance,velocity,root").unwrap();

    for (distance, velocity, root) in results {
        writeln!(results_writer, "{distance},{velocity},{root}").unwrap();
    }

    results_writer.flush().unwrap();
    println!("Results written to the results.csv in the results folder successfully.");
}

/// A full simulation implementation which runs single-threaded on the CPU.
pub fn simulate_single_cpu() -> Vec<(f64, f64, f64)> {
    println!("Single-threaded CPU simulation running...");

    // There will usually be some incomplete simulations, so the full size will not always be used.
    let mut results: Vec<(f64, f64, f64)> = Vec::with_capacity(TOTAL_STEPS as usize);
    let mut incomplete: u32 = 0;

    let start: Instant = Instant::now();

    for i in 0..DISTANCE_STEPS {
        let distance: f64 = ((DISTANCE_STEP * f64::from(i)) + DISTANCE_MIN).round_nearest(DISTANCE_STEP);

        for j in 0..VELOCITY_STEPS {
            let velocity: f64 = ((VELOCITY_STEP * f64::from(j)) + VELOCITY_MIN).round_nearest(VELOCITY_STEP);

            if let Some((root, _)) = solve(0.0, 0.0, distance, 2.1, velocity, integrate) {
                results.push((distance, velocity, root));
            }

            else {
                incomplete += 1;
            }

            // Mostly ChatGPT
            let progress_factor: f64 = f64::from(i * VELOCITY_STEPS + j + 1) / f64::from(TOTAL_STEPS);
            let progress_percent: f64 = progress_factor * 100.0;

            let progress_complete: String = CHARACTER.repeat((f64::from(LENGTH) * progress_factor).floor() as usize);
            let progress_incomplete: String = CHARACTER.repeat((f64::from(LENGTH) * (1.0 - progress_factor)).ceil() as usize);

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

    println!("\nSimulation finished with {incomplete} simulations.\x1b[?25h");
    results
}

/// An unimplemented full simulation implementation which runs multi-threaded on the CPU.
pub fn simulate_multi_cpu() -> Vec<(f64, f64, f64)> {
    unimplemented!("Multi-threaded CPU simulation unimplemented.")
}

/// An unimplemented full simulation implementation which runs on the GPU.
pub fn simulate_gpu() -> Vec<(f64, f64, f64)> {
    unimplemented!("GPU simulation unimplemented.")
}
