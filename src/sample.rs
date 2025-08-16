use std::cmp::Ordering;

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