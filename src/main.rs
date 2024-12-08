use std::env;

use simulation_helpers::*;

pub mod constants;
pub mod integrators;
pub mod math_helpers;
pub mod simulation_helpers;
pub mod solvers;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() > 1); // The first entry should be the executable path.

    let mode: &str = args[1].as_str();

    match mode {
        "run-simulation" => run_simulation(),
        "estimate-coefficients" => estimate_coefficients(),
        _ => panic!("Invalid mode: '{mode}'")
    }
}

fn run_simulation() {
    // When multi-threaded and GPU simulations are implemented, the logic to decide which to use should be here.
    let results: Vec<(f64, f64, f64)> = simulate_single_cpu();

    write_results(results);
}

fn estimate_coefficients() {
    unimplemented!("'estimate-coefficients' mode unimplemented.");
}
