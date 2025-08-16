use crate::ConfidenceInterval;
use crate::sample::prob_by;

pub fn epsilon(n: usize, alpha: f64) -> f64 {
    f64::sqrt((0.5 / n as f64) * f64::ln(2.0 / alpha))
}

pub fn cdf_upper(sample: &[f64], x: f64, epsilon: f64) -> f64 {
    let empirical_cdw = prob_by(sample, |v| v.total_cmp(&x));
    f64::min(empirical_cdw + epsilon, 1.0)
}

pub fn cdf_lower(sample: &[f64], x: f64, epsilon: f64) -> f64 {
    let empirical_cdw = prob_by(sample, |v| v.total_cmp(&x));
    f64::max(0.0, empirical_cdw - epsilon)
}

pub fn ci_lower(sample: &[f64], epsilon: f64) -> f64 {
    assert!(sample.len() >= 2);
    let first = sample[0] * cdf_upper(sample, sample[0], epsilon);
    let last = sample[sample.len() - 1] * (1.0 - cdf_upper(sample, sample[sample.len() - 2], epsilon));
    let mut others = 0.0;
    for index in 1..=sample.len() - 2 {
        others += sample[index] * (cdf_upper(sample, sample[index], epsilon) - cdf_upper(sample, sample[index - 1], epsilon))
    }
    first + last + others
}

pub fn ci_upper(sample: &[f64], epsilon: f64) -> f64 {
    assert!(sample.len() >= 2);
    let first = sample[0] * cdf_lower(sample, sample[0], epsilon);
    let last = sample[sample.len() - 1] * (1.0 - cdf_lower(sample, sample[sample.len() - 2], epsilon));
    let mut others = 0.0;
    for index in 1..=sample.len() - 2 {
        others += sample[index] * (cdf_lower(sample, sample[index], epsilon) - cdf_lower(sample, sample[index - 1], epsilon))
    }
    first + last + others
}

pub fn ci(sample: &[f64], alpha: f64) -> ConfidenceInterval {
    let epsilon = epsilon(sample.len(), alpha);
    let lower = ci_lower(sample, epsilon);
    let upper = ci_upper(sample, epsilon);
    ConfidenceInterval { lower, upper }
}