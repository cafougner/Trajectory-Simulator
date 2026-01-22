use crate::constants::physics::{GRAVITY, TIMESTEP};
use super::physics_helpers::*;

/// Euler integration that ONLY reports error when the projectile
/// FALLS onto y = y2. All other trajectories return NaN.
fn euler(
    angle: f64,
    velocity: f64,
    mut x1: f64,
    mut y1: f64,
    x2: f64,
    y2: f64,
) -> f64 {
    let (mut x_velocity, mut y_velocity) = get_initial_velocities(angle, velocity);
    let (x_drag_factor, y_drag_factor) = get_drag_factors(angle);

    loop {
        let prev_x = x1;
        let prev_y = y1;
        let prev_y_velocity = y_velocity;

        let dx_velocity = -x_drag_factor * x_velocity.abs() * x_velocity;
        let dy_velocity = -y_drag_factor * y_velocity.abs() * y_velocity - GRAVITY;

        x_velocity += dx_velocity * TIMESTEP;
        y_velocity += dy_velocity * TIMESTEP;

        x1 += x_velocity * TIMESTEP;
        y1 += y_velocity * TIMESTEP;

        // Crossing y = y2 while falling
        if (prev_y - y2) * (y1 - y2) <= 0.0
            && prev_y != y1
            && prev_y_velocity < 0.0
        {
            let t = (y2 - prev_y) / (y1 - prev_y);
            let x_hit = prev_x + t * (x1 - prev_x);
            return x_hit - x2;
        }

        // Hit ground without valid descending intersection
        if y1 <= 0.0 {
            return f64::NAN;
        }
    }
}

#[must_use]
pub fn integrate(
    angle: f64,
    velocity: f64,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
) -> f64 {
    euler(angle, velocity, x1, y1, x2, y2)
}
