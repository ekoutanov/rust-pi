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
            None => write!(f, "[{}, {}], x\u{0304} = {mean} ± {span:}", self.lower, self.upper),
            Some(precision) => write!(f, "[{:.precision$}, {:.precision$}], x\u{0304} = {mean:.precision$} ± {span:.precision$}", self.lower, self.upper)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ConfidenceInterval;

    #[test]
    fn display() {
        let ci = ConfidenceInterval {
            lower: 12.35,
            upper: 34.85,
        };
        assert_eq!("[12.35, 34.85], x\u{0304} = 23.6 ± 11.25", format!("{ci}"));
    }

    #[test]
    fn display_with_precision() {
        let ci = ConfidenceInterval {
            lower: 12.35,
            upper: 34.85,
        };
        assert_eq!("[12.3, 34.9], x\u{0304} = 23.6 ± 11.2", format!("{ci:.1}"));
    }
}