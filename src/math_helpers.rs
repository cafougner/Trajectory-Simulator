impl MathHelpers for f64 {
    /// Round to the nearest multiple of a.
    fn round_nearest(self, a: f64) -> Self {
        assert!(!self.is_nan());
        debug_assert_ne!(a, 0.0); // This should always be a programming error.

        // This can be simplified from `(self * (1.0 / a)).round() / (1.0 / a)` to `(self / a).round() * a`,
        // but then the result is not correct. This could also just be a CPU specific problem as well.
        let factor: f64 = 1.0 / a;
        (self * factor).round() / factor
    }
}

pub trait MathHelpers {
    fn round_nearest(self, a: f64) -> f64;
}
