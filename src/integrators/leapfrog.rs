use crate::constants::physics::{GRAVITY, TIMESTEP};
use super::physics_helpers::*;

/// Leapfrog integration for a "vertical goal":
/// - Tries to reach y2
/// - Returns horizontal error if y2 is reached
/// - Returns diagonal distance to (x2, y2) if ground is hit first
/// again, mostly chatgpt editing on the original version.
fn leapfrog_vertical(
    angle: f64,
    velocity: f64,
    mut x1: f64,
    mut y1: f64,
    x2: f64,
    y2: f64,
) -> f64 {
    let (mut x_velocity, mut y_velocity) = get_initial_velocities(angle, velocity);
    let (x_drag_factor, y_drag_factor) = get_drag_factors(angle);

    let mut dx_velocity = x_velocity.powi(2) * x_drag_factor;
    let mut dy_velocity = f64::mul_add(y_velocity.powi(2), y_drag_factor, GRAVITY);

    loop {
        // Store previous position for interpolation
        let prev_x = x1;
        let prev_y = y1;

        // Half step velocity update
        x_velocity -= dx_velocity * (TIMESTEP * 0.5);
        y_velocity -= dy_velocity * (TIMESTEP * 0.5);

        // Full step position update
        x1 += x_velocity * TIMESTEP;
        y1 += y_velocity * TIMESTEP;

        // Check for crossing y = y2 (goal height)
        if (prev_y - y2) * (y1 - y2) <= 0.0 && prev_y != y1 {
            let t = (y2 - prev_y) / (y1 - prev_y);
            let x_hit = prev_x + t * (x1 - prev_x);
            return x_hit - x2;
        }

        // Check for ground hit
        if y1 <= 0.0 {
            let t = if prev_y != y1 {
                (0.0 - prev_y) / (y1 - prev_y)
            } else {
                0.0
            };
            let x_hit = prev_x + t * (x1 - prev_x);

            let dx = x_hit - x2;
            let dy = -y2;
            return (dx * dx + dy * dy).sqrt();
        }

        // Recompute accelerations
        dx_velocity = x_velocity.powi(2) * x_drag_factor;
        dy_velocity = f64::mul_add(y_velocity.powi(2), y_drag_factor, GRAVITY);

        // Second half step velocity update
        x_velocity -= dx_velocity * (TIMESTEP * 0.5);
        y_velocity -= dy_velocity * (TIMESTEP * 0.5);
    }
}

pub fn integrate(
    angle: f64,
    velocity: f64,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
) -> f64 {
    leapfrog_vertical(angle, velocity, x1, y1, x2, y2)
}
