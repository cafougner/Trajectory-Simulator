impl MathHelpers for f64 {
    /// Round to the nearest multiple of a.
    fn round_nearest(self, a: f64) -> Self {
        assert!(a != 0.0);

        let factor: f64 = 1.0 / a;
        (self * factor).round() / factor
    }
}

pub trait MathHelpers {
    fn round_nearest(self, a: f64) -> Self;
}
