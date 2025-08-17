//! Computes the confidence interval for a normally distributed sample.

use statrs::distribution::{ContinuousCDF, Normal};
use crate::ConfidenceInterval;

pub fn ci(mean: f64, std_dev: f64, n: usize, alpha: f64) -> ConfidenceInterval {
    let z = Normal::standard().inverse_cdf(1.0 - alpha / 2.0);
    let span = z * std_dev / f64::sqrt(n as f64);
    ConfidenceInterval {
        lower: mean - span,
        upper: mean + span,
    }
}

#[cfg(test)]
mod tests {
    use assert_float_eq::assert_float_absolute_eq;

    #[test]
    fn ci() {
        let ci = super::ci(22.8, 2.7, 100, 0.05);
        println!("ci: {ci}");
        assert_float_absolute_eq!(22.270_809, ci.lower);
        assert_float_absolute_eq!(23.329_190, ci.upper);
    }
}