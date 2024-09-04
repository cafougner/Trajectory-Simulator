pub mod fitting_constants {
    pub const DISTANCE_MIN: f64 = 0.0; // The minimum distance that will be simulated, in m.
    pub const DISTANCE_MAX: f64 = 8.0; // The maximum distance that will be simulated, in m.
    pub const DISTANCE_STEP: f64 = 0.1; // The size of steps between the minmum and maximum distances, in m.
    pub const DISTANCE_STEPS: u64 = 1 + ((DISTANCE_MAX - DISTANCE_MIN) / DISTANCE_STEP) as u64;

    pub const VELOCITY_MIN: f64 = 7.5; // The minimum velocity that will be simulated, in m/s.
    pub const VELOCITY_MAX: f64 = 12.5; // The maximum velocity that will be simulated, in m/s.
    pub const VELOCITY_STEP: f64 = 0.1; // The size of steps between the minmum and maximum velocities, in m/s.
    pub const VELOCITY_STEPS: u64 = 1 + ((VELOCITY_MAX - VELOCITY_MIN) / VELOCITY_STEP) as u64;

    pub const TOTAL_STEPS: u64 = DISTANCE_STEPS * VELOCITY_STEPS;

    // These need to be changed in the results_analyzer.py, the results_equation_generator.py, and the .gitignore as well.
    pub const RESULTS_FOLDER: &str = "results";
    pub const RESULTS_FILE: &str = "results.csv";
}

pub mod physics_constants {
    pub const GRAVITY: f64 = 9.8; // The downward acceleration of gravity, in m/s^2.
    pub const AIR_DENSITY: f64 = 1.293; // The density of air, in kg/m^3.
    pub const PROJECTILE_MASS: f64 = 0.75; // The mass of the projectile, in kg.

    pub const X_SURFACE_AREA: f64 = 0.0175; // The surface area of the horizontal facing cross section, in m^2.
    pub const Y_SURFACE_AREA: f64 = 0.0486; // The surface area of the vertical facing cross section, in m^2.
    pub const X_DRAG_COEFFICIENT: f64 = 1.875; // The arbitrary drag coefficient for the horizontal facing cross section.
    pub const Y_DRAG_COEFFICIENT: f64 = 2.5; // The arbitrary drag coefficient for the vertical facing cross section.

    pub const SOLVER_TOLERANCE: f64 = 1e-4; // The +- tolerance for error for the solver, in m.
    pub const SOLVER_TIMESTEP: f64 = 1e-5; // The amount of time the solver steps through for every iteration, in s.
    pub const SOLVER_LIMIT: u64 = 64; // The limit for the solver to find a solution before the simulation is considered impossible.
}

pub mod progress_bar_constants {
    pub const PROGRESS_CHARACTER: &str = "━";
    pub const PROGRESS_LENGTH: u64 = 50;

    pub const COMPLETE_COLOR: &str = "\x1b[32m"; // Green, Hexadecimal ANSI.
    pub const INCOMPLETE_COLOR: &str = "\x1b[38;5;235m"; // Dark gray, Hexadecimal ANSI.
}
