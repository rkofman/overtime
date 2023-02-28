use crate::units::{Minutes, Cents};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct WageSummary {
    pub regular_time: Minutes,
    pub overtime: Minutes,
    pub regular_wages: Cents,
    pub overtime_wages: Cents
}

impl WageSummary {
    pub fn new(regular_time: Minutes, overtime: Minutes, regular_wages: Cents, overtime_wages: Cents) -> Self {
        WageSummary{regular_time, overtime, regular_wages, overtime_wages}
    }

    pub fn empty() -> Self {
        WageSummary::new(Minutes(0), Minutes(0), Cents(0), Cents(0))
    }
}