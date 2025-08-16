use std::cmp::Ordering;
use statrs::statistics::Statistics;

/// Summary statistics for a sample.
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct SummaryStatistics {
    pub mean: f64,
    pub std_dev: f64,
    pub count: usize,
    pub min: f64,
    pub max: f64,
}

impl<'a> From<&'a [f64]> for SummaryStatistics {
    fn from(sample: &'a [f64]) -> Self {
        Self {
            mean: sample.mean(),
            std_dev: sample.std_dev(),
            count: sample.len(),
            min: sample.min(),
            max: sample.max(),
        }
    }
}

pub fn quantile<T>(ordered: &[T], quantile: f64) -> &T {
    let index = (quantile * ordered.len() as f64) as usize;
    let index = if index == 0 { 0 } else { index - 1 };
    &ordered[index]
}

pub fn prob_by<'a, T: 'a, F: FnMut(&'a T) -> Ordering>(ordered: &'a [T], comp: F) -> f64 {
    match ordered.binary_search_by(comp) {
        Ok(existing) => (existing + 1) as f64 / ordered.len() as f64,
        Err(new) => new as f64 / ordered.len() as f64,
    }
}