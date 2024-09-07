use super::physics_helpers::{calculate_drag_factors, calculate_initial_velocities};
use crate::constants::physics_constants::{GRAVITY, PHYSICS_TIMESTEP};

/// A mostly optimized reference implementation of Leapfrog integration.
/// Compared to Euler integration, this gives roughly double the accuracy in half the time.
fn leapfrog(angle: f64, velocity: f64, mut x1: f64, mut y1: f64, x2: f64, y2: f64) -> f64 {
    let (mut x_velocity, mut y_velocity) = calculate_initial_velocities(angle, velocity);
    let (x_drag_factor, y_drag_factor) = calculate_drag_factors(angle);

    let mut dx_velocity: f64 = x_velocity.powi(2) * x_drag_factor;
    let mut dy_velocity: f64 = f64::mul_add(y_velocity.powi(2), y_drag_factor, GRAVITY);

    while (x1 < x2) && (y1 >= 0.0) {
        x_velocity -= dx_velocity * (PHYSICS_TIMESTEP * 0.5);
        y_velocity -= dy_velocity * (PHYSICS_TIMESTEP * 0.5);

        x1 += x_velocity * PHYSICS_TIMESTEP;
        y1 += y_velocity * PHYSICS_TIMESTEP;

        dx_velocity = x_velocity.powi(2) * x_drag_factor;
        dy_velocity = f64::mul_add(y_velocity.powi(2), y_drag_factor, GRAVITY);

        x_velocity -= dx_velocity * (PHYSICS_TIMESTEP * 0.5);
        y_velocity -= dy_velocity * (PHYSICS_TIMESTEP * 0.5);
    }

    y1 - y2
}

#[allow(dead_code)]
pub fn integrate(angle: f64, velocity: f64, x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    leapfrog(angle, velocity, x1, y1, x2, y2)
}
