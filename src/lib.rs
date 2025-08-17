use std::fmt::{Display, Formatter};

pub mod dkw;
pub mod normal;
pub mod sample;

#[derive(Debug)]
pub struct ConfidenceInterval {
    pub lower: f64,
    pub upper: f64,
}

impl Display for ConfidenceInterval {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mean = (self.lower + self.upper) / 2.0;
        let span = self.upper - mean;
        match f.precision() {
            None => write!(f, "[{}, {}], \u{0078}\u{0304} = {mean} ± {span:}", self.lower, self.upper),
            Some(precision) => write!(f, "[{:.precision$}, {:.precision$}], \u{0078}\u{0304} = {mean:.precision$} ± {span:.precision$}", self.lower, self.upper)
        }
    }
}