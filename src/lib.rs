#![feature(is_some_and)]

mod timecard;
mod wage_summary;
mod units;
mod work_week;

use timecard::Timecard;
use wage_summary::{WageSummary};
use units::{Cents, Minutes};
use work_week::WorkWeekConfig;
pub mod timeclock;


pub fn calculate_wages(timecards: Vec<Timecard>, work_week_config: WorkWeekConfig) -> WageSummary {
    return WageSummary::new(
        Minutes(0), Minutes(0), Cents(0), Cents(0)
    );
}
// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn calculate_basic_wages() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
