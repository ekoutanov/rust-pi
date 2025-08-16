use std::fmt::{Display, Formatter};

pub mod dkw;
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
        write!(f, "[{:.9}, {:.9}], \u{0078}\u{0304} = {mean:.9} ± {span:.9}", self.lower, self.upper)
    }
}