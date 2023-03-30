use std::fmt;

use chrono::{DateTime};
use chrono_tz::Tz;

use crate::units::Minutes;
use crate::units::hourly_rate::HourlyRate;

#[derive(Debug, Clone)]
pub struct TimecardRangeError;

impl fmt::Display for TimecardRangeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Timecard end must be after (or equal to) start.")
    }
}

#[derive(Debug)]
// #[derive(PartialEq)]
pub struct Timecard {
    start: DateTime<Tz>,
    end: DateTime<Tz>,
    pub hourly_rate: HourlyRate
}

impl Timecard {
    pub fn new(start: DateTime<Tz>, end: DateTime<Tz>, hourly_rate: HourlyRate) -> Result<Self, TimecardRangeError> {
        if start > end {
            return Err(TimecardRangeError);
        }
        Ok(Timecard { start, end, hourly_rate })
    }

    pub fn minutes_worked(&self) -> Minutes {
        Minutes((self.end - self.start).num_minutes())
    }
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;
    use chrono_tz::US::Pacific;

    use crate::timecard::*;

    #[test]
    fn test_creation() {

        let tc = Timecard::new(
            Pacific.with_ymd_and_hms(2022, 10, 12, 9, 0, 0).unwrap(),
            Pacific.with_ymd_and_hms(2022, 10, 12, 10, 30, 0).unwrap(),
            HourlyRate::from_cents_per_hour(2000)
        ).unwrap();
        assert_eq!(tc.start, Pacific.with_ymd_and_hms(2022, 10, 12, 9, 0, 0).unwrap());
        assert_eq!(tc.end, Pacific.with_ymd_and_hms(2022, 10, 12, 10, 30, 0).unwrap());
        assert_eq!(tc.hourly_rate, HourlyRate::from_cents_per_hour(2000));
        assert_eq!(tc.minutes_worked(), Minutes(90));
    }

    #[test]
    fn test_time_range_error() {

        let tc_result = Timecard::new(
            Pacific.with_ymd_and_hms(2022, 10, 12, 9, 0, 0).unwrap(),
            Pacific.with_ymd_and_hms(2022, 10, 12, 8, 30, 0).unwrap(),
            HourlyRate::from_cents_per_hour(2000)
        );
        assert!(tc_result.is_err());
    }
}