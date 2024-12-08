pub mod physics {
	pub const GRAVITY: f64 = 9.8; // The downward acceleration due to gravity, m/s^2.
	pub const AIR_DENSITY: f64 = 1.293; // The density of the fluid (air) around the projectile, kg/m^3.
	pub const TIMESTEP: f64 = 2e-5; // The amount of time the integrator steps through each iteration. in s.

	pub mod projectile {
		pub const MASS: f64 = 0.75; // The mass of the projectile, kg.

		pub const X_SURFACE_AREA: f64 = 0.0175; // The surface area of the horizontal facing cross section, m^2.
		pub const Y_SURFACE_AREA: f64 = 0.0486; // The surface area of the vertical facing cross section, m^2.

		pub const X_DRAG_COEFFICIENT: f64 = 1.875; // The arbitrary drag coefficient of the horizontal facing cross section.
		pub const Y_DRAG_COEFFICIENT: f64 = 2.5; // The arbitrary drag coefficient of the vertical facing cross section.
		pub const DRAG_FACTOR_CONSTANT: f64 = super::AIR_DENSITY / (MASS * 2.0);
	}
}

pub mod simulation {
	pub const DISTANCE_MIN: f64 = 0.0; // The minimum distance that will be simulated, m.
	pub const DISTANCE_MAX: f64 = 8.0; // The maximum distance that will be simulated, m.
	pub const DISTANCE_STEP: f64 = 0.1; // The distance the simulation steps through each simulation, m.
	pub const DISTANCE_STEPS: u32 = 1 + ((DISTANCE_MAX - DISTANCE_MIN) / DISTANCE_STEP) as u32;

	pub const VELOCITY_MIN: f64 = 7.5; // The minimum velocity that will be simulated, m/s.
	pub const VELOCITY_MAX: f64 = 12.5; // The maximum velocity that will be simulated, m/s.
	pub const VELOCITY_STEP: f64 = 0.1; // The velocity the simulation steps through each simulation, m/s.
	pub const VELOCITY_STEPS: u32 = 1 + ((VELOCITY_MAX - VELOCITY_MIN) / VELOCITY_STEP) as u32;

	pub const TOTAL_STEPS: u32 = DISTANCE_STEPS * VELOCITY_STEPS;

	pub mod progress_bar {
		pub const LENGTH: u32 = 50;
		pub const CHARACTER: &str = "━";

		pub const COMPLETE_COLOR: &str = "\x1b[32m"; // Green Hex ANSI
		pub const INCOMPLETE_COLOR: &str = "\x1b[38;5;235m"; // Dark gray, Hexadecimal ANSI.
	}
}

pub mod solver {
	pub const TOLERANCE: f64 = 1e-4; // Absolute tolerance for error, m.
	pub const LIMIT: u32 = 64; // Limit to find a solution before considering the given simulation impossible.
}
