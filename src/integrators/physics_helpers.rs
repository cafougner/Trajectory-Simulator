use crate::constants::physics_constants::{
    AIR_DENSITY, PROJECTILE_MASS, X_DRAG_COEFFICIENT, X_SURFACE_AREA, Y_DRAG_COEFFICIENT,
    Y_SURFACE_AREA,
};

/// Returns an [f64] for an x_velocity and a y_velocity for the initial velocities for a given angle.
pub fn calculate_initial_velocities(angle: f64, velocity: f64) -> (f64, f64) {
    // Sine and Cosine are used so that the distance travelled in one second is always the velocity,
    // so that the actual distance along the angle travelled in one second is the velocity. This is
    // not done for the drag factors, since that is just interpolation and not an actual velocity.
    let angle: f64 = angle.to_radians();

    let x_weight: f64 = f64::cos(angle);
    let y_weight: f64 = f64::sin(angle);

    let x_velocity: f64 = velocity * x_weight;
    let y_velocity: f64 = velocity * y_weight;

    (x_velocity, y_velocity)
}

/// Returns an [f64] for an x_drag-factor and a y_drag_factor for a given angle.
///
/// The [drag equation](https://en.wikipedia.org/wiki/Drag_equation) is F = 1/2pu^2 * c_d * A,
/// and can be simplified to a constant factor (for an angle), where F = C * v^2 for x and y.
///
/// This assumes that:
/// * The angle of the projectile never changes throughout a single trajectory.
/// * The transition between the horizontal and vertical surfaces (and their drag forces) is linear.
pub fn calculate_drag_factors(angle: f64) -> (f64, f64) {
    let y_weight: f64 = angle / 90.0;
    let x_weight: f64 = 1.0 - y_weight;

    let x_drag_coefficient: f64 = f64::mul_add(X_DRAG_COEFFICIENT, x_weight, Y_DRAG_COEFFICIENT * y_weight);
    let x_surface_area: f64 = f64::mul_add(X_SURFACE_AREA, x_weight, Y_SURFACE_AREA * y_weight);
    let x_drag_factor: f64 =
        x_drag_coefficient * x_surface_area * (AIR_DENSITY / (PROJECTILE_MASS * 2.0));

    let y_drag_coefficient: f64 = f64::mul_add(X_DRAG_COEFFICIENT, y_weight, Y_DRAG_COEFFICIENT * x_weight);
    let y_surface_area: f64 = f64::mul_add(X_SURFACE_AREA, y_weight, Y_SURFACE_AREA * x_weight);
    let y_drag_factor: f64 =
        y_drag_coefficient * y_surface_area * (AIR_DENSITY / (PROJECTILE_MASS * 2.0));

    (x_drag_factor, y_drag_factor)
}
