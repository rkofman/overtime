use crate::units::{Minutes, Cents};

pub struct WageSummary {
    regular_time: Minutes,
    overtime: Minutes,
    regular_wages: Cents,
    overtime_wages: Cents
}

impl WageSummary {
    pub fn new(
        regular_time: Minutes, overtime: Minutes, regular_wages: Cents, overtime_wages: Cents) -> Self {
        WageSummary{regular_time, overtime, regular_wages, overtime_wages}
    }
}