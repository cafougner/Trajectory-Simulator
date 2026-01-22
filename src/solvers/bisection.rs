use crate::constants::solver::*;

pub fn solve(
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    velocity: f64,
    integrate: fn(f64, f64, f64, f64, f64, f64) -> f64,
) -> Option<(f64, f64)> {
    const SCAN_STEP: f64 = 0.25;

    // ------------------------------------------------------------
    // 1. Scan entire angle range and collect ALL valid brackets
    // ------------------------------------------------------------
    let mut brackets: Vec<(f64, f64)> = Vec::new();

    let mut prev_angle = 0.0;
    let mut prev_error = integrate(prev_angle, velocity, x1, y1, x2, y2);

    let mut angle = prev_angle + SCAN_STEP;
    while angle <= 90.0 {
        let err = integrate(angle, velocity, x1, y1, x2, y2);

        if prev_error.is_finite()
            && err.is_finite()
            && prev_error.signum() != err.signum()
        {
            brackets.push((prev_angle, angle));
        }

        prev_angle = angle;
        prev_error = err;
        angle += SCAN_STEP;
    }

    // ------------------------------------------------------------
    // 2. Choose the bracket with the HIGHEST angle
    // ------------------------------------------------------------
    let (mut a, mut b) = brackets
        .into_iter()
        .max_by(|(_, b1), (_, b2)| b1.partial_cmp(b2).unwrap())?;

    // ------------------------------------------------------------
    // 3. Standard bisection inside that bracket
    // ------------------------------------------------------------
    let mut a_error = integrate(a, velocity, x1, y1, x2, y2);
    let mut b_error = integrate(b, velocity, x1, y1, x2, y2);

    let mut iterations = 0;

    while iterations < LIMIT {
        let c = 0.5 * (a + b);
        let c_error = integrate(c, velocity, x1, y1, x2, y2);

        if c_error.is_finite() && c_error.abs() < FINAL_TOLERANCE {
            return Some((c, c_error));
        }

        if c_error.is_finite() && a_error.signum() == c_error.signum() {
            a = c;
            a_error = c_error;
        } else {
            b = c;
            b_error = c_error;
        }

        iterations += 1;
    }

    let best = if a_error.abs() < b_error.abs() {
        (a, a_error)
    } else {
        (b, b_error)
    };

    Some(best)
}
