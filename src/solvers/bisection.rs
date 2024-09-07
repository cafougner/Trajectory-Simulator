use crate::constants::solver_constants::{SOLVER_LIMIT, SOLVER_TOLERANCE};

/// A reference implementation of the bisection method to find the angle with the lowest error.
///
/// See [Mathematical Python](https://patrickwalls.github.io/mathematicalpython/root-finding/bisection/).
pub fn solve(
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    velocity: f64,
    integrate: fn(f64, f64, f64, f64, f64, f64) -> f64,
) -> Option<(f64, f64)> {
    let mut a: f64 = 0.0;
    let mut b: f64 = 90.0;

    let mut a_error: f64 = integrate(a, velocity, x1, y1, x2, y2);
    let mut b_error: f64 = integrate(b, velocity, x1, y1, x2, y2);
    let mut iterations: u64 = 0;

    while (a_error.abs() > SOLVER_TOLERANCE)
        && (b_error.abs() > SOLVER_TOLERANCE)
        && (iterations < SOLVER_LIMIT)
    {
        let c: f64 = (a + b) / 2.0;
        let c_error: f64 = integrate(c, velocity, x1, y1, x2, y2);

        // Since c is the midpoint of a and b, if a and c are on the same side of the root, the range is reduced to b-c.
        if a_error.signum() != c_error.signum() {
            b = c;
            b_error = c_error;
        } else {
            a = c;
            a_error = c_error;
        }

        iterations += 1;
    }

    if a_error.abs() < SOLVER_TOLERANCE {
        Some((a, a_error))
    } else if b_error.abs() < SOLVER_TOLERANCE {
        Some((b, b_error))
    } else {
        None
    }
}
