use crate::constants::physics::projectile::*;

/// Returns an [f64] for an `x_velocity` and `y_velocity` for a given angle and velocity.
pub fn get_initial_velocities(angle: f64, velocity: f64) -> (f64, f64) {
    let radians: f64 = angle.to_radians();
    let (sin, cos): (f64, f64) = radians.sin_cos();

    let x_velocity: f64 = velocity * cos;
    let y_velocity: f64 = velocity * sin;

    (x_velocity, y_velocity)
}

/// Returns an [f64] for an `x_drag_factor` and `y_drag_factor` for a given angle.
///
/// The [equation for drag](https://en.wikipedia.org/wiki/Drag_equation) (F = 1/2pu^2 * c_d * A)
/// can be simplified to a constant factor (for an angle) multiplied by v^2 (u^2) for the x and y velocities.
///
/// This assumes that:
/// * The angle of the projectile is static throughout any single trajectory.
/// * The transition between the horizontal and vertical surfaces (and their drag coefficients) is linear.
pub fn get_drag_factors(angle: f64) -> (f64, f64) {
    // This just interpolates between the x and y drag coefficients and their surface areas to
    // get x and y drag factors and surface areas for a given angle to get the [de]acceleration
    // from drag. Since the equation solves for a force, it is also divided by the projectile mass.
    let y_weight: f64 = angle * (1.0 / 90.0);

    let x_drag_coefficient: f64 = X_DRAG_COEFFICIENT + (X_DRAG_COEFFICIENT - Y_DRAG_COEFFICIENT) * y_weight;
    let x_surface_area: f64 = X_SURFACE_AREA + (Y_SURFACE_AREA - X_SURFACE_AREA) * y_weight;

    let y_drag_coefficient: f64 = Y_DRAG_COEFFICIENT + (X_DRAG_COEFFICIENT - Y_DRAG_COEFFICIENT) * y_weight;
    let y_surface_area: f64 = Y_SURFACE_AREA + (X_SURFACE_AREA - Y_SURFACE_AREA) * y_weight;

    let x_drag_factor: f64 = x_drag_coefficient * x_surface_area * DRAG_FACTOR_CONSTANT;
    let y_drag_factor: f64 = y_drag_coefficient * y_surface_area * DRAG_FACTOR_CONSTANT;

    // Quick fix that is probably wrong; I noticed that when I increased the drag coefficients, the
    // time for each projectile to hit the ground decreased, like air resistant was applied opposite.
    (-x_drag_factor, -y_drag_factor)
}
