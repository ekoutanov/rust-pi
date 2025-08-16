//! Use of the Dvoretzky–Kiefer–Wolfowitz inequality to estimate confidence intervals for 
//! nonparametric distributions.

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

#[cfg(test)]
mod tests {
    use assert_float_eq::assert_float_absolute_eq;

    fn generate_sample(ratings: Vec<(f64, usize)>) -> Vec<f64> {
        let count = ratings.iter().map(|(_, count)| count).sum();
        let mut sample = Vec::with_capacity(count);
        for (rating, count) in ratings {
            for _ in 0..count {
                sample.push(rating);
            }
        }
        sample
    }

    #[test]
    fn ci_mixed() {
        let mut sample = generate_sample(vec![(1.0, 0), (2.0, 3), (3.0, 9), (4.0, 53), (5.0, 144)]);
        sample.sort_unstable_by(|a, b| a.total_cmp(b));
        let ci_05 = super::ci(&sample, 0.05);
        println!("mean: {:.9}", sample.iter().sum::<f64>() / sample.len() as f64);
        println!("ci_05: {ci_05}");
        assert_float_absolute_eq!(4.335_399, ci_05.lower);
        assert_float_absolute_eq!(4.782_937, ci_05.upper);
    }

    #[test]
    fn ci_single() {
        let mut sample = generate_sample(vec![(12.0, 10)]);
        sample.sort_unstable_by(|a, b| a.total_cmp(b));
        let ci_05 = super::ci(&sample, 0.05);
        println!("mean: {:.9}", sample.iter().sum::<f64>() / sample.len() as f64);
        println!("ci_05: {ci_05}");
        assert_float_absolute_eq!(12.0, ci_05.lower);
        assert_float_absolute_eq!(12.0, ci_05.upper);
    }

    #[test]
    fn ci_max_uncertainty() {
        let mut sample = generate_sample(vec![(1.0, 1), (2.0, 1)]);
        sample.sort_unstable_by(|a, b| a.total_cmp(b));
        let ci_05 = super::ci(&sample, 0.05);
        println!("mean: {:.9}", sample.iter().sum::<f64>() / sample.len() as f64);
        println!("ci_05: {ci_05}");
        assert_float_absolute_eq!(1.0, ci_05.lower);
        assert_float_absolute_eq!(2.0, ci_05.upper);
    }
}