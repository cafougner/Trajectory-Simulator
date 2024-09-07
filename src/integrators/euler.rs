use super::physics_helpers::{calculate_drag_factors, calculate_initial_velocities};
use crate::constants::physics_constants::{GRAVITY, PHYSICS_TIMESTEP};

// TODO: The stopping conditions need to be fixed.
// For example, both 0 and 90 degrees will always have the same error (y2) because they both hit the ground at x = 0.0.
// For more complex solvers, this isn't solveable because they seem to both be on the same sides of the root, even though they aren't.
//
// Currently, indirect trajectories also do not work. There also needs to be a way to add polygons into the simulation that the projectile has to go around.

#[cfg(not(all(target_arch = "x86_64", target_feature = "fma")))]
/// An unoptimized/reference implementation of Euler integration.
fn euler(angle: f64, velocity: f64, mut x1: f64, mut y1: f64, x2: f64, y2: f64) -> f64 {
    let (mut x_velocity, mut y_velocity) = calculate_initial_velocities(angle, velocity);
    let (x_drag_factor, y_drag_factor) = calculate_drag_factors(angle);

    while (x1 < x2) && (y1 >= 0.0) {
        let dx_velocity: f64 = x_velocity.powi(2) * x_drag_factor;
        let dy_velocity: f64 = GRAVITY + y_velocity.powi(2) * y_drag_factor;

        x_velocity -= dx_velocity * PHYSICS_TIMESTEP;
        y_velocity -= dy_velocity * PHYSICS_TIMESTEP;

        x1 += x_velocity * PHYSICS_TIMESTEP;
        y1 += y_velocity * PHYSICS_TIMESTEP;
    }

    y1 - y2
}

#[cfg(all(target_arch = "x86_64", target_feature = "fma"))]
/// An optimized implementation of Euler integration with FMA instructions.
/// This is really unnecessary, but I did it because I can, and it is faster.
///
/// See [the FMA instruction set](https://en.wikipedia.org/wiki/FMA_instruction_set).
fn euler(angle: f64, velocity: f64, mut x1: f64, mut y1: f64, x2: f64, y2: f64) -> f64 {
    use std::arch::asm;

    let (mut x_velocity, mut y_velocity) = calculate_initial_velocities(angle, velocity);
    let (x_drag_factor, y_drag_factor) = calculate_drag_factors(angle);

    while (x1 < x2) && (y1 >= 0.0) {
        unsafe {
            asm!(
                "vmulsd {1}, {1}, {1}", // x_velocity_squared = x_velocity.powi(2)
                "vmulsd {1}, {3}, {1}", // dx_velocity = x_velocity_squared * x_drag_factor

                "vfmadd132sd {1}, {2}, {5}", // x_velocity -= dx_velocity * PHYSICS_TIMESTEP
                "vfmadd231sd {0}, {1}, {4}", // x1 += x_velocity * PHYSICS_TIMESTEP

                inout(xmm_reg) x1 => x1, // 0
                inout(xmm_reg) x_velocity => x_velocity, // 1
                in(xmm_reg) x_velocity, // 2
                in(xmm_reg) x_drag_factor, // 3
                in(xmm_reg) PHYSICS_TIMESTEP, // 4
                in(xmm_reg) -PHYSICS_TIMESTEP // 5
            );

            asm!(
                "vmulsd {1}, {1}, {1}", // y_velocity_squared = y_velocity.powi(2)
                "vfmadd132sd {1}, {4}, {3}", // dy_velocity = GRAVITY + y_velocity_squared * y_drag_factor

                "vfmadd132sd {1}, {2}, {6}", // y_velocity -= dy_velocity * PHYSICS_TIMESTEP
                "vfmadd231sd {0}, {1}, {5}", // y1 += y_velocity * PHYSICS_TIMESTEP

                inout(xmm_reg) y1 => y1, // 0
                inout(xmm_reg) y_velocity => y_velocity, // 1
                in(xmm_reg) y_velocity, // 2
                in(xmm_reg) y_drag_factor, // 3
                in(xmm_reg) GRAVITY, // 4
                in(xmm_reg) PHYSICS_TIMESTEP, // 5
                in(xmm_reg) -PHYSICS_TIMESTEP // 6
            );
        }
    }

    y1 - y2
}

#[allow(dead_code)]
pub fn integrate(angle: f64, velocity: f64, x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    euler(angle, velocity, x1, y1, x2, y2)
}
