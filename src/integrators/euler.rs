use crate::constants::physics::{GRAVITY, TIMESTEP};
use super::physics_helpers::*;

// TODO: The stopping conditions need to be fixed.
// For example, both 0 and 90 degrees will always have the same error (y2) because they both hit the ground at x = 0.0.
// For more complex solvers, this isn't solveable because they seem to both be on the same sides of the root, even though they aren't.
//
// Currently, indirect trajectories also do not work. There also needs to be a way to add polygons into the simulation that the projectile has to go around.

/// An unoptimized/reference implementation of Euler integration.
fn euler(angle: f64, velocity: f64, mut x1: f64, mut y1: f64, x2: f64, y2: f64) -> f64 {
    let (mut x_velocity, mut y_velocity) = get_initial_velocities(angle, velocity);
    let (x_drag_factor, y_drag_factor) = get_drag_factors(angle);

    while (x1 < x2) && (y1 >= 0.0) {
        let dx_velocity: f64 = x_velocity.powi(2) * x_drag_factor;
        let dy_velocity: f64 = GRAVITY + y_velocity.powi(2) * y_drag_factor;

        x_velocity -= dx_velocity * TIMESTEP;
        y_velocity -= dy_velocity * TIMESTEP;

        x1 += x_velocity * TIMESTEP;
        y1 += y_velocity * TIMESTEP;
    }

    y1 - y2
}

#[must_use] pub fn integrate(angle: f64, velocity: f64, x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    euler(angle, velocity, x1, y1, x2, y2)
}
